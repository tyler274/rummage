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
    // Reduce font size slightly to keep names within bounds
    let font_size = get_card_font_size(card_size, 24.0);

    // Apply strict bounds on name length to ensure it stays within card boundaries
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
        // Keep left alignment but ensure text stays within bounds
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

/// Format card name to fit within available space with stricter bounds
fn format_card_name(name: &str, max_width: f32, font_size: f32) -> String {
    // Use a more conservative estimate for character width to prevent overflow
    let avg_char_width = font_size * 0.6; // Increased estimate to ensure text stays within bounds
    let max_chars = (max_width / avg_char_width).floor() as usize;

    // Use a minimum truncation threshold to prevent very short names
    let safe_max_chars = max_chars.min(20).max(10); // No more than 20 chars, at least 10

    if name.len() <= safe_max_chars {
        // Name fits, return as is
        name.to_string()
    } else {
        // Name is too long, truncate more aggressively and add ellipsis
        let truncated = &name[0..safe_max_chars.saturating_sub(3)];
        format!("{}...", truncated)
    }
}
