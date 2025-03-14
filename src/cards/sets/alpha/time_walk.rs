use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Time Walk card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Time Walk")
        .cost(Mana::new_with_colors(1, 0, 1, 0, 0, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Take an extra turn after this one.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Rare, Name::new("Time Walk")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Time Walk")
        .cost(Mana::new_with_colors(1, 0, 1, 0, 0, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Take an extra turn after this one.")
        .build_or_panic()
}
