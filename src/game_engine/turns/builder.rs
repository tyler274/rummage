use crate::game_engine::phase::types::{BeginningStep, Phase};
use crate::game_engine::turns::{TurnEventTracker, TurnManager};
use bevy::prelude::*;

/// Builder for TurnManager to enable chainable construction
#[derive(Debug, Clone)]
pub struct TurnManagerBuilder {
    active_player: Entity,
    player_order: Vec<Entity>,
    active_player_index: usize,
    turn_number: u32,
    eliminated_players: Vec<Entity>,
    current_phase: Phase,
}

impl TurnManagerBuilder {
    /// Creates a new builder with default values
    pub fn new() -> Self {
        Self {
            // Default value needed, will be overridden when players are added
            active_player: Entity::from_raw(0),
            player_order: Vec::new(),
            active_player_index: 0,
            turn_number: 1,
            eliminated_players: Vec::new(),
            current_phase: Phase::Beginning(BeginningStep::Untap),
        }
    }

    /// Sets the active player
    pub fn active_player(mut self, active_player: Entity) -> Self {
        self.active_player = active_player;
        self
    }

    /// Sets the player order
    pub fn player_order(mut self, player_order: Vec<Entity>) -> Self {
        self.player_order = player_order;
        self
    }

    /// Sets the active player index
    pub fn active_player_index(mut self, active_player_index: usize) -> Self {
        self.active_player_index = active_player_index;
        self
    }

    /// Sets the turn number
    pub fn turn_number(mut self, turn_number: u32) -> Self {
        self.turn_number = turn_number;
        self
    }

    /// Sets the eliminated players
    pub fn eliminated_players(mut self, eliminated_players: Vec<Entity>) -> Self {
        self.eliminated_players = eliminated_players;
        self
    }

    /// Sets the current phase
    pub fn current_phase(mut self, current_phase: Phase) -> Self {
        self.current_phase = current_phase;
        self
    }

    /// Builds the TurnManager with the configured values
    pub fn build(self) -> TurnManager {
        TurnManager {
            active_player: self.active_player,
            player_order: self.player_order,
            active_player_index: self.active_player_index,
            turn_number: self.turn_number,
            eliminated_players: self.eliminated_players,
            current_phase: self.current_phase,
        }
    }
}

/// Builder for TurnEventTracker to enable chainable construction
#[derive(Debug, Clone)]
pub struct TurnEventTrackerBuilder {
    turn_start_processed: bool,
    turn_end_processed: bool,
    untap_step_processed: bool,
    last_processed_turn: u32,
}

impl TurnEventTrackerBuilder {
    /// Creates a new builder with default values
    pub fn new() -> Self {
        Self {
            turn_start_processed: false,
            turn_end_processed: false,
            untap_step_processed: false,
            last_processed_turn: 0,
        }
    }

    /// Sets whether the turn start has been processed
    pub fn turn_start_processed(mut self, processed: bool) -> Self {
        self.turn_start_processed = processed;
        self
    }

    /// Sets whether the turn end has been processed
    pub fn turn_end_processed(mut self, processed: bool) -> Self {
        self.turn_end_processed = processed;
        self
    }

    /// Sets whether the untap step has been processed
    pub fn untap_step_processed(mut self, processed: bool) -> Self {
        self.untap_step_processed = processed;
        self
    }

    /// Sets the last processed turn number
    pub fn last_processed_turn(mut self, turn_number: u32) -> Self {
        self.last_processed_turn = turn_number;
        self
    }

    /// Builds the TurnEventTracker with the configured values
    pub fn build(self) -> TurnEventTracker {
        TurnEventTracker {
            turn_start_processed: self.turn_start_processed,
            turn_end_processed: self.turn_end_processed,
            untap_step_processed: self.untap_step_processed,
            last_processed_turn: self.last_processed_turn,
        }
    }
}
