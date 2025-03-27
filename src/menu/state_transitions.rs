use crate::camera::components::GameCamera;
use crate::cards::Card;
use crate::menu::settings::SettingsMenuState;
use crate::menu::state::{GameMenuState, StateTransitionContext};
use bevy::prelude::*;

/// Set up the transition context for settings menu
pub fn setup_settings_transition(
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    settings_current: Res<State<SettingsMenuState>>,
) {
    info!(
        "Setting up settings transition from state: {:?}, from_pause_menu: {}",
        current_state.get(),
        context.from_pause_menu
    );

    info!("Current SettingsMenuState: {:?}", settings_current.get());

    // Always reset from_pause_menu flag when transitioning from MainMenu
    if *current_state.get() == GameMenuState::MainMenu {
        info!("Resetting from_pause_menu flag because we're in MainMenu state");
        context.from_pause_menu = false;
        // Explicitly set the settings origin to MainMenu
        info!("Explicitly setting settings_origin to MainMenu");
        context.settings_origin = Some(GameMenuState::MainMenu);
    } else if context.from_pause_menu || *current_state.get() == GameMenuState::PauseMenu {
        // If the flag is set or we're coming from the pause menu, set the origin to PauseMenu
        info!("Detected transition from pause menu");
        context.settings_origin = Some(GameMenuState::PauseMenu);
    } else {
        // Fall back to checking the current state
        match current_state.get() {
            GameMenuState::Settings if context.settings_origin.is_none() => {
                // If we're already in Settings state but have no origin,
                // default to main menu
                info!("Already in Settings state with no origin, defaulting to main menu");
                context.settings_origin = Some(GameMenuState::MainMenu);
            }
            _ => {
                if context.settings_origin.is_none() {
                    // Default to main menu if coming from an unexpected state
                    info!("Entering settings from unexpected state, defaulting to main menu");
                    context.settings_origin = Some(GameMenuState::MainMenu);
                } else {
                    info!(
                        "Using existing settings origin: {:?}",
                        context.settings_origin
                    );
                }
            }
        }
    }

    // Ensure we're showing the main settings screen when entering settings
    info!(
        "Setting SettingsMenuState to Main (was {:?})",
        settings_current.get()
    );
    settings_state.set(SettingsMenuState::Main);

    // Log that we're about to exit this function
    info!("Completed settings transition setup, SettingsMenuState should now be Main");
}

/// Starts the game loading process
pub fn start_game_loading(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut context: ResMut<StateTransitionContext>,
    cards: Query<Entity, With<Card>>,
    game_cameras: Query<Entity, With<GameCamera>>,
) {
    // Check if we're coming from the pause menu
    if context.from_pause_menu {
        info!("Coming from pause menu, skipping loading process and going directly to InGame");
        // Reset the flag
        context.from_pause_menu = false;

        // When resuming from pause menu, we shouldn't spawn new cameras
        // Go directly to InGame without performing cleanup that would remove game entities
        next_state.set(GameMenuState::InGame);
        return;
    }

    // Check for camera ambiguities and clean them up if found
    let camera_count = game_cameras.iter().count();
    if camera_count > 0 {
        if camera_count > 1 {
            warn!(
                "Found {} game cameras, cleaning up all cameras to prevent ambiguities",
                camera_count
            );
            // Remove all existing cameras instead of trying to keep one,
            // as the setup_game_camera system will create a fresh one
            for entity in game_cameras.iter() {
                info!("Removing game camera entity: {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        } else {
            info!(
                "Found single game camera entity: {:?}",
                game_cameras.single()
            );
        }
    } else {
        info!("No game cameras found, a new one will be created during game setup");
    }

    // Normal loading process
    info!("Checking game state for transition...");
    info!("Number of cards: {}", cards.iter().count());
    info!(
        "Number of game cameras after cleanup: {}",
        game_cameras.iter().count()
    );

    // Only transition if cleanup is complete
    if cards.is_empty() && game_cameras.is_empty() {
        info!("Cleanup complete, transitioning to InGame state...");
        next_state.set(GameMenuState::InGame);
    } else {
        info!("Cleanup not complete yet, waiting...");
        // Force cleanup if stuck
        if game_cameras.iter().count() > 0 {
            warn!("Forcing cleanup of remaining game cameras...");
            for entity in game_cameras.iter() {
                info!("Force despawning game camera entity: {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// Finishes the game loading process
pub fn finish_loading() {
    info!("Loading state finished");
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
pub fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}
