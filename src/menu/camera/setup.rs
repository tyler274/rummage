use crate::menu::AppState;
use crate::menu::state::MenuState;
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

    if let Ok((_entity, mut camera)) = cameras.single_mut() {
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
    match camera_query.single() {
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
pub fn set_initial_zoom(
    mut cameras: Query<(Entity, &mut Projection), (With<Camera2d>, With<MenuCamera>)>,
    mut ran: Local<bool>,
) -> Result<(), BevyError> {
    if *ran {
        return Ok(());
    }
    if let Ok((_entity, mut projection_enum)) = cameras.single_mut() {
        if let Projection::Orthographic(ref mut orthographic_projection) = *projection_enum {
            // TODO: Define INITIAL_ZOOM_LEVEL, perhaps from a config or constant
            const INITIAL_ZOOM_LEVEL: f32 = 1.0;
            orthographic_projection.scale = INITIAL_ZOOM_LEVEL;
            *ran = true;
            info!(
                "Successfully set initial menu camera zoom to {}",
                INITIAL_ZOOM_LEVEL
            );
        } else {
            warn!("MenuCamera does not have an OrthographicProjection for initial zoom.");
        }
    } else {
        debug!("Menu camera not found yet for initial zoom setting...");
    }
    Ok(())
}

/// System to handle window resize events for the menu camera
pub fn handle_window_resize(
    _commands: Commands, // Added mut for potential future use with commands
    mut camera_query: Query<(Entity, &Transform, &mut Projection), With<MenuCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
) -> Result<(), BevyError> {
    // It's not clear if MenuConfig is still relevant or how it should be used for resize.
    // The previous logic was commented out. For now, this system will be a no-op
    // regarding projection changes until further requirements are specified.

    if let Ok((_camera_entity, _camera_transform, mut _projection_enum)) = camera_query.single_mut()
    {
        let _window = window_query.single().map_err(|e| {
            warn!(
                "Primary window not found in menu handle_window_resize: {}",
                e
            );
            BevyError::from(e) // Or some other appropriate BevyError variant
        })?;

        // Example of accessing orthographic projection if needed in the future:
        // if let Projection::Orthographic(ref mut _ortho_projection) = *_projection_enum {
        //     // Implement logic to handle window resize events using window.width(), window.height()
        //     // For example, adjust OrthographicProjection if this camera controls one.
        //     // info!("Menu camera handled resize. New window: {}x{}", window.width(), window.height());
        // }
    } else {
        warn!(
            "Failed to get single MenuCamera entity in handle_window_resize. This might be okay if the camera is created later."
        );
    }
    Ok(())
}

/// System to update menu camera transform based on mouse position
pub fn menu_camera_system(
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<MenuCamera>>,
    _mouse_input: Res<ButtonInput<MouseButton>>,
    _last_drag_position: Local<Option<Vec2>>,
    current_menu_state: Res<State<MenuState>>,
    current_app_state: Res<State<AppState>>,
) {
    let app_state_val = current_app_state.get();
    let menu_state_val = current_menu_state.get();

    // Determine if camera movement should be active
    let in_menu_app_state = *app_state_val == AppState::Menu;
    let in_interactive_menu_state =
        *menu_state_val == MenuState::MainMenu || *menu_state_val == MenuState::Settings;

    // Skip if not in a state where menu camera should move
    if !in_menu_app_state || !in_interactive_menu_state
    /* || camera_movement_state.locked */
    {
        return;
    }

    let window = windows
        .single()
        .expect("Primary window not found in menu_camera_system");
    if let Ok(mut _transform) = camera_query.single_mut() {
        if let Some(_cursor_position) = window.cursor_position() {
            // Implement the logic to update the camera transform based on mouse position
        }
    }
}
