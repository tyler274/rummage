use crate::game_engine::save::resources::ReplayAction;
use crate::game_engine::state::GameState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

use super::{CommanderData, GameStateData, PlayerData, ZoneData};

/// Complete game save data
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct GameSaveData {
    pub game_state: GameStateData,
    pub players: Vec<PlayerData>,
    pub zones: ZoneData,
    pub commanders: CommanderData,
    pub save_version: String,
    pub game_id: String,
    pub turn_number: u32,
    pub phase: String,
    pub active_player: Option<usize>,
    pub priority_player: Option<usize>,
    pub replay_history: Vec<ReplayAction>,
    pub board_snapshot: Option<String>,
    pub timestamp: u64,
}

impl Default for GameSaveData {
    fn default() -> Self {
        Self {
            game_state: GameStateData::default(),
            players: Vec::new(),
            zones: ZoneData::default(),
            commanders: CommanderData::default(),
            save_version: env!("CARGO_PKG_VERSION").to_string(),
            game_id: String::new(),
            turn_number: 1,
            phase: String::new(),
            active_player: None,
            priority_player: None,
            replay_history: Vec::new(),
            board_snapshot: None,
            timestamp: 0,
        }
    }
}

/// Builder for GameSaveData
#[allow(dead_code)]
#[derive(Default)]
pub struct GameSaveDataBuilder {
    game_state: GameStateData,
    players: Vec<PlayerData>,
    zones: ZoneData,
    commanders: CommanderData,
    save_version: String,
    game_id: String,
    turn_number: u32,
    phase: String,
    active_player: Option<usize>,
    priority_player: Option<usize>,
    replay_history: Vec<ReplayAction>,
    board_snapshot: Option<String>,
    timestamp: u64,
}

#[allow(dead_code)]
impl GameSaveDataBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            save_version: env!("CARGO_PKG_VERSION").to_string(),
            turn_number: 1,
            ..Default::default()
        }
    }

    /// Set the game state data
    pub fn game_state(mut self, game_state: GameStateData) -> Self {
        self.game_state = game_state;
        self
    }

    /// Set the players data
    pub fn players(mut self, players: Vec<PlayerData>) -> Self {
        self.players = players;
        self
    }

    /// Set the zones data
    pub fn zones(mut self, zones: ZoneData) -> Self {
        self.zones = zones;
        self
    }

    /// Set the commanders data
    pub fn commanders(mut self, commanders: CommanderData) -> Self {
        self.commanders = commanders;
        self
    }

    /// Set the save version
    pub fn save_version(mut self, save_version: String) -> Self {
        self.save_version = save_version;
        self
    }

    /// Set the game ID
    #[allow(dead_code)]
    pub fn game_id(mut self, game_id: String) -> Self {
        self.game_id = game_id;
        self
    }

    /// Set the turn number
    #[allow(dead_code)]
    pub fn turn_number(mut self, turn_number: u32) -> Self {
        self.turn_number = turn_number;
        self
    }

    /// Set the current phase
    #[allow(dead_code)]
    pub fn phase(mut self, phase: String) -> Self {
        self.phase = phase;
        self
    }

    /// Set the active player
    #[allow(dead_code)]
    pub fn active_player(mut self, active_player: Option<usize>) -> Self {
        self.active_player = active_player;
        self
    }

    /// Set the priority player
    #[allow(dead_code)]
    pub fn priority_player(mut self, priority_player: Option<usize>) -> Self {
        self.priority_player = priority_player;
        self
    }

    /// Set the replay history
    #[allow(dead_code)]
    pub fn replay_history(mut self, replay_history: Vec<ReplayAction>) -> Self {
        self.replay_history = replay_history;
        self
    }

    /// Set the board snapshot
    #[allow(dead_code)]
    pub fn board_snapshot(mut self, board_snapshot: Option<String>) -> Self {
        self.board_snapshot = board_snapshot;
        self
    }

    /// Set the timestamp
    #[allow(dead_code)]
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Build the GameSaveData instance
    pub fn build(self) -> GameSaveData {
        GameSaveData {
            game_state: self.game_state,
            players: self.players,
            zones: self.zones,
            commanders: self.commanders,
            save_version: self.save_version,
            game_id: self.game_id,
            turn_number: self.turn_number,
            phase: self.phase,
            active_player: self.active_player,
            priority_player: self.priority_player,
            replay_history: self.replay_history,
            board_snapshot: self.board_snapshot,
            timestamp: self.timestamp,
        }
    }
}

impl GameSaveData {
    /// Create a new builder for GameSaveData
    #[allow(dead_code)]
    pub fn builder() -> GameSaveDataBuilder {
        GameSaveDataBuilder::new()
    }

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
        entity_to_index: &HashMap<Entity, usize>,
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
            .filter_map(|(e, count)| Some((entity_to_index.get(e)?.clone(), *count)))
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

        // Build a basic save data object
        Self {
            game_state: game_state_data,
            players,
            zones: ZoneData::default(),
            commanders: CommanderData::default(),
            save_version: env!("CARGO_PKG_VERSION").to_string(),
            game_id: String::new(),
            turn_number: game_state.turn_number,
            phase: String::new(), // Would be filled in by the current phase
            active_player: Some(active_player_index),
            priority_player: Some(priority_holder_index),
            replay_history: Vec::new(),
            board_snapshot: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Extract zone data from ZoneManager and convert entity references to indices
    pub fn from_zone_manager(
        zone_manager: &crate::game_engine::zones::ZoneManager,
        entity_to_index: &HashMap<Entity, usize>,
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
        entity_to_index: &HashMap<Entity, usize>,
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

        // Initialize player zones first
        for (player_idx, _) in &self.zones.hands {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                zone_manager.init_player_zones(player);
            }
        }

        for (player_idx, _) in &self.zones.libraries {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                zone_manager.init_player_zones(player);
            }
        }

        for (player_idx, _) in &self.zones.graveyards {
            if *player_idx < index_to_entity.len() {
                let player = index_to_entity[*player_idx];
                zone_manager.init_player_zones(player);
            }
        }

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
                // Use clear_hand first to ensure there are no stale cards
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
        for (commander_idx, zone) in &self.commanders.commander_zone_status {
            if *commander_idx < index_to_entity.len() {
                let commander = index_to_entity[*commander_idx];
                commander_manager
                    .commander_zone_status
                    .insert(commander, *zone);
            }
        }

        // Restore zone transition count
        for (commander_idx, count) in &self.commanders.zone_transition_count {
            if *commander_idx < index_to_entity.len() {
                let commander = index_to_entity[*commander_idx];
                commander_manager
                    .zone_transition_count
                    .insert(commander, *count);
            }
        }

        commander_manager
    }
}

/// Convert entity to index in a serializable format
#[allow(dead_code)]
pub fn convert_entity_to_index(
    entities: Vec<Entity>,
    world: &World,
) -> (HashMap<Entity, usize>, Vec<PlayerData>) {
    let mut entity_to_index = HashMap::new();
    let mut players = Vec::new();

    // Create mapping from Entity to usize index
    for (i, entity) in entities.iter().enumerate() {
        entity_to_index.insert(*entity, i);

        // If entity has a Player component, extract player data
        if let Some(player) = world.get::<crate::player::Player>(*entity) {
            let player_data = PlayerData {
                id: i,
                name: player.name.clone(),
                life: player.life,
                mana_pool: player.mana_pool.clone(),
                player_index: player.player_index,
            };
            players.push(player_data);
        }
    }

    (entity_to_index, players)
}

/// Convert indices back to entities
#[allow(dead_code)]
pub fn convert_index_to_entity(save_data: &GameSaveData, world: &mut World) -> Vec<Entity> {
    let mut index_to_entity = Vec::new();

    // First we restore player entities
    for player_data in &save_data.players {
        // Spawn a new entity for this player
        let entity = world
            .spawn(crate::player::Player {
                name: player_data.name.clone(),
                life: player_data.life,
                mana_pool: player_data.mana_pool.clone(),
                player_index: player_data.player_index,
            })
            .id();

        // Make sure our index_to_entity vector is large enough
        // This handles the case where indices might not be sequential
        if player_data.id >= index_to_entity.len() {
            index_to_entity.resize(player_data.id + 1, Entity::from_raw(0));
        }

        // Store the entity at the correct index
        index_to_entity[player_data.id] = entity;
    }

    index_to_entity
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
