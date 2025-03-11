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
            rules_text: "Flying\n{T}: Add one mana of any color.".to_string(),
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
            rules_text: "{T}: Add {G}.".to_string(),
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
            rules_text: "Tarmogoyf's power is equal to the number of card types among cards in all graveyards and its toughness is that number plus 1.".to_string(),
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
            rules_text: "Target creature gets +3/+3 until end of turn.".to_string(),
        },
    ]
}
