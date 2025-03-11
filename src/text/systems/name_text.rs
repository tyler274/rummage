use crate::card::Card;
use crate::text::{
    components::{CardTextBundle, CardTextType, TextLayoutInfo},
    utils::{
        CardTextLayout, calculate_text_position, calculate_text_size, get_card_font_size,
        get_card_layout,
    },
};
use bevy::prelude::*;

/// Creates text entity for card name
pub fn create_name_text(
    commands: &mut Commands,
    content: &crate::text::components::CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Load font
    let font = asset_server.load("fonts/DejaVuSans-Bold.ttf");
    let layout = get_card_layout();

    // Set font size for card name
    let font_size = get_card_font_size(card_size, 18.0);

    // Create formatted card name - truncate if too long
    let name_text = format_card_name(&content.name, font_size, layout.name_width * card_size.x);

    // Position the name at the top left of the card using layout parameters
    let name_position = Vec2::new(
        layout.name_x_offset * card_size.x,
        layout.name_y_offset * card_size.y,
    );

    // Create the text entity
    commands
        .spawn(CardTextBundle {
            text_2d: Text2d::new(name_text.clone()),
            transform: Transform::from_translation(Vec3::new(
                name_position.x,
                name_position.y,
                0.9, // High z-index to ensure visibility
            )),
            global_transform: GlobalTransform::default(),
            text_font: TextFont {
                font: font.clone(),
                font_size,
                ..default()
            },
            text_color: TextColor(Color::srgba(0.0, 0.0, 0.0, 1.0)), // Black text
            text_layout: TextLayout::new_with_justify(JustifyText::Left), // Left justified
            card_text_type: CardTextType::Name,
            text_layout_info: TextLayoutInfo {
                position: card_pos + name_position,
                size: calculate_text_size(card_size, layout.name_width, 0.08),
                alignment: JustifyText::Left, // Left aligned
            },
            name: Name::new(format!("Card Name: {}", name_text)),
        })
        .id()
}

/// System implementation that finds cards and creates name text for them
pub fn name_text_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    // Load font
    let font = asset_server.load("fonts/DejaVuSans-Bold.ttf");
    let layout = CardTextLayout::default();

    for (entity, transform, card) in query.iter() {
        // Set font size for card name
        let font_size = 20.0;
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create formatted card name - truncate if too long
        let name_text = format_card_name(&card.name, font_size, layout.name_width * card_size.x);

        // Calculate position relative to card
        let name_position = Vec2::new(
            layout.name_x_offset * card_size.x,
            layout.name_y_offset * card_size.y,
        );

        // Create the text entity
        let text_entity = commands
            .spawn(CardTextBundle::new(
                name_text,
                font.clone(),
                font_size,
                Color::BLACK,
                transform.translation,
                name_position,
                JustifyText::Left,
            ))
            .id();

        // Set as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}

/// Format card name to fit within bounds and handle long names
fn format_card_name(name: &str, font_size: f32, max_width: f32) -> String {
    // Estimate average character width (this will vary with the actual font)
    let avg_char_width = font_size * 0.6; // Adjusted for better estimation

    // Calculate max chars that fit in the available width
    let max_chars = (max_width / avg_char_width).floor() as usize;

    // Use a reasonable max - reduced to ensure long names fit properly
    let max_chars = max_chars.min(20); // Reduced from 22 for better boundary control

    if name.len() <= max_chars {
        name.to_string()
    } else {
        // Truncate and add ellipsis
        format!("{}…", &name[0..max_chars.saturating_sub(1)])
    }
}
