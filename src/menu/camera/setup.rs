use bevy::prelude::*;

/// Component marker for menu cameras
#[derive(Component, Debug)]
pub struct MenuCamera;

#[derive(Resource)]
pub struct MenuCameraEntity(pub Entity);

/// Sets up a dedicated camera for the menu
pub fn setup_menu_camera(
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Camera), With<MenuCamera>>,
) {
    const MENU_CAMERA_ORDER: isize = isize::MAX / 2;

    if let Ok((_entity, mut camera)) = cameras.get_single_mut() {
        if camera.order != MENU_CAMERA_ORDER {
            info!(
                "Updating existing menu camera order to {}",
                MENU_CAMERA_ORDER
            );
            camera.order = MENU_CAMERA_ORDER;
        }
        return;
    }

    let count = cameras.iter().count();
    if count > 1 {
        warn!(
            "Multiple ({}) MenuCamera entities found. Setting order for all to {}.",
            count, MENU_CAMERA_ORDER
        );
        for (_entity, mut camera) in cameras.iter_mut() {
            camera.order = MENU_CAMERA_ORDER;
        }
        return;
    }

    info!(
        "Setting up new menu camera (none found in the query) with order {}",
        MENU_CAMERA_ORDER
    );
    let camera_entity = commands
        .spawn((
            Camera2d::default(),
            Camera {
                order: MENU_CAMERA_ORDER,
                ..default()
            },
            MenuCamera,
            Name::new("Menu Camera"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(10.0),
                ..default()
            },
            crate::camera::components::AppLayer::menu_layers(),
        ))
        .id();

    info!(
        "Menu camera created with order {} and entity {:?}",
        MENU_CAMERA_ORDER, camera_entity
    );
}

/// Cleans up the menu camera
pub fn cleanup_menu_camera(mut commands: Commands, cameras: Query<Entity, With<MenuCamera>>) {
    let count = cameras.iter().count();
    if count > 0 {
        info!("Cleaning up {} menu cameras", count);
        for entity in cameras.iter() {
            commands.entity(entity).despawn();
        }
    }
}

/// Ensure this camera renders above all other cameras.
pub fn setup_main_menu_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<MenuCamera>>,
) {
    match camera_query.get_single() {
        Ok(camera_entity) => {
            commands.entity(camera_entity).insert(Camera {
                order: isize::MAX - 1,
                ..default()
            });
            commands.insert_resource(MenuCameraEntity(camera_entity));
            info!(
                "Main Menu Camera setup complete for entity {:?}",
                camera_entity
            );
        }
        Err(e) => {
            warn!(
                "Failed to get single MenuCamera entity in setup_main_menu_camera: {}. This might be okay if the camera is created later.",
                e
            );
        }
    }
}

/// System to despawn the main menu camera
pub fn despawn_menu_camera(mut commands: Commands, camera_query: Query<Entity, With<MenuCamera>>) {
    for entity in camera_query.iter() {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<MenuCameraEntity>();
    info!("Main Menu Camera despawned");
}

/// Set initial zoom level for the menu camera
#[allow(dead_code)]
pub fn set_initial_zoom(
    mut cameras: Query<(Entity, &mut Camera), (With<Camera2d>, With<MenuCamera>)>,
    mut ran: Local<bool>,
) {
    if *ran {
        return;
    }
    if let Ok((_entity, mut camera)) = cameras.single_mut() {
        camera.scale = INITIAL_ZOOM_LEVEL;
        *ran = true;
    }
}

/// System to handle window resize events for the menu camera
pub fn handle_window_resize(
    mut commands: Commands,
    config: Res<MenuConfig>,
    camera_query: Query<(Entity, &Transform), With<MenuCamera>>,
    window_query: Query<&Window>,
) {
    match camera_query.single() {
        Ok((camera_entity, camera_transform)) => {
            let window = window_query.single();
            let cursor_pos_world = screen_to_world_coordinates(
                camera_transform.translation,
                camera_transform.scale,
                window.width(),
                window.height(),
                config.screen_to_world_scale,
                config.screen_to_world_offset,
            );
            // Implement the logic to handle window resize events
        }
        Err(e) => {
            warn!(
                "Failed to get single MenuCamera entity in handle_window_resize: {}. This might be okay if the camera is created later.",
                e
            );
        }
    }
}

/// System to update menu camera transform based on mouse position
#[allow(dead_code)]
pub fn menu_camera_system(
    windows: Query<&Window>,
    mut camera_query: Query<&mut Transform, With<MenuCamera>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    config: Res<MenuConfig>,
    mut last_drag_position: Local<Option<Vec2>>,
    current_menu_state: Res<State<MenuState>>,
    current_app_state: Res<State<AppState>>,
    camera_movement_state: Res<CameraMovementState>,
) {
    // Skip if not in the main menu or settings, or if camera movement is locked
    if (*current_app_state != AppState::MainMenu && *current_app_state != AppState::Settings)
        || camera_movement_state.locked
    {
        return;
    }

    let window = windows.single(); // Assuming a single window
    if let Ok(mut transform) = camera_query.single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            // Implement the logic to update the camera transform based on mouse position
        }
    }
}
