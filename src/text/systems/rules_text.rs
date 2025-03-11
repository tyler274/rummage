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

    // Calculate the text box size with padding applied
    let text_size = calculate_text_size(
        card_size,
        layout.text_box_width - (layout.text_box_padding * 2.0),
        layout.text_box_height - (layout.text_box_padding * 2.0),
    );

    // Adjust font size based on card size and text length
    let base_font_size = 18.0;
    let text_length_factor = (content.rules_text.len() as f32 / 100.0).clamp(0.5, 1.5);
    let adjusted_font_size = base_font_size / text_length_factor.max(1.0);
    let font_size = get_card_font_size(card_size, adjusted_font_size);

    // Format the rules text with proper line breaks
    let formatted_text = format_rules_text(&content.rules_text);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component with formatted text
            Text2d::new(formatted_text),
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
            // Use left alignment for better readability
            TextLayout::new_with_justify(JustifyText::Left),
            // Add our custom components
            CardTextType::RulesText,
            TextLayoutInfo {
                position: card_pos + local_offset, // Store absolute position for reference
                size: text_size,
                alignment: JustifyText::Left,
            },
            // Add a name for debugging
            Name::new(format!("Rules Text: {}", content.rules_text)),
        ))
        .id();

    entity
}

/// Format rules text with proper line breaks and spacing
fn format_rules_text(text: &str) -> String {
    // Split by existing line breaks
    let lines: Vec<&str> = text.split('\n').collect();

    // Process each line to ensure proper width
    let mut formatted_lines = Vec::new();

    for line in lines {
        // Skip empty lines
        if line.trim().is_empty() {
            formatted_lines.push(String::new());
            continue;
        }

        // Add the line with proper spacing
        formatted_lines.push(line.trim().to_string());
    }

    // Join lines with line breaks
    formatted_lines.join("\n")
}
