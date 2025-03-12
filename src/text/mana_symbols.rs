use crate::mana::mana_symbol_to_char;
use crate::text::utils::get_mana_symbol_color;
use bevy::prelude::*;
use bevy::text::JustifyText;

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

/// Renders a mana symbol with appropriate styling and shadow
pub fn render_mana_symbol(
    commands: &mut Commands,
    symbol: &str,
    position: Vec2,
    mana_font: Handle<Font>,
    options: ManaSymbolOptions,
    parent_entity: Entity,
) {
    let symbol_color = get_mana_symbol_color(symbol);
    let pos_3d = Vec3::new(position.x, position.y, options.z_index);

    // Convert the symbol to the appropriate character for the Mana font
    let display_symbol = mana_symbol_to_char(symbol);

    // Calculate a symbol-specific vertical alignment adjustment
    let symbol_specific_offset = match symbol.trim() {
        "{B}" => options.font_size * 0.05, // Slight adjustment for black mana
        "{W}" => options.font_size * 0.03, // Slight adjustment for white mana
        "{R}" => options.font_size * 0.04, // Slight adjustment for red mana
        "{U}" => options.font_size * 0.03, // Adjustment for blue mana
        "{T}" => options.font_size * 0.15, // Increased adjustment for tap symbol
        "{C}" => options.font_size * 0.04, // Adjustment for colorless mana
        s if s.len() >= 3 && s.starts_with('{') && s.ends_with('}') => {
            // Check if this is a generic/numeric mana symbol
            let inner = &s[1..s.len() - 1];
            if inner.parse::<u32>().is_ok() || inner == "X" {
                options.font_size * 0.05 // Vertical adjustment for generic mana
            } else {
                0.0
            }
        }
        _ => 0.0,
    };

    // Apply vertical alignment offset if specified
    let aligned_pos = Vec3::new(
        pos_3d.x,
        pos_3d.y + options.vertical_alignment_offset + symbol_specific_offset,
        pos_3d.z,
    );

    // If colored background option is enabled, add a circle
    if options.with_colored_background {
        // Make sure we're working with a clean symbol
        let clean_symbol = symbol.trim();

        // Determine background color based on symbol
        let background_color = match clean_symbol {
            "{W}" => Color::srgb(0.95, 0.95, 0.85), // White
            "{U}" => Color::srgb(0.0, 0.2, 0.63),   // Blue - adjusted to match MTG blue
            "{B}" => Color::srgb(0.15, 0.15, 0.15), // Black (not fully black for visibility)
            "{R}" => Color::srgb(0.8, 0.15, 0.15),  // Red
            "{G}" => Color::srgb(0.15, 0.7, 0.15),  // Green
            "{C}" => Color::srgb(0.8, 0.8, 0.9),    // Colorless
            _ => {
                // For generic mana and other symbols
                if clean_symbol.starts_with("{") && clean_symbol.ends_with("}") {
                    let inner = &clean_symbol[1..clean_symbol.len() - 1];
                    if inner.parse::<u32>().is_ok() || inner == "X" {
                        // Generic/X mana is light gray
                        Color::srgb(0.75, 0.73, 0.71)
                    } else if inner == "T" {
                        // Tap symbol, use darker gray
                        Color::srgb(0.4, 0.4, 0.4)
                    } else {
                        // Other symbols, use light gray
                        Color::srgb(0.7, 0.7, 0.7)
                    }
                } else {
                    Color::srgb(0.7, 0.7, 0.7) // Light gray default
                }
            }
        };

        // Size of the circle should be proportional to the font size
        let circle_size = Vec2::splat(options.font_size * 1.0);

        // Spawn the circle with the background color, ensuring it's perfectly round
        commands
            .spawn((
                Sprite {
                    color: background_color,
                    custom_size: Some(circle_size),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    aligned_pos.x,
                    aligned_pos.y,
                    aligned_pos.z - 0.05, // Slightly behind the text
                )),
                // Name to identify this as a mana circle for our circle system
                Name::new(format!("Mana Circle: {}", clean_symbol)),
                GlobalTransform::default(),
            ))
            .set_parent(parent_entity);

        // Determine text color based on background for better contrast
        let text_color = if is_dark_background(clean_symbol, &background_color) {
            // White text for dark backgrounds
            Color::srgb(1.0, 1.0, 1.0)
        } else {
            // Black text for light backgrounds
            Color::srgb(0.0, 0.0, 0.0)
        };

        // Render the symbol with the appropriate color
        commands
            .spawn((
                TextSpan::default(),
                Text2d::new(display_symbol),
                TextFont {
                    font: mana_font,
                    font_size: options.font_size,
                    ..default()
                },
                TextColor(text_color),
                Transform::from_translation(aligned_pos),
            ))
            .set_parent(parent_entity);

        return;
    }

    // Regular rendering without background
    // Render drop shadow if enabled
    if options.with_shadow {
        let shadow_offset = Vec3::new(1.5, -1.5, 0.0);
        let shadow_color = Color::srgba(0.0, 0.0, 0.0, 0.7);

        commands
            .spawn((
                TextSpan::default(),
                Text2d::new(display_symbol.clone()),
                TextFont {
                    font: mana_font.clone(),
                    font_size: options.font_size,
                    ..default()
                },
                TextColor(shadow_color),
                Transform::from_translation(
                    aligned_pos + shadow_offset - Vec3::new(0.0, 0.0, 0.05),
                ),
            ))
            .set_parent(parent_entity);
    }

    // Render the actual mana symbol
    commands
        .spawn((
            TextSpan::default(),
            Text2d::new(display_symbol),
            TextFont {
                font: mana_font.clone(),
                font_size: options.font_size,
                ..default()
            },
            TextColor(symbol_color),
            Transform::from_translation(aligned_pos),
        ))
        .set_parent(parent_entity);
}

/// Calculates the appropriate width of a mana symbol for layout purposes
pub fn get_mana_symbol_width(font_size: f32) -> f32 {
    font_size * 0.7 // Slightly narrower than square for better text integration
}

/// Checks if a string is a valid mana symbol
pub fn is_valid_mana_symbol(symbol: &str) -> bool {
    if symbol.len() < 3 || !symbol.starts_with('{') || !symbol.ends_with('}') {
        return false;
    }

    // Use our constant mapping to validate symbols
    use crate::mana::MANA_SYMBOLS;
    for (key, _) in MANA_SYMBOLS {
        if *key == symbol {
            return true;
        }
    }

    // Generic check for numbers that may not be in our map
    let inner = &symbol[1..symbol.len() - 1];
    if inner.parse::<u32>().is_ok() {
        return true;
    }

    false
}

/// Helper function to determine if a background color is dark and needs white text
fn is_dark_background(symbol: &str, _color: &Color) -> bool {
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
        "{W}" | "{R}" | "{C}" => false,
        // For generic mana symbols, check if they're the type that needs white text
        s if s.len() >= 3 && s.starts_with('{') && s.ends_with('}') => {
            let inner = &s[1..s.len() - 1];
            // Set any other dark backgrounds that need white text here
            match inner {
                // Add specific cases here
                _ => false, // Default to black text
            }
        }
        // Default to black text for any other case
        _ => false,
    }
}
