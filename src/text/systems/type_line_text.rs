use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn type line text for a card
pub fn spawn_type_line_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate relative position offsets
    let horizontal_offset = 0.0; // Centered horizontally
    let vertical_offset = layout.type_line_y_offset;

    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    let text_size = calculate_text_size(card_size, layout.type_line_width, layout.type_line_height);

    // Smaller font size for type line to better match MTG card proportions
    let font_size = get_card_font_size(card_size, 16.0);

    // Format type line consistently (match MTG style with em-dash between types)
    let formatted_type_line = format_type_line(&content.type_line);

    // Create text style bundle
    let text_style = CardTextStyleBundle {
        text_font: TextFont {
            font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
            font_size,
            ..default()
        },
        text_color: TextColor(Color::rgba(0.0, 0.0, 0.0, 0.9)),
        text_layout: TextLayout::new_with_justify(JustifyText::Left),
    };

    // Create text with CardTextBundle
    let entity = commands
        .spawn((
            Text2d::new(formatted_type_line.clone()),
            text_style,
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.2)),
            GlobalTransform::default(),
            CardTextType::TypeLine,
            TextLayoutInfo {
                position: card_pos + local_offset,
                size: text_size,
                alignment: JustifyText::Left,
            },
            Name::new(format!("Type Line: {}", formatted_type_line)),
        ))
        .id();

    entity
}

/// Format type line to match MTG standard style
fn format_type_line(type_line: &str) -> String {
    // If the type line already has a proper em-dash, return it as is
    if type_line.contains(" — ") {
        return type_line.to_string();
    }

    // Check if the type line contains a simple dash
    if type_line.contains(" - ") {
        // Replace simple dash with proper em-dash (with spacing)
        return type_line.replace(" - ", " — ");
    }

    // Check if we need to add a subtype separator
    if type_line.contains("Creature")
        || type_line.contains("Artifact")
        || type_line.contains("Enchantment")
        || type_line.contains("Land")
    {
        // Split the type line into words
        let words: Vec<&str> = type_line.split_whitespace().collect();

        // Find where subtypes might start (usually after the main type)
        let main_types = [
            "Creature",
            "Artifact",
            "Enchantment",
            "Land",
            "Planeswalker",
            "Instant",
            "Sorcery",
            "Battle",
        ];

        let mut separator_index = 0;
        for (i, word) in words.iter().enumerate() {
            if main_types.contains(word) {
                separator_index = i + 1;
            }
        }

        // If we have a subtype, format with proper em-dash
        if separator_index > 0 && separator_index < words.len() {
            let (main_part, sub_part) = words.split_at(separator_index);
            return format!("{} — {}", main_part.join(" "), sub_part.join(" "));
        }
    }

    // If no formatting needed, return original
    type_line.to_string()
}
