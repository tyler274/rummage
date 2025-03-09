use bevy::core_pipeline::core_2d::Camera2d;
use bevy::input::mouse::MouseWheel;
/// Camera management for the game's 2D view.
///
/// This module handles:
/// - Camera setup and configuration
/// - Camera movement and controls
/// - Viewport management
/// - Coordinate space transformations
use bevy::prelude::*;
use bevy::window::WindowResized;

/// Configuration for camera movement and zoom
#[derive(Resource)]
pub struct CameraConfig {
    /// Movement speed in units per second
    pub move_speed: f32,
    /// Zoom speed multiplier
    pub zoom_speed: f32,
    /// Minimum zoom level (most zoomed out)
    pub min_zoom: f32,
    /// Maximum zoom level (most zoomed in)
    pub max_zoom: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            move_speed: 500.0,
            zoom_speed: 1.0,
            min_zoom: 0.1,
            max_zoom: 10.0,
        }
    }
}

/// Sets up the main game camera with proper scaling and projection
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Transform::default(),
        GlobalTransform::default(),
    ));
}

/// Handles window resize events by maintaining a fixed vertical size and adjusting
/// the horizontal size based on aspect ratio.
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera2d>>,
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

            // Update window surface
            if let Ok(mut window) = windows.get_single_mut() {
                window
                    .resolution
                    .set(resize_event.width, resize_event.height);
            }
        }
    }
}

/// Updates camera position and zoom based on user input
pub fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    config: Res<CameraConfig>,
) {
    let Ok((mut transform, mut projection)) = camera_query.get_single_mut() else {
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

    if movement != Vec3::ZERO {
        transform.translation +=
            movement.normalize() * config.move_speed * time.delta().as_secs_f32();
    }

    // Handle mouse wheel zoom
    let mut zoom_delta = 0.0;
    for event in scroll_events.read() {
        zoom_delta += event.y;
    }

    if zoom_delta != 0.0 {
        projection.scale = (projection.scale * (1.0 - zoom_delta * config.zoom_speed * 0.1))
            .clamp(config.min_zoom, config.max_zoom);
    }
}
