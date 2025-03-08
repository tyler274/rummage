use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_green_cards() -> Vec<Card> {
    vec![
        Card {
            name: "Birds of Paradise".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 1,
                color: Color::GREEN,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 0,
                toughness: 1,
                creature_type: CreatureType::NONE,
            }),
        },
        Card {
            name: "Llanowar Elves".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 1,
                color: Color::GREEN,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 1,
                toughness: 1,
                creature_type: CreatureType::NONE,
            }),
        },
        Card {
            name: "Tarmogoyf".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 1,
                color: Color::GREEN,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 3,
                creature_type: CreatureType::NONE,
            }),
        },
        Card {
            name: "Giant Growth".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 1,
                color: Color::GREEN,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
    ]
}
