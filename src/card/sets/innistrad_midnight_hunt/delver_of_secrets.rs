use bevy::prelude::*;

use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Delver of Secrets card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Delver of Secrets")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
        }))
        .rules_text("At the beginning of your upkeep, look at the top card of your library. If it's an instant or sorcery card, you may reveal it and transform Delver of Secrets.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Uncommon,
            Name::new("Delver of Secrets"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Delver of Secrets")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::CREATURE)
        .details(CardDetails::Creature(CreatureCard {
            power: 1,
            toughness: 1,
            creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
        }))
        .rules_text("At the beginning of your upkeep, look at the top card of your library. If it's an instant or sorcery card, you may reveal it and transform Delver of Secrets.")
        .build_or_panic()
}
