use bevy::prelude::*;

use crate::cards::CardSet;

// Card modules for Legends
pub mod mana_drain;

/// Create a CardSet entity for Legends
#[allow(dead_code)]
pub fn set_info() -> CardSet {
    CardSet {
        code: "LEG".to_string(),
        name: "Legends".to_string(),
        release_date: "1994-06-01".to_string(),
    }
}

/// Spawn a specific card from this set with all its components
#[allow(dead_code)]
pub fn spawn_card(commands: &mut Commands, name: &str) -> Option<Entity> {
    match name {
        "Mana Drain" => mana_drain::spawn(commands),
        _ => None,
    }
}

/// Spawn all cards from Legends set
#[allow(dead_code)]
pub fn spawn_all_cards(commands: &mut Commands) -> Vec<Entity> {
    let mut entities = Vec::new();

    // Add each card's spawn call
    if let Some(entity) = mana_drain::spawn(commands) {
        entities.push(entity);
    }

    entities
}
