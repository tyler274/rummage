use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Mana Drain card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Mana Drain")
        .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Rare, Name::new("Mana Drain")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Mana Drain")
        .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.")
        .build_or_panic()
}
