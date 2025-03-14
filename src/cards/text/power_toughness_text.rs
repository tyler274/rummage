use bevy::prelude::*;

use crate::text::{
    components::{CardPowerToughness, CardTextStyleBundle, CardTextType, TextLayoutInfo},
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
};

/// Spawn power/toughness text for a card
pub fn spawn_power_toughness_text(
    commands: &mut Commands,
    pt_component: &CardPowerToughness,
    _card_pos: Vec2,
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

    // We don't need text_size for the simplified TextLayoutInfo
    let _text_size = calculate_text_size(card_size, layout.pt_width, layout.pt_height);

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
        text_color: TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        text_layout: TextLayout::new_with_justify(JustifyText::Center),
    };

    // Create text with CardTextBundle
    let entity = commands
        .spawn((
            Text2d::new(pt_component.power_toughness.clone()),
            text_style,
            Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.2)),
            GlobalTransform::default(),
            CardTextType::PowerToughness,
            TextLayoutInfo {
                alignment: JustifyText::Center,
            },
            Name::new(format!("Power/Toughness: {}", pt_component.power_toughness)),
        ))
        .id();

    entity
}
