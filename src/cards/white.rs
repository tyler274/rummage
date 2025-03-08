use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_white_cards() -> Vec<Card> {
    vec![
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
            name: "Wrath of God".to_string(),
            cost: Mana {
                colorless: 2,
                white: 2,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::WHITE,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Swords to Plowshares".to_string(),
            cost: Mana {
                colorless: 0,
                white: 1,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::WHITE,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Armageddon".to_string(),
            cost: Mana {
                colorless: 3,
                white: 1,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::WHITE,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Balance".to_string(),
            cost: Mana {
                colorless: 1,
                white: 1,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::WHITE,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
    ]
}
