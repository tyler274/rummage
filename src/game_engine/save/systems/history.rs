use bevy::prelude::*;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;

use super::utils::apply_game_state;

/// System to handle capturing the current game state into history
pub fn handle_capture_history(
    mut event_reader: EventReader<CaptureHistoryEvent>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    zones: Option<Res<ZoneManager>>,
    commanders: Option<Res<CommandZoneManager>>,
    mut game_history: ResMut<GameHistory>,
) {
    for _ in event_reader.read() {
        info!("Capturing current game state to history");

        let mut player_data = Vec::new();
        let mut entity_to_index = std::collections::HashMap::new();

        // Convert entity-based references to indices for serialization
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

        // Create game save data
        let mut save_data =
            GameSaveData::from_game_state(&game_state, &entity_to_index, player_data);

        // Add zone data if ZoneManager is available
        if let Some(zone_manager) = zones.as_ref() {
            save_data.zones = GameSaveData::from_zone_manager(zone_manager, &entity_to_index);
        }

        // Add commander data if CommandZoneManager is available
        if let Some(commander_manager) = commanders.as_ref() {
            save_data.commanders =
                GameSaveData::from_commander_manager(commander_manager, &entity_to_index);
        }

        // Add to history
        game_history.add_state(save_data);
    }
}

/// System to handle creating a new branch
pub fn handle_create_branch(
    mut event_reader: EventReader<CreateBranchEvent>,
    mut game_history: ResMut<GameHistory>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    zones: Option<Res<ZoneManager>>,
    commanders: Option<Res<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Creating new game history branch");

        let mut player_data = Vec::new();
        let mut entity_to_index = std::collections::HashMap::new();

        // Convert entity-based references to indices for serialization
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

        // Create game save data
        let mut save_data =
            GameSaveData::from_game_state(&game_state, &entity_to_index, player_data);

        // Add zone data if ZoneManager is available
        if let Some(zone_manager) = zones.as_ref() {
            save_data.zones = GameSaveData::from_zone_manager(zone_manager, &entity_to_index);
        }

        // Add commander data if CommandZoneManager is available
        if let Some(commander_manager) = commanders.as_ref() {
            save_data.commanders =
                GameSaveData::from_commander_manager(commander_manager, &entity_to_index);
        }

        // Create a new branch
        let branch_id = game_history.create_branch(save_data);

        // Set the branch name if provided
        if let Some(name) = &event.name {
            if let Some(branch) = game_history.branches.iter_mut().find(|b| b.id == branch_id) {
                branch.name = Some(name.clone());
                info!("Named branch {} as '{}'", branch_id, name);
            }
        }

        info!("Created new branch with ID: {}", branch_id);
    }
}

/// System to handle switching between branches
pub fn handle_switch_branch(
    mut event_reader: EventReader<SwitchBranchEvent>,
    mut game_history: ResMut<GameHistory>,
    mut game_state: Option<ResMut<GameState>>,
    mut commands: Commands,
    mut query_players: Query<(Entity, &mut Player)>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for event in event_reader.read() {
        info!("Switching to branch {}", event.branch_id);

        // Switch to the branch
        if game_history.switch_to_branch(event.branch_id) {
            // Get the current state from the branch
            if let Some(state) = game_history.current_state() {
                let branch_state = state.clone();
                apply_game_state(
                    &branch_state,
                    &mut game_state,
                    &mut commands,
                    &mut query_players,
                    &mut zones,
                    &mut commanders,
                );
                info!(
                    "Switched to branch {} at turn {}",
                    event.branch_id, branch_state.game_state.turn_number
                );
            } else {
                warn!("Branch {} exists but has no states", event.branch_id);
            }
        } else {
            warn!("Branch with ID {} not found", event.branch_id);
        }
    }
}

/// System to handle moving forward in history
pub fn handle_history_forward(
    mut event_reader: EventReader<HistoryForwardEvent>,
    mut game_history: ResMut<GameHistory>,
    mut game_state: Option<ResMut<GameState>>,
    mut commands: Commands,
    mut query_players: Query<(Entity, &mut Player)>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for _ in event_reader.read() {
        if !game_history.is_navigating {
            warn!("Not in history navigation mode");
            continue;
        }

        info!("Moving forward in history");

        // Try to move forward in history
        if let Some(state) = game_history.fast_forward() {
            let forward_state = state.clone();
            apply_game_state(
                &forward_state,
                &mut game_state,
                &mut commands,
                &mut query_players,
                &mut zones,
                &mut commanders,
            );
            info!(
                "Moved forward to turn {}",
                forward_state.game_state.turn_number
            );
        } else {
            info!("Already at latest state in branch");
        }
    }
}

/// System to handle moving backward in history
pub fn handle_history_backward(
    mut event_reader: EventReader<HistoryBackwardEvent>,
    mut game_history: ResMut<GameHistory>,
    mut game_state: Option<ResMut<GameState>>,
    mut commands: Commands,
    mut query_players: Query<(Entity, &mut Player)>,
    mut zones: Option<ResMut<ZoneManager>>,
    mut commanders: Option<ResMut<CommandZoneManager>>,
) {
    for _ in event_reader.read() {
        if !game_history.is_navigating {
            warn!("Not in history navigation mode");
            continue;
        }

        info!("Moving backward in history");

        // Try to move backward in history
        if let Some(state) = game_history.rewind() {
            let backward_state = state.clone();
            apply_game_state(
                &backward_state,
                &mut game_state,
                &mut commands,
                &mut query_players,
                &mut zones,
                &mut commanders,
            );
            info!(
                "Moved backward to turn {}",
                backward_state.game_state.turn_number
            );
        } else {
            info!("Already at earliest state in branch");
        }
    }
}
