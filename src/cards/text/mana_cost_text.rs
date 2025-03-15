use bevy::prelude::*;
use bevy::text::JustifyText;

use crate::cards::Card;
use crate::text::{
    components::{CardManaCostText, CardTextType, TextLayoutInfo},
    mana_symbols::{ManaSymbolOptions, render_mana_symbol},
    utils::{get_card_font_size, get_card_layout},
};

/// Creates a text entity for mana cost with colored mana symbols
pub fn create_mana_cost_text(
    commands: &mut Commands,
    mana_cost_component: &CardManaCostText,
    _card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Skip cards with no mana cost
    if mana_cost_component.mana_cost.is_empty() {
        return commands.spawn_empty().id();
    }

    // Load mana symbol font
    let mana_font = asset_server.load("fonts/Mana.ttf");
    let layout = get_card_layout();

    // Position the mana cost correctly
    let mana_position = Vec2::new(
        layout.mana_cost_x_offset * card_size.x,
        layout.mana_cost_y_offset * card_size.y,
    );

    // Create the parent container entity
    let root_entity = commands
        .spawn((
            // Empty root text element
            Text2d::new(""),
            Transform::from_translation(Vec3::new(
                mana_position.x,
                mana_position.y,
                1.0, // High z-index
            )),
            GlobalTransform::default(),
            TextLayout::new_with_justify(JustifyText::Right), // Right-justified
            CardTextType::ManaCost,
            TextLayoutInfo {
                alignment: JustifyText::Right,
            },
            Name::new(format!("Mana Cost: {}", mana_cost_component.mana_cost)),
        ))
        .id();

    // Create a customized options set for the mana symbols
    let mana_options = ManaSymbolOptions {
        font_size: get_card_font_size(card_size, 20.0), // 20pt at 300 DPI for clear symbols
        vertical_alignment_offset: 0.0,
        z_index: 0.1,
        with_shadow: true,
        with_colored_background: true,
    };

    // Set up the mana symbols
    let symbols = mana_cost_component
        .mana_cost
        .split(' ')
        .collect::<Vec<&str>>();
    let symbol_count = symbols.len();

    // Calculate layout for the symbols
    let symbol_width = 36.0 * (card_size.x / 750.0); // Scale symbol width based on 300 DPI card width 
    let total_width = symbol_width * symbol_count as f32;

    // Position the symbols equally spaced
    for (i, symbol) in symbols.iter().enumerate() {
        let horizontal_offset =
            -(total_width / 2.0) + (i as f32 * symbol_width) + (symbol_width / 2.0);

        // Use the unified mana symbol renderer with colored background
        render_mana_symbol(
            commands,
            symbol,
            Vec2::new(horizontal_offset, 0.0),
            mana_font.clone(),
            mana_options.clone(),
            root_entity,
        );
    }

    root_entity
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
            create_mana_cost_text(&mut commands, &content, card_pos, card_size, &asset_server);

        // Add as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}
