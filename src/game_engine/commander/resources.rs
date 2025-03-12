use super::components::CommanderZoneLocation;
use crate::mana::Color;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// The Command Zone where commanders reside when not in play
#[derive(Resource, Debug, Default)]
pub struct CommandZone {
    /// Cards in the command zone
    pub cards: Vec<Entity>,
}

impl CommandZone {
    /// Add a card to the command zone
    pub fn add_card(&mut self, card: Entity) {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
    }

    /// Remove a card from the command zone
    pub fn remove_card(&mut self, card: Entity) -> bool {
        if let Some(index) = self.cards.iter().position(|&c| c == card) {
            self.cards.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if a card is in the command zone
    pub fn contains(&self, card: Entity) -> bool {
        self.cards.contains(&card)
    }
}

/// Manager for the Command Zone and Commander state
#[derive(Resource, Debug, Default)]
pub struct CommandZoneManager {
    /// Maps player entities to their commander entities
    pub player_commanders: HashMap<Entity, Vec<Entity>>,

    /// Maps commander entities to their current zone
    pub commander_zone_status: HashMap<Entity, CommanderZoneLocation>,

    /// Tracks how many times a commander has moved zones
    pub zone_transition_count: HashMap<Entity, u32>,

    /// Tracks commander partnerships
    pub commander_partners: HashMap<Entity, Entity>,

    /// Maps commander entities to their color identity
    pub commander_colors: HashMap<Entity, HashSet<Color>>,
}

impl CommandZoneManager {
    /// Initialize with a list of players and their commanders
    pub fn initialize(&mut self, player_commanders: HashMap<Entity, Vec<Entity>>) {
        self.player_commanders = player_commanders.clone();

        // Initialize all commanders as being in the command zone
        for commanders in player_commanders.values() {
            for &commander in commanders {
                self.commander_zone_status
                    .insert(commander, CommanderZoneLocation::CommandZone);
                self.zone_transition_count.insert(commander, 0);
            }
        }
    }

    /// Set a commander's color identity
    pub fn set_commander_color_identity(
        &mut self,
        commander: Entity,
        color_identity: HashSet<Color>,
    ) {
        self.commander_colors.insert(commander, color_identity);
    }

    /// Get a player's commanders
    pub fn get_player_commanders(&self, player: Entity) -> Vec<Entity> {
        self.player_commanders
            .get(&player)
            .cloned()
            .unwrap_or_default()
    }

    /// Get a commander's current zone
    pub fn get_commander_zone(&self, commander: Entity) -> CommanderZoneLocation {
        self.commander_zone_status
            .get(&commander)
            .cloned()
            .unwrap_or(CommanderZoneLocation::CommandZone)
    }

    /// Get the number of times a commander has been cast
    pub fn get_cast_count(&self, commander: Entity) -> u32 {
        self.zone_transition_count
            .get(&commander)
            .cloned()
            .unwrap_or(0)
    }

    /// Update a commander's zone
    pub fn update_commander_zone(&mut self, commander: Entity, new_zone: CommanderZoneLocation) {
        self.commander_zone_status.insert(commander, new_zone);
    }
}
