use super::table::TableLayout;
use crate::camera::components::{AppLayer, GameCamera};
use crate::cards::text::card_text::spawn_card_text_components;
use crate::cards::{CardZone, Draggable};
use crate::game_engine::zones::Zone;

use bevy::prelude::*;

/// Helper function to spawn visual card entities
pub fn spawn_visual_cards(
    commands: &mut Commands,
    display_cards: Vec<crate::cards::Card>,
    game_cameras: &Query<Entity, With<GameCamera>>,
    card_size: &Vec2,
    spacing_multiplier: f32,
    player_position: Vec3,
    player_index: usize,
    player_entity: Entity,
    table: &TableLayout,
    asset_server_option: Option<&AssetServer>,
) {
    // Skip if no cards to spawn
    if display_cards.is_empty() {
        warn!("No cards to spawn for player {}", player_index);
        return;
    }

    info!(
        "Spawning {} cards for player {:?} (index {})",
        display_cards.len(),
        player_entity,
        player_index
    );

    // Increase the spacing between cards
    let spacing = card_size.x * spacing_multiplier * 1.5;

    // Calculate the total width of all cards with spacing
    let total_width = display_cards.len() as f32 * spacing;

    // Move the starting position further to the left for better distribution
    let start_x = -(total_width) / 2.0 + spacing / 2.0;

    // Get the card offset for this player based on table position
    let card_offset = table.get_card_offset(player_index);

    // Determine if the cards should be laid out horizontally or vertically
    let is_horizontal = table.is_horizontal_layout(player_index);

    // Calculate the starting position and direction based on layout
    let (start_pos, card_direction) = if is_horizontal {
        // Horizontal layout (cards in a row)
        (
            Vec3::new(start_x, player_position.y, 0.0) + card_offset,
            Vec3::new(spacing, 0.0, 0.0),
        )
    } else {
        // Vertical layout (cards in a column)
        (
            Vec3::new(player_position.x, start_x, 0.0) + card_offset,
            Vec3::new(0.0, spacing, 0.0),
        )
    };

    // Spawn each card with proper positioning
    for (i, card) in display_cards.into_iter().enumerate() {
        let card_clone = card.clone(); // Clone card to use later
        // Calculate z-index based on position to ensure proper layering
        let z = 0.5 + (i as f32 * 0.01); // Increase z value to avoid z-fighting

        // Calculate the position for this card
        let position = Vec3::new(
            start_pos.x + card_direction.x * i as f32,
            start_pos.y + card_direction.y * i as f32,
            z,
        );

        let transform = Transform::from_translation(position);

        info!(
            "Positioning card '{}' at ({:.2}, {:.2}, {:.2})",
            card.name.name, position.x, position.y, position.z
        );

        // Create a complete SpriteBundle rather than individual components
        let card_entity = commands
            .spawn((
                card,
                Sprite {
                    color: Color::srgb(0.2, 0.6, 0.8), // bright blue color for visibility
                    custom_size: Some(*card_size),
                    ..default()
                },
                transform,
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
                AppLayer::Cards.layer(),
                CardZone {
                    zone: Zone::Hand,
                    zone_owner: Some(player_entity),
                },
                Name::new(format!("Card: {}", card_clone.name.name)),
            ))
            .id();

        // Spawn text components directly instead of just adding marker components
        if let Some(game_asset_server) = asset_server_option {
            // Instead of using world_mut(), use the card directly
            // Convert card::components::CardRulesText to text::components::CardRulesText
            let rules_text = crate::text::components::CardRulesText {
                rules_text: card_clone.rules_text.rules_text.clone(),
            };

            // With our new Card bundle, we can get all the components directly from the card
            spawn_card_text_components(
                commands,
                card_entity,
                (
                    &card_clone, // Use the cloned Card bundle
                    &card_clone.name,
                    &card_clone.cost,
                    &card_clone.type_info,
                    &card_clone.details,
                    &rules_text, // Use the converted rules text
                ),
                &transform,
                &Sprite {
                    color: Color::srgb(0.85, 0.85, 0.85),
                    custom_size: Some(*card_size),
                    ..default()
                },
                game_asset_server,
                None,
            );
        }

        // Make the card a child of the game camera to ensure it's rendered in the game view
        for camera in game_cameras.iter() {
            info!(
                "Attaching card {:?} to game camera {:?}",
                card_entity, camera
            );
            commands.entity(camera).add_child(card_entity);
        }
    }
}
