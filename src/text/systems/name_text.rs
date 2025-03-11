use crate::card::Card;
use crate::text::{components::CardTextBundle, utils::CardTextLayout};
use bevy::prelude::*;

/// Creates text entities for card names
pub fn create_name_text(
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
    let avg_char_width = font_size * 0.55; // Conservative estimate

    // Calculate max chars that fit in the available width
    let max_chars = (max_width / avg_char_width).floor() as usize;

    // Use a reasonable max (MTG cards rarely exceed 30 chars, but be safe)
    let max_chars = max_chars.min(18);

    if name.len() <= max_chars {
        name.to_string()
    } else {
        // Truncate and add ellipsis
        format!("{}…", &name[0..max_chars.saturating_sub(1)])
    }
}
