use crate::camera::components::{AppLayer, GameCamera};
use crate::menu::components::MenuCamera;
use bevy::{ecs::system::ParamSet, prelude::*};

/// Sets up a menu camera with proper configuration
pub fn setup_menu_camera(
    mut commands: Commands,
    existing_cameras: Query<Entity, With<MenuCamera>>,
    game_cameras: Query<&Camera, With<GameCamera>>,
) {
    // Check if any menu cameras already exist
    if !existing_cameras.is_empty() {
        info!("Menu camera already exists, not creating a new one");
        return;
    }

    // Find the highest game camera order to ensure we use a higher one
    let highest_game_camera_order = game_cameras
        .iter()
        .map(|camera| camera.order)
        .max()
        .unwrap_or(0);

    // Use an order higher than any game camera
    let menu_camera_order = highest_game_camera_order + 10;

    info!("Setting up menu camera with order {}", menu_camera_order);
    let entity = commands
        .spawn((
            Camera {
                order: menu_camera_order,
                ..default()
            },
            Camera2d,
            MenuCamera,
            AppLayer::menu_layers(),
            // Add essential UI components to avoid hierarchy issues when parenting UI nodes
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ViewVisibility::default(),
            InheritedVisibility::default(),
            Visibility::Visible,
            ZIndex::default(), // For proper layering
            Name::new("Menu Camera"),
        ))
        .id();

    info!(
        "Spawned menu camera: {:?} with order {}",
        entity, menu_camera_order
    );
}

/// Ensures only one menu camera exists
pub fn ensure_single_menu_camera(
    mut commands: Commands,
    menu_cameras: Query<(Entity, &Camera), With<MenuCamera>>,
    game_cameras: Query<&Camera, With<GameCamera>>,
) {
    let camera_count = menu_cameras.iter().count();

    if camera_count > 1 {
        warn!(
            "Found {} menu cameras, cleaning up duplicates",
            camera_count
        );

        // Find the first camera and keep track of its details
        let mut cameras_to_remove = Vec::new();
        let mut highest_order = None;
        let mut highest_order_entity = None;

        // First pass: find the camera with the highest order
        for (entity, camera) in menu_cameras.iter() {
            if let Some(order) = highest_order {
                if camera.order > order {
                    highest_order = Some(camera.order);
                    highest_order_entity = Some(entity);
                } else {
                    cameras_to_remove.push(entity);
                }
            } else {
                highest_order = Some(camera.order);
                highest_order_entity = Some(entity);
            }
        }

        // Second pass: remove all cameras except the one with highest order
        for entity in cameras_to_remove {
            info!("Removing duplicate menu camera entity: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }

        if let Some(entity) = highest_order_entity {
            info!(
                "Keeping menu camera entity: {:?} with order: {:?}",
                entity, highest_order
            );
        }
    } else if camera_count == 1 {
        // If there's only one camera, make sure it has a unique order
        let (entity, camera) = menu_cameras.single();

        // Find the highest game camera order
        let highest_game_camera_order = game_cameras
            .iter()
            .map(|camera| camera.order)
            .max()
            .unwrap_or(0);

        // If the menu camera's order conflicts with any game camera, update it
        if game_cameras.iter().any(|gc| gc.order == camera.order) {
            let new_order = highest_game_camera_order + 10;
            info!(
                "Updating menu camera {:?} order from {} to {} to avoid conflicts",
                entity, camera.order, new_order
            );
            commands.entity(entity).insert(Camera {
                order: new_order,
                ..camera.clone()
            });
        }
    }
}

/// Ensures proper camera visibility when entering the InGame state
pub fn manage_camera_visibility(
    mut params: ParamSet<(
        Query<(Entity, &mut Visibility, &mut Camera), With<GameCamera>>,
        Query<(Entity, &mut Visibility), With<MenuCamera>>,
        Query<(Entity, &mut Camera), With<MenuCamera>>,
    )>,
    state: Res<State<crate::menu::state::GameMenuState>>,
    mut last_state: Local<Option<crate::menu::state::GameMenuState>>,
) {
    // Check if the state has changed since last run
    let state_changed = match last_state.as_ref() {
        Some(prev_state) => *prev_state != *state.get(),
        None => true, // First run
    };

    // Update the last state
    *last_state = Some(*state.get());

    // Only perform full checks when the state changes or first run
    let current_state = state.get();

    // Menu camera should be visible in MainMenu, Settings and PausedGame states
    let should_menu_camera_be_visible = match current_state {
        crate::menu::state::GameMenuState::MainMenu => true,
        crate::menu::state::GameMenuState::Settings => true,
        crate::menu::state::GameMenuState::PausedGame => true,
        _ => false,
    };

    // Log state changes at most once per change, not every frame
    if state_changed {
        info!(
            "Game state changed to {:?}, menu camera should be visible: {}",
            current_state, should_menu_camera_be_visible
        );
    }

    // Ensure menu camera has the right visibility - only check when state changes
    // or at most every 60 frames (approximately once per second)
    if state_changed {
        let mut menu_visibility_query = params.p1();
        for (entity, mut visibility) in menu_visibility_query.iter_mut() {
            let current_visibility = *visibility;
            let needs_change = (should_menu_camera_be_visible
                && current_visibility != Visibility::Visible)
                || (!should_menu_camera_be_visible && current_visibility == Visibility::Visible);

            if needs_change {
                if should_menu_camera_be_visible {
                    info!(
                        "Setting menu camera {:?} to visible (was {:?})",
                        entity, current_visibility
                    );
                    *visibility = Visibility::Visible;
                } else {
                    info!(
                        "Setting menu camera {:?} to inherited (was visible)",
                        entity
                    );
                    *visibility = Visibility::Inherited;
                }
            }
        }

        // Adjust game camera visibility based on state
        // Game camera should be visible in InGame state and when paused
        let game_camera_visible = match current_state {
            crate::menu::state::GameMenuState::InGame => true,
            crate::menu::state::GameMenuState::PausedGame => true,
            _ => false,
        };

        let mut game_camera_query = params.p0();
        for (entity, mut visibility, _) in game_camera_query.iter_mut() {
            let current_visibility = *visibility;
            let needs_change = (game_camera_visible && current_visibility != Visibility::Visible)
                || (!game_camera_visible && current_visibility == Visibility::Visible);

            if needs_change {
                if game_camera_visible {
                    info!(
                        "Setting game camera {:?} to visible (was {:?})",
                        entity, current_visibility
                    );
                    *visibility = Visibility::Visible;
                } else {
                    info!(
                        "Setting game camera {:?} to inherited (was visible)",
                        entity
                    );
                    *visibility = Visibility::Inherited;
                }
            }
        }
    }
}

/// Ensures proper camera visibility when entering the PausedGame state
pub fn manage_pause_camera_visibility(
    mut params: ParamSet<(
        Query<(Entity, &mut Visibility), With<GameCamera>>,
        Query<(Entity, &mut Visibility), With<MenuCamera>>,
        Query<(Entity, &mut Camera), With<MenuCamera>>,
    )>,
) {
    info!("Managing camera visibility in PausedGame state");

    // Use the first parameter (access game cameras first)
    {
        let game_cameras = params.p0();
        let camera_count = game_cameras.iter().count();
        info!("Found {} game cameras", camera_count);

        // Set all game cameras to hidden in a separate scope
        if camera_count > 0 {
            let mut game_cameras = params.p0();
            for (entity, mut visibility) in game_cameras.iter_mut() {
                info!("Setting game camera {:?} to Hidden", entity);
                *visibility = Visibility::Hidden;
            }
        }
    }

    // Use the second parameter (access menu cameras for visibility)
    {
        let menu_cameras = params.p1();
        let menu_camera_count = menu_cameras.iter().count();
        info!("Found {} menu cameras", menu_camera_count);

        // Check if we have multiple menu cameras (warning condition)
        if menu_camera_count > 1 {
            warn!(
                "Found {} menu cameras when there should only be one!",
                menu_camera_count
            );
        }

        if menu_camera_count > 0 {
            // Set all menu cameras to visible in a separate scope
            let mut menu_visibility = params.p1();
            for (entity, mut visibility) in menu_visibility.iter_mut() {
                info!("Setting menu camera {:?} to Visible", entity);
                *visibility = Visibility::Visible;
            }
        }
    }

    // Use the third parameter (access menu camera orders)
    {
        let mut menu_cameras = params.p2();
        for (entity, mut camera) in menu_cameras.iter_mut() {
            if camera.order != 2 {
                info!(
                    "Setting menu camera {:?} order from {} to 2",
                    entity, camera.order
                );
                camera.order = 2;
            }
        }
    }
}
