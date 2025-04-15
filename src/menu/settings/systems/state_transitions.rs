use crate::menu::settings::state::SettingsMenuState;
use crate::menu::state::{GameMenuState, StateTransitionContext};
use bevy::prelude::*;

/// Helper function to handle exiting the settings menu
pub fn handle_settings_exit(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
    context: &mut StateTransitionContext,
) {
    // First set settings state to disabled to trigger cleanup
    settings_menu_state.set(SettingsMenuState::Disabled);

    // Get the origin state from context, defaulting to MainMenu
    let origin = context.settings_origin.unwrap_or(GameMenuState::MainMenu);
    info!("Exiting settings, returning to {:?}", origin);

    // Then transition back to the origin state
    game_menu_state.set(origin);

    // Clear the settings origin
    context.settings_origin = None;
}

/// Helper function to handle entering the settings menu
pub fn handle_settings_enter(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
    context: &mut StateTransitionContext,
    from_state: GameMenuState,
) {
    // Save our origin for when we return
    context.settings_origin = Some(from_state);

    // Reset flags
    context.from_pause_menu = from_state == GameMenuState::PauseMenu;
    context.returning_from_settings = false;

    // Set up settings menu
    settings_menu_state.set(SettingsMenuState::Main);
    game_menu_state.set(GameMenuState::Settings);

    info!("Transitioning to settings menu from {:?}", from_state);
}
