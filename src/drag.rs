use bevy::prelude::*;

#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_dragging);
    }
}

fn handle_dragging(
    mut query: Query<(&mut Transform, &mut Draggable)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_pos) = window.cursor_position() {
        let cursor_pos = Vec2::new(cursor_pos.x, cursor_pos.y);

        for (mut transform, mut draggable) in query.iter_mut() {
            let card_pos = transform.translation.truncate();

            if mouse_buttons.just_pressed(MouseButton::Left) {
                let cursor_world_pos = camera
                    .viewport_to_world_2d(camera_transform, cursor_pos)
                    .unwrap_or_default();

                if cursor_world_pos.distance(card_pos) < 50.0 {
                    draggable.dragging = true;
                    draggable.drag_offset = card_pos - cursor_world_pos;
                    transform.translation.z = 10.0;
                }
            } else if mouse_buttons.just_released(MouseButton::Left) {
                draggable.dragging = false;
                transform.translation.z = draggable.z_index;
            }

            if draggable.dragging {
                let cursor_world_pos = camera
                    .viewport_to_world_2d(camera_transform, cursor_pos)
                    .unwrap_or_default();
                transform.translation = (cursor_world_pos + draggable.drag_offset).extend(10.0);
            }
        }
    }
}
