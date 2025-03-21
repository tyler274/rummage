use bevy::prelude::*;

use crate::{
    camera::components::GameCamera,
    cards::Card,
    menu::{
        components::{MenuVisibilityState, NeedsMainMenuSetup, UiHierarchyChecked},
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        main_menu::MainMenuPlugin,
        pause::PauseMenuPlugin,
        save_load::SaveLoadUiPlugin,
        settings::SettingsPlugin,
        state::GameMenuState,
        state::StateTransitionContext,
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
                SettingsPlugin,
                MainMenuPlugin,
                PauseMenuPlugin,
                CreditsPlugin,
                DeckManagerPlugin,
                SaveLoadUiPlugin,
                InputBlockerPlugin,
            ));

        info!("Menu plugin registered");
    }
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

    // Normal loading process
    info!("Checking game state for transition...");
    info!("Number of cards: {}", cards.iter().count());
    info!("Number of game cameras: {}", game_cameras.iter().count());

    // Force cleanup if any game cameras or cards remain
    if !game_cameras.is_empty() {
        warn!(
            "Cleaning up {} remaining game cameras",
            game_cameras.iter().count()
        );
        for entity in game_cameras.iter() {
            info!("Force despawning game camera entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    if !cards.is_empty() {
        warn!("Cleaning up {} remaining cards", cards.iter().count());
        for entity in cards.iter() {
            info!("Force despawning card entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    // Transition to InGame state
    info!("Transitioning to InGame state...");
    next_state.set(GameMenuState::InGame);
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // Implementation pending
}
