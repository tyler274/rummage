use crate::camera::components::{AppLayer, GameCamera};
use crate::menu::components::MenuCamera;
use bevy::{ecs::system::ParamSet, prelude::*};

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

/// Sets the zoom level for the menu camera
pub fn set_menu_camera_zoom(mut cameras: Query<&mut OrthographicProjection, With<MenuCamera>>) {
    for mut projection in cameras.iter_mut() {
        projection.scale = 1.0;
        info!("Set menu camera zoom to 1.0");
    }
}

/// Ensures only a single menu camera exists
pub fn ensure_single_menu_camera(
    mut commands: Commands,
    cameras: Query<(Entity, &Camera), With<MenuCamera>>,
) {
    let count = cameras.iter().count();

    if count > 1 {
        info!("Found {} menu cameras, removing extras", count);

        // Get the camera with the highest order
        let mut highest_order = 0;
        let mut highest_entity = None;

        for (entity, camera) in cameras.iter() {
            if camera.order > highest_order {
                highest_order = camera.order;
                highest_entity = Some(entity);
            }
        }

        // Keep the camera with the highest order, despawn others
        if let Some(keep_entity) = highest_entity {
            for (entity, _) in cameras.iter() {
                if entity != keep_entity {
                    info!("Despawning extra menu camera: {:?}", entity);
                    commands.entity(entity).despawn_recursive();
                }
            }
        }

        info!("Kept menu camera with order {}", highest_order);
    }
}

/// Manages the visibility of the menu camera
pub fn manage_camera_visibility(
    mut menu_cameras: Query<&mut Visibility, With<MenuCamera>>,
    state: Res<State<crate::menu::state::MenuState>>,
) {
    // Determine if the camera should be visible based on state
    let should_be_visible = matches!(
        *state.get(),
        crate::menu::state::MenuState::MainMenu
            | crate::menu::state::MenuState::PausedGame
            | crate::menu::state::MenuState::Settings
            | crate::menu::state::MenuState::Credits
    );

    // Update camera visibility
    for mut visibility in menu_cameras.iter_mut() {
        let new_visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if *visibility != new_visibility {
            info!(
                "Setting menu camera visibility to {:?} in state {:?}",
                new_visibility,
                state.get()
            );
            *visibility = new_visibility;
        }
    }
}

/// Specifically manages visibility for the pause menu camera
pub fn manage_pause_camera_visibility(mut menu_cameras: Query<&mut Visibility, With<MenuCamera>>) {
    for mut visibility in menu_cameras.iter_mut() {
        if *visibility != Visibility::Visible {
            info!("Setting pause menu camera to visible");
            *visibility = Visibility::Visible;
        }
    }
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
