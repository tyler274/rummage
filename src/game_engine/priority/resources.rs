use crate::game_engine::Phase;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

/// System for tracking priority in the game
#[derive(Resource)]
pub struct PrioritySystem {
    /// The active player (player whose turn it is)
    pub active_player: Entity,

    /// The player who currently has priority
    pub priority_player: Entity,

    /// Whether each player has passed priority this round
    pub has_priority_passed: HashMap<Entity, bool>,

    /// Whether all players have passed priority in succession
    pub all_players_passed: bool,

    /// Player order for priority passing
    pub player_order: Vec<Entity>,

    /// Current index in the player order
    pub priority_index: usize,

    /// Whether the stack is currently empty
    pub stack_is_empty: bool,

    /// Current phase and step
    pub current_phase: Phase,

    /// Whether we're waiting for a response to a triggered ability or similar
    pub waiting_for_response: bool,

    /// Optional timeout for responses
    pub response_timeout: Option<Instant>,

    /// Players who need to make simultaneous decisions
    pub simultaneous_decision_players: Vec<Entity>,

    /// Tracks the last phase we completed a full priority round for
    pub last_processed_phase: Option<Phase>,

    /// Tracks the last turn number we processed
    pub last_processed_turn: u32,

    /// Decision timeouts for simultaneous decisions
    pub decision_timeouts: HashMap<Entity, std::time::Duration>,
}

impl Default for PrioritySystem {
    fn default() -> Self {
        Self {
            active_player: Entity::PLACEHOLDER,
            priority_player: Entity::PLACEHOLDER,
            has_priority_passed: HashMap::new(),
            all_players_passed: false,
            player_order: Vec::new(),
            priority_index: 0,
            stack_is_empty: true,
            current_phase: Phase::default(),
            waiting_for_response: false,
            response_timeout: None,
            simultaneous_decision_players: Vec::new(),
            last_processed_phase: None,
            last_processed_turn: 0,
            decision_timeouts: HashMap::new(),
        }
    }
}

impl PrioritySystem {
    /// Initialize the priority system with the list of players and the active player
    pub fn initialize(&mut self, players: &[Entity], active_player: Entity) {
        self.active_player = active_player;
        self.priority_player = active_player; // Active player gets priority first
        self.player_order = players.to_vec();

        // Set starting index to the active player
        if let Some(index) = self.player_order.iter().position(|&p| p == active_player) {
            self.priority_index = index;
        }

        // Reset passing status for all players
        self.has_priority_passed.clear();
        for &player in players {
            self.has_priority_passed.insert(player, false);
        }

        self.all_players_passed = false;
    }

    /// Pass priority to the next player in turn order
    pub fn pass_priority(&mut self) {
        if self.player_order.is_empty() {
            return;
        }

        // Move to the next player in turn order
        self.priority_index = (self.priority_index + 1) % self.player_order.len();
        self.priority_player = self.player_order[self.priority_index];

        // Check if we've gone full circle
        if self.priority_player == self.active_player {
            // If we've gone all the way around and all players have passed, mark complete
            let all_passed = self.has_priority_passed.values().all(|&passed| passed);
            if all_passed {
                self.all_players_passed = true;
            }
        }
    }

    /// Reset after a stack action has resolved
    pub fn reset_after_stack_action(&mut self, players: &[Entity], active_player: Entity) {
        self.active_player = active_player;
        self.priority_player = active_player;

        // Reset pass status
        self.has_priority_passed.clear();
        for &player in players {
            self.has_priority_passed.insert(player, false);
        }

        self.all_players_passed = false;

        // Reset index to active player
        if let Some(index) = self.player_order.iter().position(|&p| p == active_player) {
            self.priority_index = index;
        }
    }

    /// Set whether the stack is empty
    pub fn set_stack_empty(&mut self, is_empty: bool) {
        self.stack_is_empty = is_empty;
    }

    /// Check if this phase has already been processed this turn
    pub fn has_processed_phase(&self, current_phase: Phase, turn_number: u32) -> bool {
        self.last_processed_phase == Some(current_phase) && self.last_processed_turn == turn_number
    }

    /// Mark the current phase as processed for this turn
    pub fn mark_phase_processed(&mut self, current_phase: Phase, turn_number: u32) {
        self.last_processed_phase = Some(current_phase);
        self.last_processed_turn = turn_number;
    }

    /// Check if the player currently has priority
    pub fn has_priority(&self, player: Entity) -> bool {
        self.priority_player == player
    }

    /// Reset the passing status for all players
    pub fn reset_passing_status(&mut self) {
        for (_, passed) in self.has_priority_passed.iter_mut() {
            *passed = false;
        }
        self.all_players_passed = false;
    }

    /// Check if a full priority round is complete
    pub fn priority_round_complete(&self) -> bool {
        self.all_players_passed
    }

    /// Check if a player has passed priority
    pub fn has_passed(&self, player: Entity) -> bool {
        self.has_priority_passed
            .get(&player)
            .copied()
            .unwrap_or(false)
    }

    /// Set a decision timeout for a player
    pub fn set_decision_timeout(&mut self, player: Entity, duration: std::time::Duration) {
        self.decision_timeouts.insert(player, duration);
    }

    /// Add a player to the simultaneous decision list
    pub fn add_simultaneous_decision_player(&mut self, player: Entity) {
        if !self.simultaneous_decision_players.contains(&player) {
            self.simultaneous_decision_players.push(player);
        }
    }

    /// Remove a player from the simultaneous decision list
    pub fn remove_simultaneous_decision_player(&mut self, player: Entity) {
        self.simultaneous_decision_players.retain(|&p| p != player);
    }
}
