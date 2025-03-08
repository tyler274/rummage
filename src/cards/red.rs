use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_red_cards() -> Vec<Card> {
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
            name: "Lightning Bolt".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 1,
                green: 0,
                color: Color::RED,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Shivan Dragon".to_string(),
            cost: Mana {
                colorless: 4,
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
                creature_type: CreatureType::DRAGON,
            }),
        },
        Card {
            name: "Fireball".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 0,
                black: 0,
                red: 1,
                green: 0,
                color: Color::RED,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Wheel of Fortune".to_string(),
            cost: Mana {
                colorless: 2,
                white: 0,
                blue: 0,
                black: 0,
                red: 1,
                green: 0,
                color: Color::RED,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
    ]
}
