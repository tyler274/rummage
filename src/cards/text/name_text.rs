use crate::cards::Card;
use crate::text::{
    components::{CardNameText, CardTextType},
    utils::{CardTextLayout, get_adaptive_font_size, get_card_layout},
};
use bevy::prelude::*;

/// Creates text entity for card name
pub fn create_name_text(
    commands: &mut Commands,
    name_text_component: &CardNameText,
    _card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Load font
    let font = asset_server.load("fonts/DejaVuSans-Bold.ttf");
    let layout = get_card_layout();

    // Calculate available width for the name
    let available_width = layout.name_width * card_size.x;

    // Calculate adaptive font size based on name length
    // Use a more aggressive minimum size reduction for longer names
    let min_font_size = if name_text_component.name.len() > 15 {
        8.0
    } else {
        9.0
    };

    let font_size = get_adaptive_font_size(
        card_size,
        16.0,
        &name_text_component.name,
        available_width,
        min_font_size,
    );

    // Position the name at the top left of the card using layout parameters
    // Ensure there's always a minimum margin from the card edge
    let margin_from_edge = card_size.x * 0.05; // 5% of card width as minimum margin
    let raw_x_pos = layout.name_x_offset * card_size.x;

    // Calculate left edge of card (assuming center is at 0)
    let card_left_edge = -card_size.x / 2.0;

    // Ensure text position is at least margin_from_edge away from the left edge
    let name_x = (raw_x_pos).max(card_left_edge + margin_from_edge);

    let name_position = Vec2::new(name_x, layout.name_y_offset * card_size.y);

    // Create the text entity
    commands
        .spawn((
            Text2d::new(name_text_component.name.clone()),
            Transform::from_translation(Vec3::new(
                name_position.x,
                name_position.y,
                0.1, // Slightly above the card
            )),
            GlobalTransform::default(),
            TextFont {
                font,
                font_size,
                ..default()
            },
            TextColor(Color::BLACK),
            TextLayout::new_with_justify(JustifyText::Left),
            CardTextType::Name,
            Name::new(format!("Card Name: {}", name_text_component.name)),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
        ))
        .id()
}

/// System implementation to spawn name text for cards
#[allow(dead_code)]
pub fn name_text_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    // Load font
    let layout = CardTextLayout::default();

    for (entity, _transform, card) in query.iter() {
        // Load font for each iteration to avoid move issues
        let font = asset_server.load("fonts/DejaVuSans-Bold.ttf");

        // Set font size for card name
        let font_size = 20.0;
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create formatted card name - truncate if too long
        let name_text =
            format_card_name(&card.name.name, font_size, layout.name_width * card_size.x);

        // Calculate position relative to card
        let name_position = Vec2::new(
            layout.name_x_offset * card_size.x,
            layout.name_y_offset * card_size.y,
        );

        // Create the text entity
        commands
            .spawn((
                Text2d::new(name_text.clone()),
                Transform::from_translation(Vec3::new(
                    name_position.x,
                    name_position.y,
                    0.1, // Slightly above the card
                )),
                GlobalTransform::default(),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(Color::BLACK),
                TextLayout::new_with_justify(JustifyText::Left),
                CardTextType::Name,
                Name::new(format!("Card Name: {}", name_text)),
            ))
            .set_parent(entity);
    }
}

/// Format card name to fit within bounds and handle long names
fn format_card_name(name: &str, font_size: f32, max_width: f32) -> String {
    // Estimate average character width (this will vary with the actual font)
    let avg_char_width = font_size * 0.55; // Adjusted for more accurate estimation

    // Calculate max chars that fit in the available width
    let max_chars = (max_width / avg_char_width).floor() as usize;

    // Use a reasonable max - reduced to ensure long names fit properly
    let max_chars = max_chars.min(24); // Increased from 20 for longer names

    if name.len() <= max_chars {
        name.to_string()
    } else {
        // Truncate and add ellipsis
        format!("{}…", &name[0..max_chars.saturating_sub(1)])
    }
}
