use crate::menu::{
    state::GameMenuState,
    systems::pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
};
use bevy::prelude::*;

/// Plugin for handling pause menu functionality
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Set up systems for pause menu
            .add_systems(OnEnter(GameMenuState::Paused), setup_pause_menu)
            .add_systems(
                Update,
                (
                    pause_menu_action.run_if(in_state(GameMenuState::Paused)),
                    handle_pause_input,
                ),
            );

        info!("PauseMenuPlugin initialized");
    }
} 