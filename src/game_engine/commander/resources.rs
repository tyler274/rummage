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
    /// Creates a new CommandZoneBuilder for chainable construction
    pub fn builder() -> CommandZoneBuilder {
        CommandZoneBuilder::new()
    }

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

/// Builder for CommandZone with a chainable API
#[derive(Debug, Clone)]
pub struct CommandZoneBuilder {
    cards: Vec<Entity>,
}

impl CommandZoneBuilder {
    /// Creates a new CommandZoneBuilder with default values
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Adds a card to the command zone
    pub fn add_card(mut self, card: Entity) -> Self {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
        self
    }

    /// Sets all cards in the command zone at once
    pub fn cards(mut self, cards: Vec<Entity>) -> Self {
        self.cards = cards;
        self
    }

    /// Builds the CommandZone instance
    pub fn build(self) -> CommandZone {
        CommandZone { cards: self.cards }
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
    /// Creates a new CommandZoneManagerBuilder for chainable construction
    pub fn builder() -> CommandZoneManagerBuilder {
        CommandZoneManagerBuilder::new()
    }

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

    /// Set the color identity for a commander
    pub fn set_commander_color_identity(
        &mut self,
        commander: Entity,
        color_identity: HashSet<Color>,
    ) {
        self.commander_colors.insert(commander, color_identity);
    }

    /// Get the commanders for a player
    pub fn get_player_commanders(&self, player: Entity) -> Vec<Entity> {
        self.player_commanders
            .get(&player)
            .cloned()
            .unwrap_or_default()
    }

    /// Get the current zone of a commander
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

    /// Update the zone of a commander
    pub fn update_commander_zone(&mut self, commander: Entity, new_zone: CommanderZoneLocation) {
        // Update zone status
        self.commander_zone_status.insert(commander, new_zone);

        // If the zone is the command zone, don't increment the counter
        if new_zone != CommanderZoneLocation::CommandZone {
            let count = self.zone_transition_count.entry(commander).or_insert(0);
            *count += 1;
        }
    }
}

/// Builder for CommandZoneManager with a chainable API
#[derive(Debug, Clone)]
pub struct CommandZoneManagerBuilder {
    player_commanders: HashMap<Entity, Vec<Entity>>,
    commander_zone_status: HashMap<Entity, CommanderZoneLocation>,
    zone_transition_count: HashMap<Entity, u32>,
    commander_partners: HashMap<Entity, Entity>,
    commander_colors: HashMap<Entity, HashSet<Color>>,
}

impl CommandZoneManagerBuilder {
    /// Creates a new CommandZoneManagerBuilder with default values
    pub fn new() -> Self {
        Self {
            player_commanders: HashMap::new(),
            commander_zone_status: HashMap::new(),
            zone_transition_count: HashMap::new(),
            commander_partners: HashMap::new(),
            commander_colors: HashMap::new(),
        }
    }

    /// Adds a commander for a player
    pub fn add_commander(mut self, player: Entity, commander: Entity) -> Self {
        self.player_commanders
            .entry(player)
            .or_insert_with(Vec::new)
            .push(commander);

        // Initialize commander in command zone with 0 transitions
        self.commander_zone_status
            .insert(commander, CommanderZoneLocation::CommandZone);
        self.zone_transition_count.insert(commander, 0);

        self
    }

    /// Sets all player commanders at once
    pub fn player_commanders(mut self, player_commanders: HashMap<Entity, Vec<Entity>>) -> Self {
        self.player_commanders = player_commanders.clone();

        // Initialize all commanders as being in the command zone
        for commanders in player_commanders.values() {
            for &commander in commanders {
                self.commander_zone_status
                    .insert(commander, CommanderZoneLocation::CommandZone);
                self.zone_transition_count.insert(commander, 0);
            }
        }

        self
    }

    /// Sets a commander's zone
    pub fn set_commander_zone(mut self, commander: Entity, zone: CommanderZoneLocation) -> Self {
        self.commander_zone_status.insert(commander, zone);
        self
    }

    /// Sets a commander's transition count
    pub fn set_transition_count(mut self, commander: Entity, count: u32) -> Self {
        self.zone_transition_count.insert(commander, count);
        self
    }

    /// Sets a commander partnership
    pub fn add_partner(mut self, commander1: Entity, commander2: Entity) -> Self {
        self.commander_partners.insert(commander1, commander2);
        self
    }

    /// Sets a commander's color identity
    pub fn set_color_identity(mut self, commander: Entity, colors: HashSet<Color>) -> Self {
        self.commander_colors.insert(commander, colors);
        self
    }

    /// Builds the CommandZoneManager instance
    pub fn build(self) -> CommandZoneManager {
        CommandZoneManager {
            player_commanders: self.player_commanders,
            commander_zone_status: self.commander_zone_status,
            zone_transition_count: self.zone_transition_count,
            commander_partners: self.commander_partners,
            commander_colors: self.commander_colors,
        }
    }
}
