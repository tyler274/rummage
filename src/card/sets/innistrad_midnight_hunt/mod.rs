use bevy::prelude::*;

use crate::card::CardSet;

// Card modules - Each card gets its own module
pub mod briarbridge_tracker;
pub mod brutal_cathar;
pub mod cathars_call;
pub mod champion_of_the_perished;
pub mod delver_of_secrets;
pub mod moonveil_regent;

/// Create a CardSet entity for Innistrad: Midnight Hunt
#[allow(dead_code)]
pub fn set_info() -> CardSet {
    CardSet {
        code: "MID".to_string(),
        name: "Innistrad: Midnight Hunt".to_string(),
        release_date: "2021-09-24".to_string(),
    }
}

/// Spawn a specific card from this set with all its components
#[allow(dead_code)]
pub fn spawn_card(commands: &mut Commands, name: &str) -> Option<Entity> {
    match name {
        "Brutal Cathar" => brutal_cathar::spawn(commands),
        "Cathar's Call" => cathars_call::spawn(commands),
        "Delver of Secrets" => delver_of_secrets::spawn(commands),
        "Champion of the Perished" => champion_of_the_perished::spawn(commands),
        "Moonveil Regent" => moonveil_regent::spawn(commands),
        "Briarbridge Tracker" => briarbridge_tracker::spawn(commands),
        _ => None,
    }
}

/// Spawn all cards from Innistrad: Midnight Hunt set
#[allow(dead_code)]
pub fn spawn_all_cards(commands: &mut Commands) -> Vec<Entity> {
    let mut entities = Vec::new();

    // Add each card's spawn call
    if let Some(entity) = brutal_cathar::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = cathars_call::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = delver_of_secrets::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = champion_of_the_perished::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = moonveil_regent::spawn(commands) {
        entities.push(entity);
    }
    if let Some(entity) = briarbridge_tracker::spawn(commands) {
        entities.push(entity);
    }

    entities
}
