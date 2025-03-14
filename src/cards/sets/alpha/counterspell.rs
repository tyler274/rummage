use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Counterspell card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Counterspell")
        .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Counter target spell.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Uncommon,
            Name::new("Counterspell"),
        ))
        .id();

    Some(entity)
}

/// Get the card bundle
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Counterspell")
        .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("Counter target spell.")
        .build_or_panic()
}
