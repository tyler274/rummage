use bevy::core_pipeline::core_2d::Camera2d;
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::view::ColorGrading;
use bevy::render::view::RenderLayers;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::camera::{
    components::{AppLayer, GameCamera},
    config::CameraConfig,
    state::CameraPanState,
};

/// Sets up the main game camera with proper scaling and projection.
///
/// This system spawns a 2D camera entity with the necessary components
/// for rendering the game world. It's typically run during the startup phase.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::camera::setup_camera;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_systems(Startup, setup_camera)
///         .run();
/// }
/// ```
pub fn setup_camera(commands: &mut Commands) {
    // Set up the camera with normal defaults
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 0,  // Explicitly set order to 0 for game camera
            hdr: true, // Enable HDR rendering for better visual quality and bloom effects
            ..default()
        },
        // Configure tonemapping for HDR
        Tonemapping::TonyMcMapface, // The default - good balance for most scenes
        // Add bloom effect for bright areas (subtle by default)
        BloomSettings::default(),
        // Configure color grading
        ColorGrading::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Transform::default(),
        GlobalTransform::default(),
        GameCamera,
        AppLayer::game_layers(), // Use combined game layers to see all game elements including cards
    ));

    // Initialize camera pan state
    commands.insert_resource(CameraPanState::default());
}

/// Sets the initial zoom level for the camera - called after camera is created
pub fn set_initial_zoom(
    mut query: Query<&mut OrthographicProjection, (With<Camera>, With<GameCamera>)>,
) {
    if let Ok(mut projection) = query.get_single_mut() {
        // Set to 2.0 for a view that's twice as zoomed out
        // In OrthographicProjection, higher scale = more zoomed out
        projection.scale = 2.0;
    }
}

/// Handles window resize events by maintaining a fixed vertical size and adjusting
/// the horizontal size based on aspect ratio.
///
/// This system ensures that cards maintain their proper proportions regardless of
/// window size by scaling the camera's projection based on the window dimensions.
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::camera::handle_window_resize;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_systems(Update, handle_window_resize)
///         .run();
/// }
/// ```
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection, (With<Camera2d>, With<GameCamera>)>,
    mut windows: Query<&mut Window>,
) {
    for resize_event in resize_events.read() {
        if let Ok(mut projection) = projection_query.get_single_mut() {
            // Scale based on card height (936px) relative to window height
            // We want the card to take up roughly 1/3 of the screen height
            let target_card_height = 936.0;
            let aspect = resize_event.width / resize_event.height;
            // Scale by aspect ratio to maintain card proportions across different window sizes
            projection.scale = (target_card_height / resize_event.height) * 2.0 * aspect;

            // Update window surface - with WSL2 error handling
            if let Ok(mut window) = windows.get_single_mut() {
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
        }
    }
}

/// Safely handles WindowResized events in WSL2 environments to prevent surface reconfiguration errors
/// and performs additional error handling to prevent crashes during window resize operations.
/// This is especially important for WSL2 where window handling can be unstable.
pub fn safe_wsl2_resize_handler(
    mut resize_events: EventReader<WindowResized>,
    mut window_query: Query<(Entity, &mut Window)>,
) {
    // Under WSL2, we need to be extremely cautious with resize events, as they can lead to
    // "Broken pipe" errors if handled incorrectly

    // Count events - too many could indicate a storm of events that might crash the app
    let event_count = resize_events.read().count();

    // If we're getting too many events at once, this is a red flag
    if event_count > 3 {
        warn!(
            "Received {} resize events in a single frame - throttling to prevent WSL2 crashes",
            event_count
        );
        // Just clear the events and return - this prevents overwhelming the window system
        resize_events.clear();
        return;
    }

    // Buffer a small amount of time between resize operations to let the window system stabilize
    static mut LAST_RESIZE_TIME: Option<std::time::Instant> = None;
    let now = std::time::Instant::now();

    // Unsafe block to access the static variable - this is simple enough to be safe in our context
    let should_process = unsafe {
        if let Some(last_time) = LAST_RESIZE_TIME {
            // Only process resize events if sufficient time has passed (50ms)
            if now.duration_since(last_time).as_millis() > 50 {
                LAST_RESIZE_TIME = Some(now);
                true
            } else {
                false
            }
        } else {
            // First resize event, so process it
            LAST_RESIZE_TIME = Some(now);
            true
        }
    };

    if !should_process {
        // Skip processing if we recently handled a resize
        resize_events.clear();
        return;
    }

    // Process at most one resize event per frame to prevent overwhelming the window system
    if let Some(resize_event) = resize_events.read().next() {
        let new_size = Vec2::new(resize_event.width, resize_event.height);

        // Validate size - don't allow degenerate dimensions
        if new_size.x <= 10.0 || new_size.y <= 10.0 || new_size.x >= 8000.0 || new_size.y >= 8000.0
        {
            warn!("Ignoring extreme window size: {:?}", new_size);
            return;
        }

        // Find the window that triggered this event
        for (entity, mut window) in &mut window_query {
            if entity == resize_event.window {
                // Get current size to see if there's a meaningful change
                let current_size = Vec2::new(window.resolution.width(), window.resolution.height());

                // Only react to significant changes to avoid unnecessary surface reconfiguration
                if (new_size - current_size).length_squared() > 100.0 {
                    info!(
                        "Updating window size from {:?} to {:?}",
                        current_size, new_size
                    );

                    // Use a try/catch pattern to handle potential errors from the resize
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        window.resolution.set(new_size.x, new_size.y);
                    })) {
                        Ok(_) => debug!("Window resize successful"),
                        Err(_) => error!("Failed to resize window - ignoring to prevent crash"),
                    }
                } else {
                    // Change too small to care about
                    debug!("Ignoring minor resize event: change too small");
                }

                // Only process one window to avoid overwhelming the system
                break;
            }
        }
    }

    // Clear any remaining events
    resize_events.clear();
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
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::camera::{camera_movement, CameraConfig};
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .insert_resource(CameraConfig::default())
///         .add_systems(Update, camera_movement)
///         .run();
/// }
/// ```
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
    let Ok((mut transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    let Ok(window) = windows.get_single() else {
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
        target_scale *= (1.0 - zoom_delta);
    }
    // Clamp the target scale
    target_scale = target_scale.clamp(config.min_zoom, config.max_zoom);

    // Smoothly interpolate to the target scale
    let delta = target_scale - projection.scale;
    let interpolation_factor = (config.zoom_interpolation_speed * time.delta_secs()).min(1.0);
    projection.scale += delta * interpolation_factor;
}
