use bevy::prelude::*;

use crate::cards::rarity::Rarity;
use crate::cards::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::Mana;

use super::set_info;

/// Spawn Moonveil Regent card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Moonveil Regent")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 4,
            toughness: 4,
            creature_type: CreatureType::DRAGON,
        }))
        .rules_text("Flying\nWhenever you cast a spell, you may discard your hand. If you do, draw a card for each of the discarded card's colors.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::MythicRare,
            Name::new("Moonveil Regent"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Moonveil Regent")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 4,
            toughness: 4,
            creature_type: CreatureType::DRAGON,
        }))
        .rules_text("Flying\nWhenever you cast a spell, you may discard your hand. If you do, draw a card for each of the discarded card's colors.")
        .build_or_panic()
}
