use bevy::prelude::*;

use crate::text::components::{CardTextContent, CardTextType, TextLayoutInfo};

/// Spawn mana cost text for a card
pub fn spawn_mana_cost_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Position the mana cost text at the top right of the card
    let text_pos = card_pos + Vec2::new(card_size.x * 0.35, card_size.y * 0.42);
    let text_size = Vec2::new(card_size.x * 0.3, card_size.y * 0.1);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.mana_cost.clone()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: card_size.y * 0.09, // Slightly larger font for the mana cost
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Right),
            // Add our custom components
            CardTextType::ManaCost,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("Mana Cost: {}", content.mana_cost)),
        ))
        .id();

    entity
}
