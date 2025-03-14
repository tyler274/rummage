use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Lightning Bolt card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Lightning Bolt")
        .cost(Mana::new_with_colors(0, 0, 0, 0, 1, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Lightning Bolt deals 3 damage to any target.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Common,
            Name::new("Lightning Bolt"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Lightning Bolt")
        .cost(Mana::new_with_colors(0, 0, 0, 0, 1, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Lightning Bolt deals 3 damage to any target.")
        .build_or_panic()
}
