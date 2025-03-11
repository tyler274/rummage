use bevy::prelude::*;

use crate::text::{
    components::{CardTextContent, CardTextType, TextLayoutInfo},
    utils::{calculate_text_position, calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn mana cost text for a card
pub fn spawn_mana_cost_text(
    commands: &mut Commands,
    content: &CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Position the mana cost text at the top right of the card
    let text_pos = calculate_text_position(
        card_pos,
        card_size,
        layout.mana_cost_x_offset,
        layout.title_y_offset,
    );

    let text_size = calculate_text_size(card_size, layout.mana_cost_width, layout.title_height);

    let font_size = get_card_font_size(card_size, 24.0);

    // Create text with Text2d component
    let entity = commands
        .spawn((
            // Text2d component
            Text2d::new(content.mana_cost.clone()),
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
            CardTextType::ManaCost,
            TextLayoutInfo {
                position: text_pos,
                size: text_size,
                alignment: JustifyText::Right,
            },
            // Add a name for debugging
            Name::new(format!("Mana Cost: {}", content.mana_cost)),
        ))
        .id();

    entity
}
