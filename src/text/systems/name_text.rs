use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn name text for a card
pub fn spawn_name_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate relative position offsets
    let horizontal_offset = layout.name_x_offset;
    let vertical_offset = layout.title_y_offset;

    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    // Calculate text size with adequate width to prevent wrapping of card names
    let text_size = calculate_text_size(card_size, layout.name_width, layout.title_height);

    // Use a more prominent font size for card names like real MTG cards
    let font_size = get_card_font_size(card_size, 28.0);

    // Truncate very long card names to fit in the available space
    let name_text = format_card_name(&content.name, text_size.x, font_size);

    // Create text style bundle
    let text_style = CardTextStyleBundle {
        text_font: TextFont {
            // Bold font for card names like MTG cards
            font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
            font_size,
            ..default()
        },
        text_color: TextColor(Color::BLACK),
        text_layout: TextLayout::new_with_justify(JustifyText::Left),
    };

    // Create text with CardTextBundle
    let entity = commands
        .spawn(CardTextBundle {
            text_2d: Text2d::new(name_text.clone()),
            // Position at the top left of the card, like real MTG cards
            transform: Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            global_transform: GlobalTransform::default(),
            text_font: text_style.text_font,
            text_color: text_style.text_color,
            text_layout: text_style.text_layout,
            card_text_type: CardTextType::Name,
            text_layout_info: TextLayoutInfo {
                position: card_pos + local_offset,
                size: text_size,
                alignment: JustifyText::Left,
            },
            name: Name::new(format!("Card Name: {}", name_text)),
        })
        .id();

    entity
}

/// Format card name to fit within available space
fn format_card_name(name: &str, max_width: f32, font_size: f32) -> String {
    // Estimate max characters that will fit in the available space
    let avg_char_width = font_size * 0.5; // Approximate width per character
    let max_chars = (max_width / avg_char_width).floor() as usize;

    if name.len() <= max_chars {
        // Name fits, return as is
        name.to_string()
    } else {
        // Name is too long, truncate and add ellipsis
        let truncated = &name[0..max_chars.saturating_sub(3)];
        format!("{}...", truncated)
    }
}
