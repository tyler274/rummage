use crate::mana::mana_symbol_to_char;
use crate::text::utils::get_mana_symbol_color;
use bevy::prelude::*;

/// Represents rendering options for mana symbols
#[derive(Clone)]
pub struct ManaSymbolOptions {
    /// Font size for the mana symbol
    pub font_size: f32,
    /// Vertical alignment offset to align with text baseline
    pub vertical_alignment_offset: f32,
    /// Z-index for the mana symbol rendering
    pub z_index: f32,
    /// Whether to render with drop shadow
    pub with_shadow: bool,
}

impl Default for ManaSymbolOptions {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            vertical_alignment_offset: 0.0,
            z_index: 0.1,
            with_shadow: true,
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

    // Apply vertical alignment offset if specified
    let aligned_pos = if options.vertical_alignment_offset != 0.0 {
        Vec3::new(
            pos_3d.x,
            pos_3d.y + options.vertical_alignment_offset,
            pos_3d.z,
        )
    } else {
        pos_3d
    };

    // Render drop shadow if enabled
    if options.with_shadow {
        let shadow_offset = Vec3::new(1.5, -1.5, 0.0);
        let shadow_color = Color::srgba(0.0, 0.0, 0.0, 0.8);

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
