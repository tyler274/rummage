use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
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

    // Calculate relative position offsets
    let horizontal_offset = layout.pt_x_offset;
    let vertical_offset = layout.pt_y_offset;

    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    let text_size = calculate_text_size(card_size, layout.pt_width, layout.pt_height);

    // Make power/toughness prominent like MTG cards
    let font_size = get_card_font_size(card_size, 26.0);

    // Create text style bundle
    let text_style = CardTextStyleBundle {
        text_font: TextFont {
            // Bold font for power/toughness like MTG cards
            font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
            font_size,
            ..default()
        },
        text_color: TextColor(Color::BLACK),
        text_layout: TextLayout::new_with_justify(JustifyText::Center),
    };

    // Create text with CardTextBundle
    let entity = commands
        .spawn(CardTextBundle {
            text_2d: Text2d::new(pt.to_string()),
            transform: Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            global_transform: GlobalTransform::default(),
            text_font: text_style.text_font,
            text_color: text_style.text_color,
            text_layout: text_style.text_layout,
            card_text_type: CardTextType::PowerToughness,
            text_layout_info: TextLayoutInfo {
                position: card_pos + local_offset, // Store absolute position for reference
                size: text_size,
                alignment: JustifyText::Center,
            },
            name: Name::new(format!("P/T: {}", pt)),
        })
        .id();

    entity
}
