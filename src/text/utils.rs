use crate::text::components::CardTextType;
use bevy::prelude::*;

/// Standard Magic card text layout constants
#[derive(Debug, Clone)]
pub struct CardTextLayout {
    /// The width of the card
    pub card_width: f32,
    /// The height of the card
    pub card_height: f32,
    /// X offset of name text from left edge of card (normalized -0.5 to 0.5)
    pub name_x_offset: f32,
    /// Y offset of name text from top edge of card (normalized -0.5 to 0.5)
    pub name_y_offset: f32,
    /// Width constraint for name as percentage of card width
    pub name_width: f32,
    /// X offset of mana cost from right edge of card (normalized -0.5 to 0.5)
    pub mana_cost_x_offset: f32,
    /// Y offset of mana cost from top edge of card (normalized -0.5 to 0.5)
    pub mana_cost_y_offset: f32,
    /// The margin between text and the edge of the card for rules text (normalized)
    pub vertical_margin: f32,
    /// The horizontal margin for text relative to the card edge (normalized)
    pub horizontal_margin: f32,
}

impl Default for CardTextLayout {
    fn default() -> Self {
        Self {
            card_width: 63.0 * 1.4,
            card_height: 88.0 * 1.4,
            // Adjusted name positioning to be more center-left
            name_x_offset: -0.3,
            name_y_offset: -0.41,
            // Reduced name width to make room for mana cost
            name_width: 0.5,
            // Adjusted mana cost to be more right-aligned
            mana_cost_x_offset: 0.35,
            mana_cost_y_offset: -0.41,
            vertical_margin: 0.05,
            horizontal_margin: 0.1,
        }
    }
}

/// Spawn debug bounds visualization for text
pub fn spawn_debug_bounds(
    commands: &mut Commands,
    _card_pos: Vec2,
    card_size: Vec2,
    text_pos: Vec2,
) -> Entity {
    // Calculate the bounds size based on card size
    let bounds_size = Vec2::new(card_size.x * 0.7, card_size.y * 0.2);

    // Spawn a rectangle to visualize the text bounds
    let entity = commands
        .spawn((
            Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 0.3),
                custom_size: Some(bounds_size),
                ..default()
            },
            Transform::from_translation(text_pos.extend(6.0)),
            GlobalTransform::default(),
            CardTextType::Debug,
        ))
        .id();

    // Spawn a dot to mark the text anchor point
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        Transform::from_translation(text_pos.extend(7.0)),
        GlobalTransform::default(),
        CardTextType::Debug,
    ));

    entity
}

/// Calculate text position relative to a card
///
/// This function calculates the position of text elements relative to a card,
/// ensuring that the text stays properly aligned with the card regardless of
/// the card's position on screen.
///
/// Instead of using absolute world coordinates, we use the card's transform
/// as the parent and calculate relative offsets. This ensures that text
/// stays properly aligned with cards even when they're away from the center
/// of the screen.
pub fn calculate_text_position(
    card_pos: Vec2,
    card_size: Vec2,
    horizontal_offset: f32,
    vertical_offset: f32,
) -> Vec2 {
    // Calculate the offset in local card space
    let offset = Vec2::new(
        card_size.x * horizontal_offset,
        card_size.y * vertical_offset,
    );

    // Apply the offset to the card position
    card_pos + offset
}

/// Calculate text size relative to a card
pub fn calculate_text_size(card_size: Vec2, width_percentage: f32, height_percentage: f32) -> Vec2 {
    Vec2::new(
        card_size.x * width_percentage,
        card_size.y * height_percentage,
    )
}

/// Get the appropriate font size for a card based on its size
pub fn get_card_font_size(card_size: Vec2, base_size: f32) -> f32 {
    // Scale font size based on card width
    let scale_factor = card_size.x / 300.0; // 300 is a reference width
    base_size * scale_factor.clamp(0.5, 2.0) // Clamp to avoid extreme sizes
}

/// Get standard card layout measurements
pub fn get_card_layout() -> CardTextLayout {
    CardTextLayout::default()
}
