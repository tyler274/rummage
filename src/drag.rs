/// Drag and drop functionality for game objects.
///
/// This module provides:
/// - Mouse-based drag and drop interactions
/// - Z-index management for dragged objects
/// - Collision detection for drag targets
/// - Visual feedback during drag operations
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

/// Component for marking entities that can be dragged
#[derive(Component)]
pub struct Draggable {
    /// Whether the entity is currently being dragged
    pub dragging: bool,
    /// Offset from the mouse cursor to the entity's origin
    pub drag_offset: Vec2,
    /// Z-index for rendering order
    pub z_index: f32,
}

/// Resource for caching entity positions during drag operations
#[derive(Resource, Default)]
pub struct DragCache {
    /// Map of entity to its current position
    pub positions: HashMap<Entity, Vec3>,
}

/// Plugin for handling drag and drop interactions
pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragCache>()
            .add_systems(Update, (drag_system, update_draggables, start_drag));
    }
}

/// System for handling drag and drop interactions
pub fn drag_system(
    mut draggable_query: Query<(&mut Transform, &mut Draggable), Without<crate::cards::Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
) {
    // Get the primary window
    let Ok(window) = windows.get_single() else {
        return;
    };

    // Get the camera
    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return;
    };

    // Get the current cursor position
    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor position to world coordinates
        let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
            return;
        };

        // Handle mouse press - start dragging
        if mouse_button.just_pressed(MouseButton::Left) {
            let mut highest_z = f32::NEG_INFINITY;
            let mut top_draggable = None;

            // Find the draggable with highest z-index under cursor
            for (transform, draggable) in draggable_query.iter() {
                let pos = transform.translation.truncate();
                if pos.distance(world_pos) < 50.0 && draggable.z_index > highest_z {
                    highest_z = draggable.z_index;
                    top_draggable = Some((transform.translation.truncate(), draggable.z_index));
                }
            }

            // Start dragging the top draggable
            if let Some((pos, z)) = top_draggable {
                for (_, mut draggable) in draggable_query.iter_mut() {
                    if draggable.z_index == z {
                        draggable.dragging = true;
                        draggable.drag_offset = pos - world_pos;
                    }
                }
            }
        }

        // Handle mouse release - stop dragging
        if mouse_button.just_released(MouseButton::Left) {
            for (_, mut draggable) in draggable_query.iter_mut() {
                draggable.dragging = false;
            }
        }

        // Update dragged entities
        for (mut transform, draggable) in draggable_query.iter_mut() {
            if draggable.dragging {
                transform.translation =
                    (world_pos + draggable.drag_offset).extend(draggable.z_index);
            }
        }
    }
}

// Helper system to handle updating draggable object positions during drag
fn update_draggables(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Draggable, &mut Visibility)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut position_cache: ResMut<DragCache>,
) {
    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => return, // No camera, can't process dragging
    };

    let window = match window_query.get_single() {
        Ok(result) => result,
        Err(_) => return, // No window, can't process dragging
    };

    let world_position = if let Some(screen_pos) = window.cursor_position() {
        screen_to_world(camera, camera_transform, screen_pos, window)
    } else {
        None
    };

    for (entity, mut transform, draggable, mut visibility) in query.iter_mut() {
        if draggable.dragging {
            if mouse_button_input.pressed(MouseButton::Left) {
                if let Some(world_pos) = world_position {
                    // Offset mouse position by the drag offset
                    let target_position =
                        Vec3::new(world_pos.x, world_pos.y, 0.0) + Vec3::new(0.0, 0.0, 40.0); // Use z-index of 40.0 to stay above all other cards and playmats
                    // Update the entity position
                    transform.translation = target_position;
                    // Store the current position for snapping if needed
                    position_cache
                        .positions
                        .insert(entity, transform.translation);
                    // Ensure the entity is visible while being dragged
                    *visibility = Visibility::Visible;
                }
            } else {
                // Mouse button released while dragging
                commands.entity(entity).insert(Draggable {
                    dragging: false,
                    drag_offset: draggable.drag_offset,
                    z_index: 30.0, // Return to the standard card z-index when done dragging
                });
            }
        }
    }
}

// System to start dragging a card when clicked
fn start_drag(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    draggable_query: Query<(Entity, &GlobalTransform, &Draggable)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return; // Not a left click, don't do anything
    }

    let (camera, camera_transform) = match camera_query.get_single() {
        Ok(result) => result,
        Err(_) => return, // No camera, can't process dragging
    };

    let window = match window_query.get_single() {
        Ok(result) => result,
        Err(_) => return, // No window, can't process dragging
    };

    let cursor_position = match window.cursor_position() {
        Some(pos) => pos,
        None => return, // No cursor position, can't process dragging
    };

    let world_position = match screen_to_world(camera, camera_transform, cursor_position, window) {
        Some(pos) => pos,
        None => return, // Couldn't convert to world position
    };

    // Find the topmost draggable entity under the cursor
    let mut entities_under_cursor = Vec::new();

    for (entity, transform, draggable) in draggable_query.iter() {
        if is_cursor_over_entity(world_position, transform) {
            entities_under_cursor.push((entity, draggable.z_index));
        }
    }

    // Sort by z-index to get the topmost entity
    entities_under_cursor.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    if let Some((entity, _)) = entities_under_cursor.first() {
        // When starting a drag, increase the z-index to ensure the card stays on top of all other cards
        let new_z_index = 40.0; // Increased from 30.0 to ensure it's above non-dragged cards

        commands.entity(*entity).insert(Draggable {
            dragging: true,
            drag_offset: Vec2::ZERO,
            z_index: new_z_index,
        });
    }
}

/// Convert a screen position to a world position
fn screen_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    screen_pos: Vec2,
    window: &Window,
) -> Option<Vec2> {
    camera
        .viewport_to_world_2d(camera_transform, screen_pos)
        .ok()
}

/// Check if the cursor is over an entity based on its size
fn is_cursor_over_entity(cursor_world_pos: Vec2, transform: &GlobalTransform) -> bool {
    // Simple distance-based check
    // This could be improved with actual sprite size information
    let entity_pos = transform.translation().truncate();
    let distance = entity_pos.distance(cursor_world_pos);
    distance < 50.0 // Assuming entities are roughly 100 units wide
}
