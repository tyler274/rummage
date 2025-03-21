use bevy::prelude::*;

use crate::{
    camera::components::GameCamera,
    cards::Card,
    menu::{
        camera::{
            ensure_single_menu_camera, manage_camera_visibility, manage_pause_camera_visibility,
            setup_menu_camera,
        },
        cleanup::{
            cleanup_game, cleanup_main_menu, cleanup_menu_camera, cleanup_pause_menu,
            cleanup_star_of_david_thoroughly,
        },
        components::{
            MenuBackground, MenuCamera, MenuItem, MenuRoot, MenuVisibilityState,
            NeedsMainMenuSetup, UiHierarchyChecked,
        },
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        logo::StarOfDavid,
        main::MainMenuPlugin,
        main_menu::{menu_action, set_menu_camera_zoom},
        pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
        save_load::SaveLoadUiPlugin,
        settings::{SettingsMenuState, SettingsPlugin},
        stars,
        state::GameMenuState,
        state::StateTransitionContext,
        state_transitions,
        systems::{
            check_menu_items_exist,
            debug_menu_visibility,
            detect_ui_hierarchy_issues,
            ensure_menu_item_visibility,
            fix_changed_main_menu_visibility,
            fix_visibility_for_changed_items,
            force_main_menu_items_visibility,
            force_startup_visibility,
            handle_main_menu_interactions,
            log_settings_exit,
            monitor_state_transitions,
            perform_main_menu_setup_if_needed,
            // Main menu systems
            setup_main_menu,
            // Logo systems
            setup_main_menu_star,
            setup_menu_background,
            setup_pause_star,
            // State management systems
            setup_settings_transition,
            update_menu_background,
            // Visibility systems
            update_menu_visibility_state,
        },
        ui,
    },
};

// Import types from the ui module
use crate::menu::ui::{
    MenuVisibilityLogState, MenuVisibilityState as UiMenuVisibilityState, PreviousWindowSize,
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
            .init_resource::<MenuVisibilityLogState>()
            .init_resource::<MenuVisibilityState>()
            .init_resource::<NeedsMainMenuSetup>()
            .init_resource::<UiHierarchyChecked>()
            // Setup plugins
            .add_plugins((
                SettingsPlugin,
                MainMenuPlugin,
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

/// Finishes the game loading process
fn finish_loading() {
    info!("Loading state finished");
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}

/// Creates the logo container for menu items
fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Visibility::Visible,
        ZIndex::default(),    // Ensure we have a ZIndex for proper UI hierarchy
        Transform::default(), // Use Transform instead of TransformBundle
        GlobalTransform::default(), // Add GlobalTransform explicitly
    )
}
