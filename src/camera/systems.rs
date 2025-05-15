use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::system::Local;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::camera::{
    components::{AppLayer, GameCamera},
    config::CameraConfig,
    state::CameraPanState,
};
use crate::menu::state::GameMenuState;

/// Resource to track previously logged card positions to avoid redundant logging
#[derive(Resource, Default)]
pub struct CardPositionCache {
    positions: std::collections::HashMap<Entity, Vec3>,
}

/// Manages game camera visibility based on the current game state
pub fn manage_game_camera_visibility(
    mut game_cameras: Query<&mut Visibility, With<GameCamera>>,
    game_state: Res<State<GameMenuState>>,
) {
    // Determine if the camera should be visible based on game state
    let should_be_visible = matches!(
        *game_state.get(),
        GameMenuState::InGame | GameMenuState::PauseMenu
    );

    // Update camera visibility
    for mut visibility in game_cameras.iter_mut() {
        let new_visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if *visibility != new_visibility {
            info!(
                "Setting game camera {:?} visibility to {:?} in state {:?}",
                "(ID unknown)",
                new_visibility,
                game_state.get()
            );
            *visibility = new_visibility;
        }
    }
}

/// Sets up the main game camera with proper scaling and projection.
///
/// This system spawns a 2D camera entity with the necessary components
/// for rendering the game world. It's typically run during the startup phase.
pub fn setup_camera(mut commands: Commands) {
    info!("Setting up game camera...");

    // Set up the camera with improved position to see all cards clearly
    let camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                order: 0, // Explicitly set order to 0 for game camera
                ..default()
            },
            Visibility::Visible, // Explicitly set to Visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
            // Position the camera looking at the center of the game board
            // We're using a 2D camera, so we need a high Z value to see everything
            Transform::from_xyz(0.0, 0.0, 999.0),
            GlobalTransform::default(),
            GameCamera,
            // Make sure we explicitly include all game-related layers
            RenderLayers::from_layers(&[
                AppLayer::Game.as_usize(),
                AppLayer::Cards.as_usize(),
                AppLayer::GameWorld.as_usize(),
                AppLayer::Background.as_usize(),
                AppLayer::GameUI.as_usize(),
                AppLayer::Shared.as_usize(),
            ]),
            Name::new("Game Camera"),
        ))
        .id();

    info!("Game camera spawned with entity {:?}", camera_entity);
    info!("Camera render layers set to include Cards layer");

    // Initialize camera pan state
    commands.insert_resource(CameraPanState::default());
}

/// Sets the initial zoom level for the camera - called after camera is created
/// Runs in Update until it succeeds once.
pub fn set_initial_zoom(
    mut query: Query<&mut OrthographicProjection, (With<Camera>, With<GameCamera>)>,
    mut initial_zoom_set: Local<bool>, // Track if zoom has been set
) {
    // Only run if the initial zoom hasn't been set yet
    if *initial_zoom_set {
        return;
    }

    if let Ok(mut projection) = query.single_mut() {
        // Use a much wider view to ensure all cards are visible
        // In OrthographicProjection, higher scale = more zoomed out
        // projection.scale = 500.0; // Drastically increased scale to see distant playmats
        // Let's try a much smaller initial scale
        projection.scale = 5.0; // Significantly reduced scale

        info!(
            "Successfully set initial camera zoom level to {:.2}",
            projection.scale
        );
        *initial_zoom_set = true; // Mark as done
    } else {
        // It's okay if the camera isn't found immediately, log minimally
        debug!("Game camera not found yet for initial zoom setting...");
        // Warn removed: warn!("No game camera found when setting initial zoom");
    }
}

/// Handles window resize events by maintaining a fixed vertical size and adjusting
/// the horizontal size based on aspect ratio.
///
/// This system ensures that cards maintain their proper proportions regardless of
/// window size by scaling the camera's projection based on the window dimensions.
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection, (With<Camera2d>, With<GameCamera>)>,
    _windows: Query<&Window>,
    _config: Res<CameraConfig>,
) {
    // Define the desired fixed vertical view size in world units.
    // This could be based on your game's design, e.g., ensuring a certain
    // number of units are always visible vertically. Let's use a value from config or a constant.
    // Assuming CameraConfig has a field like `fixed_vertical_world_units`
    // If not, let's define a reasonable constant for now.
    const FIXED_VERTICAL_VIEW: f32 = 1000.0; // Example: Keep 1000 world units vertically visible

    for resize_event in resize_events.read() {
        if let Ok(mut projection) = projection_query.single_mut() {
            let aspect_ratio = resize_event.width / resize_event.height;
            let new_height = FIXED_VERTICAL_VIEW; // Fixed vertical size
            let new_width = FIXED_VERTICAL_VIEW * aspect_ratio; // Calculate width based on aspect ratio

            // Update the projection's view area
            projection.area = Rect::new(
                -new_width / 2.0,
                -new_height / 2.0,
                new_width / 2.0,
                new_height / 2.0,
            );

            info!(
                "WindowResize: Updated projection area to Rect {{ min: ({:.1}, {:.1}), max: ({:.1}, {:.1}) }} (Window: {}x{}, Aspect: {:.2})",
                projection.area.min.x,
                projection.area.min.y,
                projection.area.max.x,
                projection.area.max.y,
                resize_event.width,
                resize_event.height,
                aspect_ratio
            );

            // Update window surface - with WSL2 error handling
            // REMOVED: Explicitly setting window resolution here can interfere with resizing.
            // Bevy's WindowPlugin should handle updating the Window resource.
            /*
            if let Ok(mut window) = windows.single_mut() {
                // Set the new resolution but don't panic if the surface reconfiguration fails
                // This handles the common Vulkan/WSL2 "Surface does not support the adapter's queue family" error
                let prev_width = window.resolution.width();
                let prev_height = window.resolution.height();

                // Only attempt to update if the size actually changed
                if resize_event.width != prev_width || resize_event.height != prev_height {
                    // Set the new resolution
                    window
                        .resolution
                        .set(resize_event.width, resize_event.height);

                    // Log that we updated the window resolution
                    debug!(
                        "Window resized to {}x{}",
                        resize_event.width, resize_event.height
                    );
                }
            }
            */
        }
    }
}

/// Updates camera position and zoom based on user input.
///
/// This system handles:
/// - WASD/Arrow key movement
/// - Middle mouse button camera panning
/// - Mouse wheel zoom with smooth interpolation
/// - Zoom limits based on configuration
///
/// Camera movement can be controlled in two ways:
/// 1. Keyboard (WASD/Arrow keys) for precise movement
/// 2. Middle mouse button drag for quick panning
///
/// The camera's position is updated based on the current projection scale
/// to maintain consistent movement speed regardless of zoom level.
pub fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<Camera>, With<GameCamera>),
    >,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    config: Res<CameraConfig>,
    mut pan_state: ResMut<CameraPanState>,
) {
    let Ok((mut transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // Handle keyboard movement
    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }

    // Apply movement speed and delta time
    if movement != Vec3::ZERO {
        movement = movement.normalize() * config.move_speed * time.delta_secs();
        // Scale movement by current zoom level to maintain consistent speed
        movement *= projection.scale;
        transform.translation += movement;
    }

    // Handle middle mouse button panning
    if mouse_button.just_pressed(MouseButton::Middle) {
        pan_state.is_panning = true;
        if let Some(cursor_pos) = window.cursor_position() {
            pan_state.last_mouse_pos = Some(cursor_pos);
        }
    } else if mouse_button.just_released(MouseButton::Middle) {
        pan_state.is_panning = false;
        pan_state.last_mouse_pos = None;
    }

    if pan_state.is_panning {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(last_pos) = pan_state.last_mouse_pos {
                let delta = cursor_pos - last_pos;
                let movement = Vec3::new(
                    -delta.x * config.pan_sensitivity * projection.scale,
                    delta.y * config.pan_sensitivity * projection.scale,
                    0.0,
                );
                transform.translation += movement;
                pan_state.last_mouse_pos = Some(cursor_pos);
            }
        }
    }

    // Handle zoom with smooth interpolation
    let mut target_scale = projection.scale;
    for ev in scroll_events.read() {
        let zoom_delta = ev.y * config.zoom_speed;
        target_scale *= 1.0 - zoom_delta;
    }

    // Clamp the target scale to configured min/max zoom levels
    // Lower scale = more zoomed in, higher scale = more zoomed out
    target_scale = target_scale.clamp(config.min_zoom, config.max_zoom);

    // Smoothly interpolate to the target scale
    // This creates a more natural zoom feel rather than abrupt changes
    let delta = target_scale - projection.scale;
    let interpolation_factor = (config.zoom_interpolation_speed * time.delta_secs()).min(1.0);
    let final_scale = projection.scale + delta * interpolation_factor;
    if (final_scale - projection.scale).abs() > f32::EPSILON {
        info!(
            "CameraMovement: Target Scale: {:.2}, Current Scale: {:.2}, Final Applied Scale: {:.2}",
            target_scale, projection.scale, final_scale
        );
    }
    projection.scale = final_scale;
}

/// Draws debug visualization for card positions
pub fn debug_draw_card_positions(
    mut gizmos: Gizmos,
    card_query: Query<(Entity, &Transform, &Name), With<crate::cards::Card>>,
    mut position_cache: Local<CardPositionCache>,
) {
    // Draw center indicator - a large crosshair at origin
    gizmos.line_2d(
        Vec2::new(-100.0, 0.0),
        Vec2::new(100.0, 0.0),
        Color::srgba(0.0, 1.0, 0.0, 1.0), // Green horizontal line
    );
    gizmos.line_2d(
        Vec2::new(0.0, -100.0),
        Vec2::new(0.0, 100.0),
        Color::srgba(0.0, 1.0, 0.0, 1.0), // Green vertical line
    );

    // Draw a circle at origin
    gizmos.circle_2d(
        Vec2::ZERO,
        50.0,                             // Larger circle at origin
        Color::srgba(0.0, 1.0, 0.0, 0.3), // Semi-transparent green
    );

    // Draw all cards
    for (entity, transform, name) in card_query.iter() {
        // Draw a larger circle at each card position
        gizmos.circle_2d(
            transform.translation.truncate(),
            20.0,                             // Much larger circle to spot cards easily
            Color::srgba(1.0, 0.0, 0.0, 0.7), // Brighter red color
        );

        // Only log if the position changed significantly or is new
        let current_pos = transform.translation;
        let should_log = match position_cache.positions.get(&entity) {
            Some(prev_pos) => (*prev_pos - current_pos).length_squared() > 0.01,
            None => true, // Always log new cards
        };

        if should_log {
            debug!("Card '{}' position changed to: {:?}", name, current_pos);
            position_cache.positions.insert(entity, current_pos);
        }
    }
}
