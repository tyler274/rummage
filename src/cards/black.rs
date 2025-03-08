use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_black_cards() -> Vec<Card> {
    vec![
        Card {
            name: "Dark Ritual".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 1,
                red: 0,
                green: 0,
                color: Color::BLACK,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Demonic Tutor".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 0,
                black: 1,
                red: 0,
                green: 0,
                color: Color::BLACK,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Black Knight".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 2,
                red: 0,
                green: 0,
                color: Color::BLACK,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 2,
                creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
            }),
        },
        Card {
            name: "Mind Twist".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 0,
                black: 1,
                red: 0,
                green: 0,
                color: Color::BLACK,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Hypnotic Specter".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 2,
                red: 0,
                green: 0,
                color: Color::BLACK,
            },
            types: CardTypes::CREATURE,
            card_details: CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 2,
                creature_type: CreatureType::NONE,
            }),
        },
    ]
}
