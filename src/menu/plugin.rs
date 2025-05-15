use bevy::prelude::*;

use crate::{
    cards::Card,
    menu::{
        backgrounds::BackgroundsPlugin,
        camera::setup::{cleanup_menu_camera, setup_main_menu_camera, setup_menu_camera},
        cleanup::{CleanupPlugin, pause_menu::cleanup_pause_menu},
        components::{MenuVisibilityState, /* NeedsMainMenuSetup, */ UiHierarchyChecked},
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        logo::LogoPlugin,
        main_menu::{
            MainMenuPlugin,
            systems::{interactions::handle_main_menu_interactions, setup::setup_main_menu},
        },
        pause::PauseMenuPlugin,
        save_load::SaveLoadUiPlugin,
        settings::SettingsPlugin,
        star_of_david::StarOfDavidPlugin,
        state::StateTransitionContext,
        state::{AppState, GameMenuState},
        state_transitions,
        systems::pause_menu::{interactions::pause_menu_action, setup::setup_pause_menu},
        visibility::MenuVisibilityPlugin,
    },
};

/// Plugin for handling all menu-related functionality
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the states
            .init_state::<AppState>()
            .init_state::<GameMenuState>()
            // Register resources
            .insert_resource(AppState::Menu)
            .insert_resource(StateTransitionContext::default())
            .init_resource::<MenuVisibilityState>()
            // .insert_resource(NeedsMainMenuSetup(true))
            .init_resource::<UiHierarchyChecked>()
            // Setup plugins
            .add_plugins((
                CleanupPlugin,
                MenuVisibilityPlugin,
                BackgroundsPlugin,
                SettingsPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                CreditsPlugin,
                DeckManagerPlugin,
                SaveLoadUiPlugin,
                InputBlockerPlugin,
                StarOfDavidPlugin,
                LogoPlugin,
            ))
            // Schedule camera setup on startup
            .add_systems(Startup, setup_menu_camera)
            // Main Menu systems
            .add_systems(
                OnEnter(GameMenuState::MainMenu),
                (setup_main_menu, ApplyDeferred, setup_main_menu_camera).chain(),
            )
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                cleanup_menu_camera, // Schedule cleanup on exit
            )
            // Loading systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                state_transitions::start_game_loading,
            )
            .add_systems(
                Update,
                state_transitions::check_loading_complete.run_if(in_state(GameMenuState::Loading)),
            )
            // Pause Menu systems
            .add_systems(
                OnEnter(GameMenuState::PauseMenu),
                (setup_menu_camera, ApplyDeferred, setup_pause_menu).chain(),
            )
            .add_systems(
                OnExit(GameMenuState::PauseMenu),
                (cleanup_pause_menu, cleanup_menu_camera).chain(),
            )
            // General Update systems for interactions
            .add_systems(
                Update,
                handle_main_menu_interactions.run_if(in_state(GameMenuState::MainMenu)),
            )
            .add_systems(
                Update,
                pause_menu_action.run_if(in_state(GameMenuState::PauseMenu)),
            );

        info!("Menu plugin registered");
    }
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // Implementation pending
}
