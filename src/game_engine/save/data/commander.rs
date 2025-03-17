use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Commander zone location type
pub type CommanderZoneLocation = crate::game_engine::commander::components::CommanderZoneLocation;

/// Serializable commander data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderData {
    // Maps player indices to their commander indices
    pub player_commanders: HashMap<usize, Vec<usize>>,

    // Maps commander indices to their current zone
    pub commander_zone_status: HashMap<usize, CommanderZoneLocation>,

    // Tracks how many times a commander has moved zones
    pub zone_transition_count: HashMap<usize, u32>,
}

impl Default for CommanderData {
    fn default() -> Self {
        Self {
            player_commanders: HashMap::new(),
            commander_zone_status: HashMap::new(),
            zone_transition_count: HashMap::new(),
        }
    }
}

/// Serializable data for a commander pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderPairData {
    pub player_index: usize,
    pub commander_indices: Vec<usize>,
    pub partner_commander: bool,
}

impl Default for CommanderPairData {
    fn default() -> Self {
        Self {
            player_index: 0,
            commander_indices: Vec::new(),
            partner_commander: false,
        }
    }
}
