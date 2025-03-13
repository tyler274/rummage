use bevy::prelude::*;

use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Shivan Dragon card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Shivan Dragon")
        .cost(Mana::new_with_colors(4, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 5,
            toughness: 5,
            creature_type: CreatureType::DRAGON,
        }))
        .rules_text("Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Rare,
            Name::new("Shivan Dragon"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Shivan Dragon")
        .cost(Mana::new_with_colors(4, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 5,
            toughness: 5,
            creature_type: CreatureType::DRAGON,
        }))
        .rules_text("Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.")
        .build_or_panic()
} 