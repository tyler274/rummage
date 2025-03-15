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
    // Serialize all zone contents
    // This will need customization based on how your ZoneManager works
}

/// Serializable commander data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderData {
    // Serialize all commander-specific data
    // This will need customization based on how your CommandZoneManager works
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
        let active_player = index_to_entity[self.game_state.active_player_index];
        let priority_holder = index_to_entity[self.game_state.priority_holder_index];

        let turn_order = VecDeque::from(
            self.game_state
                .turn_order_indices
                .iter()
                .map(|&i| index_to_entity[i])
                .collect::<Vec<_>>(),
        );

        let lands_played = self
            .game_state
            .lands_played
            .iter()
            .map(|(i, count)| (index_to_entity[*i], *count))
            .collect();

        let drawn_this_turn = self
            .game_state
            .drawn_this_turn
            .iter()
            .map(|&i| index_to_entity[i])
            .collect();

        let eliminated_players = self
            .game_state
            .eliminated_players
            .iter()
            .map(|&i| index_to_entity[i])
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
}
