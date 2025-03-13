use bevy::prelude::*;

use crate::card::CardSet;

// Card modules for Scourge
pub mod dragon_mage;

/// Create a CardSet entity for Scourge
#[allow(dead_code)]
pub fn set_info() -> CardSet {
    CardSet {
        code: "SCG".to_string(),
        name: "Scourge".to_string(),
        release_date: "2003-05-26".to_string(),
    }
}

/// Spawn a specific card from this set with all its components
#[allow(dead_code)]
pub fn spawn_card(commands: &mut Commands, name: &str) -> Option<Entity> {
    match name {
        "Dragon Mage" => dragon_mage::spawn(commands),
        _ => None,
    }
}

/// Spawn all cards from Scourge set
#[allow(dead_code)]
pub fn spawn_all_cards(commands: &mut Commands) -> Vec<Entity> {
    let mut entities = Vec::new();

    // Add each card's spawn call
    if let Some(entity) = dragon_mage::spawn(commands) {
        entities.push(entity);
    }

    entities
}
