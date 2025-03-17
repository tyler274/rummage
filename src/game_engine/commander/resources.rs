use super::components::CommanderZoneLocation;
use crate::mana::ManaColor;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// The Command Zone where commanders reside when not in play
#[derive(Resource, Debug, Default)]
pub struct CommandZone {
    /// Cards in the command zone
    #[allow(dead_code)]
    pub cards: Vec<Entity>,
}

impl CommandZone {
    /// Creates a new CommandZoneBuilder for chainable construction
    ///
    /// Will be used in future implementations for more flexible initialization
    #[allow(dead_code)]
    pub fn builder() -> CommandZoneBuilder {
        CommandZoneBuilder::new()
    }

    /// Add a card to the command zone
    #[allow(dead_code)]
    pub fn add_card(&mut self, card: Entity) {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
    }

    /// Remove a card from the command zone
    #[allow(dead_code)]
    pub fn remove_card(&mut self, card: Entity) -> bool {
        if let Some(index) = self.cards.iter().position(|&c| c == card) {
            self.cards.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if a card is in the command zone
    #[allow(dead_code)]
    pub fn contains(&self, card: Entity) -> bool {
        self.cards.contains(&card)
    }
}

/// Builder for CommandZone to enable chainable construction
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CommandZoneBuilder {
    cards: Vec<Entity>,
}

impl CommandZoneBuilder {
    /// Creates a new builder with default values
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the command zone being built
    #[allow(dead_code)]
    pub fn add_card(mut self, card: Entity) -> Self {
        if !self.cards.contains(&card) {
            self.cards.push(card);
        }
        self
    }

    /// Set the cards in the command zone
    #[allow(dead_code)]
    pub fn cards(mut self, cards: Vec<Entity>) -> Self {
        self.cards = cards;
        self
    }

    /// Build the CommandZone with configured values
    #[allow(dead_code)]
    pub fn build(self) -> CommandZone {
        CommandZone { cards: self.cards }
    }
}

/// Manager for commander-specific rules and state
#[derive(Resource, Debug, Default)]
pub struct CommandZoneManager {
    /// Maps player entities to their commander entities
    #[allow(dead_code)]
    pub player_commanders: HashMap<Entity, Vec<Entity>>,

    /// Maps commander entities to their current zone
    pub commander_zone_status: HashMap<Entity, CommanderZoneLocation>,

    /// Tracks how many times a commander has moved zones
    pub zone_transition_count: HashMap<Entity, u32>,

    /// Tracks commander partnerships
    #[allow(dead_code)]
    pub commander_partners: HashMap<Entity, Entity>,

    /// Maps commander entities to their color identity
    #[allow(dead_code)]
    pub commander_colors: HashMap<Entity, HashSet<ManaColor>>,
}

impl CommandZoneManager {
    /// Creates a new CommandZoneManagerBuilder for chainable construction
    ///
    /// Will be used in future implementations for more flexible manager initialization
    #[allow(dead_code)]
    pub fn builder() -> CommandZoneManagerBuilder {
        CommandZoneManagerBuilder::new()
    }

    /// Initialize the commander manager with player-commander mappings
    #[allow(dead_code)]
    pub fn initialize(&mut self, player_commanders: HashMap<Entity, Vec<Entity>>) {
        self.player_commanders = player_commanders;

        // Initialize commanders to be in the command zone
        for commanders in self.player_commanders.values() {
            for &commander in commanders {
                self.commander_zone_status
                    .insert(commander, CommanderZoneLocation::CommandZone);
                self.zone_transition_count.insert(commander, 0);
            }
        }
    }

    /// Sets the color identity for a commander
    #[allow(dead_code)]
    pub fn set_commander_color_identity(
        &mut self,
        commander: Entity,
        color_identity: HashSet<ManaColor>,
    ) {
        self.commander_colors.insert(commander, color_identity);
    }

    /// Gets a player's commanders
    #[allow(dead_code)]
    pub fn get_player_commanders(&self, player: Entity) -> Vec<Entity> {
        self.player_commanders
            .get(&player)
            .cloned()
            .unwrap_or_default()
    }

    /// Gets a commander's current zone
    #[allow(dead_code)]
    pub fn get_commander_zone(&self, commander: Entity) -> CommanderZoneLocation {
        self.commander_zone_status
            .get(&commander)
            .copied()
            .unwrap_or(CommanderZoneLocation::CommandZone)
    }

    /// Gets the cast count for a commander
    pub fn get_cast_count(&self, commander: Entity) -> u32 {
        self.zone_transition_count
            .get(&commander)
            .copied()
            .unwrap_or(0)
    }

    /// Updates a commander's zone and increments its transition count if needed
    pub fn update_commander_zone(&mut self, commander: Entity, new_zone: CommanderZoneLocation) {
        // Update the commander's location
        self.commander_zone_status.insert(commander, new_zone);

        if new_zone == CommanderZoneLocation::Battlefield {
            // Commander was cast, so increment the commander tax counter
            let current_count = self
                .zone_transition_count
                .get(&commander)
                .copied()
                .unwrap_or(0);
            self.zone_transition_count
                .insert(commander, current_count + 1);
        }
    }
}

/// Builder for CommandZoneManager to enable chainable construction
///
/// This builder is part of the commander management design but is not
/// actively used in the current implementation. It will be needed
/// for future commander rule implementations.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CommandZoneManagerBuilder {
    player_commanders: HashMap<Entity, Vec<Entity>>,
    commander_zone_status: HashMap<Entity, CommanderZoneLocation>,
    zone_transition_count: HashMap<Entity, u32>,
    commander_partners: HashMap<Entity, Entity>,
    commander_colors: HashMap<Entity, HashSet<ManaColor>>,
}

impl CommandZoneManagerBuilder {
    /// Creates a new CommandZoneManagerBuilder with default values
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            player_commanders: HashMap::new(),
            commander_zone_status: HashMap::new(),
            zone_transition_count: HashMap::new(),
            commander_partners: HashMap::new(),
            commander_colors: HashMap::new(),
        }
    }

    /// Add a commander for a player
    #[allow(dead_code)]
    pub fn add_commander(mut self, player: Entity, commander: Entity) -> Self {
        self.player_commanders
            .entry(player)
            .or_insert_with(Vec::new)
            .push(commander);

        // By default, commanders start in the command zone
        self.commander_zone_status
            .insert(commander, CommanderZoneLocation::CommandZone);
        self.zone_transition_count.insert(commander, 0);
        self
    }

    /// Set all player commanders at once
    #[allow(dead_code)]
    pub fn player_commanders(mut self, player_commanders: HashMap<Entity, Vec<Entity>>) -> Self {
        self.player_commanders = player_commanders;

        // Initialize all commanders to command zone
        for commanders in self.player_commanders.values() {
            for &commander in commanders {
                self.commander_zone_status
                    .insert(commander, CommanderZoneLocation::CommandZone);
                self.zone_transition_count.insert(commander, 0);
            }
        }
        self
    }

    /// Set the zone for a commander
    #[allow(dead_code)]
    pub fn set_commander_zone(mut self, commander: Entity, zone: CommanderZoneLocation) -> Self {
        self.commander_zone_status.insert(commander, zone);
        self
    }

    /// Set the transition count for a commander
    #[allow(dead_code)]
    pub fn set_transition_count(mut self, commander: Entity, count: u32) -> Self {
        self.zone_transition_count.insert(commander, count);
        self
    }

    /// Set two commanders as partners
    #[allow(dead_code)]
    pub fn add_partner(mut self, commander1: Entity, commander2: Entity) -> Self {
        self.commander_partners.insert(commander1, commander2);
        self.commander_partners.insert(commander2, commander1);
        self
    }

    /// Set the color identity for a commander
    #[allow(dead_code)]
    pub fn set_color_identity(mut self, commander: Entity, colors: HashSet<ManaColor>) -> Self {
        self.commander_colors.insert(commander, colors);
        self
    }

    /// Build the CommandZoneManager with configured values
    #[allow(dead_code)]
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
