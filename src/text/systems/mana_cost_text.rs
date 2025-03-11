use crate::card::Card;
use crate::text::{
    components::{CardTextBundle, CardTextType, TextLayoutInfo},
    utils::{
        calculate_text_position, calculate_text_size, get_card_font_size, get_card_layout,
        get_mana_symbol_color,
    },
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

    // Drop shadow parameters - more subtle like rules text
    let shadow_offset = Vec3::new(0.8, -0.8, 0.0); // Slightly larger than rules text but still subtle
    let shadow_color = Color::rgba(0.0, 0.0, 0.0, 0.35); // Semi-transparent black for subtle shadow

    // Add each mana symbol as its own entity with correct positioning
    for (i, symbol) in symbols.iter().enumerate() {
        let symbol_color = get_mana_symbol_color(symbol);

        // Change how we position the mana symbols with sequential placement
        // The horizontal offset is calculated to place each symbol correctly
        let horizontal_offset =
            -(total_width / 2.0) + (i as f32 * symbol_width) + (symbol_width / 2.0);

        // First, spawn the shadow copy of the symbol
        commands
            .spawn((
                TextSpan::default(),
                Text2d::new(symbol.clone()),
                TextColor(shadow_color),
                TextFont {
                    font: mana_font.clone(),
                    font_size,
                    ..default()
                },
                // Position shadow with an offset to create the drop shadow effect
                Transform::from_translation(Vec3::new(horizontal_offset, 0.0, 0.0) + shadow_offset),
            ))
            .set_parent(parent_entity);

        // Then spawn the actual colored symbol on top
        commands
            .spawn((
                TextSpan::default(),
                Text2d::new(symbol.clone()),
                TextColor(symbol_color),
                TextFont {
                    font: mana_font.clone(),
                    font_size,
                    ..default()
                },
                // Position each symbol with a specific offset
                Transform::from_translation(Vec3::new(horizontal_offset, 0.0, 0.1)),
            ))
            .set_parent(parent_entity);
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
