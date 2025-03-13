use bevy::prelude::*;

use crate::card::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Ancestral Recall card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Ancestral Recall")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Target player draws three cards.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Rare,
            Name::new("Ancestral Recall"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Ancestral Recall")
        .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Target player draws three cards.")
        .build_or_panic()
} 