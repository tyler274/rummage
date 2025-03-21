use bevy::prelude::*;

/// Component marker for menu cameras
#[derive(Component, Debug)]
pub struct MenuCamera;

/// Sets up a dedicated camera for the menu
pub fn setup_menu_camera(
    mut commands: Commands,
    cameras: Query<(Entity, Option<&Camera>), With<MenuCamera>>,
) {
    // Check if a menu camera already exists to avoid duplicates
    if !cameras.is_empty() {
        info!("Menu camera already exists, skipping creation");
        return;
    }

    info!("Setting up menu camera");

    // Find the highest camera order from existing cameras
    let mut highest_order = 0;
    for (_, camera) in cameras.iter() {
        if let Some(camera) = camera {
            if camera.order > highest_order {
                highest_order = camera.order;
            }
        }
    }

    // Create a new camera with a higher order
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: highest_order + 1,
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
        InheritedVisibility::default(),
        Visibility::Visible,
        // Give it a ZIndex for proper layering
        ZIndex::default(),
    ));

    info!("Menu camera created with order {}", highest_order + 1);
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
