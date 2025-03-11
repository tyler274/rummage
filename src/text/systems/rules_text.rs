use bevy::prelude::*;

use crate::text::{
    components::{CardTextContent, CardTextType, TextLayoutInfo},
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn rules text for a card
pub fn spawn_rules_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate relative position offsets
    let horizontal_offset = 0.0; // Centered horizontally
    let vertical_offset = layout.text_box_y_offset;

    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    let text_size = calculate_text_size(card_size, layout.text_box_width, layout.text_box_height);

    let font_size = get_card_font_size(card_size, 18.0);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.rules_text.clone()),
            // Use a relative transform instead of absolute world position
            // The z value is relative to the parent (card)
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Center),
            // Add our custom components
            CardTextType::RulesText,
            TextLayoutInfo {
                position: card_pos + local_offset, // Store absolute position for reference
                size: text_size,
                alignment: JustifyText::Center,
            },
            // Add a name for debugging
            Name::new(format!("Rules Text: {}", content.rules_text)),
        ))
        .id();

    entity
}
