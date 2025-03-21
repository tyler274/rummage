use bevy::prelude::*;

use crate::{
    cards::Card,
    menu::{
        backgrounds::BackgroundsPlugin,
        cleanup::CleanupPlugin,
        components::{MenuVisibilityState, NeedsMainMenuSetup, UiHierarchyChecked},
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        logo::LogoPlugin,
        main_menu::MainMenuPlugin,
        pause::PauseMenuPlugin,
        save_load::SaveLoadUiPlugin,
        settings::SettingsPlugin,
        star_of_david::StarOfDavidPlugin,
        state::GameMenuState,
        state::StateTransitionContext,
        visibility::VisibilityPlugin,
    },
};

/// Plugin for handling all menu-related functionality
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the menu states
            .init_state::<GameMenuState>()
            // Register resources
            .insert_resource(GameMenuState::MainMenu)
            .insert_resource(StateTransitionContext::default())
            .init_resource::<MenuVisibilityState>()
            .init_resource::<NeedsMainMenuSetup>()
            .init_resource::<UiHierarchyChecked>()
            // Setup plugins
            .add_plugins((
                CleanupPlugin,
                VisibilityPlugin,
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
            ));

        info!("Menu plugin registered");
    }
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // Implementation pending
}
