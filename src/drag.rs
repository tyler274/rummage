use bevy::prelude::*;
use crate::card::Card;

#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

/// Handles dragging of cards and their associated text.
/// Uses parent-child relationships to automatically update text positions.
pub fn handle_drag_and_text(
    mut card_query: Query<(Entity, &mut Draggable, &mut Transform), With<Card>>,
    mouse_button: Res<bevy::input::ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    let Ok(window) = windows.get_single() else { return };
    let Ok((camera, camera_transform)) = camera.get_single() else { return };

    let card_width = 100.0;
    let card_height = card_width * 1.4;

    if let Some(cursor_pos) = window.cursor_position() {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            if mouse_button.just_pressed(MouseButton::Left) {
                let mut max_z_index: f32 = 0.0;
                for (_, draggable, _) in card_query.iter() {
                    max_z_index = max_z_index.max(draggable.z_index);
                }

                let mut topmost_card: Option<(Entity, f32)> = None;
                for (entity, draggable, transform) in card_query.iter() {
                    let card_bounds = Rect::from_center_size(
                        transform.translation.truncate(),
                        Vec2::new(card_width, card_height),
                    );
                    if card_bounds.contains(world_position) {
                        if let Some((_, current_z)) = topmost_card {
                            if draggable.z_index > current_z {
                                topmost_card = Some((entity, draggable.z_index));
                            }
                        } else {
                            topmost_card = Some((entity, draggable.z_index));
                        }
                    }
                }

                if let Some((target_entity, _)) = topmost_card {
                    if let Ok((_, mut draggable, transform)) = card_query.get_mut(target_entity) {
                        draggable.dragging = true;
                        draggable.drag_offset = world_position - transform.translation.truncate();
                        draggable.z_index = max_z_index + 1.0;
                    }
                }
            }

            if mouse_button.just_released(MouseButton::Left) {
                for (_, mut draggable, _) in card_query.iter_mut() {
                    draggable.dragging = false;
                }
            }

            for (_entity, draggable, mut card_transform) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_position - draggable.drag_offset;
                    card_transform.translation = new_pos.extend(draggable.z_index);

                    // Debug marker for card position
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(0.0, 0.0, 1.0),
                            custom_size: Some(Vec2::new(3.0, 3.0)),
                            ..default()
                        },
                        Transform::from_xyz(new_pos.x, new_pos.y, 100.0),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        ViewVisibility::default(),
                    ));
                }
            }
        }
    }
} 