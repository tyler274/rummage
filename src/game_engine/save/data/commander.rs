use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Commander zone location type
pub type CommanderZoneLocation = crate::game_engine::commander::components::CommanderZoneLocation;

/// Serializable commander data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CommanderData {
    // Maps player indices to their commander indices
    pub player_commanders: HashMap<usize, Vec<usize>>,

    // Maps commander indices to their current zone
    pub commander_zone_status: HashMap<usize, CommanderZoneLocation>,

    // Tracks how many times a commander has moved zones
    pub zone_transition_count: HashMap<usize, u32>,
}


/// Serializable data for a commander pair
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CommanderPairData {
    pub player_index: usize,
    pub commander_indices: Vec<usize>,
    pub partner_commander: bool,
}

