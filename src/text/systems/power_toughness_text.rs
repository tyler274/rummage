use bevy::prelude::*;

use crate::text::{
    components::{CardTextType, TextLayoutInfo},
    utils::{calculate_text_position, calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn power/toughness text for a card
pub fn spawn_power_toughness_text(
    commands: &mut Commands,
    pt: &str,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Position the power/toughness text at the bottom right of the card
    let text_pos =
        calculate_text_position(card_pos, card_size, layout.pt_x_offset, layout.pt_y_offset);

    let text_size = calculate_text_size(card_size, layout.pt_width, layout.pt_height);

    let font_size = get_card_font_size(card_size, 24.0);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(pt.to_string()),
            // Add transform for positioning - use z=10 to ensure text is above the card
            Transform::from_translation(text_pos.extend(10.0)),
            GlobalTransform::default(),
            // Add text styling
            TextFont {
                font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Right),
            // Add our custom components
            CardTextType::PowerToughness,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("P/T: {}", pt)),
        ))
        .id();

    entity
}
