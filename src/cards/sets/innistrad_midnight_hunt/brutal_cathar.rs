use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, CreatureCard, CreatureType, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Brutal Cathar card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Brutal Cathar")
        .cost(Mana::new_with_colors(2, 1, 0, 0, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 3,
            toughness: 3,
            creature_type: CreatureType::HUMAN | CreatureType::SOLDIER,
        }))
        .rules_text("Daybound (If a player casts no spells during their own turn, it becomes night next turn.)\nWhen this creature enters the battlefield or transforms into Brutal Cathar, exile target creature an opponent controls until this creature leaves the battlefield.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Rare, Name::new("Brutal Cathar")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Brutal Cathar")
        .cost(Mana::new_with_colors(2, 1, 0, 0, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 3,
            toughness: 3,
            creature_type: CreatureType::HUMAN | CreatureType::SOLDIER,
        }))
        .rules_text("Daybound (If a player casts no spells during their own turn, it becomes night next turn.)\nWhen this creature enters the battlefield or transforms into Brutal Cathar, exile target creature an opponent controls until this creature leaves the battlefield.")
        .build_or_panic()
}
