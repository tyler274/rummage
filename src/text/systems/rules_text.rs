use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::mana::mana_symbol_to_char;
use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    mana_symbols::{
        ManaSymbolOptions, get_mana_symbol_width, is_valid_mana_symbol, render_mana_symbol,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout, get_mana_symbol_color},
};

/// Directly replace mana symbols in text with their Unicode equivalents
fn replace_mana_symbols_with_unicode(text: &str) -> String {
    use crate::mana::MANA_SYMBOLS;

    let mut result = text.to_string();

    // Replace all mana symbols with their Unicode equivalents
    for (symbol, unicode) in MANA_SYMBOLS {
        result = result.replace(symbol, &unicode.to_string());
    }

    result
}

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

    // Load fonts - both regular and mana fonts
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

    // Render rules text line by line
    let lines = formatted_text.split('\n').collect::<Vec<_>>();
    let line_height = font_size * 1.2; // Line height with some spacing

    for (line_idx, line) in lines.iter().enumerate() {
        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        let y_pos = -(line_idx as f32) * line_height;

        // Use our improved inline mana symbol renderer for all lines
        render_inline_mana_symbols(
            commands,
            line,
            y_pos,
            font_size,
            &regular_font,
            &mana_font,
            parent_entity,
        );
    }

    parent_entity
}

/// Renders a line of text with inline mana symbols
fn render_inline_mana_symbols(
    commands: &mut Commands,
    line: &str,
    y_pos: f32,
    font_size: f32,
    regular_font: &Handle<Font>,
    mana_font: &Handle<Font>,
    parent_entity: Entity,
) {
    // Extract segments of text, separating mana symbols from regular text
    let segments = extract_mana_symbol_segments(line);
    let mut current_x = 0.0;

    // Check if this is an activated ability line (contains {T}: or other mana symbol followed by colon)
    let is_activated_ability = line.contains("{T}:")
        || (line.contains(':')
            && segments
                .iter()
                .any(|(text, is_symbol)| *is_symbol && text.contains('{')));

    // Special handling for compact rendering of activated abilities
    if is_activated_ability {
        // For activated abilities, we'll use a simpler approach:
        // Just render each segment sequentially with proper spacing
        for (segment_text, is_mana_symbol) in segments {
            if segment_text.is_empty() {
                continue;
            }

            if is_mana_symbol {
                // Create symbol-specific vertical adjustment
                let symbol_specific_offset = match segment_text.trim() {
                    "{T}" => font_size * 0.12, // Tap symbol needs specific adjustment
                    "{G}" => font_size * 0.04, // Green mana
                    "{R}" => font_size * 0.04, // Red mana
                    "{B}" => font_size * 0.05, // Black mana
                    "{W}" => font_size * 0.03, // White mana
                    "{U}" => font_size * 0.03, // Blue mana
                    "{C}" => font_size * 0.04, // Colorless mana
                    s if s.len() >= 3 && s.starts_with('{') && s.ends_with('}') => {
                        // Generic mana cost symbols
                        let inner = &s[1..s.len() - 1];
                        if inner.parse::<u32>().is_ok() || inner == "X" {
                            font_size * 0.05
                        } else {
                            0.0
                        }
                    }
                    _ => 0.0,
                };

                // Use the mana symbol renderer for consistent appearance
                render_mana_symbol(
                    commands,
                    &segment_text,
                    Vec2::new(current_x, y_pos + symbol_specific_offset),
                    mana_font.clone(),
                    ManaSymbolOptions {
                        font_size,
                        vertical_alignment_offset: 0.0,
                        z_index: 0.2, // Render above regular text
                        with_shadow: true,
                        alignment: JustifyText::Left,
                    },
                    parent_entity,
                );

                // Advance x position by mana symbol width
                current_x += get_mana_symbol_width(font_size);

                // Add extra spacing after tap symbol
                if segment_text.trim() == "{T}" {
                    current_x += font_size * 0.25; // Add extra spacing after tap symbol
                }

                // Add spacing after a colon if present
                if segment_text.contains(':') {
                    current_x += font_size * 0.15; // Extra spacing after colon
                }
            } else {
                // Render regular text segment
                let segment_clone = segment_text.clone();
                commands
                    .spawn((
                        TextSpan::default(),
                        Text2d::new(segment_clone.clone()),
                        TextFont {
                            font: regular_font.clone(),
                            font_size,
                            ..default()
                        },
                        TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                        Transform::from_translation(Vec3::new(current_x, y_pos, 0.0)),
                        TextLayout::new_with_justify(JustifyText::Left),
                    ))
                    .set_parent(parent_entity);

                // Advance x position based on text width (approximate)
                current_x += segment_clone.len() as f32 * (font_size * 0.5);
            }
        }

        return; // Skip the standard rendering for activated abilities
    }

    // Standard rendering for non-activated ability text
    for (segment_text, is_mana_symbol) in segments {
        if segment_text.is_empty() {
            continue;
        }

        if is_mana_symbol {
            // Create mana symbol options with appropriate vertical alignment
            let symbol_specific_offset = match segment_text.trim() {
                "{T}" => font_size * 0.15, // Tap symbol needs more adjustment
                "{B}" => font_size * 0.05, // Black mana
                "{W}" => font_size * 0.03, // White mana
                "{R}" => font_size * 0.04, // Red mana
                "{U}" => font_size * 0.03, // Blue mana
                "{C}" => font_size * 0.04, // Colorless mana
                s if s.len() >= 3 && s.starts_with('{') && s.ends_with('}') => {
                    // Generic mana cost symbols
                    let inner = &s[1..s.len() - 1];
                    if inner.parse::<u32>().is_ok() || inner == "X" {
                        font_size * 0.05
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            };

            // Use the mana symbol renderer for consistent appearance
            render_mana_symbol(
                commands,
                &segment_text,
                Vec2::new(current_x, y_pos + symbol_specific_offset),
                mana_font.clone(),
                ManaSymbolOptions {
                    font_size,
                    vertical_alignment_offset: 0.0, // We're applying it directly to position
                    z_index: 0.1,
                    with_shadow: true,
                    alignment: JustifyText::Left,
                },
                parent_entity,
            );

            // Advance x position by mana symbol width
            current_x += get_mana_symbol_width(font_size);
        } else {
            // Render regular text segment
            let segment_clone = segment_text.clone();
            commands
                .spawn((
                    TextSpan::default(),
                    Text2d::new(segment_clone.clone()),
                    TextFont {
                        font: regular_font.clone(),
                        font_size,
                        ..default()
                    },
                    TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                    Transform::from_translation(Vec3::new(current_x, y_pos, 0.0)),
                    TextLayout::new_with_justify(JustifyText::Left),
                ))
                .set_parent(parent_entity);

            // Advance x position based on text width (approximate)
            current_x += segment_clone.len() as f32 * (font_size * 0.5);
        }
    }
}

/// Extract segments of text, separating mana symbols from regular text
fn extract_mana_symbol_segments(text: &str) -> Vec<(String, bool)> {
    let mut segments = Vec::new();
    let mut current_pos = 0;

    while current_pos < text.len() {
        if let Some(start) = text[current_pos..].find('{') {
            let symbol_start = current_pos + start;

            // Add text before the symbol
            if symbol_start > current_pos {
                segments.push((text[current_pos..symbol_start].to_string(), false));
            }

            // Find the end of the symbol
            if let Some(end) = text[symbol_start..].find('}') {
                let symbol_end = symbol_start + end + 1;
                let symbol = &text[symbol_start..symbol_end];

                if is_valid_mana_symbol(symbol) {
                    segments.push((symbol.to_string(), true));
                } else {
                    segments.push((symbol.to_string(), false));
                }

                current_pos = symbol_end;
            } else {
                // No closing brace, treat as regular text
                segments.push((text[current_pos..].to_string(), false));
                break;
            }
        } else {
            // No more symbols, add remaining text
            segments.push((text[current_pos..].to_string(), false));
            break;
        }
    }

    segments
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

    // Process each paragraph for proper wrapping
    let mut result = String::new();
    let avg_char_width = font_size * 0.5; // Approximate character width
    let chars_per_line = (max_width / avg_char_width) as usize;

    for (i, paragraph) in paragraphs.iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }

        // Simple word wrap algorithm
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_line_len = 0;

        for word in words {
            let word_len = word.len();

            // Check if adding this word would exceed line width
            if current_line_len + word_len + 1 > chars_per_line && !current_line.is_empty() {
                // Line would be too long, add a line break
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
                current_line_len = word_len;
            } else {
                // Add space before word (except at beginning of line)
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_line_len += 1;
                }
                current_line.push_str(word);
                current_line_len += word_len;
            }
        }

        // Add the last line of the paragraph
        result.push_str(&current_line);
    }

    // Add flavor text if present
    if let Some(flavor) = flavor_part {
        // Add a blank line and the separator
        result.push_str("\n\n—");

        // Simple word wrap for flavor text
        let words: Vec<&str> = flavor.split_whitespace().collect();
        let mut current_line = String::new();
        let mut current_line_len = 0;
        let flavor_chars_per_line = chars_per_line - 2; // Slight indent for flavor text

        for word in words {
            let word_len = word.len();

            // Check if adding this word would exceed line width
            if current_line_len + word_len + 1 > flavor_chars_per_line && !current_line.is_empty() {
                // Line would be too long, add a line break
                result.push_str(&current_line);
                result.push('\n');
                current_line = word.to_string();
                current_line_len = word_len;
            } else {
                // Add space before word (except at beginning of line)
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_line_len += 1;
                }
                current_line.push_str(word);
                current_line_len += word_len;
            }
        }

        // Add the last line of the flavor text
        result.push_str(&current_line);
    }

    result
}
