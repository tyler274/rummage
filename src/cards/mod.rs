mod penacony;

use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};
use bevy::prelude::*;
use bevy_turborand::prelude::*;

pub struct CardsPlugin;

impl Plugin for CardsPlugin {
    fn build(&self, app: &mut App) {
        // Nothing to initialize
    }
}

/// Returns a list of example cards for testing and development
pub fn get_example_cards(mut rng: ResMut<GlobalRng>) -> Vec<Card> {
    let mut all_cards = vec![
        // Original cards
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
        },
        // Additional iconic cards
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
    ];

    // Shuffle the cards using Fisher-Yates algorithm
    for i in (1..all_cards.len()).rev() {
        let j = rng.usize(0..=i);
        all_cards.swap(i, j);
    }
    all_cards.truncate(5);
    all_cards
}
