use bevy::prelude::*;

use crate::cards::Card;
use crate::cards::components::Draggable;
use crate::menu::input_blocker::InteractionBlockState;
use crate::text;

pub fn handle_card_dragging(
    mut card_query: Query<(Entity, &mut Transform, &mut Draggable, &GlobalTransform), With<Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
    player_config: Res<crate::player::resources::PlayerConfig>,
    interaction_block: Res<InteractionBlockState>,
) {
    // Skip interaction if blocked by menus
    if interaction_block.should_block {
        return;
    }

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

                    // Get the base card size from player config
                    let base_card_size = player_config.card_size;

                    // Apply the same size multiplier as in card spawning (2.5)
                    // This ensures the draggable area matches the visual card size
                    let actual_card_size = base_card_size * 2.5;

                    // Check if the cursor is within the card bounds
                    // Use the actual sized card for hit detection with a small margin for easier selection
                    let hit_area_multiplier = 1.1; // Just 10% larger hit area for precision with buffer
                    let selection_size = actual_card_size * hit_area_multiplier;

                    if world_pos.x >= card_pos.x - selection_size.x / 2.0
                        && world_pos.x <= card_pos.x + selection_size.x / 2.0
                        && world_pos.y >= card_pos.y - selection_size.y / 2.0
                        && world_pos.y <= card_pos.y + selection_size.y / 2.0
                    {
                        // Debug card hit test
                        info!(
                            "Card hit test - Entity: {:?}, z-index: {}",
                            entity, draggable.z_index
                        );

                        if draggable.z_index > highest_z {
                            highest_z = draggable.z_index;
                            top_card = Some((entity, card_pos));
                        }
                    }
                }

                // Second pass: start dragging only the top card
                if let Some((top_entity, card_pos)) = top_card {
                    info!("Dragging card: {:?}", top_entity);

                    // Find the highest z-index among all cards
                    let mut max_z = 10.0f32; // Start at a sensible baseline
                    for (_, _, draggable, _) in card_query.iter() {
                        max_z = max_z.max(draggable.z_index + 0.1);
                    }

                    for (entity, mut transform, mut draggable, _) in card_query.iter_mut() {
                        if entity == top_entity {
                            draggable.dragging = true;
                            draggable.drag_offset = card_pos - world_pos;
                            // Set the dragged card's z-index higher than all others
                            let new_z = max_z + 5.0; // Add a significant bump to ensure it's on top
                            draggable.z_index = new_z;
                            transform.translation.z = new_z;

                            info!("Card {:?} now has z-index: {}", entity, new_z);
                        }
                    }
                }
            }

            // Handle mouse release - stop dragging and update z-index
            if mouse_button.just_released(MouseButton::Left) {
                // Find any cards that were being dragged
                let mut any_dragged = false;

                for (entity, _, draggable, _) in card_query.iter() {
                    if draggable.dragging {
                        any_dragged = true;
                        info!("Dropping card: {:?}", entity);
                    }
                }

                if any_dragged {
                    // Find highest z-index
                    let mut max_z = 10.0f32;
                    for (_, _, draggable, _) in card_query.iter() {
                        max_z = max_z.max(draggable.z_index);
                    }

                    // Update cards that were being dragged
                    for (entity, mut transform, mut draggable, _) in card_query.iter_mut() {
                        if draggable.dragging {
                            draggable.dragging = false;
                            // Place the dropped card on top of all other cards
                            let new_z = max_z + 1.0;
                            draggable.z_index = new_z;
                            transform.translation.z = new_z;
                            info!("Dropped card {:?} at z-index: {}", entity, new_z);
                        }
                    }
                }
            }

            // Update position of dragged cards
            for (_entity, mut transform, draggable, _) in card_query.iter_mut() {
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
    player_config: Res<crate::player::resources::PlayerConfig>,
) {
    if !config.show_text_positions {
        return;
    }

    for (transform, _) in card_query.iter() {
        let card_pos = transform.translation.truncate();
        let card_size = player_config.card_size;

        // Note: Using Color::srgb instead of Color::rgb as rgb is deprecated

        // Name position (top left) - red dot
        let name_pos = card_pos + Vec2::new(-card_size.x * 0.25, card_size.y * 0.35);
        gizmos.circle_2d(name_pos, 3.0, Color::srgb(1.0, 0.0, 0.0));

        // Mana cost position (top right) - blue dot
        let cost_pos = card_pos + Vec2::new(card_size.x * 0.35, card_size.y * 0.35);
        gizmos.circle_2d(cost_pos, 3.0, Color::srgb(0.0, 0.0, 1.0));

        // Type position (middle center) - green dot
        let type_pos = card_pos + Vec2::new(0.0, card_size.y * 0.05);
        gizmos.circle_2d(type_pos, 3.0, Color::srgb(0.0, 1.0, 0.0));

        // Rules text position (middle/bottom center) - yellow dot
        let rules_pos = card_pos + Vec2::new(0.0, -card_size.y * 0.15);
        gizmos.circle_2d(rules_pos, 3.0, Color::srgb(1.0, 1.0, 0.0));

        // Power/toughness position (bottom right) - purple dot
        let pt_pos = card_pos + Vec2::new(card_size.x * 0.35, -card_size.y * 0.35);
        gizmos.circle_2d(pt_pos, 3.0, Color::srgb(1.0, 0.0, 1.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::details::CardDetails;
    use crate::cards::types::CardTypes;
    use crate::mana::Mana;

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
        app.world_mut().spawn((
            Card::builder("Draggable Card")
                .cost(Mana::default())
                .types(CardTypes::default())
                .details(CardDetails::default())
                .build_or_panic(),
            Draggable::default(),
        ));

        // Create a non-draggable card
        app.world_mut().spawn(
            Card::builder("Non-Draggable Card")
                .cost(Mana::default())
                .types(CardTypes::default())
                .details(CardDetails::default())
                .build_or_panic(),
        );

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
        app2.world_mut().spawn(
            Card::builder("Non-Draggable Card")
                .cost(Mana::default())
                .types(CardTypes::default())
                .details(CardDetails::default())
                .build_or_panic(),
        );

        fn test_system_no_draggable(card_query: Query<(), (With<Card>, With<Draggable>)>) {
            let has_draggable_cards = draggable_card_filter(card_query);
            assert!(!has_draggable_cards);
        }

        app2.add_systems(Update, test_system_no_draggable);
        app2.update();
    }
}
