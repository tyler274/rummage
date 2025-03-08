mod penacony;

use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

/// Returns a list of example cards for testing and development
pub fn get_example_cards() -> Vec<Card> {
    vec![
        Card {
            name: "Dragon Mage".to_string(),
            cost: Mana {
                colorless: 5,
                white: 0,
                blue: 0,
                black: 0,
                red: 2,
                green: 0,
                color: Color::RED,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 5,
                toughness: 5,
                creature_type: CreatureType::DRAGON | CreatureType::WIZARD,
            }),
        },
        Card {
            name: "Serra Angel".to_string(),
            cost: Mana {
                colorless: 3,
                white: 2,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::WHITE,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 4,
                toughness: 4,
                creature_type: CreatureType::ANGEL,
            }),
        },
        Card {
            name: "Sol Ring".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::COLORLESS,
            },
            types: CardTypes::ARTIFACT,
            card_details: CardDetails::Other,
        },
    ]
}
