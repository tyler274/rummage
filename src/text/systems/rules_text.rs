use bevy::prelude::*;

use crate::text::components::{CardTextContent, CardTextType, TextLayoutInfo};

/// Spawn rules text for a card
pub fn spawn_rules_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Position the rules text in the lower middle of the card
    let text_pos = card_pos + Vec2::new(0.0, -card_size.y * 0.1);
    let text_size = Vec2::new(card_size.x * 0.85, card_size.y * 0.35);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.rules_text.clone()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: card_size.y * 0.07,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
            // Add our custom components
            CardTextType::RulesText,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Center,
            },
            // Add a name for debugging
            Name::new(format!("Rules Text: {}", content.rules_text)),
        ))
        .id();

    entity
}
