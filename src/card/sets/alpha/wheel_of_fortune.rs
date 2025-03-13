use bevy::prelude::*;

use crate::card::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Wheel of Fortune card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Wheel of Fortune")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 1, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Each player discards their hand, then draws seven cards.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Rare,
            Name::new("Wheel of Fortune"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Wheel of Fortune")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 1, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Each player discards their hand, then draws seven cards.")
        .build_or_panic()
} 