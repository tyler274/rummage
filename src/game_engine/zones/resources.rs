use super::types::Zone;
use bevy::prelude::*;
use std::collections::HashMap;

/// Resource managing game zones and card movement between zones
#[derive(Resource)]
pub struct ZoneManager {
    /// Libraries (decks) for each player
    pub libraries: HashMap<Entity, Vec<Entity>>,

    /// Hands for each player
    pub hands: HashMap<Entity, Vec<Entity>>,

    /// Shared battlefield (all permanents in play)
    pub battlefield: Vec<Entity>,

    /// Graveyards for each player
    pub graveyards: HashMap<Entity, Vec<Entity>>,

    /// Shared exile zone
    pub exile: Vec<Entity>,

    /// Command zone (stores commanders when not in play)
    pub command_zone: Vec<Entity>,

    /// Maps each card to its current zone
    pub card_zone_map: HashMap<Entity, Zone>,
}

impl Default for ZoneManager {
    fn default() -> Self {
        Self {
            libraries: HashMap::new(),
            hands: HashMap::new(),
            battlefield: Vec::new(),
            graveyards: HashMap::new(),
            exile: Vec::new(),
            command_zone: Vec::new(),
            card_zone_map: HashMap::new(),
        }
    }
}

impl ZoneManager {
    /// Initialize zones for a new player
    pub fn init_player_zones(&mut self, player: Entity) {
        self.libraries.entry(player).or_insert_with(Vec::new);
        self.hands.entry(player).or_insert_with(Vec::new);
        self.graveyards.entry(player).or_insert_with(Vec::new);
    }

    /// Move a card from one zone to another
    pub fn move_card(
        &mut self,
        card: Entity,
        owner: Entity,
        source: Zone,
        destination: Zone,
    ) -> bool {
        // Remove from source zone
        let removed = match source {
            Zone::Library => self.remove_from_library(card, owner),
            Zone::Hand => self.remove_from_hand(card, owner),
            Zone::Battlefield => self.remove_from_battlefield(card),
            Zone::Graveyard => self.remove_from_graveyard(card, owner),
            Zone::Exile => self.remove_from_exile(card),
            Zone::CommandZone => self.remove_from_command_zone(card),
            Zone::Stack => true, // Stack items are removed when they resolve
        };

        if !removed {
            return false;
        }

        // Add to destination zone
        match destination {
            Zone::Library => self.add_to_library(owner, card),
            Zone::Hand => self.add_to_hand(owner, card),
            Zone::Battlefield => self.add_to_battlefield(owner, card),
            Zone::Graveyard => self.add_to_graveyard(owner, card),
            Zone::Exile => self.add_to_exile(card),
            Zone::CommandZone => self.add_to_command_zone(card),
            Zone::Stack => {} // Stack items are added via GameStack
        }

        // Update zone mapping
        self.card_zone_map.insert(card, destination);

        true
    }

    /// Add a card to a player's library
    pub fn add_to_library(&mut self, owner: Entity, card: Entity) {
        if let Some(library) = self.libraries.get_mut(&owner) {
            library.push(card);
            self.card_zone_map.insert(card, Zone::Library);
        }
    }

    /// Remove a card from a player's library
    pub fn remove_from_library(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(library) = self.libraries.get_mut(&owner) {
            if let Some(index) = library.iter().position(|&c| c == card) {
                library.remove(index);
                return true;
            }
        }
        false
    }

    /// Add a card to a player's hand
    pub fn add_to_hand(&mut self, owner: Entity, card: Entity) {
        if let Some(hand) = self.hands.get_mut(&owner) {
            hand.push(card);
            self.card_zone_map.insert(card, Zone::Hand);
        }
    }

    /// Remove a card from a player's hand
    pub fn remove_from_hand(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(hand) = self.hands.get_mut(&owner) {
            if let Some(index) = hand.iter().position(|&c| c == card) {
                hand.remove(index);
                return true;
            }
        }
        false
    }

    /// Add a card to the battlefield
    pub fn add_to_battlefield(&mut self, _owner: Entity, card: Entity) {
        self.battlefield.push(card);
        self.card_zone_map.insert(card, Zone::Battlefield);
    }

    /// Remove a card from the battlefield
    pub fn remove_from_battlefield(&mut self, card: Entity) -> bool {
        if let Some(index) = self.battlefield.iter().position(|&c| c == card) {
            self.battlefield.remove(index);
            return true;
        }
        false
    }

    /// Add a card to a player's graveyard
    pub fn add_to_graveyard(&mut self, owner: Entity, card: Entity) {
        if let Some(graveyard) = self.graveyards.get_mut(&owner) {
            graveyard.push(card);
            self.card_zone_map.insert(card, Zone::Graveyard);
        }
    }

    /// Remove a card from a player's graveyard
    pub fn remove_from_graveyard(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(graveyard) = self.graveyards.get_mut(&owner) {
            if let Some(index) = graveyard.iter().position(|&c| c == card) {
                graveyard.remove(index);
                return true;
            }
        }
        false
    }

    /// Add a card to the exile zone
    fn add_to_exile(&mut self, card: Entity) {
        self.exile.push(card);
        self.card_zone_map.insert(card, Zone::Exile);
    }

    /// Remove a card from the exile zone
    fn remove_from_exile(&mut self, card: Entity) -> bool {
        if let Some(index) = self.exile.iter().position(|&c| c == card) {
            self.exile.remove(index);
            return true;
        }
        false
    }

    /// Add a card to the command zone
    fn add_to_command_zone(&mut self, card: Entity) {
        self.command_zone.push(card);
        self.card_zone_map.insert(card, Zone::CommandZone);
    }

    /// Remove a card from the command zone
    fn remove_from_command_zone(&mut self, card: Entity) -> bool {
        if let Some(index) = self.command_zone.iter().position(|&c| c == card) {
            self.command_zone.remove(index);
            return true;
        }
        false
    }

    /// Get a player's zone (library, hand, or graveyard)
    pub fn get_player_zone(&self, player: Entity, zone: Zone) -> Option<&Vec<Entity>> {
        match zone {
            Zone::Library => self.libraries.get(&player),
            Zone::Hand => self.hands.get(&player),
            Zone::Graveyard => self.graveyards.get(&player),
            Zone::Battlefield => Some(&self.battlefield),
            Zone::Exile => Some(&self.exile),
            Zone::CommandZone => Some(&self.command_zone),
            Zone::Stack => None, // Stack is managed separately
        }
    }

    /// Get the owner of a card (if found in a player zone)
    pub fn get_card_owner(&self, card: Entity) -> Option<Entity> {
        // Check libraries
        for (&player, library) in &self.libraries {
            if library.contains(&card) {
                return Some(player);
            }
        }

        // Check hands
        for (&player, hand) in &self.hands {
            if hand.contains(&card) {
                return Some(player);
            }
        }

        // Check graveyards
        for (&player, graveyard) in &self.graveyards {
            if graveyard.contains(&card) {
                return Some(player);
            }
        }

        // For shared zones, we'd need to track ownership separately
        None
    }

    /// Get the current zone of a card
    pub fn get_card_zone(&self, card: Entity) -> Option<Zone> {
        self.card_zone_map.get(&card).copied()
    }
}
