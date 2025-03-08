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
            rules_text: "Add {B}{B}{B}.".to_string(),
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
            rules_text: "Search your library for a card and put that card into your hand. Then shuffle your library.".to_string(),
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
            rules_text: "First strike, protection from white".to_string(),
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
            rules_text: "Target player discards X cards at random.".to_string(),
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
            rules_text: "Flying\nWhenever Hypnotic Specter deals damage to an opponent, that player discards a card at random.".to_string(),
        },
    ]
}
