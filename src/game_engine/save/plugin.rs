use crate::game_engine::save::events::*;
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
            .add_systems(Startup, setup_save_system)
            .add_systems(
                Update,
                (
                    handle_save_game.run_if(|res: Option<Res<GameState>>| resource_exists(res)),
                    handle_load_game,
                    handle_auto_save,
                    handle_start_replay,
                    handle_step_replay,
                    handle_stop_replay,
                ),
            );
    }
}
