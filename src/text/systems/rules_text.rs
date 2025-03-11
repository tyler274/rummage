use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout, get_mana_symbol_color},
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
    let base_font_size = 16.0; // Slightly smaller base font for better readability
    let text_length_factor = (content.rules_text.len() as f32 / 100.0).clamp(0.5, 1.5);
    let adjusted_font_size = base_font_size / text_length_factor.max(1.0);
    let font_size = get_card_font_size(card_size, adjusted_font_size);

    // Calculate the maximum width for text in pixels
    let max_text_width = text_size.x;

    // Format the rules text with proper line breaks and wrapping
    let formatted_text = format_rules_text(&content.rules_text, max_text_width, font_size);

    // Load fonts
    let regular_font = asset_server.load("fonts/DejaVuSans.ttf");
    let mana_font = asset_server.load("fonts/Mana.ttf");

    // Create the parent text entity
    let parent_entity = commands
        .spawn((
            // Empty root for text container
            Text2d::new(""),
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.2)),
            GlobalTransform::default(),
            TextLayout::new_with_justify(JustifyText::Left),
            CardTextType::RulesText,
            TextLayoutInfo {
                position: card_pos + local_offset,
                size: text_size,
                alignment: JustifyText::Left,
            },
            Name::new(format!("Rules Text: {}", formatted_text.replace('\n', " "))),
        ))
        .id();

    // Special case handling for rules text with mana symbols
    // This time we'll create a system that renders the text in segments with appropriate styling

    // Parse formatted_text into segments (regular text and mana symbols)
    let segments = parse_text_with_mana_symbols(&formatted_text);

    // Current X position tracker for aligning text segments
    let mut current_x = 0.0;
    let mut current_line = 0;
    let line_height = font_size * 1.2; // Line height with some spacing

    for (segment, is_mana_symbol) in segments {
        // Skip empty segments
        if segment.is_empty() {
            continue;
        }

        // Reset X position and increment Y position for new lines
        if segment == "\n" {
            current_x = 0.0;
            current_line += 1;
            continue;
        }

        // Calculate Y position based on current line
        let y_pos = -current_line as f32 * line_height;

        if is_mana_symbol {
            // This is a mana symbol - use mana font and appropriate color
            let symbol_color = get_mana_symbol_color(&segment);

            commands
                .spawn((
                    TextSpan::default(),
                    Text2d::new(segment.clone()),
                    TextFont {
                        font: mana_font.clone(),
                        font_size,
                        ..default()
                    },
                    TextColor(symbol_color),
                    // Position at current_x, with y offset based on line number
                    Transform::from_translation(Vec3::new(current_x, y_pos, 0.1)),
                ))
                .set_parent(parent_entity);

            // Advance X position - mana symbols are roughly square
            current_x += font_size * 0.8;
        } else {
            // This is regular text - use regular font
            commands
                .spawn((
                    TextSpan::default(),
                    Text2d::new(segment.clone()),
                    TextFont {
                        font: regular_font.clone(),
                        font_size,
                        ..default()
                    },
                    TextColor(Color::rgba(0.0, 0.0, 0.0, 0.9)),
                    // Position at current_x
                    Transform::from_translation(Vec3::new(current_x, y_pos, 0.0)),
                ))
                .set_parent(parent_entity);

            // Approximately calculate width based on character count and font size
            // This is imprecise but works for a rough layout
            current_x += segment.chars().count() as f32 * (font_size * 0.5);
        }
    }

    parent_entity
}

/// Parse text into segments, identifying mana symbols and regular text
fn parse_text_with_mana_symbols(text: &str) -> Vec<(String, bool)> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut i = 0;

    // Special case for known MTG symbols in text, like {R}:
    // Look for pattern like "{R}:" which is a cost symbol followed by colon

    while i < text.len() {
        if i + 1 < text.len() && text[i..i + 1] == *"{" {
            // Possible start of a mana symbol

            // First, add any accumulated text
            if !current_text.is_empty() {
                segments.push((current_text.clone(), false));
                current_text.clear();
            }

            // Find the end of the potential symbol
            let mut j = i + 1;
            let mut valid_symbol = false;

            while j < text.len() && text[j..j + 1] != *"}" {
                j += 1;
            }

            if j < text.len() {
                // We found a closing brace
                let symbol = &text[i..=j];

                if is_valid_mana_symbol(symbol) {
                    // This is a valid mana symbol
                    segments.push((symbol.to_string(), true));
                    valid_symbol = true;
                    i = j + 1;
                }
            }

            if !valid_symbol {
                // Not a valid symbol, just add the opening brace as text
                current_text.push('{');
                i += 1;
            }
        } else if text[i..i + 1] == *"\n" {
            // Handle line breaks specially

            // Add any accumulated text first
            if !current_text.is_empty() {
                segments.push((current_text.clone(), false));
                current_text.clear();
            }

            // Add the line break as a special segment
            segments.push(("\n".to_string(), false));
            i += 1;
        } else {
            // Regular character, add to current text
            current_text.push_str(&text[i..i + 1]);
            i += 1;
        }
    }

    // Add any remaining text
    if !current_text.is_empty() {
        segments.push((current_text, false));
    }

    segments
}

/// Check if a string is a valid mana symbol
fn is_valid_mana_symbol(symbol: &str) -> bool {
    if symbol.len() < 3 || !symbol.starts_with('{') || !symbol.ends_with('}') {
        return false;
    }

    let inner = &symbol[1..symbol.len() - 1];
    match inner {
        "W" | "U" | "B" | "R" | "G" | "C" | "T" => true,
        "X" => true,
        _ => {
            // Check if it's a numeric generic mana cost
            inner.parse::<u32>().is_ok()
        }
    }
}

/// Format rules text with proper line breaks, spacing, and wrapping
/// Now includes better paragraph separation and support for flavor text
fn format_rules_text(text: &str, max_width: f32, font_size: f32) -> String {
    // Check for flavor text separator (MTG uses a line between rules and flavor)
    let (rules_part, flavor_part) = if text.contains("—") {
        let parts: Vec<&str> = text.splitn(2, "—").collect();
        if parts.len() > 1 {
            (parts[0].trim(), Some(parts[1].trim()))
        } else {
            (text, None)
        }
    } else {
        (text, None)
    };

    // Split rules by existing line breaks
    let paragraphs: Vec<&str> = rules_part.split('\n').collect();

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

    // Format the result with proper MTG-style paragraph spacing
    let rules_text = formatted_paragraphs.join("\n");

    // Add flavor text if present (with proper formatting and separator)
    if let Some(flavor) = flavor_part {
        // Format the flavor text with the same line wrapping logic
        let flavor_paragraphs: Vec<&str> = flavor.split('\n').collect();
        let mut formatted_flavor = Vec::new();

        for paragraph in flavor_paragraphs {
            if paragraph.trim().is_empty() {
                formatted_flavor.push(String::new());
                continue;
            }

            // Italicize flavor text by adding markdown-style indicators
            // Note: This assumes your text rendering supports these markers
            formatted_flavor.push(format!("*{}*", paragraph.trim()));
        }

        // Join with double line break to separate rules text from flavor text
        format!("{}\n\n—{}", rules_text, formatted_flavor.join("\n"))
    } else {
        rules_text
    }
}
