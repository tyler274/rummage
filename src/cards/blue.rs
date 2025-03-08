use crate::card::{Card, CardDetails, CardTypes};
use crate::mana::{Color, Mana};

pub fn get_blue_cards() -> Vec<Card> {
    vec![
        Card {
            name: "Counterspell".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 2,
                black: 0,
                red: 0,
                green: 0,
                color: Color::BLUE,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Force of Will".to_string(),
            cost: Mana {
                colorless: 3,
                white: 0,
                blue: 2,
                black: 0,
                red: 0,
                green: 0,
                color: Color::BLUE,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Ancestral Recall".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 1,
                black: 0,
                red: 0,
                green: 0,
                color: Color::BLUE,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Time Walk".to_string(),
            cost: Mana {
                colorless: 1,
                white: 0,
                blue: 1,
                black: 0,
                red: 0,
                green: 0,
                color: Color::BLUE,
            },
            types: CardTypes::SORCERY,
            card_details: CardDetails::Other,
        },
        Card {
            name: "Mana Drain".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 2,
                black: 0,
                red: 0,
                green: 0,
                color: Color::BLUE,
            },
            types: CardTypes::INSTANT,
            card_details: CardDetails::Other,
        },
    ]
}
