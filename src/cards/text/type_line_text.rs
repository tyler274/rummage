use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::text::{
    components::{CardTextStyleBundle, CardTextType, CardTypeLine},
    utils::{get_adaptive_font_size, get_card_layout},
};

/// Spawn the type line text for a card
pub fn spawn_type_line_text(
    commands: &mut Commands,
    type_line_component: &CardTypeLine,
    _card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate the position for the type line - centered horizontally, below the art
    let type_line_x = layout.type_line_x_offset * card_size.x;
    let type_line_y = layout.type_line_y_offset * card_size.y;

    // Calculate the available width for the type line
    let available_width = layout.type_line_width * card_size.x;

    // Use adaptive font sizing based on type line length
    // Base size of 14pt, minimum 9pt
    let font_size = get_adaptive_font_size(
        card_size,
        14.0,
        &type_line_component.type_line,
        available_width,
        9.0,
    );

    // Get the font
    let font = asset_server.load("fonts/DejaVuSans.ttf");

    // Spawn the type line entity with proper styling
    commands
        .spawn((
            Text2d::new(type_line_component.type_line.clone()),
            Transform::from_translation(Vec3::new(type_line_x, type_line_y, 0.1)),
            GlobalTransform::default(),
            CardTextStyleBundle {
                text_font: TextFont {
                    font,
                    font_size,
                    ..default()
                },
                text_color: TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                text_layout: TextLayout::new_with_justify(JustifyText::Left),
            },
            CardTextType::TypeLine,
            Name::new(format!("Type Line: {}", type_line_component.type_line)),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id()
}

/// Format type line to match MTG standard style
#[allow(dead_code)]
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
