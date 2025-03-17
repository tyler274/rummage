use crate::game_engine::state::GameState;
use crate::mana::ManaPool;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Complete game save data
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GameSaveData {
    pub game_state: GameStateData,
    pub players: Vec<PlayerData>,
    pub zones: ZoneData,
    pub commanders: CommanderData,
    pub save_version: String,
}

impl Default for GameSaveData {
    fn default() -> Self {
        Self {
            game_state: GameStateData::default(),
            players: Vec::new(),
            zones: ZoneData::default(),
            commanders: CommanderData::default(),
            save_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Serializable game state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
    pub lands_played: Vec<(usize, u32)>,
    pub main_phase_action_taken: bool,
    pub drawn_this_turn: Vec<usize>,
    pub eliminated_players: Vec<usize>,
    pub use_commander_damage: bool,
    pub commander_damage_threshold: u32,
    pub starting_life: i32,
}

impl Default for GameStateData {
    fn default() -> Self {
        Self {
            turn_number: 1,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(),
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        }
    }
}

/// Serializable player data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: usize,
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub player_index: usize,
}

/// Serializable zone data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneData {
    // Maps player indices to their libraries
    pub libraries: std::collections::HashMap<usize, Vec<usize>>,

    // Maps player indices to their hands
    pub hands: std::collections::HashMap<usize, Vec<usize>>,

    // Shared battlefield (all permanents in play)
    pub battlefield: Vec<usize>,

    // Maps player indices to their graveyards
    pub graveyards: std::collections::HashMap<usize, Vec<usize>>,

    // Shared exile zone
    pub exile: Vec<usize>,

    // Command zone
    pub command_zone: Vec<usize>,

    // Maps card indices to their current zone
    pub card_zone_map: std::collections::HashMap<usize, crate::game_engine::zones::types::Zone>,
}

impl Default for ZoneData {
    fn default() -> Self {
        Self {
            libraries: std::collections::HashMap::new(),
            hands: std::collections::HashMap::new(),
            battlefield: Vec::new(),
            graveyards: std::collections::HashMap::new(),
            exile: Vec::new(),
            command_zone: Vec::new(),
            card_zone_map: std::collections::HashMap::new(),
        }
    }
}

/// Serializable commander data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderData {
    // Maps player indices to their commander indices
    pub player_commanders: std::collections::HashMap<usize, Vec<usize>>,

    // Maps commander indices to their current zone
    pub commander_zone_status: std::collections::HashMap<
        usize,
        crate::game_engine::commander::components::CommanderZoneLocation,
    >,

    // Tracks how many times a commander has moved zones
    pub zone_transition_count: std::collections::HashMap<usize, u32>,
}

impl Default for CommanderData {
    fn default() -> Self {
        Self {
            player_commanders: std::collections::HashMap::new(),
            commander_zone_status: std::collections::HashMap::new(),
            zone_transition_count: std::collections::HashMap::new(),
        }
    }
}

/// Information about a single save file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveInfo {
    pub slot_name: String,
    pub timestamp: u64,
    pub description: String,
    pub turn_number: u32,
    pub player_count: usize,
}

impl GameSaveData {
    /// Convert serialized game state data back into a GameState resource
    pub fn to_game_state(&self, index_to_entity: &[Entity]) -> GameState {
        // Add safety checks to handle empty entity lists
        let active_player = if !index_to_entity.is_empty()
            && self.game_state.active_player_index < index_to_entity.len()
        {
            index_to_entity[self.game_state.active_player_index]
        } else {
            // Return a default Entity if there are no entities
            Entity::from_raw(0)
        };

        let priority_holder = if !index_to_entity.is_empty()
            && self.game_state.priority_holder_index < index_to_entity.len()
        {
            index_to_entity[self.game_state.priority_holder_index]
        } else {
            // Return a default Entity if there are no entities
            Entity::from_raw(0)
        };

        let turn_order = VecDeque::from(
            self.game_state
                .turn_order_indices
                .iter()
                .filter_map(|&i| {
                    if i < index_to_entity.len() {
                        Some(index_to_entity[i])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );

        let lands_played = self
            .game_state
            .lands_played
            .iter()
            .filter_map(|(i, count)| {
                if *i < index_to_entity.len() {
                    Some((index_to_entity[*i], *count))
                } else {
                    None
                }
            })
            .collect();

        let drawn_this_turn = self
            .game_state
            .drawn_this_turn
            .iter()
            .filter_map(|&i| {
                if i < index_to_entity.len() {
                    Some(index_to_entity[i])
                } else {
                    None
                }
            })
            .collect();

        let eliminated_players = self
            .game_state
            .eliminated_players
            .iter()
            .filter_map(|&i| {
                if i < index_to_entity.len() {
                    Some(index_to_entity[i])
                } else {
                    None
                }
            })
            .collect();

        GameState {
            turn_number: self.game_state.turn_number,
            active_player,
            priority_holder,
            turn_order,
            lands_played,
            main_phase_action_taken: self.game_state.main_phase_action_taken,
            drawn_this_turn,
            state_based_actions_performed: false, // Reset this flag
            eliminated_players,
            use_commander_damage: self.game_state.use_commander_damage,
            commander_damage_threshold: self.game_state.commander_damage_threshold,
            starting_life: self.game_state.starting_life,
        }
    }

    /// Create serializable game state data from a GameState resource
    pub fn from_game_state(
        game_state: &GameState,
        entity_to_index: &std::collections::HashMap<Entity, usize>,
        players: Vec<PlayerData>,
    ) -> Self {
        // Transform GameState to serializable GameStateData
        let active_player_index = *entity_to_index.get(&game_state.active_player).unwrap_or(&0);
        let priority_holder_index = *entity_to_index
            .get(&game_state.priority_holder)
            .unwrap_or(&0);

        let turn_order_indices = game_state
            .turn_order
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let lands_played = game_state
            .lands_played
            .iter()
            .filter_map(|(e, count)| entity_to_index.get(e).map(|i| (*i, *count)))
            .collect();

        let drawn_this_turn = game_state
            .drawn_this_turn
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let eliminated_players = game_state
            .eliminated_players
            .iter()
            .filter_map(|e| entity_to_index.get(e).cloned())
            .collect();

        let game_state_data = GameStateData {
            turn_number: game_state.turn_number,
            active_player_index,
            priority_holder_index,
            turn_order_indices,
            lands_played,
            main_phase_action_taken: game_state.main_phase_action_taken,
            drawn_this_turn,
            eliminated_players,
            use_commander_damage: game_state.use_commander_damage,
            commander_damage_threshold: game_state.commander_damage_threshold,
            starting_life: game_state.starting_life,
        };

        // TODO: Extract and serialize zone data
        let zone_data = ZoneData {
            // Implement based on your ZoneManager structure
        };

        // TODO: Extract and serialize commander data
        let commander_data = CommanderData {
            // Implement based on your CommandZoneManager structure  
        };

        Self {
            game_state: game_state_data,
            players,
            zones: zone_data,
            commanders: commander_data,
            save_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Extract zone data from ZoneManager and convert entity references to indices
    pub fn from_zone_manager(
        zone_manager: &crate::game_engine::zones::ZoneManager,
        entity_to_index: &std::collections::HashMap<Entity, usize>,
    ) -> ZoneData {
        let mut zone_data = ZoneData::default();

        // Process libraries
        for (player, cards) in &zone_manager.libraries {
            if let Some(&player_idx) = entity_to_index.get(player) {
                let card_indices: Vec<usize> = cards
                    .iter()
                    .filter_map(|card| entity_to_index.get(card).cloned())
                    .collect();
                zone_data.libraries.insert(player_idx, card_indices);
            }
        }

        // Process hands
        for (player, cards) in &zone_manager.hands {
            if let Some(&player_idx) = entity_to_index.get(player) {
                let card_indices: Vec<usize> = cards
                    .iter()
                    .filter_map(|card| entity_to_index.get(card).cloned())
                    .collect();
                zone_data.hands.insert(player_idx, card_indices);
            }
        }

        // Process battlefield
        zone_data.battlefield = zone_manager
            .battlefield
            .iter()
            .filter_map(|card| entity_to_index.get(card).cloned())
            .collect();

        // Process graveyards
        for (player, cards) in &zone_manager.graveyards {
            if let Some(&player_idx) = entity_to_index.get(player) {
                let card_indices: Vec<usize> = cards
                    .iter()
                    .filter_map(|card| entity_to_index.get(card).cloned())
                    .collect();
                zone_data.graveyards.insert(player_idx, card_indices);
            }
        }

        // Process exile
        zone_data.exile = zone_manager
            .exile
            .iter()
            .filter_map(|card| entity_to_index.get(card).cloned())
            .collect();

        // Process command zone
        zone_data.command_zone = zone_manager
            .command_zone
            .iter()
            .filter_map(|card| entity_to_index.get(card).cloned())
            .collect();

        // Process card_zone_map
        for (card, zone) in &zone_manager.card_zone_map {
            if let Some(&card_idx) = entity_to_index.get(card) {
                zone_data.card_zone_map.insert(card_idx, *zone);
            }
        }

        zone_data
    }

    /// Extract commander data from CommandZoneManager and convert entity references to indices
    pub fn from_commander_manager(
        commander_manager: &crate::game_engine::commander::CommandZoneManager,
        entity_to_index: &std::collections::HashMap<Entity, usize>,
    ) -> CommanderData {
        let mut commander_data = CommanderData::default();

        // Process player commanders
        for (player, commanders) in &commander_manager.player_commanders {
            if let Some(&player_idx) = entity_to_index.get(player) {
                let commander_indices: Vec<usize> = commanders
                    .iter()
                    .filter_map(|card| entity_to_index.get(card).cloned())
                    .collect();
                commander_data
                    .player_commanders
                    .insert(player_idx, commander_indices);
            }
        }

        // Process commander zone status
        for (commander, zone) in &commander_manager.commander_zone_status {
            if let Some(&commander_idx) = entity_to_index.get(commander) {
                commander_data
                    .commander_zone_status
                    .insert(commander_idx, *zone);
            }
        }

        // Process zone transition count
        for (commander, count) in &commander_manager.zone_transition_count {
            if let Some(&commander_idx) = entity_to_index.get(commander) {
                commander_data
                    .zone_transition_count
                    .insert(commander_idx, *count);
            }
        }

        commander_data
    }

    /// Restore ZoneManager from saved data
    pub fn to_zone_manager(
        &self,
        index_to_entity: &[Entity],
    ) -> crate::game_engine::zones::ZoneManager {
        let mut zone_manager = crate::game_engine::zones::ZoneManager::default();

        // Restore libraries
        for (player_idx, cards) in &self.zones.libraries {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                let cards_vec: Vec<Entity> = cards
                    .iter()
                    .filter_map(|&card_idx| {
                        if card_idx < index_to_entity.len() {
                            Some(index_to_entity[card_idx])
                        } else {
                            None
                        }
                    })
                    .collect();
                zone_manager.libraries.insert(player, cards_vec);
            }
        }

        // Restore hands
        for (player_idx, cards) in &self.zones.hands {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                let cards_vec: Vec<Entity> = cards
                    .iter()
                    .filter_map(|&card_idx| {
                        if card_idx < index_to_entity.len() {
                            Some(index_to_entity[card_idx])
                        } else {
                            None
                        }
                    })
                    .collect();
                zone_manager.hands.insert(player, cards_vec);
            }
        }

        // Restore battlefield
        zone_manager.battlefield = self
            .zones
            .battlefield
            .iter()
            .filter_map(|&card_idx| {
                if card_idx < index_to_entity.len() {
                    Some(index_to_entity[card_idx])
                } else {
                    None
                }
            })
            .collect();

        // Restore graveyards
        for (player_idx, cards) in &self.zones.graveyards {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                let cards_vec: Vec<Entity> = cards
                    .iter()
                    .filter_map(|&card_idx| {
                        if card_idx < index_to_entity.len() {
                            Some(index_to_entity[card_idx])
                        } else {
                            None
                        }
                    })
                    .collect();
                zone_manager.graveyards.insert(player, cards_vec);
            }
        }

        // Restore exile
        zone_manager.exile = self
            .zones
            .exile
            .iter()
            .filter_map(|&card_idx| {
                if card_idx < index_to_entity.len() {
                    Some(index_to_entity[card_idx])
                } else {
                    None
                }
            })
            .collect();

        // Restore command zone
        zone_manager.command_zone = self
            .zones
            .command_zone
            .iter()
            .filter_map(|&card_idx| {
                if card_idx < index_to_entity.len() {
                    Some(index_to_entity[card_idx])
                } else {
                    None
                }
            })
            .collect();

        // Restore card zone map
        for (card_idx, zone) in &self.zones.card_zone_map {
            if *card_idx < index_to_entity.len() {
                let card = index_to_entity[*card_idx];
                zone_manager.card_zone_map.insert(card, *zone);
            }
        }

        zone_manager
    }

    /// Restore CommandZoneManager from saved data
    pub fn to_commander_manager(
        &self,
        index_to_entity: &[Entity],
    ) -> crate::game_engine::commander::CommandZoneManager {
        let mut commander_manager = crate::game_engine::commander::CommandZoneManager::default();

        // Restore player commanders
        for (player_idx, commanders) in &self.commanders.player_commanders {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                let commanders_vec: Vec<Entity> = commanders
                    .iter()
                    .filter_map(|&card_idx| {
                        if card_idx < index_to_entity.len() {
                            Some(index_to_entity[card_idx])
                        } else {
                            None
                        }
                    })
                    .collect();
                commander_manager
                    .player_commanders
                    .insert(player, commanders_vec);
            }
        }

        // Restore commander zone status
        for (card_idx, zone) in &self.commanders.commander_zone_status {
            if *card_idx < index_to_entity.len() {
                let card = index_to_entity[*card_idx];
                commander_manager.commander_zone_status.insert(card, *zone);
            }
        }

        // Restore zone transition count
        for (card_idx, count) in &self.commanders.zone_transition_count {
            if *card_idx < index_to_entity.len() {
                let card = index_to_entity[*card_idx];
                commander_manager.zone_transition_count.insert(card, *count);
            }
        }

        commander_manager
    }
}
