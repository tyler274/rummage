use bevy::prelude::*;

/// Represents rendering options for mana symbols
#[derive(Clone, Debug)]
pub struct ManaSymbolOptions {
    /// Font size for the mana symbol
    pub font_size: f32,
    /// Vertical alignment offset to align with text baseline
    pub vertical_alignment_offset: f32,
    /// Z-index for the mana symbol rendering
    pub z_index: f32,
    /// Whether to render with drop shadow
    pub with_shadow: bool,
    /// Whether to render with colored circle background (MTG style)
    pub with_colored_background: bool,
}

impl Default for ManaSymbolOptions {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            vertical_alignment_offset: 0.0,
            z_index: 0.1,
            with_shadow: true,
            with_colored_background: false,
        }
    }
}

/// Calculates the appropriate width of a mana symbol for layout purposes
pub fn get_mana_symbol_width(font_size: f32) -> f32 {
    font_size * 0.7 // Slightly narrower than square for better text integration
}
