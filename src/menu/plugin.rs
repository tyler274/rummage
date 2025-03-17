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
        main_menu::{menu_action, set_menu_camera_zoom, setup_main_menu, MenuBackground},
        pause_menu::{handle_pause_input, pause_menu_action, setup_pause_menu},
        settings::{SettingsMenuState, SettingsPlugin},
        state::{GameMenuState, StateTransitionContext},
    },
};

/// Resource to control logging frequency for menu visibility
#[derive(Resource)]
struct MenuVisibilityLogState {
    last_log_time: f32,
    log_interval: f32,
    last_camera_count: usize,
    last_item_count: usize,
    last_visible_items: usize,
    camera_states: std::collections::HashMap<Entity, Visibility>,
}

impl Default for MenuVisibilityLogState {
    fn default() -> Self {
        Self {
            last_log_time: 0.0,
            log_interval: 30.0, // Log every 30 seconds at most
            last_camera_count: 0,
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
                (menu_action, render_star_of_david, debug_menu_visibility, update_menu_background)
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
fn setup_pause_star(mut commands: Commands) {
    use crate::menu::logo::create_star_of_david;
    commands.spawn(create_star_of_david());
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

/// Debug system to check visibility of menu elements
fn debug_menu_visibility(
    menu_cameras: Query<(Entity, &Visibility), With<MenuCamera>>,
    menu_items: Query<(Entity, &Visibility), With<MenuItem>>,
    time: Res<Time>,
    mut log_state: ResMut<MenuVisibilityLogState>,
) {
    // Count cameras and menu items
    let camera_count = menu_cameras.iter().count();
    let menu_item_count = menu_items.iter().count();
    let visible_items = menu_items
        .iter()
        .filter(|(_, visibility)| matches!(visibility, Visibility::Visible))
        .count();

    // Collect current camera states
    let mut current_camera_states = std::collections::HashMap::new();
    for (entity, visibility) in menu_cameras.iter() {
        current_camera_states.insert(entity, *visibility);
    }

    // Check if camera states have changed
    let camera_state_changed = camera_count != log_state.last_camera_count
        || current_camera_states != log_state.camera_states;

    // Check if menu items have changed
    let menu_items_changed = menu_item_count != log_state.last_item_count
        || visible_items != log_state.last_visible_items;

    // Determine if there's been any meaningful change worth logging
    let state_changed = camera_state_changed || menu_items_changed;

    // Check if we should log based on time interval or state changes
    let current_time = time.elapsed_secs();
    let force_log_by_time = current_time - log_state.last_log_time > log_state.log_interval;
    let should_log = force_log_by_time || state_changed;

    // If we need to log, update state and output logs
    if should_log {
        log_state.last_log_time = current_time;
        log_state.last_camera_count = camera_count;
        log_state.last_item_count = menu_item_count;
        log_state.last_visible_items = visible_items;
        log_state.camera_states = current_camera_states;

        // Log camera visibility (at debug level to reduce spam)
        for (entity, visibility) in menu_cameras.iter() {
            debug!("Menu camera {:?} visibility: {:?}", entity, visibility);
        }

        debug!("Total menu items: {}", menu_item_count);
        debug!("Visible menu items: {}/{}", visible_items, menu_item_count);
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
    mut backgrounds: Query<&mut Node, With<MenuBackground>>,
) {
    // Get the primary window
    if let Ok(window) = windows.get_single() {
        // Get all background image nodes and update their size
        for mut node in &mut backgrounds {
            // Set the UI node size to match the window size exactly
            node.width = Val::Px(window.width());
            node.height = Val::Px(window.height());
            
            // Log window size changes at debug level to avoid log spam
            debug!(
                "Window size: {}x{}, updating menu background size",
                window.width(),
                window.height()
            );
        }
    }
}
