use bevy::prelude::*;

use crate::cards::rarity::Rarity;
use crate::cards::{Card, CardDetails, CardTypes};
use crate::mana::Mana;

use super::set_info;

/// Spawn Fireball card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Fireball")
        .cost(Mana::new_with_colors(1, 0, 0, 0, 1, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Fireball deals X damage divided evenly, rounded down, among any number of targets.\nFireball costs {1} more to cast for each target beyond the first.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Uncommon, Name::new("Fireball")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Fireball")
        .cost(Mana::new_with_colors(1, 0, 0, 0, 1, 0))
        .types(CardTypes::SORCERY)
        .details(CardDetails::Other)
        .rules_text("Fireball deals X damage divided evenly, rounded down, among any number of targets.\nFireball costs {1} more to cast for each target beyond the first.")
        .build_or_panic()
}
