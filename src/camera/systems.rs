use bevy::core_pipeline::core_2d::Camera2d;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
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
pub fn setup_camera(mut commands: Commands) {
    // Set up the camera with normal defaults but at a better position to see the cards
    let camera_entity = commands
        .spawn((
            Camera2d::default(),
            Camera {
                order: 0, // Explicitly set order to 0 for game camera
                ..default()
            },
            Visibility::Visible, // Explicitly set to Visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
            // Position the camera directly above the center at Z=999 to see all cards
            Transform::from_xyz(0.0, 0.0, 999.0),
            GlobalTransform::default(),
            GameCamera,
            AppLayer::all_layers(), // Use ALL layers to ensure everything is visible
            Name::new("Game Camera"),
        ))
        .id();

    info!("Game camera spawned with entity {:?}", camera_entity);

    // Initialize camera pan state
    commands.insert_resource(CameraPanState::default());
}

/// Sets the initial zoom level for the camera - called after camera is created
pub fn set_initial_zoom(
    mut query: Query<&mut OrthographicProjection, (With<Camera>, With<GameCamera>)>,
) {
    if let Ok(mut projection) = query.get_single_mut() {
        // Reduce scale for a closer view - smaller values zoom in more
        // In OrthographicProjection, higher scale = more zoomed out
        projection.scale = 0.5; // Changed from 5.0 to 0.5 for much closer view

        info!("Set initial camera zoom level to {:.2}", projection.scale);
    } else {
        warn!("No game camera found when setting initial zoom");
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
        target_scale *= 1.0 - zoom_delta;
    }

    // Clamp the target scale to configured min/max zoom levels
    // Lower scale = more zoomed in, higher scale = more zoomed out
    target_scale = target_scale.clamp(config.min_zoom, config.max_zoom);

    // Smoothly interpolate to the target scale
    // This creates a more natural zoom feel rather than abrupt changes
    let delta = target_scale - projection.scale;
    let interpolation_factor = (config.zoom_interpolation_speed * time.delta_secs()).min(1.0);
    projection.scale += delta * interpolation_factor;
}

/// Draws debug visualization for card positions
pub fn debug_draw_card_positions(
    mut gizmos: Gizmos,
    card_query: Query<(&Transform, &Name), With<crate::cards::Card>>,
) {
    for (transform, name) in card_query.iter() {
        // Draw a circle at each card position
        gizmos.circle_2d(
            transform.translation.truncate(),
            0.5,
            Color::srgba(1.0, 0.0, 0.0, 1.0), // Red color
        );

        // Draw lines connecting adjacent cards
        // This helps visualize the spacing
        if let Some((prev_transform, _)) = card_query
            .iter()
            .filter(|(t, _)| {
                (t.translation.x < transform.translation.x)
                    && (t.translation.y - transform.translation.y).abs() < 0.1
            })
            .max_by(|(a, _), (b, _)| a.translation.x.partial_cmp(&b.translation.x).unwrap())
        {
            gizmos.line_2d(
                prev_transform.translation.truncate(),
                transform.translation.truncate(),
                Color::srgba(1.0, 1.0, 0.0, 1.0), // Yellow color
            );
        }

        // Add debug text for card positions if needed
        // This requires a debug text rendering system
        debug!("Card '{}' position: {:?}", name, transform.translation);
    }
}
