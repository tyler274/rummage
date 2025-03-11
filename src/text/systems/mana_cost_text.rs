use crate::card::Card;
use crate::text::{components::CardTextBundle, utils::CardTextLayout};
use bevy::prelude::*;

/// Creates text entities for mana costs
pub fn create_mana_cost_text(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Card)>,
    asset_server: Res<AssetServer>,
) {
    // Load mana symbol font
    let mana_font = asset_server.load("fonts/Mana.ttf");
    let layout = CardTextLayout::default();

    for (entity, transform, card) in query.iter() {
        // Skip cards with no mana cost
        if card.cost.is_empty() {
            continue;
        }

        // Font size for mana symbols
        let font_size = 26.0;
        let card_size = Vec2::new(layout.card_width, layout.card_height);

        // Format mana cost text using the specialized Mana font
        let mana_text = format_mana_cost(&card.cost.to_string());

        // Calculate position relative to card
        let mana_position = Vec2::new(
            layout.mana_cost_x_offset * card_size.x,
            layout.mana_cost_y_offset * card_size.y,
        );

        // Create the text entity
        let text_entity = commands
            .spawn(CardTextBundle::new(
                mana_text,
                mana_font.clone(),
                font_size,
                Color::BLACK,
                transform.translation,
                mana_position,
                JustifyText::Right,
            ))
            .id();

        // Set as child of the card entity
        commands.entity(entity).add_child(text_entity);
    }
}

/// Format mana cost text to use Mana.ttf font characters
fn format_mana_cost(mana_cost: &str) -> String {
    // The Mana.ttf font maps specific characters to mana symbols
    // We need to convert MTG-style mana syntax (e.g., {R}) to the correct characters

    let mut formatted = String::new();
    let mut in_brace = false;
    let mut current_symbol = String::new();

    for c in mana_cost.chars() {
        match c {
            '{' => {
                in_brace = true;
                current_symbol.clear();
            }
            '}' => {
                if in_brace {
                    // Map the symbol to the corresponding character in Mana.ttf
                    let symbol_char = map_mana_symbol(&current_symbol);
                    formatted.push(symbol_char);
                    in_brace = false;
                }
            }
            _ if in_brace => {
                current_symbol.push(c);
            }
            _ => formatted.push(c), // Keep characters outside braces (shouldn't normally happen)
        }
    }

    formatted
}

/// Map a mana symbol string to the corresponding character in Mana.ttf
fn map_mana_symbol(symbol: &str) -> char {
    match symbol {
        // Colored mana
        "W" => 'w', // White
        "U" => 'u', // Blue
        "B" => 'b', // Black
        "R" => 'r', // Red
        "G" => 'g', // Green

        // Colorless mana
        "0" => '0',
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        "10" => 'a', // 10
        "11" => 'b', // 11
        "12" => 'c', // 12
        "13" => 'd', // 13
        "14" => 'e', // 14
        "15" => 'f', // 15
        "16" => 'g', // 16
        "17" => 'h', // 17
        "18" => 'i', // 18
        "19" => 'j', // 19
        "20" => 'k', // 20

        // Hybrid mana (examples)
        "W/U" => 'q', // White/Blue hybrid
        "U/B" => 'r', // Blue/Black hybrid
        "B/R" => 's', // Black/Red hybrid
        "R/G" => 't', // Red/Green hybrid
        "G/W" => 'v', // Green/White hybrid

        // Phyrexian mana
        "W/P" => 'o', // White Phyrexian
        "U/P" => 'p', // Blue Phyrexian
        "B/P" => 'n', // Black Phyrexian
        "R/P" => 'm', // Red Phyrexian
        "G/P" => 'l', // Green Phyrexian

        // Generic/colorless mana symbol (C)
        "C" => 'c',

        // Snow mana
        "S" => 's',

        // X and variable mana
        "X" => 'x',

        // Default fallback if symbol is unknown
        _ => '?',
    }
}
