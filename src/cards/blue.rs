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
            rules_text: "Counter target spell.".to_string(),
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
            rules_text: "You may pay 1 life and exile a blue card from your hand rather than pay Force of Will's mana cost.\nCounter target spell.".to_string(),
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
            rules_text: "Target player draws three cards.".to_string(),
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
            rules_text: "Take an extra turn after this one.".to_string(),
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
            rules_text: "Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.".to_string(),
        },
    ]
}
