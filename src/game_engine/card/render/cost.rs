use bevy::prelude::*;

/// Converts a mana symbol string (e.g., "{W}") to its corresponding color.
pub fn symbol_to_color(mana_symbol: &str) -> Color {
    let clean_symbol = mana_symbol.trim();
    match clean_symbol {
        "{W}" => Color::srgb(0.95, 0.95, 0.85), // White mana (off-white)
        "{U}" => Color::srgb(0.0, 0.4, 0.8),    // Blue mana - more vibrant
        "{B}" => Color::srgb(0.0, 0.0, 0.0),    // Black mana - true black
        "{R}" => Color::srgb(0.9, 0.1, 0.1),    // Red mana - more vivid red
        "{G}" => Color::srgb(0.0, 0.6, 0.0),    // Green mana - brighter green
        "{C}" => Color::srgb(0.7, 0.7, 0.8),    // Colorless mana - slight blue tint
        _ => {
            // Generic/numeric mana or other symbols
            if clean_symbol.starts_with('{') && clean_symbol.ends_with('}') {
                let inner = &clean_symbol[1..clean_symbol.len() - 1];
                if inner.parse::<u32>().is_ok() || inner == "X" {
                    // Generic mana is light gray with a slight brown tint
                    Color::srgb(0.75, 0.73, 0.71)
                } else {
                    // Other symbols like tap
                    Color::srgb(0.3, 0.3, 0.3)
                }
            } else {
                // Default to black for other text
                Color::srgb(0.0, 0.0, 0.0)
            }
        }
    }
}
