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
    #[allow(dead_code)]
    pub vertical_margin: f32,
    /// The horizontal margin for text relative to the card edge (normalized)
    #[allow(dead_code)]
    pub horizontal_margin: f32,
    /// X offset of power/toughness from right edge of card (normalized -0.5 to 0.5)
    pub pt_x_offset: f32,
    /// Y offset of power/toughness from bottom edge of card (normalized -0.5 to 0.5)
    pub pt_y_offset: f32,
    /// Width constraint for power/toughness as percentage of card width
    pub pt_width: f32,
    /// Height constraint for power/toughness as percentage of card height
    #[allow(dead_code)]
    pub pt_height: f32,
    /// X offset of type line from left edge of card (normalized -0.5 to 0.5)
    pub type_line_x_offset: f32,
    /// Y offset of type line from top edge of card (normalized -0.5 to 0.5)
    pub type_line_y_offset: f32,
    /// Width constraint for type line as percentage of card width
    pub type_line_width: f32,
    /// Height constraint for type line as percentage of card height
    #[allow(dead_code)]
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
            // Standard Magic card is 2.5" × 3.5" (63mm × 88mm)
            // At 300 DPI, that would be 750 × 1050 pixels
            card_width: 750.0,   // 2.5 inches * 300 DPI
            card_height: 1050.0, // 3.5 inches * 300 DPI

            // Position name with a proper margin from the left edge of the card frame
            name_x_offset: -0.22,
            name_y_offset: 0.41,

            // Adjusted width to prevent long names from extending beyond card boundaries
            name_width: 0.42,

            // Adjusted mana cost positioning to avoid going out of bounds
            mana_cost_x_offset: 0.32,
            mana_cost_y_offset: 0.41,

            // Margins for text layout
            vertical_margin: 0.05,
            horizontal_margin: 0.1,

            // Power/toughness positioning
            pt_x_offset: 0.35,
            pt_y_offset: -0.38,
            pt_width: 0.15,
            pt_height: 0.08,

            // Type line positioning - adjusted to create clearer separation from art and rules text
            type_line_x_offset: -0.32,
            type_line_y_offset: -0.05,
            type_line_width: 0.76, // Slightly reduced to ensure text fits
            type_line_height: 0.07,

            // Text box positioning - refined to create better visual balance
            text_box_y_offset: -0.22,
            text_box_width: 0.65,
            text_box_height: 0.30,
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
/// Base sizes by text type:
/// - Card Name: ~16pt
/// - Mana Cost: ~14pt
/// - Type Line: ~14pt
/// - Rules Text: ~12pt
/// - P/T: ~14pt
pub fn get_card_font_size(card_size: Vec2, base_size: f32) -> f32 {
    // Calculate font size based on 300 DPI standard
    // Standard Magic card width is 2.5" at 300 DPI = 750 pixels
    // Font sizes are typically measured in points where 1 point = 1/72 inch
    // At 300 DPI, 1 point = 300/72 = 4.167 pixels
    let dpi_factor = 300.0 / 72.0; // Convert points to pixels at 300 DPI
    let reference_width = 750.0; // Full-sized card width at 300 DPI
    let scale_factor = card_size.x / reference_width;

    // Apply scaling with improved limits for better readability at various zoom levels
    // Use a more conservative scaling to prevent text overflow
    base_size * dpi_factor * scale_factor.clamp(0.5, 4.0)
}

/// Get adaptive font size based on text content and container size
pub fn get_adaptive_font_size(
    card_size: Vec2,
    base_size: f32,
    text_content: &str,
    max_width: f32,
    min_font_size: f32,
) -> f32 {
    // Start with the standard card font size
    let initial_size = get_card_font_size(card_size, base_size);

    // Calculate approximate size needed based on text length
    // This is a heuristic that should be adjusted based on testing
    let text_length = text_content.len() as f32;

    // More aggressive scaling for long text
    // Decrease average character width estimate for longer text for better sizing
    let char_width_factor = if text_length > 20.0 {
        0.40 // More aggressive for very long names
    } else if text_length > 12.0 {
        0.42 // Medium-length names need more scaling too
    } else {
        0.50 // Short names are fine with default
    };

    let max_chars_per_line = max_width / (initial_size * char_width_factor);

    // Approximate number of lines needed at initial font size
    let target_line_count = (text_length / max_chars_per_line).ceil();

    // Scale factor based on text length (longer text = smaller font)
    // More aggressive scaling based on name length
    let length_scale_factor = if target_line_count > 1.5 {
        0.8 / target_line_count.sqrt() // More aggressive reduction
    } else if target_line_count > 1.0 {
        0.9 / target_line_count.sqrt() // Medium reduction
    } else {
        1.0 // No reduction for short names
    };

    // Additional scaling for very long names (e.g., "Champion of the Perished")
    let long_name_factor = if text_length > 20.0 {
        0.85 // Further reduce font size for very long names
    } else if text_length > 14.0 {
        0.90 // Medium reduction for medium-length names
    } else {
        1.0 // No reduction for short names
    };

    // Apply the length-based scaling, but never go below minimum size
    (initial_size * length_scale_factor * long_name_factor).max(min_font_size)
}

/// Get standard card layout measurements
pub fn get_card_layout() -> CardTextLayout {
    CardTextLayout::default()
}

/// Create a specific card layout with custom parameters
#[allow(dead_code)]
pub fn custom_card_layout(width: f32, height: f32) -> CardTextLayout {
    let mut layout = CardTextLayout::default();
    layout.card_width = width;
    layout.card_height = height;
    layout
}

/// Standard battlefield card size multiplier
pub fn get_battlefield_card_size_multiplier() -> f32 {
    4.0 // Adjusted for better text sizing and readability
}
