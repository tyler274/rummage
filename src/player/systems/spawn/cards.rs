use super::table::TableLayout;
use crate::camera::components::{AppLayer, GameCamera};
use crate::card::{Card, CardDetails, Draggable};
use crate::mana::convert_rules_text_to_symbols;
use crate::text::CardTextContent;
use bevy::prelude::*;

/// Helper function to spawn visual card entities
pub fn spawn_visual_cards(
    commands: &mut Commands,
    display_cards: Vec<Card>,
    game_cameras: &Query<Entity, With<GameCamera>>,
    card_size: &Vec2,
    spacing_multiplier: f32,
    player_position: Vec3,
    player_index: usize,
    table: &TableLayout,
) {
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

    info!(
        "Spawning {} cards for player {} at position ({:.2}, {:.2}, {:.2})",
        display_cards.len(),
        player_index,
        start_pos.x,
        start_pos.y,
        start_pos.z
    );

    // Get game camera entity to set render target
    let game_camera_entities: Vec<Entity> = game_cameras.iter().collect();
    if !game_camera_entities.is_empty() {
        info!(
            "Found game camera for card rendering: {:?}",
            game_camera_entities[0]
        );
    } else {
        info!("No game camera found, using default camera");
    }

    // Spawn visual cards in appropriate arrangement
    for (i, card) in display_cards.into_iter().enumerate() {
        // Use a base z-index based on player index
        let base_z = player_index as f32 * 100.0;
        let z = base_z + i as f32;

        // Calculate position based on direction and starting position
        let position = Vec3::new(
            start_pos.x + card_direction.x * i as f32,
            start_pos.y + card_direction.y * i as f32,
            z,
        );

        let transform = Transform::from_translation(position);

        info!(
            "Positioning card '{}' at ({:.2}, {:.2}, {:.2})",
            card.name, position.x, position.y, position.z
        );

        let card_entity = commands
            .spawn((
                card.clone(),
                Sprite {
                    color: Color::srgb(0.85, 0.85, 0.85),
                    custom_size: Some(*card_size),
                    ..default()
                },
                transform,
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
                Draggable {
                    dragging: false,
                    drag_offset: Vec2::ZERO,
                    z_index: z,
                },
                AppLayer::Cards.layer(), // Use the specific Cards layer
            ))
            .id();

        // Spawn card text content
        let text_entity = commands
            .spawn((
                CardTextContent {
                    name: card.name.clone(),
                    mana_cost: card.cost.to_string(),
                    type_line: card.type_line(),
                    rules_text: convert_rules_text_to_symbols(&card.rules_text),
                    power_toughness: if let CardDetails::Creature(creature) = &card.card_details {
                        Some(format!("{}/{}", creature.power, creature.toughness))
                    } else {
                        None
                    },
                },
                Transform::default(),
                AppLayer::Cards.layer(), // Use the specific Cards layer
            ))
            .set_parent(card_entity)
            .id();

        info!(
            "Spawned CardTextContent entity {:?} as child of card entity {:?}",
            text_entity, card_entity
        );
    }

    info!(
        "Finished spawning cards, total width={:.2}, using spacing={:.2}",
        total_width, spacing
    );
}
