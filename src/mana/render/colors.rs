use bevy::prelude::*;

/// Returns the appropriate color for a mana symbol
pub fn get_mana_symbol_color(symbol: &str) -> Color {
    // Make sure we're working with a clean symbol
    let clean_symbol = symbol.trim();

    let color = match clean_symbol {
        "{W}" => Color::srgb(0.95, 0.95, 0.85), // White mana (off-white)
        "{U}" => Color::srgb(0.0, 0.4, 0.8),    // Blue mana - more vibrant
        "{B}" => Color::srgb(0.0, 0.0, 0.0),    // Black mana - true black
        "{R}" => Color::srgb(0.9, 0.1, 0.1),    // Red mana - more vivid red
        "{G}" => Color::srgb(0.0, 0.6, 0.0),    // Green mana - brighter green
        "{C}" => Color::srgb(0.7, 0.7, 0.8),    // Colorless mana - slight blue tint
        _ => {
            // Generic/numeric mana or other symbols
            if clean_symbol.starts_with("{") && clean_symbol.ends_with("}") {
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
    };

    color
}

/// Helper function to determine if a background color is dark and needs white text
pub fn is_dark_background(symbol: &str, _color: &Color) -> bool {
    // Standard dark mana backgrounds that should have white text
    if symbol == "{B}" || symbol == "{G}" || symbol == "{U}" {
        return true;
    }

    // Check for symbols with Phyrexian mana (contains "P")
    if symbol.contains("P")
        && (symbol.contains("B") || symbol.contains("G") || symbol.contains("U"))
    {
        return true;
    }

    // Check for tap symbol and other special symbols that use dark backgrounds
    if symbol == "{T}" {
        return true;
    }

    // For all other symbols, check based on the symbol itself since we know our color mapping
    match symbol.trim() {
        // Dark background symbols that need white text
        "{B}" | "{U}" | "{G}" | "{T}" => true,
        // Light background symbols that need black text
        "{W}" | "{R}" | "{C}" | "{X}" => false,
        // Default for generic mana costs and others (light gray backgrounds)
        _ => {
            if symbol.starts_with("{") && symbol.ends_with("}") {
                let inner = &symbol[1..symbol.len() - 1];
                if inner.parse::<u32>().is_ok() {
                    false // Generic mana costs have light backgrounds
                } else {
                    true // Special symbols generally have dark backgrounds
                }
            } else {
                false // Default to black text on light background
            }
        }
    }
}
