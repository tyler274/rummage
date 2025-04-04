use bevy::prelude::*;

/// Component marker for menu cameras
#[derive(Component, Debug)]
pub struct MenuCamera;

#[derive(Resource)]
pub struct MenuCameraEntity(pub Entity);

/// Sets up a dedicated camera for the menu
pub fn setup_menu_camera(
    mut commands: Commands,
    cameras: Query<(Entity, Option<&Camera>), With<MenuCamera>>,
    all_cameras: Query<&Camera>,
) {
    // Check if a menu camera already exists
    // Instead of removing existing cameras, we'll update their order
    if !cameras.is_empty() {
        info!("Menu camera already exists, will update camera order");

        // Find the highest camera order from all existing cameras
        let mut highest_order = 0;
        for camera in all_cameras.iter() {
            if camera.order > highest_order {
                highest_order = camera.order;
            }
        }

        // Update all menu cameras to ensure unique orders
        for (entity, _) in cameras.iter() {
            let new_order = highest_order + 1;
            info!("Updating menu camera order to {}", new_order);
            commands.entity(entity).insert(Camera {
                order: new_order,
                ..default()
            });
            highest_order = new_order; // Increment for next camera if multiple exist
        }

        return;
    }

    info!("Setting up new menu camera (none found in the query)");
    info!("Total cameras in scene: {}", all_cameras.iter().count());

    // Find the highest camera order from all existing cameras
    let mut highest_order = 0;
    for camera in all_cameras.iter() {
        if camera.order > highest_order {
            highest_order = camera.order;
        }
    }

    // Create a new camera with a higher order
    let new_order = highest_order + 1;
    let camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                order: new_order,
                ..default()
            },
            MenuCamera,
            Name::new("Menu Camera"),
            // Add essential UI components to make it a valid UI parent
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            // Full visibility components to ensure UI items inherit visibility properly
            ViewVisibility::default(),
            InheritedVisibility::VISIBLE,
            Visibility::Visible,
            // Standard ZIndex
            ZIndex::default(),
            // Add render layers for menu items
            crate::camera::components::AppLayer::menu_layers(),
        ))
        .id();

    info!(
        "Menu camera created with order {} and entity {:?}",
        new_order, camera_entity
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
    let camera_entity = camera_query.single();
    commands.entity(camera_entity).insert(Camera {
        order: isize::MAX - 1,
        ..default()
    });
    commands.insert_resource(MenuCameraEntity(camera_entity));

    info!("Main Menu Camera setup complete");
}

/// System to despawn the main menu camera
pub fn despawn_menu_camera(mut commands: Commands, camera_query: Query<Entity, With<MenuCamera>>) {
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<MenuCameraEntity>();
    info!("Main Menu Camera despawned");
}
