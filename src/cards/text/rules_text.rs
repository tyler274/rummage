use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::text::{
    components::{CardRulesText, CardTextType, TextLayoutInfo},
    mana_symbols::is_valid_mana_symbol,
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Directly replace mana symbols in text with their Unicode equivalents
/// This is a simpler alternative to the more complex inline mana symbol rendering
/// that can be used for plain text displays or debugging purposes.
#[allow(dead_code)]
pub fn replace_mana_symbols_with_unicode(text: &str) -> String {
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
    rules_text_component: &CardRulesText,
    _card_pos: Vec2,
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

    // Set font size based on card dimensions
    // Using a slightly smaller base size for rules text to fit more content
    let font_size = get_card_font_size(card_size, 14.0);

    // Format the rules text to fit within the specified width
    let formatted_text =
        format_rules_text(&rules_text_component.rules_text, text_size.x, font_size);

    // Load fonts
    let regular_font: Handle<Font> = asset_server.load("fonts/DejaVuSans.ttf");
    let _mana_font: Handle<Font> = asset_server.load("fonts/Mana.ttf"); // Keep for future mana symbol rendering

    // Spawn the text entity with proper positioning
    let text_entity = commands
        .spawn((
            Text2d::new(formatted_text.clone()),
            Transform::from_translation(Vec3::new(
                local_offset.x,
                local_offset.y,
                0.1, // Slightly above the card surface
            )),
            GlobalTransform::default(),
            TextFont {
                font: regular_font.clone(),
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Left),
            CardTextType::RulesText,
            TextLayoutInfo {
                alignment: JustifyText::Left,
            },
            Name::new("Card Rules Text"),
        ))
        .id();

    // For now, we're not adding inline mana symbols
    // Future: add_mana_symbols_as_children(commands, text_entity, &formatted_text, font_size, &regular_font, &mana_font);

    text_entity
}

/// Add mana symbols as child entities with TextSpan components - Deprecated
/// This function is no longer used and kept for reference
#[allow(dead_code)]
fn add_mana_symbols_as_children(
    _commands: &mut Commands,
    _parent_entity: Entity,
    _formatted_text: &str,
    _font_size: f32,
    _regular_font: &Handle<Font>,
    _mana_font: &Handle<Font>,
) {
    // This approach caused the TextSpan warning and has been removed
    // We now use a simpler approach with just a plain Text2d component
    unimplemented!()
}

/// Extract segments of text, separating mana symbols from regular text
#[allow(dead_code)]
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

/// Format rules text to fit within the specified width
fn format_rules_text(text: &str, max_width: f32, font_size: f32) -> String {
    // Calculate approximate characters per line based on font size
    // Using a conservative estimate for proportional font
    let approximate_char_width = font_size * 0.5; // Roughly half the font size
    let chars_per_line = (max_width / approximate_char_width).floor() as usize;

    // If text is empty, return empty string
    if text.is_empty() {
        return String::new();
    }

    let mut formatted = String::new();
    let mut current_line_length = 0;

    // Split on existing newlines first to respect source formatting
    for paragraph in text.split('\n') {
        if !formatted.is_empty() {
            formatted.push('\n');
            current_line_length = 0;
        }

        let words = paragraph.split_whitespace().collect::<Vec<&str>>();

        for (i, word) in words.iter().enumerate() {
            // Check if adding this word would exceed the line width
            if current_line_length + word.len() + 1 > chars_per_line && current_line_length > 0 {
                formatted.push('\n');
                current_line_length = 0;
            } else if i > 0 && current_line_length > 0 {
                // Add space before word unless it's the first word of a line
                formatted.push(' ');
                current_line_length += 1;
            }

            // Special handling for mana symbols to keep them together
            if word.contains('{') && word.contains('}') {
                // Add the word without breaking it
                formatted.push_str(word);
                current_line_length += word.len();
            } else {
                // Add the word
                formatted.push_str(word);
                current_line_length += word.len();
            }
        }
    }

    formatted
}

/// Renders a line of text with inline mana symbols
#[allow(dead_code)] // Kept for reference but no longer used
fn render_inline_mana_symbols(
    _commands: &mut Commands,
    _line: &str,
    _y_pos: f32,
    _font_size: f32,
    _regular_font: &Handle<Font>,
    _mana_font: &Handle<Font>,
    _parent_entity: Entity,
) {
    // This function is kept for reference but is no longer used
    // We now build the Text component directly with sections
}
