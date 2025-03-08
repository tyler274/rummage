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
            rules_text: "Flying\nWhenever Dragon Mage deals combat damage to a player, each player discards their hand, then draws seven cards.".to_string(),
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
            rules_text: "Lightning Bolt deals 3 damage to any target.".to_string(),
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
            rules_text: "Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.".to_string(),
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
            rules_text: "Fireball deals X damage divided evenly, rounded down, among any number of targets.\nFireball costs {1} more to cast for each target beyond the first.".to_string(),
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
            rules_text: "Each player discards their hand, then draws seven cards.".to_string(),
        },
    ]
}
