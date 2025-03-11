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
    /// X offset of power/toughness from right edge of card (normalized -0.5 to 0.5)
    pub pt_x_offset: f32,
    /// Y offset of power/toughness from bottom edge of card (normalized -0.5 to 0.5)
    pub pt_y_offset: f32,
    /// Width constraint for power/toughness as percentage of card width
    pub pt_width: f32,
    /// Height constraint for power/toughness as percentage of card height
    pub pt_height: f32,
    /// X offset of type line from left edge of card (normalized -0.5 to 0.5)
    pub type_line_x_offset: f32,
    /// Y offset of type line from top edge of card (normalized -0.5 to 0.5)
    pub type_line_y_offset: f32,
    /// Width constraint for type line as percentage of card width
    pub type_line_width: f32,
    /// Height constraint for type line as percentage of card height
    pub type_line_height: f32,
    /// Y offset of text box from top edge of card (normalized -0.5 to 0.5)
    pub text_box_y_offset: f32,
    /// Width constraint for text box as percentage of card width
    pub text_box_width: f32,
    /// Height constraint for text box as percentage of card height
    pub text_box_height: f32,
    /// Padding inside the text box (normalized)
    pub text_box_padding: f32,
}

impl Default for CardTextLayout {
    fn default() -> Self {
        Self {
            card_width: 63.0 * 1.4,
            card_height: 88.0 * 1.4,
            // Position name with a small margin from the left edge of the card frame
            name_x_offset: -0.32, // Moved more toward center to provide proper left margin
            name_y_offset: 0.41,
            // Widened to accommodate most card names
            name_width: 0.65,
            // Keep mana cost in the top right, but position it more visibly
            mana_cost_x_offset: 0.33, // Adjusted position to be more visible
            mana_cost_y_offset: 0.41,
            vertical_margin: 0.05,
            horizontal_margin: 0.1,
            // Power/toughness positioning
            pt_x_offset: 0.35,
            pt_y_offset: -0.35,
            pt_width: 0.15,
            pt_height: 0.08,
            // Type line positioning
            type_line_x_offset: -0.3,
            type_line_y_offset: 0.25,
            type_line_width: 0.8,
            type_line_height: 0.08,
            // Text box positioning
            text_box_y_offset: 0.0,
            text_box_width: 0.8,
            text_box_height: 0.35,
            text_box_padding: 0.025,
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
