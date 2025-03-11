use crate::card::{Card, CardDetails, CardTypes};
use crate::mana::{Color, Mana};

pub fn get_artifact_cards() -> Vec<Card> {
    vec![
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
            rules_text: "{T}: Add {2}.".to_string(),
        },
        Card {
            name: "Black Lotus".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::COLORLESS,
            },
            types: CardTypes::ARTIFACT,
            card_details: CardDetails::Other,
            rules_text: "Sacrifice Black Lotus: Add three mana of any one color to your mana pool."
                .to_string(),
        },
        Card {
            name: "Mox Sapphire".to_string(),
            cost: Mana {
                colorless: 0,
                white: 0,
                blue: 0,
                black: 0,
                red: 0,
                green: 0,
                color: Color::COLORLESS,
            },
            types: CardTypes::ARTIFACT,
            card_details: CardDetails::Other,
            rules_text: "{T}: Add {U}.".to_string(),
        },
    ]
}
