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
            commands.entity(entity).despawn_recursive();
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
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<MenuCameraEntity>();
    info!("Main Menu Camera despawned");
}
