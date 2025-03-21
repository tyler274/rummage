use crate::menu::state::{GameMenuState, StateTransitionContext};
use bevy::prelude::*;

/// Handles keyboard input when in the pause menu, specifically ESC to toggle pause
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
) {
    // Only respond to Escape key press
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameMenuState::Paused => {
                // If we're in the pause menu, resume the game
                info!("Escape key pressed while paused, resuming game");
                next_state.set(GameMenuState::InGame);
            }
            GameMenuState::InGame => {
                // If we're in-game, pause the game
                info!("Escape key pressed during gameplay, pausing game");
                next_state.set(GameMenuState::Paused);
            }
            GameMenuState::MainMenu => {
                // Do nothing in main menu
                debug!("Escape pressed in main menu, ignoring");
            }
            _ => {
                // For other states, check where we came from
                if context.from_pause_menu {
                    info!("Escape pressed in submenu, returning to pause menu");
                    next_state.set(GameMenuState::Paused);
                } else {
                    info!("Escape pressed in submenu, returning to main menu");
                    next_state.set(GameMenuState::MainMenu);
                }
            }
        }
    }
}
