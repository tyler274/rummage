use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

/// Helper function to apply a game state to the world
pub fn apply_game_state(
    save_data: &GameSaveData,
    game_state: &mut Option<ResMut<GameState>>,
    commands: &mut Commands,
    query_players: &mut Query<(Entity, &mut Player)>,
    zones: &mut Option<ResMut<ZoneManager>>,
    commanders: &mut Option<ResMut<CommandZoneManager>>,
) {
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

    // Update player entities
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

    // Restore game state
    if let Some(game_state) = game_state.as_mut() {
        // If there's a corrupted mapping, fall back to basic properties
        if index_to_entity.is_empty() || index_to_entity.contains(&Entity::PLACEHOLDER) {
            // At minimum, restore basic properties not tied to player entities
            game_state.turn_number = save_data.game_state.turn_number;

            // For empty player list, set reasonable defaults for player-related fields
            if save_data.game_state.turn_order_indices.is_empty() {
                // Create a fallback turn order
                game_state.turn_order = VecDeque::new();
            }
        } else {
            // Full restore with valid player entities
            **game_state = save_data.to_game_state(&index_to_entity);
        }
    } else {
        if !index_to_entity.is_empty() {
            commands.insert_resource(save_data.to_game_state(&index_to_entity));
        } else {
            commands.insert_resource(GameState::default());
            warn!("No player entities found when loading game, using default state");
        }
    }

    // Restore zone contents
    if let Some(zone_manager) = &mut *zones {
        // Use the GameSaveData method to restore ZoneManager
        **zone_manager = save_data.to_zone_manager(&index_to_entity);
    }

    // Restore commander zone contents
    if let Some(commander_manager) = &mut *commanders {
        // Use the GameSaveData method to restore CommandZoneManager
        **commander_manager = save_data.to_commander_manager(&index_to_entity);
    }
}
