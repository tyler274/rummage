use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bincode;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

/// System to set up the save system on startup
pub fn setup_save_system(mut commands: Commands) {
    // Create save directory if it doesn't exist
    let config = SaveConfig::default();
    std::fs::create_dir_all(&config.save_directory).unwrap_or_else(|e| {
        error!("Failed to create save directory: {}", e);
    });

    commands.insert_resource(config);
    commands.insert_resource(AutoSaveTracker::default());
    commands.insert_resource(ReplayState::default());

    // Initialize persistent save metadata
    let save_metadata = Persistent::builder()
        .name("save_metadata")
        .format(StorageFormat::Bincode)
        .path("saves/metadata.bin")
        .default(SaveMetadata::default())
        .build()
        .expect("Failed to create persistent save metadata");

    commands.insert_resource(save_metadata);
}

/// System to handle save game requests
pub fn handle_save_game(
    mut event_reader: EventReader<SaveGameEvent>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    _zones: Option<Res<ZoneManager>>,
    _commanders: Option<Res<CommandZoneManager>>,
    mut save_metadata: ResMut<Persistent<SaveMetadata>>,
    _config: Res<SaveConfig>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        info!("Saving game to slot: {}", event.slot_name);

        let mut player_data = Vec::new();

        // Convert entity-based references to indices for serialization
        let mut entity_to_index = HashMap::new();

        for (i, (entity, player)) in query_players.iter().enumerate() {
            entity_to_index.insert(entity, i);

            player_data.push(PlayerData {
                id: i,
                name: player.name.clone(),
                life: player.life,
                mana_pool: player.mana_pool.clone(),
                player_index: i,
            });
        }

        // Create game save data using helper method
        let save_data = GameSaveData::from_game_state(&game_state, &entity_to_index, player_data);
        let save_path = format!("saves/{}.bin", event.slot_name);

        // Insert as a resource first, then create persistent
        commands.insert_resource(save_data.clone());

        // Create persistent resource for this save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(&format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(&save_path)
            .build();

        match persistent_save {
            Ok(save) => {
                // Persist the save immediately
                if let Err(e) = save.persist() {
                    error!("Failed to save game: {}", e);
                    continue;
                }

                info!("Game saved successfully to {}", save_path);

                // Update metadata
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                let save_info = SaveInfo {
                    slot_name: event.slot_name.clone(),
                    timestamp,
                    description: format!("Turn {}", game_state.turn_number),
                    turn_number: game_state.turn_number,
                    player_count: query_players.iter().count(),
                };

                // Add or update save info in metadata
                if let Some(existing) = save_metadata
                    .saves
                    .iter_mut()
                    .find(|s| s.slot_name == event.slot_name)
                {
                    *existing = save_info;
                } else {
                    save_metadata.saves.push(save_info);
                }

                // Save metadata
                if let Err(e) = save_metadata.persist() {
                    error!("Failed to update save metadata: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to create persistent save: {}", e);
            }
        }
    }
}

/// System to handle load game requests
pub fn handle_load_game(
    mut event_reader: EventReader<LoadGameEvent>,
    mut commands: Commands,
    _config: Res<SaveConfig>,
    mut query_players: Query<(Entity, &mut Player)>,
    mut game_state: Option<ResMut<GameState>>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Loading game from slot: {}", event.slot_name);

        let save_path = format!("saves/{}.bin", event.slot_name);

        // Create a persistent resource to load the save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(&format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(&save_path)
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

                // Restore game state
                if let Some(game_state) = &mut game_state {
                    **game_state = save_data.to_game_state(&index_to_entity);
                } else {
                    commands.insert_resource(save_data.to_game_state(&index_to_entity));
                }

                // TODO: Restore zone contents
                if let Some(zones) = &mut zones {
                    // Implement zone restoration based on your ZoneManager
                }

                // TODO: Restore commander zone contents
                if let Some(commanders) = &mut commanders {
                    // Implement commander zone restoration based on your CommandZoneManager
                }

                info!("Game loaded successfully from {}", save_path);
            }
            Err(e) => {
                error!("Failed to load save: {}", e);
            }
        }
    }
}

/// System to handle auto-saving
pub fn handle_auto_save(
    mut event_reader: EventReader<CheckStateBasedActionsEvent>,
    mut event_writer: EventWriter<SaveGameEvent>,
    mut auto_save_tracker: ResMut<AutoSaveTracker>,
    config: Res<SaveConfig>,
) {
    // Skip if auto-save is disabled
    if !config.auto_save_enabled {
        return;
    }

    for _ in event_reader.read() {
        auto_save_tracker.counter += 1;

        // Check if it's time to auto-save
        if auto_save_tracker.counter >= config.auto_save_frequency {
            auto_save_tracker.counter = 0;

            // Trigger auto-save
            event_writer.send(SaveGameEvent {
                slot_name: "auto_save".to_string(),
            });
        }
    }
}

/// System to handle starting a replay session
pub fn handle_start_replay(
    mut event_reader: EventReader<StartReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
    _commands: Commands,
    _config: Res<SaveConfig>,
    mut load_events: EventWriter<LoadGameEvent>,
) {
    for event in event_reader.read() {
        info!("Starting replay from save slot: {}", event.slot_name);

        let save_path = format!("saves/{}.bin", event.slot_name);

        // Create a persistent resource to load the save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(&format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(&save_path)
            .build();

        match persistent_save {
            Ok(save) => {
                // Get the loaded data
                let save_data = save.clone();

                // Set up replay state with the loaded save
                replay_state.active = true;
                replay_state.original_save = Some(save_data.clone());
                replay_state.current_game_state = Some(save_data);
                replay_state.current_step = 0;

                // Load initial actions
                // TODO: Load replay actions from a separate file

                info!("Replay started from save {}", event.slot_name);

                // Send a load event to actually load the game state
                load_events.send(LoadGameEvent {
                    slot_name: event.slot_name.clone(),
                });
            }
            Err(e) => {
                error!("Failed to load replay save: {}", e);
            }
        }
    }
}

/// System to handle stepping through a replay
pub fn handle_step_replay(
    mut event_reader: EventReader<StepReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
    game_state: Option<ResMut<GameState>>,
) {
    // Skip if replay is not active or no game state
    if !replay_state.active || game_state.is_none() {
        for _ in event_reader.read() {
            warn!("Cannot step through replay: replay not active or game state missing");
        }
        return;
    }

    let mut game_state = game_state.unwrap();

    for event in event_reader.read() {
        let steps = event.steps.max(1); // Ensure at least 1 step

        info!("Stepping through replay: {} step(s)", steps);

        for _ in 0..steps {
            // Check if we have actions in the queue
            if let Some(action) = replay_state.action_queue.pop_front() {
                // Apply the action to the game state
                apply_replay_action(&mut game_state, &action);
                replay_state.current_step += 1;

                info!(
                    "Applied replay action: {:?} (Step {})",
                    action.action_type, replay_state.current_step
                );
            } else {
                info!("No more actions in replay queue");
                break;
            }
        }
    }
}

/// System to handle stopping a replay
pub fn handle_stop_replay(
    mut event_reader: EventReader<StopReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
) {
    for _ in event_reader.read() {
        if replay_state.active {
            info!("Stopping replay");

            // Reset replay state
            replay_state.active = false;
            replay_state.original_save = None;
            replay_state.current_game_state = None;
            replay_state.action_queue.clear();
            replay_state.current_step = 0;
        }
    }
}

/// Helper function to apply a replay action to the game state
fn apply_replay_action(game_state: &mut GameState, action: &ReplayAction) {
    // This is where you'd implement the actual game action application
    // For now this is just a placeholder

    match action.action_type {
        ReplayActionType::PlayCard => {
            // Logic for playing a card
        }
        ReplayActionType::DeclareAttackers => {
            // Logic for declaring attackers
        }
        ReplayActionType::DeclareBlockers => {
            // Logic for declaring blockers
        }
        ReplayActionType::ActivateAbility => {
            // Logic for activating an ability
        }
        ReplayActionType::ResolveEffect => {
            // Logic for resolving an effect
        }
        ReplayActionType::DrawCard => {
            // Logic for drawing a card
        }
        ReplayActionType::PassPriority => {
            // Logic for passing priority
        }
        ReplayActionType::CastSpell => {
            // Logic for casting a spell
        }
        ReplayActionType::EndTurn => {
            // Logic for ending a turn
            game_state.turn_number += 1;
        }
    }
}

/// Capture a game action for replay purposes
pub fn capture_game_action(
    action_type: ReplayActionType,
    player_index: usize,
    data: String,
    game_state: &GameState,
    phase: String,
) -> ReplayAction {
    ReplayAction {
        action_type,
        player_index,
        data,
        turn: game_state.turn_number,
        phase,
    }
}
