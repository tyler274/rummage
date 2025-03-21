use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game_engine::zones::Zone;

/// Marker component that identifies an entity as a card
#[derive(Component, Debug, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CardEntity;

/// Component to track which zone a card is in
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize, PartialEq, Eq)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CardZone {
    /// The current zone the card is in
    pub zone: Zone,
    /// The owner of the zone (for player-specific zones)
    pub zone_owner: Option<Entity>,
}

impl CardZone {
    /// Create a new card zone component
    #[allow(dead_code)]
    pub fn new(zone: Zone, zone_owner: Option<Entity>) -> Self {
        Self { zone, zone_owner }
    }

    /// Check if the card is in a specific zone
    #[allow(dead_code)]
    pub fn is_in_zone(&self, zone: Zone) -> bool {
        self.zone == zone
    }

    /// Update the zone
    pub fn set_zone(&mut self, zone: Zone, zone_owner: Option<Entity>) {
        self.zone = zone;
        self.zone_owner = zone_owner;
    }

    // Constants for easier access in tests
    #[allow(dead_code)]
    pub const HAND: Self = Self {
        zone: Zone::Hand,
        zone_owner: None,
    };
    #[allow(dead_code)]
    pub const BATTLEFIELD: Self = Self {
        zone: Zone::Battlefield,
        zone_owner: None,
    };
    #[allow(dead_code)]
    pub const GRAVEYARD: Self = Self {
        zone: Zone::Graveyard,
        zone_owner: None,
    };
}

/// Component to track the owner of a card (the player whose deck it came from)
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CardOwner(pub Entity);

impl CardOwner {
    /// Create a new card owner component
    #[allow(dead_code)]
    pub fn new(player: Entity) -> Self {
        Self(player)
    }
}

impl From<Entity> for CardOwner {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}
