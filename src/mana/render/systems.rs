use bevy::prelude::*;

use crate::cards::Card;
use crate::mana::render::colors::{get_mana_symbol_color, is_dark_background};
use crate::mana::render::components::CardManaCostText;
use crate::mana::render::styles::ManaSymbolOptions;
use crate::mana::symbols::mana_symbol_to_char;
use crate::text::components::CardTextType;
use crate::text::layout::{get_adaptive_font_size, get_card_layout};

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
                Text2d::new(display_symbol),
                TextFont {
                    font: mana_font,
                    font_size: options.font_size,
                    ..default()
                },
                TextColor(text_color),
                Transform::from_translation(aligned_pos),
                GlobalTransform::default(),
                Name::new(format!("Mana Symbol: {}", clean_symbol)),
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
                GlobalTransform::default(),
                Name::new(format!("Mana Symbol Shadow: {}", symbol)),
            ))
            .set_parent(parent_entity);
    }

    // Render the actual mana symbol
    commands
        .spawn((
            Text2d::new(display_symbol),
            TextFont {
                font: mana_font.clone(),
                font_size: options.font_size,
                ..default()
            },
            TextColor(symbol_color),
            Transform::from_translation(aligned_pos),
            GlobalTransform::default(),
            Name::new(format!("Mana Symbol: {}", symbol)),
        ))
        .set_parent(parent_entity);
}

/// Spawn mana cost text for a card
pub fn spawn_mana_cost_text(
    commands: &mut Commands,
    mana_cost_component: &CardManaCostText,
    _card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    let layout = get_card_layout();

    // Calculate position - on the right side of the card top
    let mana_position = Vec2::new(
        layout.mana_cost_x_offset * card_size.x,
        layout.mana_cost_y_offset * card_size.y,
    );

    // Calculate available width for the mana cost
    let available_width = card_size.x * 0.15; // Use 15% of card width for mana cost

    // Use adaptive font sizing based on mana cost complexity
    let font_size = get_adaptive_font_size(
        card_size,
        14.0, // Base size for mana cost
        &mana_cost_component.mana_cost,
        available_width,
        10.0, // Minimum size
    );

    // Create parent entity for all mana symbols
    let parent_entity = commands
        .spawn((
            // Use separate components instead of SpatialBundle
            Transform::from_translation(Vec3::new(mana_position.x, mana_position.y, 0.1)),
            // GlobalTransform is automatically added
            Visibility::default(),
            CardTextType::ManaCost,
            TextLayout::new_with_justify(JustifyText::Right),
            Name::new(format!("Mana Cost: {}", mana_cost_component.mana_cost)),
        ))
        .id();

    // Load the specialized mana font
    let mana_font = asset_server.load("fonts/Mana.ttf");

    // Parse the mana cost string and create individual symbols
    let mana_string = &mana_cost_component.mana_cost;

    // Look for patterns like {W}, {U}, {B}, {R}, {G}, {1}, etc.
    let symbol_spacing = font_size * 0.8;

    // Count symbols to center properly (right-aligned)
    let mut symbols = Vec::new();
    let mut current_symbol = String::new();
    let mut in_symbol = false;

    for c in mana_string.chars() {
        match c {
            '{' => {
                in_symbol = true;
                current_symbol.push('{');
            }
            '}' => {
                if in_symbol {
                    current_symbol.push('}');
                    symbols.push(current_symbol.clone());
                    current_symbol.clear();
                    in_symbol = false;
                }
            }
            _ => {
                if in_symbol {
                    current_symbol.push(c);
                }
            }
        }
    }

    // Calculate total width for right alignment
    let total_width = symbols.len() as f32 * symbol_spacing;

    // Render each symbol
    for (i, symbol) in symbols.iter().enumerate() {
        // Position for this symbol (right-aligned)
        let pos_x = -total_width / 2.0 + i as f32 * symbol_spacing;

        // Render the mana symbol with the appropriate styling
        render_mana_symbol(
            commands,
            symbol,
            Vec2::new(pos_x, 0.0), // Local position relative to parent
            mana_font.clone(),
            ManaSymbolOptions {
                font_size,
                with_colored_background: true,
                ..default()
            },
            parent_entity,
        );
    }

    parent_entity
}

/// System implementation that finds cards and creates mana cost text for them
pub fn mana_cost_text_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, card) in query.iter() {
        // Skip cards with no mana cost
        if card.cost.cost.is_empty() {
            continue;
        }

        // Convert Mana struct to display string
        let mana_cost_string = card.cost.cost.to_string();

        // Get card position and size
        let card_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let layout = get_card_layout();
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create content for mana cost text
        let content = CardManaCostText {
            mana_cost: mana_cost_string,
        };

        // Create the mana cost text
        let text_entity =
            spawn_mana_cost_text(&mut commands, &content, card_pos, card_size, &asset_server);

        // Add as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}

/// Directly replace mana symbols in text with their Unicode equivalents
/// This is a simpler alternative to the more complex inline mana symbol rendering
/// that can be used for plain text displays or debugging purposes.
#[allow(dead_code)]
pub fn replace_mana_symbols_with_unicode(text: &str) -> String {
    use crate::mana::MANA_SYMBOLS;

    let mut result = text.to_string();

    // Replace all mana symbols with their Unicode equivalents
    for (symbol, unicode) in MANA_SYMBOLS {
        result = result.replace(symbol, &unicode.to_string());
    }

    result
}
