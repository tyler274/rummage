use bevy::prelude::*;

/// Standard Magic card layout measurements for text positioning
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
            // Increased card size for better rendering quality and DPI
            card_width: 63.0 * 3.0,  // Increased from 2.0
            card_height: 88.0 * 3.0, // Increased from 2.0

            // Position name with a proper margin from the left edge of the card frame
            name_x_offset: -0.18, // Adjusted to prevent overflow
            name_y_offset: 0.41,

            // Adjusted width to prevent long names from extending beyond card boundaries
            name_width: 0.55, // Reduced to avoid overlap with mana cost

            // Adjusted mana cost positioning to avoid going out of bounds
            mana_cost_x_offset: 0.30, // Moved slightly toward center from 0.33
            mana_cost_y_offset: 0.41,

            // Margins for text layout
            vertical_margin: 0.05,
            horizontal_margin: 0.1,

            // Power/toughness positioning
            pt_x_offset: 0.35,
            pt_y_offset: -0.35,
            pt_width: 0.15,
            pt_height: 0.08,

            // Type line positioning - moved down to leave space for art
            type_line_x_offset: -0.3,
            type_line_y_offset: 0.05,
            type_line_width: 0.8,
            type_line_height: 0.08,

            // Text box positioning - moved up to ensure it stays within card bounds
            text_box_y_offset: -0.12,
            text_box_width: 0.65, // Reduced from 0.7 to keep text within horizontal bounds
            text_box_height: 0.25,
            text_box_padding: 0.025,
        }
    }
}

/// Calculate text size based on card size and percentage constraints
pub fn calculate_text_size(card_size: Vec2, width_percentage: f32, height_percentage: f32) -> Vec2 {
    Vec2::new(
        card_size.x * width_percentage,
        card_size.y * height_percentage,
    )
}

/// Get the appropriate font size for a card based on its size
pub fn get_card_font_size(card_size: Vec2, base_size: f32) -> f32 {
    // Scale font size based on card width with an improved scaling factor
    let scale_factor = card_size.x / 250.0;
    base_size * scale_factor.clamp(0.7, 3.0) // Increased max scale for better text visibility
}

/// Get standard card layout measurements
pub fn get_card_layout() -> CardTextLayout {
    CardTextLayout::default()
}

/// Create a specific card layout with custom parameters
pub fn custom_card_layout(width: f32, height: f32) -> CardTextLayout {
    let mut layout = CardTextLayout::default();
    layout.card_width = width;
    layout.card_height = height;
    layout
}

/// Standard battlefield card size multiplier
pub fn get_battlefield_card_size_multiplier() -> f32 {
    2.0 // Increased from 1.5 for better visibility
}
