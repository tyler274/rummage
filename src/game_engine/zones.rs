use crate::game_engine::PrioritySystem;
use crate::game_engine::stack::{Effect, GameStack};
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

/// The zones in MTG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    CommandZone,
}

/// Event fired when a card changes zones
#[derive(Event, Debug)]
pub struct ZoneChangeEvent {
    /// The card that changed zones
    pub card: Entity,
    /// The player who owns the card
    pub owner: Entity,
    /// The source zone
    pub source: Zone,
    /// The destination zone
    pub destination: Zone,
    /// Whether the card was visible in the source zone
    pub was_visible: bool,
    /// Whether the card is visible in the destination zone
    pub is_visible: bool,
}

/// Manages the game zones and card movements
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
    /// Initialize a new player's zones
    pub fn init_player_zones(&mut self, player: Entity) {
        self.libraries.insert(player, Vec::new());
        self.hands.insert(player, Vec::new());
        self.graveyards.insert(player, Vec::new());
    }

    /// Move a card from one zone to another
    pub fn move_card(
        &mut self,
        card: Entity,
        owner: Entity,
        source: Zone,
        destination: Zone,
    ) -> bool {
        // First, remove the card from its source zone
        let removed = match source {
            Zone::Library => self.remove_from_library(card, owner),
            Zone::Hand => self.remove_from_hand(card, owner),
            Zone::Battlefield => self.remove_from_battlefield(card),
            Zone::Graveyard => self.remove_from_graveyard(card, owner),
            Zone::Stack => true, // Stack is handled by the GameStack resource
            Zone::Exile => self.remove_from_exile(card),
            Zone::CommandZone => self.remove_from_command_zone(card),
        };

        if !removed {
            return false;
        }

        // Then, add the card to its destination zone
        match destination {
            Zone::Library => self.add_to_library(owner, card),
            Zone::Hand => self.add_to_hand(owner, card),
            Zone::Battlefield => self.add_to_battlefield(owner, card),
            Zone::Graveyard => self.add_to_graveyard(owner, card),
            Zone::Stack => {} // Stack is handled by the GameStack resource
            Zone::Exile => self.add_to_exile(card),
            Zone::CommandZone => self.add_to_command_zone(card),
        }

        // Update the card's zone mapping
        self.card_zone_map.insert(card, destination);

        true
    }

    /// Add a card to the library
    pub fn add_to_library(&mut self, owner: Entity, card: Entity) {
        if let Some(library) = self.libraries.get_mut(&owner) {
            library.push(card);
            self.card_zone_map.insert(card, Zone::Library);
        }
    }

    /// Remove a card from the library
    pub fn remove_from_library(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(library) = self.libraries.get_mut(&owner) {
            if let Some(index) = library.iter().position(|&c| c == card) {
                library.remove(index);
                self.card_zone_map.remove(&card);
                return true;
            }
        }
        false
    }

    /// Add a card to the hand
    pub fn add_to_hand(&mut self, owner: Entity, card: Entity) {
        if let Some(hand) = self.hands.get_mut(&owner) {
            hand.push(card);
            self.card_zone_map.insert(card, Zone::Hand);
        }
    }

    /// Remove a card from the hand
    pub fn remove_from_hand(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(hand) = self.hands.get_mut(&owner) {
            if let Some(index) = hand.iter().position(|&c| c == card) {
                hand.remove(index);
                self.card_zone_map.remove(&card);
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
            self.card_zone_map.remove(&card);
            return true;
        }
        false
    }

    /// Add a card to the graveyard
    pub fn add_to_graveyard(&mut self, owner: Entity, card: Entity) {
        if let Some(graveyard) = self.graveyards.get_mut(&owner) {
            graveyard.push(card);
            self.card_zone_map.insert(card, Zone::Graveyard);
        }
    }

    /// Remove a card from the graveyard
    pub fn remove_from_graveyard(&mut self, card: Entity, owner: Entity) -> bool {
        if let Some(graveyard) = self.graveyards.get_mut(&owner) {
            if let Some(index) = graveyard.iter().position(|&c| c == card) {
                graveyard.remove(index);
                self.card_zone_map.remove(&card);
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
            self.card_zone_map.remove(&card);
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
            self.card_zone_map.remove(&card);
            return true;
        }
        false
    }

    /// Get a player's zone by type
    pub fn get_player_zone(&self, player: Entity, zone: Zone) -> Option<&Vec<Entity>> {
        match zone {
            Zone::Library => self.libraries.get(&player),
            Zone::Hand => self.hands.get(&player),
            Zone::Graveyard => self.graveyards.get(&player),
            _ => None,
        }
    }

    /// Get the owner of a card
    pub fn get_card_owner(&self, card: Entity) -> Option<Entity> {
        // Check each player's zones to find the card
        for (player, library) in &self.libraries {
            if library.contains(&card) {
                return Some(*player);
            }
        }

        for (player, hand) in &self.hands {
            if hand.contains(&card) {
                return Some(*player);
            }
        }

        for (player, graveyard) in &self.graveyards {
            if graveyard.contains(&card) {
                return Some(*player);
            }
        }

        // For shared zones, we need another way to track ownership
        None
    }

    /// Get the current zone of a card
    pub fn get_card_zone(&self, card: Entity) -> Option<Zone> {
        self.card_zone_map.get(&card).copied()
    }
}

/// Event for when a permanent enters the battlefield
#[derive(Event)]
pub struct EntersBattlefieldEvent {
    /// The permanent that entered the battlefield
    pub permanent: Entity,
    /// The owner of the permanent
    pub owner: Entity,
    /// Whether the permanent entered tapped
    pub enters_tapped: bool,
}

/// System to handle permanents entering the battlefield
pub fn handle_enters_battlefield(
    _commands: Commands,
    mut enter_events: EventReader<EntersBattlefieldEvent>,
    turn_manager: Option<Res<crate::game_engine::turns::TurnManager>>,
) {
    for event in enter_events.read() {
        let _permanent = event.permanent;

        // Handle "enters the battlefield tapped" effects
        if event.enters_tapped {
            // Add a Tapped component to the entity
            // This would be implemented in a more complete system
        }

        // Only check summoning sickness if turn manager is available
        if let Some(_turn_manager) = turn_manager.as_ref() {
            // If it entered during the controller's first turn or later, it doesn't have summoning sickness
            // This logic would be expanded in a complete implementation
        }
    }
}

/// Setup the zone manager
pub fn setup_zone_manager(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let mut zone_manager = ZoneManager::default();

    // Initialize zones for each player
    for player in player_query.iter() {
        zone_manager.init_player_zones(player);
    }

    commands.insert_resource(zone_manager);
}

/// Register zone-related systems with the app
pub fn register_zone_systems(app: &mut App) {
    app.add_event::<ZoneChangeEvent>()
        .add_event::<EntersBattlefieldEvent>()
        .add_systems(Startup, setup_zone_manager)
        .add_systems(Update, handle_enters_battlefield)
        .add_systems(Update, handle_zone_changes);
}

pub fn handle_zone_changes(
    _commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut events: EventReader<ZoneChangeEvent>,
    _turn_manager: Option<Res<crate::game_engine::turns::TurnManager>>,
) {
    for event in events.read() {
        zone_manager.move_card(event.card, event.owner, event.source, event.destination);
    }
}
