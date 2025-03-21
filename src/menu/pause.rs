use crate::menu::{
    state::GameMenuState,
    systems::pause_menu::{
        input_handler::pause_menu_input, interactions::pause_menu_action, setup::setup_pause_menu,
    },
};
use bevy::prelude::*;

/// Plugin for pause menu functionality
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register systems
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_menu)
            .add_systems(
                Update,
                (
                    pause_menu_action.run_if(in_state(GameMenuState::PausedGame)),
                    pause_menu_input,
                ),
            );

        info!("Pause menu plugin registered");
    }
}
