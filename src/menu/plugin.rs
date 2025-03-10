use crate::menu::{
    cleanup::{cleanup_game, cleanup_main_menu, cleanup_menu_camera, cleanup_pause_menu},
    logo::{StarOfDavidPlugin, render_star_of_david},
    main_menu::{menu_action, set_menu_camera_zoom, setup_main_menu},
    pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
    state::{GameMenuState, StateTransitionContext},
};
use crate::{camera::GameCamera, card::Card};
use bevy::prelude::*;

/// Plugin that sets up the menu system and its related systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .insert_resource(GameMenuState::MainMenu)
            .insert_resource(StateTransitionContext::default())
            .add_plugins(StarOfDavidPlugin)
            // Main Menu state
            .add_systems(
                OnEnter(GameMenuState::MainMenu),
                (
                    cleanup_game,
                    cleanup_menu_camera,
                    setup_main_menu,
                    set_menu_camera_zoom,
                )
                    .chain(),
            )
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (cleanup_main_menu, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                (menu_action, render_star_of_david).run_if(in_state(GameMenuState::MainMenu)),
            )
            // Loading state systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                (cleanup_game, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                start_game_loading.run_if(in_state(GameMenuState::Loading)),
            )
            .add_systems(OnExit(GameMenuState::Loading), finish_loading)
            // Pause menu systems
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_menu)
            .add_systems(OnExit(GameMenuState::PausedGame), cleanup_pause_menu)
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(GameMenuState::PausedGame)),
            )
            .add_systems(Update, handle_pause_input);
    }
}

/// Starts the game loading process
fn start_game_loading(
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
        // Go directly to InGame without performing cleanup
        next_state.set(GameMenuState::InGame);
        return;
    }

    // Normal loading process
    info!("Checking game state for transition...");
    info!("Number of cards: {}", cards.iter().count());
    info!("Number of game cameras: {}", game_cameras.iter().count());

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
fn finish_loading() {
    info!("Loading state finished");
}
