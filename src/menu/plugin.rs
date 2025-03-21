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
        components::{MenuBackground, MenuCamera, MenuItem, MenuRoot},
        components::{MenuVisibilityState, NeedsMainMenuSetup, UiHierarchyChecked},
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        input_blocker::InputBlockerPlugin,
        logo::{StarOfDavid, StarOfDavidPlugin, render_star_of_david},
        main::MainMenuPlugin,
        main_menu::{menu_action, set_menu_camera_zoom, setup_main_menu},
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
use crate::menu::ui::{MenuVisibilityLogState, MenuVisibilityState, PreviousWindowSize};

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
            // Setup systems that run once on startup
            .add_systems(
                Startup,
                (
                    setup_menu_camera,
                    apply_deferred,
                    setup_menu_background,
                    apply_deferred,
                    setup_main_menu_star,
                    apply_deferred,
                    setup_main_menu,
                    apply_deferred,
                    force_startup_visibility,
                )
                    .chain(),
            )
            // Systems that run in main menu state
            .add_systems(
                OnEnter(GameMenuState::MainMenu),
                (
                    perform_main_menu_setup_if_needed,
                    force_main_menu_items_visibility,
                ),
            )
            .add_systems(
                Update,
                handle_main_menu_interactions.run_if(in_state(GameMenuState::MainMenu)),
            )
            // Systems that run when settings is entered
            .add_systems(OnEnter(GameMenuState::Settings), setup_settings_transition)
            .add_systems(OnExit(GameMenuState::Settings), log_settings_exit)
            // Systems that run in paused game state
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_star)
            // Generic menu systems that run in all states
            .add_systems(
                Update,
                (
                    update_menu_visibility_state,
                    debug_menu_visibility,
                    update_menu_background,
                    monitor_state_transitions,
                    check_menu_items_exist,
                    ensure_menu_item_visibility,
                    fix_visibility_for_changed_items,
                    fix_changed_main_menu_visibility.run_if(in_state(GameMenuState::MainMenu)),
                    detect_ui_hierarchy_issues.run_if(resource_equals(UiHierarchyChecked(false))),
                ),
            )
            .add_plugins((
                StarOfDavidPlugin,
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

/// Set up the transition context for settings menu
fn setup_settings_transition(
    mut context: ResMut<StateTransitionContext>,
    current_state: Res<State<GameMenuState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
) {
    info!(
        "Setting up settings transition from state: {:?}, from_pause_menu: {}",
        current_state.get(),
        context.from_pause_menu
    );

    // Always reset from_pause_menu flag when transitioning from MainMenu
    if *current_state.get() == GameMenuState::MainMenu {
        info!("Resetting from_pause_menu flag because we're in MainMenu state");
        context.from_pause_menu = false;
        // Explicitly set the settings origin to MainMenu
        info!("Explicitly setting settings_origin to MainMenu");
        context.settings_origin = Some(GameMenuState::MainMenu);
    } else if context.from_pause_menu || *current_state.get() == GameMenuState::PausedGame {
        // If the flag is set or we're coming from the pause menu, set the origin to PausedGame
        info!("Detected transition from pause menu");
        context.settings_origin = Some(GameMenuState::PausedGame);
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
    info!("Setting SettingsMenuState to Main");
    settings_state.set(SettingsMenuState::Main);
}

/// Setup Star of David for pause menu
fn setup_pause_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
    existing_stars: Query<Entity, With<StarOfDavid>>,
) {
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    // Skip if we already have stars to prevent duplicates
    if !existing_stars.is_empty() {
        info!("Stars of David already exist, skipping creation in pause menu");
        return;
    }

    info!("Setting up Star of David for pause menu");

    // Find the menu camera for text elements
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to pause menu camera: {:?}",
            camera_entity
        );

        // Create a complete entity hierarchy using a single commands operation
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((create_logo(), Name::new("Pause Logo Group")))
                .with_children(|logo_parent| {
                    // Spawn the Star of David with the logo container as parent
                    logo_parent.spawn((create_star_of_david(), Name::new("Pause Star of David")));

                    // Spawn the Hebrew text below the star
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Pause Hebrew Logo Text"),
                    ));

                    // Spawn the English text below the Hebrew text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("Pause English Logo Text"),
                    ));
                });
        });

        info!("Created pause menu logo container with Star of David and text elements");
    } else {
        warn!("No menu camera found for pause menu Star of David and text");

        // If no camera is found, create a standalone UI node hierarchy
        commands
            .spawn((
                // Ensure this is a proper UI node at the root
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::NONE),
                Name::new("Pause Menu Root"),
                MenuRoot,
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((create_logo(), Name::new("Pause Logo Group")))
                    .with_children(|logo_parent| {
                        // Spawn the Star of David with the logo container as parent
                        logo_parent
                            .spawn((create_star_of_david(), Name::new("Pause Star of David")));

                        // Spawn the Hebrew text below the star
                        logo_parent.spawn((
                            create_hebrew_text(&asset_server),
                            Name::new("Pause Hebrew Logo Text"),
                        ));

                        // Spawn the English text below the Hebrew text
                        logo_parent.spawn((
                            create_english_text(&asset_server),
                            Name::new("Pause English Logo Text"),
                        ));
                    });
            });

        info!("Created standalone pause menu logo container with Star of David and text elements");
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

/// Update menu visibility state resource
fn update_menu_visibility_state(
    menu_items: Query<&Visibility, With<MenuItem>>,
    mut menu_state: ResMut<MenuVisibilityState>,
) {
    let total_items = menu_items.iter().count();
    let visible_items = menu_items
        .iter()
        .filter(|visibility| matches!(visibility, Visibility::Visible))
        .count();

    // Only update if changed
    if menu_state.item_count != total_items || menu_state.visible_count != visible_items {
        menu_state.item_count = total_items;
        menu_state.visible_count = visible_items;
    }
}

/// Debug system to check visibility of menu elements
fn debug_menu_visibility(
    menu_cameras: Query<(Entity, &Visibility), (With<MenuCamera>, Changed<Visibility>)>,
    menu_items: Query<(Entity, &Visibility), (With<MenuItem>, Changed<Visibility>)>,
    mut log_state: ResMut<MenuVisibilityLogState>,
    menu_state: Res<MenuVisibilityState>,
) {
    // Count cameras and menu items with changed visibility
    let camera_count = menu_cameras.iter().count();
    let menu_item_count = menu_items.iter().count();

    // Only proceed if any component actually changed
    if camera_count == 0 && menu_item_count == 0 {
        return;
    }

    // Collect current camera states for changed cameras
    for (entity, visibility) in menu_cameras.iter() {
        log_state.camera_states.insert(entity, *visibility);
        debug!(
            "Menu camera {:?} visibility changed to: {:?}",
            entity, visibility
        );
    }

    // Count visible items
    if menu_item_count > 0 {
        // We need to query all items to get the total count when visibility changes
        let all_items = menu_state.item_count;
        let visible_items = menu_state.visible_count;

        // Only log if visibility actually changed
        if visible_items != log_state.last_visible_items {
            log_state.last_item_count = all_items;
            log_state.last_visible_items = visible_items;
            debug!("Total menu items: {}", all_items);
            debug!("Visible menu items: {}/{}", visible_items, all_items);
        }
    }
}

/// Handle cleanup when returning to main menu
#[allow(dead_code)]
fn handle_game_cleanup(_commands: Commands, _cards: Query<Entity, With<Card>>) {
    // ... existing code ...
}

/// System to update the menu background image size based on window dimensions
fn update_menu_background(
    windows: Query<&Window>,
    mut backgrounds: Query<(&mut Node, &mut PreviousWindowSize), With<MenuBackground>>,
    mut missing_size_backgrounds: Query<
        (Entity, &mut Node),
        (With<MenuBackground>, Without<PreviousWindowSize>),
    >,
    mut commands: Commands,
) {
    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let current_width = window.width();
        let current_height = window.height();

        // Get all background image nodes and update their size
        for (mut node, mut prev_size) in &mut backgrounds {
            // Check if window size has changed
            if prev_size.width != current_width || prev_size.height != current_height {
                // Update the UI node size to match the window size exactly
                node.width = Val::Px(current_width);
                node.height = Val::Px(current_height);

                // Update the previous size
                prev_size.width = current_width;
                prev_size.height = current_height;

                // Log window size changes at debug level
                debug!(
                    "Window size changed: {}x{}, updating menu background size",
                    current_width, current_height
                );
            }
        }

        // Add PreviousWindowSize component to any background nodes that don't have it
        for (entity, mut node) in missing_size_backgrounds.iter_mut() {
            // Update the node size
            node.width = Val::Px(current_width);
            node.height = Val::Px(current_height);

            // Add the PreviousWindowSize component
            commands.entity(entity).insert(PreviousWindowSize {
                width: current_width,
                height: current_height,
            });

            debug!(
                "Added PreviousWindowSize component to menu background. Window size: {}x{}",
                current_width, current_height
            );
        }
    }
}

/// Sets up a Star of David for the main menu and attaches it to the menu camera
fn setup_main_menu_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
) {
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    info!("Setting up Star of David for main menu");

    // Check if we have a menu camera to attach to
    if let Some(camera) = menu_cameras.iter().next() {
        info!("Found menu camera for Star of David: {:?}", camera);

        // Create the star of david with text directly under camera
        commands.entity(camera).with_children(|parent| {
            // Create a logo container
            parent
                .spawn((
                    create_logo(),
                    Name::new("Main Menu Logo Container"),
                    MenuItem,
                    Visibility::Visible,
                    GlobalZIndex(30),
                ))
                .with_children(|container| {
                    // Add the Star of David
                    container.spawn((
                        create_star_of_david(),
                        Name::new("Main Menu Star of David"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(31),
                    ));

                    // Add Hebrew text
                    container.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Main Menu Hebrew Text"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(31),
                    ));

                    // Add English text
                    container.spawn((
                        create_english_text(&asset_server),
                        Name::new("Main Menu English Text"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(31),
                    ));
                });
        });

        info!("Created and attached Star of David and text to menu camera");
    } else {
        // No camera found, create standalone logo
        warn!("No menu camera found, creating standalone logo");

        // Create a root node with the logo as its child
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Name::new("Main Menu Logo Root"),
                MenuRoot,
                MenuItem,
                Visibility::Visible,
                GlobalZIndex(30),
            ))
            .with_children(|parent| {
                // Create the logo container as a child of the root
                parent
                    .spawn((
                        create_logo(),
                        Name::new("Main Menu Logo Container"),
                        MenuItem,
                        Visibility::Visible,
                        GlobalZIndex(31),
                    ))
                    .with_children(|container| {
                        // Add the Star of David
                        container.spawn((
                            create_star_of_david(),
                            Name::new("Main Menu Star of David"),
                            MenuItem,
                            Visibility::Visible,
                            GlobalZIndex(32),
                        ));

                        // Add Hebrew text
                        container.spawn((
                            create_hebrew_text(&asset_server),
                            Name::new("Main Menu Hebrew Text"),
                            MenuItem,
                            Visibility::Visible,
                            GlobalZIndex(32),
                        ));

                        // Add English text
                        container.spawn((
                            create_english_text(&asset_server),
                            Name::new("Main Menu English Text"),
                            MenuItem,
                            Visibility::Visible,
                            GlobalZIndex(32),
                        ));
                    });
            });

        info!("Created standalone Star of David and text");
    }
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

/// Resource to track when we need to set up the main menu
#[derive(Resource, Default)]
struct NeedsMainMenuSetup(bool);
