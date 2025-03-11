use bevy::prelude::*;

use crate::text::components::{CardTextType, TextLayoutInfo};

/// Spawn power/toughness text for a card
pub fn spawn_power_toughness_text(
    commands: &mut Commands,
    pt: &str,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Position the power/toughness text at the bottom right of the card
    let text_pos = card_pos + Vec2::new(card_size.x * 0.35, -card_size.y * 0.42);
    let text_size = Vec2::new(card_size.x * 0.2, card_size.y * 0.1);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(pt.to_string()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size: card_size.y * 0.09, // Slightly larger font for P/T
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Right),
            // Add our custom components
            CardTextType::PowerToughness,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("P/T: {}", pt)),
        ))
        .id();

    entity
}
