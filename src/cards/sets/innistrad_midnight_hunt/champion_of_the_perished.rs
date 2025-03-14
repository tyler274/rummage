use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, CreatureCard, CreatureType, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Champion of the Perished card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Champion of the Perished")
        .cost(Mana::new_with_colors(0, 0, 0, 1, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::ZOMBIE,
        }))
        .rules_text("Whenever another Zombie enters the battlefield under your control, put a +1/+1 counter on Champion of the Perished.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Rare,
            Name::new("Champion of the Perished"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Champion of the Perished")
        .cost(Mana::new_with_colors(0, 0, 0, 1, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::ZOMBIE,
        }))
        .rules_text("Whenever another Zombie enters the battlefield under your control, put a +1/+1 counter on Champion of the Perished.")
        .build_or_panic()
}
