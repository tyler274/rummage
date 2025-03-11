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

    // Calculate the maximum width for text in pixels
    let max_text_width = text_size.x;

    // Format the rules text with proper line breaks and wrapping
    let formatted_text = format_rules_text(&content.rules_text, max_text_width, font_size);

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
            // Add linebreak behavior to ensure text wrapping
            TextLineBreak::default(),
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

/// Format rules text with proper line breaks, spacing, and wrapping
fn format_rules_text(text: &str, max_width: f32, font_size: f32) -> String {
    // Split by existing line breaks
    let paragraphs: Vec<&str> = text.split('\n').collect();

    // Process each paragraph to ensure proper width
    let mut formatted_paragraphs = Vec::new();

    for paragraph in paragraphs {
        // Skip empty paragraphs
        if paragraph.trim().is_empty() {
            formatted_paragraphs.push(String::new());
            continue;
        }

        // Estimate characters per line based on font size and max width
        // This is an approximation - actual text rendering may vary
        let avg_char_width = font_size * 0.5; // Approximate width of a character
        let chars_per_line = (max_width / avg_char_width).floor() as usize;

        if chars_per_line <= 0 {
            // If the line is too narrow, just add the paragraph as is
            formatted_paragraphs.push(paragraph.trim().to_string());
            continue;
        }

        // Perform manual word wrapping
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_line_length = 0;

        for word in words {
            let word_length = word.len();

            // Check if adding this word would exceed the line length
            if current_line_length + word_length + 1 > chars_per_line && !current_line.is_empty() {
                // Line would be too long, add the current line to formatted lines
                formatted_paragraphs.push(current_line.trim().to_string());
                current_line = word.to_string();
                current_line_length = word_length;
            } else {
                // Add word to current line
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_line_length += 1;
                }
                current_line.push_str(word);
                current_line_length += word_length;
            }
        }

        // Add the last line if it's not empty
        if !current_line.is_empty() {
            formatted_paragraphs.push(current_line.trim().to_string());
        }
    }

    // Join paragraphs with line breaks
    formatted_paragraphs.join("\n")
}
