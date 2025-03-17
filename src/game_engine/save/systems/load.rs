use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

use super::get_storage_path;

/// System to handle load game requests
pub fn handle_load_game(
    mut event_reader: EventReader<LoadGameEvent>,
    mut commands: Commands,
    config: Res<SaveConfig>,
    mut query_players: Query<(Entity, &mut Player)>,
    mut game_state: Option<ResMut<GameState>>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Loading game from slot: {}", event.slot_name);

        let save_path = get_storage_path(&config, &format!("{}.bin", event.slot_name));

        // Check if the save file exists (only on native platforms)
        #[cfg(not(target_arch = "wasm32"))]
        if !save_path.exists() {
            error!("Save file not found at: {:?}", save_path);
            continue;
        }

        // Create a persistent resource to load the save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(&format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(save_path)
            .default(GameSaveData::default())
            .build();

        match persistent_save {
            Ok(save) => {
                // Get the loaded data
                let save_data = save.clone();

                // Rebuild entity mapping
                let mut index_to_entity = Vec::new();
                let mut existing_player_entities = HashMap::new();

                // Map existing players if possible
                for (entity, player) in query_players.iter() {
                    for saved_player in &save_data.players {
                        if player.name == saved_player.name {
                            existing_player_entities.insert(saved_player.id, entity);
                            break;
                        }
                    }
                }

                // Recreate player entities
                for player_data in &save_data.players {
                    if let Some(&entity) = existing_player_entities.get(&player_data.id) {
                        index_to_entity.push(entity);

                        // Update existing player data
                        if let Ok((_, mut player)) = query_players.get_mut(entity) {
                            player.life = player_data.life;
                            player.mana_pool = player_data.mana_pool.clone();
                        }
                    } else {
                        // Create new player entity
                        let player_entity = commands
                            .spawn((Player {
                                name: player_data.name.clone(),
                                life: player_data.life,
                                mana_pool: player_data.mana_pool.clone(),
                                ..Default::default()
                            },))
                            .id();

                        index_to_entity.push(player_entity);
                    }
                }

                // Handle empty player list case gracefully
                if save_data.players.is_empty() {
                    debug!("Loading a save with no players");
                    // Add a placeholder to index_to_entity for GameState to reference safely
                    if index_to_entity.is_empty() {
                        index_to_entity.push(Entity::PLACEHOLDER);
                    }
                }

                // Restore game state - always attempt to restore basic properties even with empty players
                if let Some(game_state) = &mut game_state {
                    // If there's a corrupted mapping, fall back to basic properties
                    if index_to_entity.is_empty() || index_to_entity.contains(&Entity::PLACEHOLDER)
                    {
                        // Always restore turn number first
                        game_state.turn_number = save_data.game_state.turn_number;

                        // Handle empty turn order case
                        if save_data.game_state.turn_order_indices.is_empty() {
                            debug!("Empty turn order detected, keeping turn order unchanged");
                            // Keep the existing turn order
                        } else {
                            // Create a fallback turn order based on indices
                            let mut turn_order = VecDeque::new();
                            for &idx in &save_data.game_state.turn_order_indices {
                                turn_order.push_back(Entity::from_raw(idx as u32));
                            }
                            game_state.turn_order = turn_order;
                        }
                    } else {
                        // Full restore with valid player entities
                        **game_state = save_data.to_game_state(&index_to_entity);
                    }
                } else {
                    if !index_to_entity.is_empty() {
                        commands.insert_resource(save_data.to_game_state(&index_to_entity));
                    } else {
                        // Even with empty index, we should still restore basic properties
                        let mut default_state = GameState::default();

                        // Only update turn number if turn order is not empty
                        if !save_data.game_state.turn_order_indices.is_empty() {
                            default_state.turn_number = save_data.game_state.turn_number;
                        } else {
                            debug!(
                                "Empty turn order detected when creating default state, keeping turn number unchanged"
                            );
                        }

                        commands.insert_resource(default_state);
                        warn!(
                            "No player entities found when loading game, using modified default state"
                        );
                    }
                }

                // Restore zone contents
                if let Some(zone_manager) = &mut zones {
                    // Use the GameSaveData method to restore ZoneManager
                    **zone_manager = save_data.to_zone_manager(&index_to_entity);
                }

                // Restore commander zone contents
                if let Some(commander_manager) = &mut commanders {
                    // Use the GameSaveData method to restore CommandZoneManager
                    **commander_manager = save_data.to_commander_manager(&index_to_entity);
                }

                info!("Game loaded successfully from slot {}", event.slot_name);
            }
            Err(e) => {
                error!("Failed to load save: {}", e);
            }
        }
    }
}
