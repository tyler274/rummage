use bevy::prelude::*;

use crate::text::components::{CardTextContent, CardTextType, TextLayoutInfo};

/// Spawn type line text for a card
pub fn spawn_type_line_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Position the type line text in the middle of the card
    let text_pos = card_pos + Vec2::new(0.0, card_size.y * 0.15);
    let text_size = Vec2::new(card_size.x * 0.85, card_size.y * 0.1);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.type_line.clone()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: card_size.y * 0.08,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
            // Add our custom components
            CardTextType::TypeLine,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Center,
            },
            // Add a name for debugging
            Name::new(format!("Type Line: {}", content.type_line)),
        ))
        .id();

    entity
}
