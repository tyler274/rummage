use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Card zone types
pub type ZoneType = crate::game_engine::zones::types::Zone;

/// Serializable zone data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneData {
    // Maps player indices to their libraries
    pub libraries: HashMap<usize, Vec<usize>>,

    // Maps player indices to their hands
    pub hands: HashMap<usize, Vec<usize>>,

    // Shared battlefield (all permanents in play)
    pub battlefield: Vec<usize>,

    // Maps player indices to their graveyards
    pub graveyards: HashMap<usize, Vec<usize>>,

    // Shared exile zone
    pub exile: Vec<usize>,

    // Command zone
    pub command_zone: Vec<usize>,

    // Maps card indices to their current zone
    pub card_zone_map: HashMap<usize, ZoneType>,
}

impl Default for ZoneData {
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

/// Serializable card data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub id: usize,
    pub card_name: String,
    pub card_type: String,
    pub owner_index: usize,
    pub controller_index: usize,
    pub zone: ZoneType,
    pub tapped: bool,
    pub counters: HashMap<String, i32>,
}

impl Default for CardData {
    fn default() -> Self {
        Self {
            id: 0,
            card_name: String::new(),
            card_type: String::new(),
            owner_index: 0,
            controller_index: 0,
            zone: ZoneType::Library,
            tapped: false,
            counters: HashMap::new(),
        }
    }
} 