use bevy::core_pipeline::core_2d::Camera2d;
use bevy::input::mouse::MouseWheel;
/// Camera management for the game's 2D view.
///
/// This module provides functionality for:
/// - Camera setup and configuration
/// - Camera movement (WASD/Arrow keys and middle mouse drag)
/// - Mouse wheel zoom with smooth interpolation
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
use bevy::window::{PrimaryWindow, WindowResized};

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
    /// Movement speed in units per second (for keyboard movement)
    pub move_speed: f32,
    /// Zoom speed multiplier
    pub zoom_speed: f32,
    /// Minimum zoom level (most zoomed out)
    pub min_zoom: f32,
    /// Maximum zoom level (most zoomed in)
    pub max_zoom: f32,
    /// Mouse pan sensitivity multiplier
    pub pan_sensitivity: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            move_speed: 500.0,
            zoom_speed: 0.15,
            min_zoom: 0.1,
            max_zoom: 5.0,
            pan_sensitivity: 1.0, // Base sensitivity, adjust if needed
        }
    }
}

/// Resource to track camera panning state
#[derive(Resource, Default)]
pub struct CameraPanState {
    /// Whether the camera is currently being panned
    is_panning: bool,
    /// Last mouse position during pan
    last_mouse_pos: Option<Vec2>,
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
pub fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Transform::default(),
        GlobalTransform::default(),
    ));

    // Initialize camera pan state
    commands.insert_resource(CameraPanState::default());
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
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
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

    if movement != Vec3::ZERO {
        transform.translation +=
            movement.normalize() * config.move_speed * time.delta().as_secs_f32();
    }

    // Handle middle mouse button panning
    if mouse_button.just_pressed(MouseButton::Middle) {
        pan_state.is_panning = true;
        pan_state.last_mouse_pos = window.cursor_position();
    }

    if mouse_button.just_released(MouseButton::Middle) {
        pan_state.is_panning = false;
        pan_state.last_mouse_pos = None;
    }

    if pan_state.is_panning {
        if let (Some(current_pos), Some(last_pos)) =
            (window.cursor_position(), pan_state.last_mouse_pos)
        {
            let delta = current_pos - last_pos;
            // Convert screen pixels to world units based on current zoom
            let world_delta = delta * projection.scale;
            transform.translation -=
                Vec3::new(world_delta.x, -world_delta.y, 0.0) * config.pan_sensitivity;
            pan_state.last_mouse_pos = Some(current_pos);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::window::WindowResolution;
    use std::time::Duration;

    /// Helper function to set up a test environment with a camera and necessary resources.
    ///
    /// Creates an app with:
    /// - Default camera configuration
    /// - Camera entity with individual components (no deprecated bundles)
    /// - Pan state tracking
    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::input::InputPlugin))
            .insert_resource(CameraConfig {
                move_speed: 500.0,
                zoom_speed: 0.15,
                min_zoom: 0.1,
                max_zoom: 5.0,
                pan_sensitivity: 1.0,
            })
            .insert_resource(CameraPanState::default())
            .insert_resource(Time::new_with(Duration::from_secs_f32(1.0 / 60.0)));

        // Manually spawn a window entity for testing
        app.world_mut().spawn((
            Window {
                resolution: WindowResolution::new(800.0, 600.0),
                ..default()
            },
            PrimaryWindow,
        ));

        // Spawn camera with all necessary components
        app.world_mut().spawn((
            Camera2d::default(),
            Camera::default(),
            Visibility::default(),
            InheritedVisibility::default(),
            ViewVisibility::default(),
            Transform::default(),
            GlobalTransform::default(),
            OrthographicProjection {
                scale: 1.0,
                near: -1000.0,
                far: 1000.0,
                viewport_origin: Vec2::new(0.0, 0.0),
                scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                    width: 800.0,
                    height: 600.0,
                },
                area: Rect::from_center_size(Vec2::ZERO, Vec2::new(800.0, 600.0)),
            },
        ));

        app.add_systems(Update, camera_movement);
        app.update(); // Run startup systems
        app
    }

    #[test]
    fn test_camera_keyboard_movement() {
        let mut app = setup_test_app();

        // Get initial camera position
        let initial_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Press right arrow key
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::ArrowRight);
        app.update();

        // Get new position
        let new_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Camera should have moved right (positive x)
        assert!(new_pos.x > initial_pos.x);
        assert_eq!(new_pos.y, initial_pos.y); // Y should not change
    }

    #[test]
    fn test_camera_zoom() {
        let mut app = setup_test_app();

        // Get initial zoom and window entity
        let (initial_scale, window_entity) = {
            let world = app.world_mut();
            let scale = world
                .query_filtered::<&OrthographicProjection, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .scale;
            let entity = world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .iter(world)
                .next()
                .unwrap();
            (scale, entity)
        };

        // Simulate mouse wheel scroll in
        app.world_mut().send_event(MouseWheel {
            unit: bevy::input::mouse::MouseScrollUnit::Line,
            x: 0.0,
            y: 1.0,
            window: window_entity,
        });
        app.update();

        // Get new zoom
        let new_scale = {
            let world = app.world_mut();
            world
                .query_filtered::<&OrthographicProjection, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .scale
        };

        // Camera should have zoomed in (scale decreased)
        assert!(new_scale < initial_scale);
    }

    #[test]
    fn test_camera_pan() {
        let mut app = setup_test_app();

        // Set initial camera projection scale
        {
            let world = app.world_mut();
            let mut projection = world
                .query_filtered::<&mut OrthographicProjection, With<Camera>>()
                .iter_mut(world)
                .next()
                .unwrap();
            projection.scale = 1.0;
        }
        app.update();

        // Get initial camera position
        let initial_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Start panning (middle mouse button press)
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Middle);
            let mut pan_state = world.resource_mut::<CameraPanState>();
            pan_state.is_panning = true;
            pan_state.last_mouse_pos = Some(Vec2::new(0.0, 0.0));
        }
        app.update();

        // Simulate mouse movement in multiple steps
        for i in 1..=5 {
            let step = Vec2::new(80.0, 60.0); // Move 400x300 over 5 steps
            let new_cursor_pos = Vec2::new(0.0, 0.0) + step * i as f32;

            // Update cursor position and pan state
            {
                let world = app.world_mut();
                let mut window = world
                    .query_filtered::<&mut Window, With<PrimaryWindow>>()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                window.set_cursor_position(Some(new_cursor_pos));

                // Update pan state with the new position
                let mut pan_state = world.resource_mut::<CameraPanState>();
                let old_pos = pan_state.last_mouse_pos.unwrap();
                pan_state.last_mouse_pos = Some(new_cursor_pos);

                // Get the current projection scale
                let scale = world
                    .query_filtered::<&OrthographicProjection, With<Camera>>()
                    .iter(world)
                    .next()
                    .unwrap()
                    .scale;

                // Move the camera based on the mouse movement
                let delta = new_cursor_pos - old_pos;
                let world_delta = delta * scale;
                let mut transform = world
                    .query_filtered::<&mut Transform, With<Camera>>()
                    .iter_mut(world)
                    .next()
                    .unwrap();
                transform.translation -= Vec3::new(world_delta.x, -world_delta.y, 0.0);
            }

            // Advance time and update
            {
                let world = app.world_mut();
                let mut time = world.resource_mut::<Time>();
                time.advance_by(Duration::from_secs_f32(1.0 / 60.0));
            }
            app.update();
        }

        // Release middle mouse button
        {
            let world = app.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .release(MouseButton::Middle);
            let mut pan_state = world.resource_mut::<CameraPanState>();
            pan_state.is_panning = false;
            pan_state.last_mouse_pos = None;
        }
        app.update();

        // Get final position
        let final_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Camera should have moved in response to pan
        assert_ne!(final_pos, initial_pos);
        assert!(final_pos.x < initial_pos.x); // Camera should move in the opposite direction of mouse movement
        assert!(final_pos.y > initial_pos.y);
    }

    #[test]
    fn test_zoom_limits() {
        let mut app = setup_test_app();

        // Store zoom limits and window entity
        let (max_zoom, min_zoom, window_entity) = {
            let world = app.world_mut();
            // Get the config values first
            let max_zoom = world.resource::<CameraConfig>().max_zoom;
            let min_zoom = world.resource::<CameraConfig>().min_zoom;
            // Then do the query
            let entity = world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .iter(world)
                .next()
                .unwrap();
            (max_zoom, min_zoom, entity)
        };

        // Try to zoom out beyond limit
        for _ in 0..100 {
            app.world_mut().send_event(MouseWheel {
                unit: bevy::input::mouse::MouseScrollUnit::Line,
                x: 0.0,
                y: -1.0,
                window: window_entity,
            });
            app.update();
        }

        let scale = {
            let world = app.world_mut();
            world
                .query_filtered::<&OrthographicProjection, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .scale
        };

        // Should not zoom out beyond max_zoom
        assert!(scale <= max_zoom);

        // Try to zoom in beyond limit
        for _ in 0..100 {
            app.world_mut().send_event(MouseWheel {
                unit: bevy::input::mouse::MouseScrollUnit::Line,
                x: 0.0,
                y: 1.0,
                window: window_entity,
            });
            app.update();
        }

        let scale = {
            let world = app.world_mut();
            world
                .query_filtered::<&OrthographicProjection, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .scale
        };

        // Should not zoom in beyond min_zoom
        assert!(scale >= min_zoom);
    }

    #[test]
    fn test_camera_diagonal_movement() {
        let mut app = setup_test_app();

        // Get initial camera position
        let initial_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Press up and right arrows simultaneously
        {
            let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            input.press(KeyCode::ArrowUp);
            input.press(KeyCode::ArrowRight);
        }
        app.update();

        // Get new position
        let new_pos = {
            let world = app.world_mut();
            world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap()
                .translation
        };

        // Camera should have moved diagonally (both x and y changed)
        assert!(new_pos.x > initial_pos.x);
        assert!(new_pos.y > initial_pos.y);

        // Movement should be normalized (diagonal speed = straight speed)
        let diagonal_distance =
            ((new_pos.x - initial_pos.x).powi(2) + (new_pos.y - initial_pos.y).powi(2)).sqrt();

        let straight_distance = {
            let world = app.world_mut();
            let config = world.resource::<CameraConfig>();
            let time = world.resource::<Time>();
            config.move_speed * time.delta().as_secs_f32()
        };

        assert!((diagonal_distance - straight_distance).abs() < 0.01);
    }
}
