use crate::menu::{
    cleanup::{cleanup_game, cleanup_main_menu, cleanup_menu_camera, cleanup_pause_menu},
    main_menu::{menu_action, setup_main_menu},
    pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
    state::GameMenuState,
};
use bevy::prelude::*;

/// Plugin that sets up the menu system and its related systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .insert_resource(GameMenuState::MainMenu)
            // Main Menu state
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu)
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (cleanup_main_menu, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                menu_action.run_if(in_state(GameMenuState::MainMenu)),
            )
            // Loading state systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                (cleanup_game, cleanup_menu_camera, start_game_loading).chain(),
            )
            .add_systems(OnExit(GameMenuState::Loading), finish_loading)
            // Pause menu systems
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_menu)
            .add_systems(
                OnExit(GameMenuState::PausedGame),
                (cleanup_pause_menu, cleanup_menu_camera).chain(),
            )
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(GameMenuState::PausedGame)),
            )
            .add_systems(Update, handle_pause_input)
            // Add cleanup when entering main menu from game
            .add_systems(OnEnter(GameMenuState::MainMenu), cleanup_game);
    }
}

/// Starts the game loading process
fn start_game_loading(mut next_state: ResMut<NextState<GameMenuState>>) {
    // TODO: Implement game loading logic
    next_state.set(GameMenuState::InGame);
}

/// Finishes the game loading process
fn finish_loading() {
    // TODO: Implement any final loading steps
}
