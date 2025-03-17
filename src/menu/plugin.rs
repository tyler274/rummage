use bevy::{ecs::system::ParamSet, prelude::*};

use crate::{
    camera::components::{AppLayer, GameCamera, MenuCamera},
    cards::Card,
    menu::{
        cleanup::{
            cleanup_game, cleanup_main_menu, cleanup_menu_camera, cleanup_pause_menu,
            cleanup_star_of_david_thoroughly,
        },
        components::MenuItem,
        credits::CreditsPlugin,
        deck::DeckManagerPlugin,
        logo::{StarOfDavidPlugin, render_star_of_david},
        main::MainMenuPlugin,
        main_menu::{MenuBackground, menu_action, set_menu_camera_zoom, setup_main_menu},
        pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
        settings::{SettingsMenuState, SettingsPlugin},
        state::{GameMenuState, StateTransitionContext},
    },
};

/// Tracks the previous window size to detect changes
#[derive(Component)]
pub struct PreviousWindowSize {
    pub width: f32,
    pub height: f32,
}

/// Resource to track menu visibility state
#[derive(Resource, Default)]
pub struct MenuVisibilityState {
    pub item_count: usize,
    pub visible_count: usize,
}

/// Resource to control logging frequency for menu visibility
#[derive(Resource)]
struct MenuVisibilityLogState {
    last_item_count: usize,
    last_visible_items: usize,
    camera_states: std::collections::HashMap<Entity, Visibility>,
}

impl Default for MenuVisibilityLogState {
    fn default() -> Self {
        Self {
            last_item_count: 0,
            last_visible_items: 0,
            camera_states: std::collections::HashMap::new(),
        }
    }
}

/// Plugin that sets up the menu system and its related systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameMenuState>()
            .insert_resource(GameMenuState::MainMenu)
            .insert_resource(StateTransitionContext::default())
            .init_resource::<MenuVisibilityLogState>()
            .init_resource::<MenuVisibilityState>()
            .add_plugins((
                StarOfDavidPlugin,
                SettingsPlugin,
                MainMenuPlugin,
                CreditsPlugin,
                DeckManagerPlugin,
            ))
            // Main Menu state
            .add_systems(
                OnEnter(GameMenuState::MainMenu),
                (
                    cleanup_game,
                    cleanup_menu_camera,
                    cleanup_star_of_david_thoroughly,
                    setup_main_menu,
                    setup_menu_camera,
                    set_menu_camera_zoom,
                    ensure_single_menu_camera,
                    setup_main_menu_star,
                )
                    .chain(),
            )
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                (
                    cleanup_main_menu,
                    cleanup_menu_camera,
                    cleanup_star_of_david_thoroughly,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    menu_action,
                    render_star_of_david,
                    update_menu_visibility_state,
                    debug_menu_visibility,
                    update_menu_background,
                )
                    .run_if(in_state(GameMenuState::MainMenu)),
            )
            // Loading state systems
            .add_systems(
                OnEnter(GameMenuState::Loading),
                (
                    cleanup_game,
                    cleanup_menu_camera,
                    cleanup_star_of_david_thoroughly,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                start_game_loading.run_if(in_state(GameMenuState::Loading)),
            )
            .add_systems(OnExit(GameMenuState::Loading), finish_loading)
            // Pause menu systems
            .add_systems(
                OnEnter(GameMenuState::PausedGame),
                (
                    cleanup_menu_camera,
                    cleanup_star_of_david_thoroughly,
                    setup_pause_menu,
                    setup_menu_camera,
                    ensure_single_menu_camera,
                    manage_pause_camera_visibility,
                    setup_pause_star,
                ),
            )
            .add_systems(
                OnExit(GameMenuState::PausedGame),
                (cleanup_pause_menu, cleanup_star_of_david_thoroughly).chain(),
            )
            .add_systems(
                Update,
                (pause_menu_action, render_star_of_david)
                    .run_if(in_state(GameMenuState::PausedGame)),
            )
            // Settings menu systems
            .add_systems(
                OnEnter(GameMenuState::Settings),
                (
                    cleanup_pause_menu,
                    setup_menu_camera,
                    ensure_single_menu_camera,
                    setup_settings_transition,
                ),
            )
            .add_systems(
                OnExit(GameMenuState::Settings),
                |mut settings_state: ResMut<NextState<SettingsMenuState>>| {
                    info!("Exiting settings, resetting SettingsMenuState to Disabled");
                    settings_state.set(SettingsMenuState::Disabled);
                },
            )
            // InGame state systems
            .add_systems(
                OnEnter(GameMenuState::InGame),
                (
                    cleanup_menu_camera,
                    manage_camera_visibility,
                    cleanup_star_of_david_thoroughly,
                )
                    .chain(),
            )
            .add_systems(Update, handle_pause_input);
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

/// Creates a menu camera with the proper configuration
pub fn setup_menu_camera(
    mut commands: Commands,
    existing_cameras: Query<Entity, With<MenuCamera>>,
) {
    // Check if any menu cameras already exist
    if !existing_cameras.is_empty() {
        info!("Menu camera already exists, not creating a new one");
        return;
    }

    info!("Setting up menu camera");
    let entity = commands
        .spawn((
            Camera {
                order: 1,
                ..default()
            },
            Camera2d,
            MenuCamera,
            AppLayer::menu_layers(),
            Name::new("Menu Camera"),
        ))
        .id();

    info!("Spawned menu camera: {:?}", entity);
}

/// Setup Star of David for pause menu
fn setup_pause_star(
    mut commands: Commands,
    menu_cameras: Query<Entity, With<MenuCamera>>,
    asset_server: Res<AssetServer>,
) {
    use crate::menu::components::MenuDecorativeElement;
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    // Find the menu camera
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to pause menu camera: {:?}",
            camera_entity
        );

        // Spawn the logo group as a child of the menu camera
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((
                    create_logo(),
                    Name::new("Pause Logo Group"),
                    MenuDecorativeElement,
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David
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
    } else {
        warn!("No menu camera found for Star of David and text attachment");

        // If no camera is found, still spawn the logo group
        commands
            .spawn((
                create_logo(),
                Name::new("Pause Logo Group"),
                MenuDecorativeElement,
            ))
            .with_children(|logo_parent| {
                // Spawn the Star of David
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
    }
}

/// Ensures only one menu camera exists
fn ensure_single_menu_camera(
    mut commands: Commands,
    menu_cameras: Query<(Entity, &Camera), With<MenuCamera>>,
) {
    let camera_count = menu_cameras.iter().count();
    if camera_count > 1 {
        warn!(
            "Found {} menu cameras, cleaning up duplicates",
            camera_count
        );

        // Find the first camera and keep track of its details
        let mut cameras_to_remove = Vec::new();
        let mut highest_order = None;
        let mut highest_order_entity = None;

        // First pass: find the camera with the highest order
        for (entity, camera) in menu_cameras.iter() {
            if let Some(order) = highest_order {
                if camera.order > order {
                    highest_order = Some(camera.order);
                    highest_order_entity = Some(entity);
                } else {
                    cameras_to_remove.push(entity);
                }
            } else {
                highest_order = Some(camera.order);
                highest_order_entity = Some(entity);
            }
        }

        // Second pass: remove all cameras except the one with highest order
        for entity in cameras_to_remove {
            info!("Removing duplicate menu camera entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        if let Some(entity) = highest_order_entity {
            info!(
                "Keeping menu camera entity: {:?} with order: {:?}",
                entity, highest_order
            );
        }
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

    // Check for camera ambiguities and clean them up if found
    let camera_count = game_cameras.iter().count();
    if camera_count > 1 {
        warn!(
            "Found {} game cameras, cleaning up duplicates",
            camera_count
        );
        // Keep only the first camera and despawn the rest
        let mut first_found = false;
        for entity in game_cameras.iter() {
            if !first_found {
                first_found = true;
                info!("Keeping game camera entity: {:?}", entity);
            } else {
                info!("Removing duplicate game camera entity: {:?}", entity);
                commands.entity(entity).despawn_recursive();
            }
        }
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

/// Ensures proper camera visibility when entering the InGame state
fn manage_camera_visibility(
    mut params: ParamSet<(
        Query<(Entity, &mut Visibility, &mut Camera), With<GameCamera>>,
        Query<(Entity, &mut Visibility), With<MenuCamera>>,
        Query<(Entity, &mut Camera), With<MenuCamera>>,
    )>,
    context: Res<StateTransitionContext>,
) {
    info!("Managing camera visibility in InGame state");
    info!("Found {} game cameras", params.p0().iter().count());
    info!("Found {} menu cameras", params.p1().iter().count());
    info!("Coming from pause menu: {}", context.from_pause_menu);

    // Set all game cameras to visible and ensure they have a unique order
    let mut current_order = 0;
    for (entity, mut visibility, mut camera) in params.p0().iter_mut() {
        *visibility = Visibility::Inherited;
        if camera.order != current_order {
            info!(
                "Setting game camera {:?} order from {} to {}",
                entity, camera.order, current_order
            );
            camera.order = current_order;
        }
        current_order += 1;
    }

    // Hide all menu cameras
    for (entity, mut visibility) in params.p1().iter_mut() {
        info!("Setting menu camera {:?} to Hidden", entity);
        *visibility = Visibility::Hidden;
    }
}

/// Ensures proper camera visibility when entering the PausedGame state
fn manage_pause_camera_visibility(
    mut params: ParamSet<(
        Query<(Entity, &mut Visibility, &mut Camera), With<GameCamera>>,
        Query<(Entity, &mut Visibility), With<MenuCamera>>,
        Query<(Entity, &mut Camera), With<MenuCamera>>,
    )>,
) {
    info!("Managing camera visibility in PausedGame state");
    info!("Found {} game cameras", params.p0().iter().count());
    info!("Found {} menu cameras", params.p1().iter().count());

    // Set all game cameras to hidden
    for (_, mut visibility, _) in params.p0().iter_mut() {
        *visibility = Visibility::Hidden;
    }

    // Check if we have multiple menu cameras (warning condition)
    let menu_camera_count = params.p1().iter().count();
    if menu_camera_count > 1 {
        warn!(
            "Found {} menu cameras when there should only be one!",
            menu_camera_count
        );
    }

    // Set all menu cameras to visible and ensure they have order = 2
    for (entity, mut visibility) in params.p1().iter_mut() {
        info!("Setting menu camera {:?} to Visible", entity);
        *visibility = Visibility::Inherited;
    }

    // Set all menu camera orders to 2
    for (entity, mut camera) in params.p2().iter_mut() {
        if camera.order != 2 {
            info!(
                "Setting menu camera {:?} order from {} to 2",
                entity, camera.order
            );
            camera.order = 2;
        }
    }
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
    use crate::menu::components::MenuDecorativeElement;
    use crate::menu::logo::{
        create_english_text, create_hebrew_text, create_logo, create_star_of_david,
    };

    // Find the menu camera
    if let Some(camera_entity) = menu_cameras.iter().next() {
        info!(
            "Attaching Star of David and text to main menu camera: {:?}",
            camera_entity
        );

        // Spawn the logo group as a child of the menu camera
        commands.entity(camera_entity).with_children(|parent| {
            // Create a parent entity that will contain the star and text elements
            parent
                .spawn((
                    create_logo(),
                    Name::new("Logo Group"),
                    MenuDecorativeElement,
                ))
                .with_children(|logo_parent| {
                    // Spawn the Star of David
                    logo_parent.spawn((create_star_of_david(), Name::new("Star of David")));

                    // Spawn the Hebrew text below the star
                    logo_parent.spawn((
                        create_hebrew_text(&asset_server),
                        Name::new("Hebrew Logo Text"),
                    ));

                    // Spawn the English text below the Hebrew text
                    logo_parent.spawn((
                        create_english_text(&asset_server),
                        Name::new("English Logo Text"),
                    ));
                });
        });
    } else {
        warn!("No menu camera found for Star of David and text attachment");

        // If no camera is found, still spawn the logo group
        commands
            .spawn((
                create_logo(),
                Name::new("Logo Group"),
                MenuDecorativeElement,
            ))
            .with_children(|logo_parent| {
                // Spawn the Star of David
                logo_parent.spawn((create_star_of_david(), Name::new("Star of David")));

                // Spawn the Hebrew text below the star
                logo_parent.spawn((
                    create_hebrew_text(&asset_server),
                    Name::new("Hebrew Logo Text"),
                ));

                // Spawn the English text below the Hebrew text
                logo_parent.spawn((
                    create_english_text(&asset_server),
                    Name::new("English Logo Text"),
                ));
            });
    }
}
