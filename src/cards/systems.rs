use bevy::prelude::*;

use super::{Card, components::Draggable};
use crate::text;

pub fn handle_card_dragging(
    mut card_query: Query<(Entity, &mut Transform, &mut Draggable, &GlobalTransform), With<Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
) {
    // Safely get window and camera
    let Ok(window) = windows.get_single() else {
        return; // No window available
    };

    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return; // No camera available
    };

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor position to world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Handle mouse press - start dragging
            if mouse_button.just_pressed(MouseButton::Left) {
                let mut highest_z = f32::NEG_INFINITY;
                let mut top_card = None;

                // First pass: find the card with highest z-index at cursor position
                for (entity, _, draggable, global_transform) in card_query.iter() {
                    let card_pos = global_transform.translation().truncate();
                    // Using actual card dimensions (672x936) to match Magic card proportions
                    let card_size = Vec2::new(672.0, 936.0);
                    // No additional scaling needed since viewport_to_world_2d already gives us
                    // coordinates in the same space as our card positions
                    let scaled_size = card_size * 1.0;

                    // Check if the cursor is within the card bounds
                    // The hit detection area now perfectly matches the visible card boundaries
                    if world_pos.x >= card_pos.x - scaled_size.x / 2.0
                        && world_pos.x <= card_pos.x + scaled_size.x / 2.0
                        && world_pos.y >= card_pos.y - scaled_size.y / 2.0
                        && world_pos.y <= card_pos.y + scaled_size.y / 2.0
                    {
                        if draggable.z_index > highest_z {
                            highest_z = draggable.z_index;
                            top_card = Some(entity);
                        }
                    }
                }

                // Second pass: start dragging only the top card
                if let Some(top_entity) = top_card {
                    // Find the highest z-index among all cards
                    let mut max_z = highest_z;
                    for (_, _, draggable, _) in card_query.iter() {
                        max_z = max_z.max(draggable.z_index);
                    }

                    for (entity, mut transform, mut draggable, global_transform) in
                        card_query.iter_mut()
                    {
                        if entity == top_entity {
                            let card_pos = global_transform.translation().truncate();
                            draggable.dragging = true;
                            draggable.drag_offset = card_pos - world_pos;
                            // Set the dragged card's z-index higher than all others
                            draggable.z_index = max_z + 1.0;
                            transform.translation.z = max_z + 1.0;
                        }
                    }
                }
            }

            // Handle mouse release - stop dragging and update z-index
            if mouse_button.just_released(MouseButton::Left) {
                let mut max_z = f32::NEG_INFINITY;

                // First find the highest z-index
                for (_, _, draggable, _) in card_query.iter() {
                    max_z = max_z.max(draggable.z_index);
                }

                // Then update the previously dragged card
                for (_, _, mut draggable, _) in card_query.iter_mut() {
                    if draggable.dragging {
                        draggable.dragging = false;
                        draggable.z_index = max_z + 1.0; // Place it on top
                    }
                }
            }

            // Update position of dragged cards
            for (_, mut transform, draggable, _) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_pos + draggable.drag_offset;
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                    // Maintain the z-index we set when dragging started
                    transform.translation.z = draggable.z_index;
                }
            }
        }
    }
}

pub fn debug_render_text_positions(
    mut gizmos: Gizmos,
    card_query: Query<(&Transform, &Card), With<Card>>,
    config: Res<text::DebugConfig>,
) {
    if !config.show_text_positions {
        return;
    }

    for (transform, _) in card_query.iter() {
        let card_pos = transform.translation.truncate();
        let card_width = 100.0;
        let card_height = card_width * 1.4;

        // Note: Using Color::srgb instead of Color::rgb as rgb is deprecated

        // Name position (top left) - red dot
        let name_pos = card_pos + Vec2::new(-card_width * 0.25, card_height * 0.35);
        gizmos.circle_2d(name_pos, 3.0, Color::srgb(1.0, 0.0, 0.0));

        // Mana cost position (top right) - blue dot
        let cost_pos = card_pos + Vec2::new(card_width * 0.35, card_height * 0.35);
        gizmos.circle_2d(cost_pos, 3.0, Color::srgb(0.0, 0.0, 1.0));

        // Type position (middle center) - green dot
        let type_pos = card_pos + Vec2::new(0.0, card_height * 0.05);
        gizmos.circle_2d(type_pos, 3.0, Color::srgb(0.0, 1.0, 0.0));

        // Rules text position (middle/bottom center) - yellow dot
        let rules_pos = card_pos + Vec2::new(0.0, -card_height * 0.15);
        gizmos.circle_2d(rules_pos, 3.0, Color::srgb(1.0, 1.0, 0.0));

        // Power/toughness position (bottom right) - purple dot
        let pt_pos = card_pos + Vec2::new(card_width * 0.35, -card_height * 0.35);
        gizmos.circle_2d(pt_pos, 3.0, Color::srgb(1.0, 0.0, 1.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    // Define the draggable_card_filter function for testing
    fn draggable_card_filter(card: Query<(), (With<Card>, With<Draggable>)>) -> bool {
        !card.is_empty()
    }

    /// Test for the draggable_card_filter function
    #[test]
    fn test_draggable_card_filter() {
        // Create a new app for testing
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a card entity with the Card and Draggable components
        app.world.spawn((
            Card::builder("Draggable Card").build(),
            Draggable::default(),
        ));

        // Create a non-draggable card
        app.world.spawn(Card::builder("Non-Draggable Card").build());

        // Run the system that uses the filter
        fn test_system(card_query: Query<(), (With<Card>, With<Draggable>)>) {
            let has_draggable_cards = draggable_card_filter(card_query);
            assert!(has_draggable_cards);
        }

        app.add_systems(Update, test_system);
        app.update();

        // Test with a world that has no draggable cards
        let mut app2 = App::new();
        app2.add_plugins(MinimalPlugins);

        // Only add non-draggable cards
        app2.world
            .spawn(Card::builder("Non-Draggable Card").build());

        fn test_system_no_draggable(card_query: Query<(), (With<Card>, With<Draggable>)>) {
            let has_draggable_cards = draggable_card_filter(card_query);
            assert!(!has_draggable_cards);
        }

        app2.add_systems(Update, test_system_no_draggable);
        app2.update();
    }
}
