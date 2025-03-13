use bevy::prelude::*;

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

/// Component marking an entity as belonging to a specific zone
#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct ZoneMarker {
    /// The type of zone the entity is in
    pub zone_type: Zone,
    /// The owner of the zone (if applicable)
    pub owner: Option<Entity>,
}
