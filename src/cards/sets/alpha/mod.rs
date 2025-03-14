use bevy::prelude::*;

use crate::cards::CardSet;

// Card modules for Alpha
pub mod ancestral_recall;
pub mod counterspell;
pub mod fireball;
pub mod lightning_bolt;
pub mod shivan_dragon;
pub mod time_walk;
pub mod wheel_of_fortune;

/// Create a CardSet entity for Limited Edition Alpha
#[allow(dead_code)]
pub fn set_info() -> CardSet {
    CardSet {
        code: "LEA".to_string(),
        name: "Limited Edition Alpha".to_string(),
        release_date: "1993-08-05".to_string(),
    }
}

/// Spawn a specific card from this set with all its components
#[allow(dead_code)]
pub fn spawn_card(commands: &mut Commands, name: &str) -> Option<Entity> {
    match name {
        "Ancestral Recall" => ancestral_recall::spawn(commands),
        "Counterspell" => counterspell::spawn(commands),
        "Fireball" => fireball::spawn(commands),
        "Lightning Bolt" => lightning_bolt::spawn(commands),
        "Shivan Dragon" => shivan_dragon::spawn(commands),
        "Time Walk" => time_walk::spawn(commands),
        "Wheel of Fortune" => wheel_of_fortune::spawn(commands),
        _ => None,
    }
}

/// Spawn all cards from Alpha set
#[allow(dead_code)]
pub fn spawn_all_cards(commands: &mut Commands) -> Vec<Entity> {
    let mut entities = Vec::new();

    // Add each card's spawn call
    if let Some(entity) = ancestral_recall::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = counterspell::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = fireball::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = lightning_bolt::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = shivan_dragon::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = time_walk::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = wheel_of_fortune::spawn(commands) {
        entities.push(entity);
    }

    entities
}
