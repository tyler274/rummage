use bevy::prelude::*;

use crate::text::{
    components::{
        CardTextBundle, CardTextContent, CardTextStyleBundle, CardTextType, TextLayoutInfo,
    },
    utils::{calculate_text_size, get_card_font_size, get_card_layout},
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

    // Calculate relative position offsets - align to the right side of the card title area
    let horizontal_offset = layout.mana_cost_x_offset;
    let vertical_offset = layout.title_y_offset;

    // Calculate the relative position in local space
    let local_offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    let text_size = calculate_text_size(card_size, layout.mana_cost_width, layout.title_height);

    // Adjust font size for better alignment with the card name
    let font_size = get_card_font_size(card_size, 24.0);

    // Format the mana cost for better display
    let mana_text = format_mana_cost(&content.mana_cost);

    // Create text style bundle
    let text_style = CardTextStyleBundle {
        text_font: TextFont {
            font: asset_server.load("fonts/DejaVuSans-Bold.ttf"),
            font_size,
            ..default()
        },
        text_color: TextColor(Color::BLACK),
        text_layout: TextLayout::new_with_justify(JustifyText::Right),
    };

    // Create text with CardTextBundle
    let entity = commands
        .spawn(CardTextBundle {
            text_2d: Text2d::new(mana_text.clone()),
            transform: Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            global_transform: GlobalTransform::default(),
            text_font: text_style.text_font,
            text_color: text_style.text_color,
            text_layout: text_style.text_layout,
            card_text_type: CardTextType::ManaCost,
            text_layout_info: TextLayoutInfo {
                position: card_pos + local_offset,
                size: text_size,
                alignment: JustifyText::Right,
            },
            name: Name::new(format!("Mana Cost: {}", mana_text)),
        })
        .id();

    entity
}

/// Format mana cost for better display
fn format_mana_cost(mana_cost: &str) -> String {
    // For now, just return the mana cost as is
    // In future, this could be enhanced to handle special mana symbols
    mana_cost.to_string()
}
