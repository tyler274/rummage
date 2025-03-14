use bevy::prelude::*;

use crate::cards::{Card, CardDetails, CardTypes, Rarity};
use crate::mana::Mana;

use super::set_info;

/// Spawn Force of Will card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Force of Will")
        .cost(Mana::new_with_colors(3, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("You may pay 1 life and exile a blue card from your hand rather than pay Force of Will's mana cost.\nCounter target spell.")
        .build_or_panic();

    let entity = commands
        .spawn((
            card,
            set_info(),
            Rarity::Uncommon,
            Name::new("Force of Will"),
        ))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Force of Will")
        .cost(Mana::new_with_colors(3, 0, 2, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Other)
        .rules_text("You may pay 1 life and exile a blue card from your hand rather than pay Force of Will's mana cost.\nCounter target spell.")
        .build_or_panic()
}
