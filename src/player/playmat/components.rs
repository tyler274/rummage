// Add necessary bevy imports
use bevy::prelude::*;
// Add project-specific imports
use crate::game_engine::zones::Zone;

/// Playmat component to identify and query the player\\'s playmat
#[derive(Component, Debug)]
pub struct PlayerPlaymat {
    /// The player this playmat belongs to
    pub player_id: Entity,
    /// The player\\'s index (0-3) for positioning
    pub player_index: usize,
}

/// Zone component for all playmat zones
#[derive(Component, Debug)]
pub struct PlaymatZone {
    /// The player this zone belongs to
    pub player_id: Entity,
    /// The type of zone, using the game engine Zone enum
    pub zone_type: Zone,
}
