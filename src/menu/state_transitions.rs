use crate::camera::components::GameCamera;
use crate::cards::Card;
use crate::game_engine::state::GameState;
use crate::menu::state::{AppState, GameMenuState, StateTransitionContext};
use bevy::prelude::*;

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

/// Checks if the game state has been loaded and transitions to InGame if ready.
pub fn check_loading_complete(
    current_menu_state: Res<State<GameMenuState>>,
    game_state: Option<Res<GameState>>,
    mut next_menu_state: ResMut<NextState<GameMenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    // Only run this logic if we are currently in the Loading state
    if *current_menu_state.get() != GameMenuState::Loading {
        return;
    }

    match game_state {
        Some(state) => {
            // If the GameState resource exists, we consider loading complete
            // regardless of the turn number (handles new game case where turn is 0)
            info!(
                "GameState resource found (Turn {}), transitioning from Loading to InGame...",
                state.turn_number
            );
            next_menu_state.set(GameMenuState::InGame);
            next_app_state.set(AppState::InGame);
        }
        None => {
            // GameState resource doesn't exist yet, still waiting
            debug!("In Loading state, GameState resource not found yet, waiting...");
        }
    }
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
pub fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}
