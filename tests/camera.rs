use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use rummage::camera::*;
use rummage::menu::GameCamera;
use std::time::Duration;

#[cfg(test)]
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
        GameCamera,
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
    println!("Initial camera position: {:?}", initial_pos);

    // Set initial cursor position and press middle mouse button
    {
        let world = app.world_mut();
        let mut window = world
            .query_filtered::<&mut Window, With<PrimaryWindow>>()
            .iter_mut(world)
            .next()
            .unwrap();
        let initial_cursor_pos = Vec2::new(0.0, 0.0);
        window.set_cursor_position(Some(initial_cursor_pos));
        println!("Set initial cursor position: {:?}", initial_cursor_pos);
    }
    app.update(); // Let the system process initial cursor position

    // Press middle mouse button
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Middle);
        println!("Middle mouse button pressed");
    }
    app.update(); // Let the system process mouse press

    // Simulate mouse movement in multiple steps
    for i in 1..=5 {
        let step = Vec2::new(80.0, 60.0); // Move 400x300 over 5 steps
        let new_cursor_pos = Vec2::new(0.0, 0.0) + step * i as f32;

        // Update cursor position
        {
            let world = app.world_mut();
            let mut window = world
                .query_filtered::<&mut Window, With<PrimaryWindow>>()
                .iter_mut(world)
                .next()
                .unwrap();
            window.set_cursor_position(Some(new_cursor_pos));
            println!("Step {}: Set cursor position to {:?}", i, new_cursor_pos);
        }
        app.update(); // Let the system process cursor movement

        // Debug: Print current camera position
        {
            let world = app.world_mut();
            let transform = world
                .query_filtered::<&Transform, With<Camera>>()
                .iter(world)
                .next()
                .unwrap();
            println!("Step {}: Camera pos: {:?}", i, transform.translation);
        }

        // Advance time
        {
            let world = app.world_mut();
            let mut time = world.resource_mut::<Time>();
            time.advance_by(Duration::from_secs_f32(0.1));
        }
        app.update(); // Let the system process the movement
    }

    // Release middle mouse button
    {
        let world = app.world_mut();
        world
            .resource_mut::<ButtonInput<MouseButton>>()
            .release(MouseButton::Middle);
        println!("Middle mouse button released");
    }
    app.update(); // Let the system process mouse release

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
    println!("Final camera position: {:?}", final_pos);

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
