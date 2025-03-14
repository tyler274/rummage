use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, CreatureCard, CreatureType, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Briarbridge Tracker card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Briarbridge Tracker")
        .cost(Mana::new_with_colors(3, 0, 0, 0, 0, 1))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 4,
            toughness: 3,
            creature_type: CreatureType::HUMAN | CreatureType::SCOUT,
        }))
        .rules_text("When Briarbridge Tracker enters the battlefield, investigate. (Create a colorless Clue artifact token with \"{2}, Sacrifice this artifact: Draw a card.\")\nWhenever Briarbridge Tracker attacks, if you control three or more Clue tokens, it gets +2/+2 until end of turn.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Uncommon,
            Name::new("Briarbridge Tracker"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Briarbridge Tracker")
        .cost(Mana::new_with_colors(3, 0, 0, 0, 0, 1))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 4,
            toughness: 3,
            creature_type: CreatureType::HUMAN | CreatureType::SCOUT,
        }))
        .rules_text("When Briarbridge Tracker enters the battlefield, investigate. (Create a colorless Clue artifact token with \"{2}, Sacrifice this artifact: Draw a card.\")\nWhenever Briarbridge Tracker attacks, if you control three or more Clue tokens, it gets +2/+2 until end of turn.")
        .build_or_panic()
}
