use bevy::prelude::*;

use crate::card::{Card, CardDetails, CardTypes, Rarity, SpellCard, SpellType};
use crate::mana::Mana;

use super::set_info;

/// Spawn Cathar's Call card
#[allow(dead_code)]
pub fn spawn(commands: &mut Commands) -> Option<Entity> {
    let card = Card::builder("Cathar's Call")
        .cost(Mana::new_with_colors(2, 1, 0, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Instant(SpellCard {
            spell_type: SpellType::Instant,
            targets: vec!["creature".to_string()],
        }))
        .rules_text("Create a 1/1 white Human creature token. Target creature gets +1/+1 until end of turn.")
        .build_or_panic();

    let entity = commands
        .spawn((card, set_info(), Rarity::Common, Name::new("Cathar's Call")))
        .id();

    Some(entity)
}

/// Get the card definition
#[allow(dead_code)]
pub fn get_card() -> Card {
    Card::builder("Cathar's Call")
        .cost(Mana::new_with_colors(2, 1, 0, 0, 0, 0))
        .types(CardTypes::INSTANT)
        .details(CardDetails::Instant(SpellCard {
            spell_type: SpellType::Instant,
            targets: vec!["creature".to_string()],
        }))
        .rules_text("Create a 1/1 white Human creature token. Target creature gets +1/+1 until end of turn.")
        .build_or_panic()
}
