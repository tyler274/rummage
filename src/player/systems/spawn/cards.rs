use super::table::TableLayout;
use crate::camera::components::{AppLayer, GameCamera};
use crate::cards::components::card_entity::CardZone;
use crate::cards::drag::Draggable;
use crate::cards::text::card_text::spawn_card_text_components;
use crate::game_engine::zones::types::Zone;

use bevy::prelude::*;

// Context struct for spawn_visual_cards
pub struct CardSpawnContext<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub game_cameras: &'a Query<'w, 's, Entity, With<GameCamera>>,
    pub card_size: &'a Vec2,
    pub spacing_multiplier: f32,
    pub player_position: Vec3,
    pub player_index: usize,
    pub player_entity: Entity,
    pub table: &'a TableLayout,
    pub asset_server_option: Option<&'a AssetServer>,
}

/// Helper function to spawn visual card entities
pub fn spawn_visual_cards(context: &mut CardSpawnContext, display_cards: Vec<crate::cards::Card>) {
    // Skip if no cards to spawn
    if display_cards.is_empty() {
        warn!("No cards to spawn for player {}", context.player_index);
        return;
    }

    debug!(
        "Spawning {} cards for player {:?} (index {})",
        display_cards.len(),
        context.player_entity,
        context.player_index
    );

    // Increase the spacing between cards, but use a smaller multiplier
    let spacing = context.card_size.x * context.spacing_multiplier * 0.6; // Reduced from 1.5 to 0.6 for tighter card layout

    // Calculate the total width of all cards with spacing
    let total_width = display_cards.len() as f32 * spacing;

    // Store card count before moving display_cards
    let card_count = display_cards.len();

    // Calculate start position with better centering
    let start_x = -(total_width) / 2.0 + spacing / 2.0;

    // Get the card offset for this player based on table position
    let card_offset = context.table.get_card_offset(context.player_index);

    // Determine if the cards should be laid out horizontally or vertically
    let is_horizontal = context.table.is_horizontal_layout(context.player_index);

    // Calculate the starting position and direction based on layout
    let (start_pos, card_direction) = if is_horizontal {
        // Horizontal layout (cards in a row)
        (
            Vec3::new(start_x, context.player_position.y, 0.0) + card_offset,
            Vec3::new(spacing, 0.0, 0.0),
        )
    } else {
        // Vertical layout (cards in a column)
        (
            Vec3::new(context.player_position.x, start_x, 0.0) + card_offset,
            Vec3::new(0.0, spacing, 0.0),
        )
    };

    info!(
        "Starting spawn of {} cards for player {}",
        card_count, context.player_index
    );

    // Spawn each card with proper positioning
    for (i, card) in display_cards.into_iter().enumerate() {
        let card_clone = card.clone(); // Clone card to use later

        // Calculate z-index based on position to ensure proper layering
        // Use a higher base z-value (30.0) to ensure cards are always above playmats (which use around 10.0)
        let z = 30.0 + (i as f32 * 1.0); // Increased z-index base to stay above playmats

        // Calculate the position for this card
        let position = Vec3::new(
            start_pos.x + card_direction.x * i as f32,
            start_pos.y + card_direction.y * i as f32,
            z,
        );

        // Draw cards at a much larger internal size for better text layout
        // but scale them down visually to fit in the playmat
        let internal_card_size = *context.card_size * 6.0; // Much larger internal size for text positioning
        let display_scale = 2.5 / 6.0; // Scale factor to display correctly in the playmat

        // Create a card with a grayish white background for better readability
        let card_entity = context
            .commands
            .spawn(Sprite {
                color: Color::srgb(0.92, 0.92, 0.94), // Grayish white for better readability
                custom_size: Some(internal_card_size),
                ..default()
            })
            .insert(Transform {
                translation: position,
                scale: Vec3::splat(display_scale), // Scale down for display
                ..default()
            })
            .insert(GlobalTransform::default())
            .insert(Visibility::Visible)
            .insert(InheritedVisibility::default())
            .insert(ViewVisibility::default())
            .insert(card)
            .insert(Draggable {
                dragging: false,
                drag_offset: Vec2::ZERO,
                z_index: z,
            })
            .insert(AppLayer::Cards.layer())
            .insert(CardZone {
                zone: Zone::Hand,
                zone_owner: Some(context.player_entity),
            })
            .insert(Name::new(format!("Card: {}", card_clone.name.name)))
            .id();

        // Debug information for every card
        info!(
            "Spawned card '{}' at position ({:.2}, {:.2}, {:.2}) with scale {:.2} and entity {:?}",
            card_clone.name.name, position.x, position.y, position.z, display_scale, card_entity
        );

        // Spawn text components directly instead of just adding marker components
        if let Some(game_asset_server) = context.asset_server_option {
            // Convert card::components::CardRulesText to text::components::CardRulesText
            let rules_text = crate::text::components::CardRulesText {
                rules_text: card_clone.rules_text.rules_text.clone(),
            };

            // With our new Card bundle, we can get all the components directly from the card
            spawn_card_text_components(
                context.commands,
                card_entity,
                (
                    &card_clone, // Use the cloned Card bundle
                    &card_clone.name,
                    &card_clone.cost,
                    &card_clone.type_info,
                    &card_clone.details,
                    &rules_text, // Use the converted rules text
                ),
                &Transform::from_translation(Vec3::ZERO), // Position at origin since text is relative to card
                &Sprite {
                    color: Color::srgb(0.85, 0.85, 0.85),
                    custom_size: Some(internal_card_size),
                    ..default()
                },
                game_asset_server,
                None,
            );
        }

        // Make the card a child of the game camera to ensure it's rendered in the game view
        for camera in context.game_cameras.iter() {
            debug!(
                "Attaching card for player {} to game camera {:?}",
                context.player_index, camera
            );
            context.commands.entity(camera).add_child(card_entity);
        }
    }
}
