use crate::card::Card;
use crate::text::{
    components::{CardTextBundle, CardTextType, TextLayoutInfo},
    mana_symbols::{ManaSymbolOptions, render_mana_symbol},
    utils::{calculate_text_position, calculate_text_size, get_card_font_size, get_card_layout},
};
use bevy::prelude::*;

/// Creates a text entity for mana cost with colored mana symbols
pub fn create_mana_cost_text(
    commands: &mut Commands,
    content: &crate::text::components::CardTextContent,
    card_pos: Vec2,
    card_size: Vec2,
    asset_server: &AssetServer,
) -> Entity {
    // Skip cards with no mana cost
    if content.mana_cost.is_empty() {
        return commands.spawn_empty().id();
    }

    // Load mana symbol font
    let mana_font = asset_server.load("fonts/Mana.ttf");
    let layout = get_card_layout();

    // Font size for mana symbols
    let font_size = 25.0;

    // Position the mana cost correctly
    let mana_position = Vec2::new(
        layout.mana_cost_x_offset * card_size.x,
        layout.mana_cost_y_offset * card_size.y,
    );

    // Extract individual mana symbols from the mana cost
    let mana_cost = content.mana_cost.clone();
    let mut symbols = Vec::new();
    let mut current_symbol = String::new();
    let mut inside_brace = false;

    for c in mana_cost.chars() {
        match c {
            '{' => {
                inside_brace = true;
                current_symbol.push(c);
            }
            '}' => {
                inside_brace = false;
                current_symbol.push(c);
                symbols.push(current_symbol.clone());
                current_symbol.clear();
            }
            _ => {
                if inside_brace {
                    current_symbol.push(c);
                }
            }
        }
    }

    // Create the parent container entity
    let parent_entity = commands
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
                position: card_pos + mana_position,
                size: Vec2::new(card_size.x * 0.3, card_size.y * 0.1),
                alignment: JustifyText::Right,
            },
            Name::new(format!("Mana Cost: {}", content.mana_cost)),
        ))
        .id();

    // The spacing value between mana symbols
    let symbol_width = font_size * 1.2;
    let num_symbols = symbols.len() as f32;
    let total_width = symbol_width * num_symbols;

    // Create mana symbol options
    let mana_options = ManaSymbolOptions {
        font_size,
        vertical_alignment_offset: 0.0, // No baseline adjustment needed for mana cost
        z_index: 0.1,
        with_shadow: true,
    };

    // Add each mana symbol as its own entity with correct positioning
    for (i, symbol) in symbols.iter().enumerate() {
        // Calculate the horizontal offset for sequential placement
        let horizontal_offset =
            -(total_width / 2.0) + (i as f32 * symbol_width) + (symbol_width / 2.0);

        // Use the unified mana symbol renderer
        render_mana_symbol(
            commands,
            symbol,
            Vec2::new(horizontal_offset, 0.0),
            mana_font.clone(),
            mana_options.clone(),
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
        if card.cost.is_empty() {
            continue;
        }

        // Convert Mana struct to display string
        let mana_cost_string = card.cost.to_string();

        // Get card position and size
        let card_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let layout = get_card_layout();
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Create content for mana cost text
        let content = crate::text::components::CardTextContent {
            name: card.name.clone(),
            mana_cost: mana_cost_string,
            type_line: card.type_line(),
            rules_text: card.rules_text.clone(),
            power_toughness: match &card.card_details {
                crate::card::CardDetails::Creature(creature) => {
                    Some(format!("{}/{}", creature.power, creature.toughness))
                }
                _ => None,
            },
        };

        // Create the mana cost text
        let text_entity =
            create_mana_cost_text(&mut commands, &content, card_pos, card_size, &asset_server);

        // Add as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}
