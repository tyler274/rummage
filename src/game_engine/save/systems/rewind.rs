use bevy::prelude::*;
use std::collections::HashMap;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

use super::utils::apply_game_state;

/// System to handle the start rewind event
pub fn handle_rewind(
    mut event_reader: EventReader<StartRewindEvent>,
    mut game_history: ResMut<GameHistory>,
    mut game_state: Option<ResMut<GameState>>,
    mut commands: Commands,
    mut query_players: Query<(Entity, &mut Player)>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Rewinding game by {} steps", event.steps);

        // Ensure we have at least one game state in history
        if game_history.active_branch().is_none()
            || game_history.active_branch().unwrap().states.is_empty()
        {
            warn!("Cannot rewind: no history available");
            continue;
        }

        // If not already in navigation mode, we need to capture current state first
        if !game_history.is_navigating {
            // Capture current state before rewinding
            let mut player_data = Vec::new();
            let mut entity_to_index = HashMap::new();

            // Create a mapping for existing entities
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

            if let Some(state) = game_state.as_ref() {
                // Create game save data
                let mut current_save_data =
                    GameSaveData::from_game_state(state, &entity_to_index, player_data);

                // Add zone data if ZoneManager is available
                if let Some(zone_manager) = zones.as_ref() {
                    current_save_data.zones =
                        GameSaveData::from_zone_manager(zone_manager, &entity_to_index);
                }

                // Add commander data if CommandZoneManager is available
                if let Some(commander_manager) = commanders.as_ref() {
                    current_save_data.commanders =
                        GameSaveData::from_commander_manager(commander_manager, &entity_to_index);
                }

                // Create a new branch from current state when starting to rewind
                // This preserves the original timeline
                game_history.create_branch(current_save_data);
                info!("Created new branch for rewind operation");
            }

            game_history.is_navigating = true;
        }

        // Perform the rewind steps
        let mut rewound_state = None;
        for _ in 0..event.steps {
            if let Some(state) = game_history.rewind() {
                rewound_state = Some(state.clone());
            } else {
                info!("Reached beginning of history branch");
                break;
            }
        }

        // Apply the rewound state if available
        if let Some(rewound_state) = rewound_state {
            apply_game_state(
                &rewound_state,
                &mut game_state,
                &mut commands,
                &mut query_players,
                &mut zones,
                &mut commanders,
            );
            info!("Rewound to turn {}", rewound_state.game_state.turn_number);
        }
    }
}

/// System to handle rewinding to a specific turn
pub fn handle_rewind_to_turn(
    mut event_reader: EventReader<RewindToTurnEvent>,
    mut game_history: ResMut<GameHistory>,
    mut game_state: Option<ResMut<GameState>>,
    mut commands: Commands,
    mut query_players: Query<(Entity, &mut Player)>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Rewinding to turn {}", event.turn);

        // Ensure we have game states in history
        if game_history.active_branch().is_none()
            || game_history.active_branch().unwrap().states.is_empty()
        {
            warn!("Cannot rewind: no history available");
            continue;
        }

        // If not already in navigation mode, we need to capture current state first
        if !game_history.is_navigating {
            // Capture current state before rewinding
            let mut player_data = Vec::new();
            let mut entity_to_index = HashMap::new();

            // Create a mapping for existing entities
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

            if let Some(state) = game_state.as_ref() {
                // Create game save data
                let mut current_save_data =
                    GameSaveData::from_game_state(state, &entity_to_index, player_data);

                // Add zone data if ZoneManager is available
                if let Some(zone_manager) = zones.as_ref() {
                    current_save_data.zones =
                        GameSaveData::from_zone_manager(zone_manager, &entity_to_index);
                }

                // Add commander data if CommandZoneManager is available
                if let Some(commander_manager) = commanders.as_ref() {
                    current_save_data.commanders =
                        GameSaveData::from_commander_manager(commander_manager, &entity_to_index);
                }

                // Create a new branch from current state when starting to rewind
                // This preserves the original timeline
                game_history.create_branch(current_save_data);
                info!("Created new branch for rewind operation");
            }

            game_history.is_navigating = true;
        }

        // Try to jump to the specific turn
        if let Some(state) = game_history.go_to_turn(event.turn) {
            let rewound_state = state.clone();
            apply_game_state(
                &rewound_state,
                &mut game_state,
                &mut commands,
                &mut query_players,
                &mut zones,
                &mut commanders,
            );
            info!("Rewound to turn {}", event.turn);
        } else {
            warn!("Turn {} not found in history", event.turn);
        }
    }
}

/// System to handle rollback to a previous checkpoint
pub fn handle_rollback(
    mut event_reader: EventReader<RollbackEvent>,
    _config: Res<SaveConfig>,
    mut load_events: EventWriter<LoadGameEvent>,
) {
    for event in event_reader.read() {
        let slot_name = if let Some(checkpoint_name) = &event.checkpoint_name {
            info!("Rolling back to checkpoint: {}", checkpoint_name);
            checkpoint_name.clone()
        } else {
            info!("Rolling back to last auto-save");
            "auto_save".to_string()
        };

        // Just use the load system to perform the rollback
        load_events.write(LoadGameEvent { slot_name });
    }
}
