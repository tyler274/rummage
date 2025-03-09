/// Drag and drop functionality for game objects.
///
/// This module provides:
/// - Mouse-based drag and drop interactions
/// - Z-index management for dragged objects
/// - Collision detection for drag targets
/// - Visual feedback during drag operations
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

/// Plugin for handling drag and drop interactions
pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drag_system);
    }
}

/// System for handling drag and drop interactions
pub fn drag_system(
    mut draggable_query: Query<(&mut Transform, &mut Draggable)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
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
