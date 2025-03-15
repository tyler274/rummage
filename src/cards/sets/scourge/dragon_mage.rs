use bevy::prelude::*;

use crate::cards::rarity::Rarity;
use crate::cards::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::Mana;

use super::set_info;

/// Spawn Dragon Mage card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Dragon Mage")
        .cost(Mana::new_with_colors(5, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 5,
            toughness: 5,
            creature_type: CreatureType::DRAGON | CreatureType::WIZARD,
        }))
        .rules_text("Flying\nWhenever Dragon Mage deals combat damage to a player, each player discards their hand, then draws seven cards.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Rare, Name::new("Dragon Mage")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Dragon Mage")
        .cost(Mana::new_with_colors(5, 0, 0, 0, 2, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 5,
            toughness: 5,
            creature_type: CreatureType::DRAGON | CreatureType::WIZARD,
        }))
        .rules_text("Flying\nWhenever Dragon Mage deals combat damage to a player, each player discards their hand, then draws seven cards.")
        .build_or_panic()
}
