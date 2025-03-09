use bevy::core_pipeline::core_2d::Camera2d;
use bevy::input::mouse::MouseWheel;
/// Camera management for the game's 2D view.
///
/// This module provides functionality for:
/// - Camera setup and configuration
/// - Camera movement and controls
/// - Viewport management
/// - Coordinate space transformations
///
/// # Important Note for Bevy 0.15.x Compatibility
/// As of Bevy 0.15.x, Camera2dBundle and other *Bundle types are deprecated.
/// Instead, spawn camera entities with individual components:
/// ```ignore
/// // OLD (deprecated):
/// commands.spawn(Camera2dBundle::default());
///
/// // NEW (correct):
/// commands.spawn((
///     Camera2d::default(),
///     Camera::default(),
///     Transform::default(),
///     GlobalTransform::default(),
///     Visibility::default(),
///     InheritedVisibility::default(),
///     ViewVisibility::default(),
/// ));
/// ```
///
/// # Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use rummage::camera::{setup_camera, camera_movement, CameraConfig};
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .insert_resource(CameraConfig::default())
///         .add_systems(Startup, setup_camera)
///         .add_systems(Update, camera_movement)
///         .run();
/// }
/// ```
use bevy::prelude::*;
use bevy::window::WindowResized;

/// Configuration for camera movement and zoom behavior.
///
/// This resource controls how the camera responds to user input,
/// including movement speed and zoom limits.
///
/// # Examples
///
/// ```
/// use rummage::camera::CameraConfig;
///
/// let config = CameraConfig {
///     move_speed: 500.0,
///     zoom_speed: 1.0,
///     min_zoom: 0.1,
///     max_zoom: 10.0,
/// };
///
/// assert!(config.min_zoom < config.max_zoom);
/// ```
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
            zoom_speed: 0.15, // Significantly reduced for much smoother zooming
            min_zoom: 0.1,
            max_zoom: 5.0,
        }
    }
}

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

/// Updates camera position and zoom based on user input.
///
/// This system handles:
/// - WASD/Arrow key movement
/// - Mouse wheel zoom
/// - Zoom limits based on configuration
///
/// Movement speed and zoom limits are controlled by the [`CameraConfig`] resource.
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

    // Handle mouse wheel zoom with smoother interpolation
    let mut zoom_delta = 0.0;
    for event in scroll_events.read() {
        zoom_delta += event.y;
    }

    if zoom_delta != 0.0 {
        // Use a gentler logarithmic scaling for smoother zoom
        let zoom_factor = if zoom_delta > 0.0 {
            1.0 - (config.zoom_speed * zoom_delta.min(1.0))
        } else {
            1.0 + (config.zoom_speed * (-zoom_delta).min(1.0))
        };

        projection.scale = (projection.scale * zoom_factor).clamp(config.min_zoom, config.max_zoom);
    }
}
