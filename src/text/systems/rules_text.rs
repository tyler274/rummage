use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    mana_symbols::{
        ManaSymbolOptions, get_mana_symbol_width, is_valid_mana_symbol, render_mana_symbol,
    },
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

    // Parse formatted_text into segments (regular text and mana symbols)
    let segments = parse_text_with_mana_symbols(&formatted_text);

    // Current X position tracker for aligning text segments
    let mut current_x = 0.0;
    let mut current_line = 0;
    let line_height = font_size * 1.2; // Line height with some spacing
    let mut new_line_started = true; // Track if we're at the start of a line

    for (idx, (segment, is_mana_symbol)) in segments.iter().enumerate() {
        // Skip empty segments
        if segment.is_empty() {
            continue;
        }

        // Reset X position and increment Y position for new lines
        if segment == "\n" {
            current_x = 0.0;
            current_line += 1;
            new_line_started = true;
            continue;
        }

        // Calculate Y position based on current line
        let y_pos = -current_line as f32 * line_height;

        if *is_mana_symbol {
            // This is a mana symbol - use our unified rendering function
            let mana_options = ManaSymbolOptions {
                font_size,
                vertical_alignment_offset: font_size * 0.2, // Adjusted for better baseline alignment
                z_index: 0.1,
                with_shadow: true,
            };

            // Special alignment adjustment for different mana symbols
            let extra_horizontal_offset = if segment == "{R}" {
                font_size * 0.1 // Adjusted offset for red mana
            } else if segment == "{G}" {
                font_size * 0.05 // Small adjustment for green mana
            } else {
                0.0
            };

            render_mana_symbol(
                commands,
                segment,
                Vec2::new(current_x + extra_horizontal_offset, y_pos),
                mana_font.clone(),
                mana_options,
                parent_entity,
            );

            // Advance X position using our standardized function
            current_x += get_mana_symbol_width(font_size) + extra_horizontal_offset;
            new_line_started = false;

            // Check if next segment starts with a colon (part of an activated ability)
            // We won't add extra spacing in that case
            let is_part_of_ability = segments
                .get(idx + 1)
                .map(|(next_text, _)| next_text.starts_with(':'))
                .unwrap_or(false);

            if !is_part_of_ability {
                current_x += font_size * 0.05; // Small spacing after standalone mana symbols
            }
        } else {
            // This is regular text - use regular font

            // Special case for ability text that starts with a colon
            let colon_adjustment = if segment.starts_with(':') {
                -font_size * 0.1 // Move colon slightly closer to mana symbol
            } else {
                0.0
            };

            commands
                .spawn((
                    TextSpan::default(),
                    Text2d::new(segment.clone()),
                    TextFont {
                        font: regular_font.clone(),
                        font_size,
                        ..default()
                    },
                    TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                    // Position at current_x with potential colon adjustment
                    Transform::from_translation(Vec3::new(
                        current_x + colon_adjustment,
                        y_pos,
                        0.0,
                    )),
                ))
                .set_parent(parent_entity);

            // Approximately calculate width based on character count and font size
            // Add a bit more precision for different character widths
            let char_width_factor = if segment.contains(':') {
                0.45 // Make colons narrower for better spacing in "{R}: effect" constructs
            } else if segment.trim().is_empty() {
                0.25 // Very narrow for spaces and whitespace
            } else if segment.trim() == ":" {
                0.3 // Even narrower for isolated colons
            } else {
                0.5 // Standard spacing for regular text
            };

            current_x += segment.chars().count() as f32 * (font_size * char_width_factor);

            // Add extra space after colon to improve readability of ability costs
            if segment.contains(':') {
                current_x += font_size * 0.15; // Increased extra space after colons
            }
        }
    }

    parent_entity
}

/// Parse text into segments, identifying mana symbols and regular text
fn parse_text_with_mana_symbols(text: &str) -> Vec<(String, bool)> {
    let mut segments = Vec::new();

    // First, check if the text contains an activated ability pattern like "{R}:"
    if let Some(ability_matches) = find_activated_abilities(text) {
        // Text contains one or more activated abilities
        let mut current_pos = 0;

        for (start, end) in ability_matches {
            // Add any text before this ability
            if start > current_pos {
                let before_text = &text[current_pos..start];
                if !before_text.is_empty() {
                    segments.extend(split_regular_text(before_text));
                }
            }

            // Extract the ability text
            let ability_text = &text[start..end];

            // Process the ability - first find the mana symbol
            if let Some(symbol_end) = ability_text.find('}') {
                // Add a newline before the mana symbol if it's not at the beginning of the text
                // and the previous character isn't already a newline
                let needs_newline = start > 0 && !text[start - 1..start].contains('\n');
                if needs_newline {
                    segments.push(("\n".to_string(), false));
                }

                // Get the symbol
                let symbol = &ability_text[0..symbol_end + 1];

                // Handle mana symbol positioning - we want it left-aligned
                segments.push((symbol.to_string(), true)); // Add mana symbol

                // Handle the text after the mana symbol until the colon
                let after_symbol = &ability_text[symbol_end + 1..];

                // Find the colon
                if let Some(colon_pos) = after_symbol.find(':') {
                    // Add the colon with minimal spacing
                    segments.push((":".to_string(), false));

                    // Add the rest of the ability text after the colon
                    if colon_pos + 1 < after_symbol.len() {
                        let after_colon = &after_symbol[colon_pos + 1..];

                        // Ensure proper spacing after the colon
                        if after_colon.starts_with(' ') {
                            segments.push((" ".to_string(), false)); // Add one space

                            // Add the rest of the text without the space
                            if after_colon.len() > 1 {
                                segments.push((after_colon[1..].to_string(), false));
                            }
                        } else {
                            segments.push((" ".to_string(), false)); // Add one space
                            segments.push((after_colon.to_string(), false)); // Add the text
                        }
                    }
                } else {
                    // No colon found, just add the rest as is
                    segments.push((after_symbol.to_string(), false));
                }
            }

            current_pos = end;
        }

        // Add any remaining text after the last ability
        if current_pos < text.len() {
            let remaining = &text[current_pos..];
            if !remaining.is_empty() {
                segments.extend(split_regular_text(remaining));
            }
        }
    } else {
        // No activated abilities, just process normally
        segments.extend(split_regular_text(text));
    }

    // Final pass to handle newlines separately
    let mut final_segments = Vec::new();
    for (segment, is_symbol) in segments {
        if is_symbol {
            final_segments.push((segment, true));
        } else {
            // For text segments, handle line breaks separately
            let mut current_pos = 0;
            while current_pos < segment.len() {
                if let Some(nl_pos) = segment[current_pos..].find('\n') {
                    // Add text before newline
                    let before_nl = &segment[current_pos..current_pos + nl_pos];
                    if !before_nl.is_empty() {
                        final_segments.push((before_nl.to_string(), false));
                    }

                    // Add newline as separate segment
                    final_segments.push(("\n".to_string(), false));

                    current_pos += nl_pos + 1;
                } else {
                    // No more newlines, add the rest as a single segment
                    let rest = &segment[current_pos..];
                    if !rest.is_empty() {
                        final_segments.push((rest.to_string(), false));
                    }
                    break;
                }
            }
        }
    }

    final_segments
}

/// Find all occurrences of activated ability patterns like "{R}:" or "{T}:" in text
fn find_activated_abilities(text: &str) -> Option<Vec<(usize, usize)>> {
    let mut matches = Vec::new();
    let mut i = 0;

    while i < text.len() {
        if let Some(start) = text[i..].find('{') {
            let start_pos = i + start;

            // Make sure we don't go out of bounds
            if start_pos >= text.len() - 1 {
                break;
            }

            // Look for the closing brace
            if let Some(brace_end) = text[start_pos..].find('}') {
                let brace_end_pos = start_pos + brace_end + 1;

                // Make sure we don't go out of bounds
                if brace_end_pos >= text.len() {
                    i = start_pos + 1;
                    continue;
                }

                // Check if this is a valid mana symbol or tap symbol
                let symbol = &text[start_pos..brace_end_pos];
                if !is_valid_mana_symbol(symbol) {
                    i = start_pos + 1;
                    continue;
                }

                // Look for a colon after the symbol (allowing for whitespace)
                let after_brace = &text[brace_end_pos..];
                let mut colon_pos = None;

                // Look for the colon, allowing for whitespace
                for (offset, ch) in after_brace.char_indices() {
                    if ch == ':' {
                        colon_pos = Some(offset);
                        break;
                    } else if !ch.is_whitespace() {
                        // If we hit a non-whitespace character that's not a colon, this isn't an activated ability
                        break;
                    }
                }

                if let Some(colon_offset) = colon_pos {
                    // This is an activated ability!
                    let colon_pos = brace_end_pos + colon_offset;

                    // Find the end of the ability (next period, newline, or end of text)
                    let after_colon = &text[colon_pos + 1..];
                    let ability_end = after_colon
                        .find('.')
                        .map(|p| p + 1)
                        .unwrap_or_else(|| after_colon.find('\n').unwrap_or(after_colon.len()));

                    matches.push((start_pos, colon_pos + 1 + ability_end));
                    i = colon_pos + 1 + ability_end;
                    continue;
                }
            }

            // If we get here, we didn't find a complete ability at this position
            i = start_pos + 1;
        } else {
            // No more opening braces
            break;
        }
    }

    if matches.is_empty() {
        None
    } else {
        Some(matches)
    }
}

/// Split regular text (non-ability text) into segments
fn split_regular_text(text: &str) -> Vec<(String, bool)> {
    let mut segments = Vec::new();
    let mut i = 0;

    while i < text.len() {
        if let Some(start) = text[i..].find('{') {
            let start_pos = i + start;

            // Add text before the symbol if any
            if start_pos > i {
                segments.push((text[i..start_pos].to_string(), false));
            }

            // Find the closing brace
            if let Some(end) = text[start_pos..].find('}') {
                let end_pos = start_pos + end + 1;
                let symbol = &text[start_pos..end_pos];

                if is_valid_mana_symbol(symbol) {
                    segments.push((symbol.to_string(), true));
                } else {
                    segments.push((symbol.to_string(), false));
                }

                i = end_pos;
                continue;
            } else {
                // No closing brace - treat as regular text
                segments.push((text[i..].to_string(), false));
                break;
            }
        } else {
            // No more symbols
            segments.push((text[i..].to_string(), false));
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
