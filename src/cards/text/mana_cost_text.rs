use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::cards::Card;
use crate::text::{
    components::{CardManaCostText, CardTextType},
    mana_symbols::{ManaSymbolOptions, render_mana_symbol},
    utils::{get_adaptive_font_size, get_card_layout},
};

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
    let symbol_spacing = font_size * 0.8; // Spacing between symbols

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
#[allow(dead_code)]
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
