use bevy::prelude::*;

use crate::text::components::{CardTextContent, CardTextType, TextLayoutInfo};

/// Spawn name text for a card
pub fn spawn_name_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Position the name text at the top of the card, slightly to the left
    let text_pos = card_pos + Vec2::new(-card_size.x * 0.25, card_size.y * 0.42);
    let text_size = Vec2::new(card_size.x * 0.7, card_size.y * 0.1);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.name.clone()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: card_size.y * 0.09, // Slightly larger font for the name
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Left),
            // Add our custom components
            CardTextType::Name,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Left,
            },
            // Add a name for debugging
            Name::new(format!("Text: {}", content.name)),
        ))
        .id();

    entity
}
