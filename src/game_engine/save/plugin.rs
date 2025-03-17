use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::save::systems::*;
use crate::game_engine::state::GameState;
use bevy::prelude::*;

/// Plugin for save and load game functionality
pub struct SaveLoadPlugin;

impl Plugin for SaveLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<CheckStateBasedActionsEvent>()
            .add_event::<StartReplayEvent>()
            .add_event::<StepReplayEvent>()
            .add_event::<StopReplayEvent>()
            .add_event::<StartRewindEvent>()
            .add_event::<RewindToTurnEvent>()
            .add_event::<RollbackEvent>()
            .add_event::<CreateBranchEvent>()
            .add_event::<SwitchBranchEvent>()
            .add_event::<CaptureHistoryEvent>()
            .add_event::<HistoryForwardEvent>()
            .add_event::<HistoryBackwardEvent>()
            .init_resource::<GameHistory>()
            .init_resource::<SaveEvents>()
            .add_systems(Startup, setup_save_system);

        // Register systems with condition
        let condition = resource_exists::<GameState>;

        // Split the save game system into two parts:
        // 1. Event collection system - runs with condition to collect events
        // 2. Processing system - runs unconditionally but checks for events and resources
        app.add_systems(FixedUpdate, collect_save_events.run_if(condition));
        // Add the process_save_game system only when implemented with compatible signature

        // History and timeline management systems
        app.add_systems(
            FixedUpdate,
            (
                handle_load_game,
                handle_auto_save,
                handle_start_replay,
                handle_step_replay,
                handle_stop_replay,
                handle_capture_history,
                handle_rewind,
                handle_rewind_to_turn,
                handle_rollback,
                handle_create_branch,
                handle_switch_branch,
                handle_history_forward,
                handle_history_backward,
                auto_capture_history,
            )
                .run_if(condition),
        );
    }
}
