use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::text::{
    components::{CardPowerToughness, CardTextStyleBundle, CardTextType},
    utils::{get_adaptive_font_size, get_card_layout},
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

    // Calculate the position for P/T - bottom right of the card
    let pt_x = layout.pt_x_offset * card_size.x;
    let pt_y = layout.pt_y_offset * card_size.y;

    // Calculate available width for P/T display
    let available_width = layout.pt_width * card_size.x;

    // Use adaptive font sizing
    // Base size of 14pt, minimum 10pt
    let font_size = get_adaptive_font_size(
        card_size,
        14.0,
        &pt_component.power_toughness,
        available_width,
        10.0,
    );

    // Get the font
    let font = asset_server.load("fonts/DejaVuSans-Bold.ttf");

    // Spawn the P/T entity
    commands
        .spawn((
            Text2d::new(pt_component.power_toughness.clone()),
            Transform::from_translation(Vec3::new(pt_x, pt_y, 0.1)),
            GlobalTransform::default(),
            CardTextStyleBundle {
                text_font: TextFont {
                    font,
                    font_size,
                    ..default()
                },
                text_color: TextColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
                text_layout: TextLayout::new_with_justify(JustifyText::Center),
            },
            CardTextType::PowerToughness,
            Name::new(format!("Power/Toughness: {}", pt_component.power_toughness)),
        ))
        .id()
}
