use crate::text::components::CardTextType;
use bevy::prelude::*;

/// Standard Magic card text layout constants
pub struct CardTextLayout {
    // Title bar region (for name and mana cost)
    pub title_y_offset: f32,
    pub title_height: f32,
    pub name_x_offset: f32,
    pub name_width: f32,
    pub mana_cost_x_offset: f32,
    pub mana_cost_width: f32,

    // Type line region
    pub type_line_y_offset: f32,
    pub type_line_height: f32,
    pub type_line_width: f32,

    // Text box region (for rules text)
    pub text_box_y_offset: f32,
    pub text_box_height: f32,
    pub text_box_width: f32,
    pub text_box_padding: f32,

    // Power/toughness box region
    pub pt_x_offset: f32,
    pub pt_y_offset: f32,
    pub pt_width: f32,
    pub pt_height: f32,

    // Margins
    pub horizontal_margin: f32,
}

impl Default for CardTextLayout {
    fn default() -> Self {
        Self {
            // Title bar (top ~10% of card)
            title_y_offset: 0.43,     // Slightly adjusted for better positioning
            title_height: 0.09,       // Slightly reduced height
            name_x_offset: -0.35,     // Move name more to the left to match MTG cards
            name_width: 0.7,          // Wider to accommodate full card names
            mana_cost_x_offset: 0.38, // More right-aligned
            mana_cost_width: 0.25,

            // Type line (middle divider, ~8% of card)
            type_line_y_offset: 0.16, // Adjusted position
            type_line_height: 0.08,
            type_line_width: 0.9, // Wider to match MTG cards

            // Text box (middle ~55% of card)
            // Adjusted to better match standard Magic card layout
            text_box_y_offset: -0.08, // Move up slightly to center better
            text_box_height: 0.48,    // Increased for more text space like MTG cards
            text_box_width: 0.85,     // Wider to match MTG text boxes
            text_box_padding: 0.04,   // Better edge margins

            // Power/toughness box (bottom right corner)
            pt_x_offset: 0.37, // Adjusted to match MTG cards
            pt_y_offset: -0.43,
            pt_width: 0.15, // More compact like MTG cards
            pt_height: 0.09,

            // Margins
            horizontal_margin: 0.05,
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
