use bevy::prelude::*;
use bevy_persistent::prelude::*;

use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::state::GameState;

use super::get_storage_path;

/// System to handle starting a replay session
pub fn handle_start_replay(
    mut event_reader: EventReader<StartReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
    _commands: Commands,
    config: Res<SaveConfig>,
    mut load_events: EventWriter<LoadGameEvent>,
) {
    for event in event_reader.read() {
        info!("Starting replay from save slot: {}", event.slot_name);

        let save_path = get_storage_path(&config, &format!("{}.bin", event.slot_name));

        // Create a persistent resource to load the save
        let persistent_save = Persistent::<GameSaveData>::builder()
            .name(format!("game_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(save_path)
            .default(GameSaveData::default())
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

/// Captures a game action for replaying
#[allow(dead_code)]
pub fn capture_game_action(
    action_type: ReplayActionType,
    player_index: usize,
    data: String,
    game_state: &GameState,
    phase: String,
) -> ReplayAction {
    ReplayAction::new(action_type)
        .with_player(player_index)
        .with_data(data)
        .with_turn(game_state.turn_number)
        .with_phase(phase)
}
