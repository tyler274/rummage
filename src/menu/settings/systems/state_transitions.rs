use crate::menu::settings::state::SettingsMenuState;
use crate::menu::state::{GameMenuState, StateTransitionContext};
use bevy::prelude::*;

/// Helper function to handle exiting the settings menu
pub fn handle_settings_exit(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
    context: &mut StateTransitionContext,
) {
    // Get the origin state from context, defaulting to MainMenu
    let origin = get_settings_origin(context);
    info!("Exiting settings, returning to {:?}", origin);

    // Prepare for cleanup
    prepare_settings_cleanup(context);

    // Log context just before transition
    info!(
        "Context before transition: origin={:?}, returning_from_settings={}, from_pause_menu={}",
        context.settings_origin, context.returning_from_settings, context.from_pause_menu
    );

    // Perform state transitions
    transition_from_settings(settings_menu_state, game_menu_state, origin);

    // Reset context
    reset_settings_context(context);
}

/// Helper function to handle entering the settings menu
pub fn handle_settings_enter(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
    context: &mut StateTransitionContext,
    from_state: GameMenuState,
) {
    // Save context for return journey
    save_settings_origin(context, from_state);
    // Log the origin immediately after setting it
    info!(
        "handle_settings_enter: Saved settings_origin = {:?}",
        context.settings_origin
    );

    // Reset flags
    prepare_settings_entry(context, from_state);

    // Perform state transitions
    transition_to_settings(settings_menu_state, game_menu_state);

    info!("Transitioning to settings menu from {:?}", from_state);
}

/// Helper to get the origin state to return to
fn get_settings_origin(context: &StateTransitionContext) -> GameMenuState {
    context.settings_origin.unwrap_or(GameMenuState::MainMenu)
}

/// Helper to prepare context for settings cleanup
fn prepare_settings_cleanup(context: &mut StateTransitionContext) {
    context.returning_from_settings = true;
}

/// Helper to perform state transitions when exiting settings
fn transition_from_settings(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
    target_state: GameMenuState,
) {
    // First set settings state to disabled to trigger cleanup
    settings_menu_state.set(SettingsMenuState::Disabled);
    // Then transition to target state
    game_menu_state.set(target_state);
}

/// Helper to save the origin state for later return
fn save_settings_origin(context: &mut StateTransitionContext, from_state: GameMenuState) {
    context.settings_origin = Some(from_state);
}

/// Helper to prepare context for settings entry
fn prepare_settings_entry(context: &mut StateTransitionContext, from_state: GameMenuState) {
    context.from_pause_menu = from_state == GameMenuState::PauseMenu;
    context.returning_from_settings = false;
}

/// Helper to perform state transitions when entering settings
fn transition_to_settings(
    settings_menu_state: &mut NextState<SettingsMenuState>,
    game_menu_state: &mut NextState<GameMenuState>,
) {
    settings_menu_state.set(SettingsMenuState::Main);
    game_menu_state.set(GameMenuState::Settings);
}

/// Helper to reset context after exiting settings
fn reset_settings_context(context: &mut StateTransitionContext) {
    context.settings_origin = None;
    context.from_pause_menu = false;
}

/// Helper to check if we're in a settings submenu
pub fn is_in_settings_submenu(settings_state: SettingsMenuState) -> bool {
    matches!(
        settings_state,
        SettingsMenuState::Video
            | SettingsMenuState::Audio
            | SettingsMenuState::Gameplay
            | SettingsMenuState::Controls
    )
}

/// Helper to check if we should handle settings back input
pub fn should_handle_settings_back(settings_state: SettingsMenuState) -> bool {
    settings_state != SettingsMenuState::Disabled
}
