use crate::menu::{
    // cleanup::despawn_screen, // Commented out - definition not found
    // components::OnPauseMenuScreen, // Commented out - definition not found
    state::{AppState, GameMenuState},
};
use bevy::prelude::*;

use super::systems::pause_menu::{
    // Correct path for systems
    input_handler::{esc_key_system, handle_pause_trigger},
    interactions::pause_menu_action,
    setup::setup_pause_menu,
};

/// Plugin for the pause menu
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter pause menu setup
            .add_systems(OnEnter(GameMenuState::PauseMenu), setup_pause_menu)
            // Systems to run only in the pause menu state and when app is paused
            .add_systems(
                Update,
                (
                    pause_menu_action,
                    esc_key_system,
                )
                .run_if(in_state(GameMenuState::PauseMenu).and(in_state(AppState::Paused))),
            )
            // System to *trigger* the pause menu from the game
            .add_systems(Update, handle_pause_trigger.run_if(in_state(AppState::InGame)))
            // Exit pause menu cleanup - Commented out
            // .add_systems(
            //     OnExit(GameMenuState::PauseMenu),
            //     despawn_screen::<OnPauseMenuScreen>,
            // );
            ;

        info!("Pause menu plugin registered");
    }
}
