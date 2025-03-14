use bevy::prelude::*;

use crate::cards::CardSet;

// Card modules for Alliances
pub mod force_of_will;

/// Create a CardSet entity for Alliances
#[allow(dead_code)]
pub fn set_info() -> CardSet {
    CardSet {
        code: "ALL".to_string(),
        name: "Alliances".to_string(),
        release_date: "1996-06-10".to_string(),
    }
}

/// Spawn a specific card from this set with all its components
#[allow(dead_code)]
pub fn spawn_card(commands: &mut Commands, name: &str) -> Option<Entity> {
    match name {
        "Force of Will" => force_of_will::spawn(commands),
        _ => None,
    }
}

/// Spawn all cards from Alliances set
#[allow(dead_code)]
pub fn spawn_all_cards(commands: &mut Commands) -> Vec<Entity> {
    let mut entities = Vec::new();

    // Add each card's spawn call
    if let Some(entity) = force_of_will::spawn(commands) {
        entities.push(entity);
    }

    entities
}
