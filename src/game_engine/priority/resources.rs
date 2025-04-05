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
    #[allow(dead_code)]
    pub current_phase: Phase,

    /// Whether we're waiting for a response to a triggered ability or similar
    #[allow(dead_code)]
    pub waiting_for_response: bool,

    /// Optional timeout for responses
    #[allow(dead_code)]
    pub response_timeout: Option<Instant>,

    /// Players who need to make simultaneous decisions
    pub simultaneous_decision_players: Vec<Entity>,

    /// Tracks the last phase we completed a full priority round for
    pub last_processed_phase: Option<Phase>,

    /// Tracks the last turn number we processed
    pub last_processed_turn: u32,

    /// Decision timeouts for simultaneous decisions
    #[allow(dead_code)]
    pub decision_timeouts: HashMap<Entity, std::time::Duration>,
}

impl PrioritySystem {
    /// Creates a new PrioritySystemBuilder for chainable construction
    #[allow(dead_code)]
    pub fn builder() -> PrioritySystemBuilder {
        PrioritySystemBuilder::new()
    }

    /// Initialize the priority system with the list of players and the active player
    pub fn initialize(&mut self, players: &[Entity], active_player: Entity) {
        self.player_order = players.to_vec();
        self.active_player = active_player;
        self.priority_player = active_player;
        self.priority_index = self
            .player_order
            .iter()
            .position(|&p| p == active_player)
            .unwrap_or(0);

        // Initialize the passing status for all players
        self.has_priority_passed.clear();
        for &player in players {
            self.has_priority_passed.insert(player, false);
        }

        self.all_players_passed = false;
        self.simultaneous_decision_players.clear();
    }

    /// Pass priority to the next player in turn order
    pub fn pass_priority(&mut self) {
        if self.player_order.is_empty() {
            return;
        }

        // Mark the current player as having passed
        self.has_priority_passed.insert(self.priority_player, true);

        // Move to the next player
        self.priority_index = (self.priority_index + 1) % self.player_order.len();
        self.priority_player = self.player_order[self.priority_index];

        // Check if we're back to the active player or first player
        if self.priority_player == self.active_player {
            // We've completed a round of priority passing
            self.all_players_passed = self.has_priority_passed.values().all(|&passed| passed);
        }
    }

    /// Reset after a stack action has resolved
    pub fn reset_after_stack_action(&mut self, players: &[Entity], active_player: Entity) {
        self.player_order = players.to_vec();
        self.active_player = active_player;

        // Priority goes to the active player after a stack item resolves
        self.priority_player = active_player;
        self.priority_index = self
            .player_order
            .iter()
            .position(|&p| p == active_player)
            .unwrap_or(0);

        // Reset the passing status for all players
        self.has_priority_passed.clear();
        for &player in players {
            self.has_priority_passed.insert(player, false);
        }

        self.all_players_passed = false;
    }

    /// Set whether the stack is empty (affects priority passing)
    #[allow(dead_code)]
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
        for (_player, passed) in self.has_priority_passed.iter_mut() {
            *passed = false;
        }

        self.all_players_passed = false;
    }

    /// Check if a full priority round is complete
    pub fn priority_round_complete(&self) -> bool {
        self.all_players_passed
    }

    /// Check if a player has passed priority
    #[allow(dead_code)]
    pub fn has_passed(&self, player: Entity) -> bool {
        self.has_priority_passed
            .get(&player)
            .copied()
            .unwrap_or(false)
    }

    /// Set a timeout for a player's decision
    #[allow(dead_code)]
    pub fn set_decision_timeout(&mut self, player: Entity, duration: std::time::Duration) {
        self.decision_timeouts.insert(player, duration);
    }

    /// Add a player to the simultaneous decision list
    #[allow(dead_code)]
    pub fn add_simultaneous_decision_player(&mut self, player: Entity) {
        if !self.simultaneous_decision_players.contains(&player) {
            self.simultaneous_decision_players.push(player);
        }
    }

    /// Remove a player from the simultaneous decision list
    #[allow(dead_code)]
    pub fn remove_simultaneous_decision_player(&mut self, player: Entity) {
        self.simultaneous_decision_players.retain(|p| *p != player);
    }
}

impl Default for PrioritySystem {
    fn default() -> Self {
        PrioritySystemBuilder::new().build()
    }
}

/// Builder for PrioritySystem with a chainable API
#[derive(Clone, Debug)]
pub struct PrioritySystemBuilder {
    active_player: Entity,
    priority_player: Entity,
    has_priority_passed: HashMap<Entity, bool>,
    all_players_passed: bool,
    player_order: Vec<Entity>,
    priority_index: usize,
    stack_is_empty: bool,
    current_phase: Phase,
    waiting_for_response: bool,
    response_timeout: Option<Instant>,
    simultaneous_decision_players: Vec<Entity>,
    last_processed_phase: Option<Phase>,
    last_processed_turn: u32,
    decision_timeouts: HashMap<Entity, std::time::Duration>,
}

impl Default for PrioritySystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PrioritySystemBuilder {
    /// Creates a new PrioritySystemBuilder with default values
    pub fn new() -> Self {
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

    /// Sets the active player
    #[allow(dead_code)]
    pub fn active_player(mut self, active_player: Entity) -> Self {
        self.active_player = active_player;
        self
    }

    /// Sets the priority player
    #[allow(dead_code)]
    pub fn priority_player(mut self, priority_player: Entity) -> Self {
        self.priority_player = priority_player;
        self
    }

    /// Sets the priority passing status for all players
    #[allow(dead_code)]
    pub fn has_priority_passed(mut self, has_priority_passed: HashMap<Entity, bool>) -> Self {
        self.has_priority_passed = has_priority_passed;
        self
    }

    /// Sets whether all players have passed
    #[allow(dead_code)]
    pub fn all_players_passed(mut self, all_players_passed: bool) -> Self {
        self.all_players_passed = all_players_passed;
        self
    }

    /// Sets the player order
    #[allow(dead_code)]
    pub fn player_order(mut self, player_order: Vec<Entity>) -> Self {
        self.player_order = player_order;
        self
    }

    /// Sets the current index in the player order
    #[allow(dead_code)]
    pub fn priority_index(mut self, priority_index: usize) -> Self {
        self.priority_index = priority_index;
        self
    }

    /// Sets whether the stack is empty
    #[allow(dead_code)]
    pub fn stack_is_empty(mut self, stack_is_empty: bool) -> Self {
        self.stack_is_empty = stack_is_empty;
        self
    }

    /// Sets the current phase
    #[allow(dead_code)]
    pub fn current_phase(mut self, current_phase: Phase) -> Self {
        self.current_phase = current_phase;
        self
    }

    /// Sets whether we're waiting for a response
    #[allow(dead_code)]
    pub fn waiting_for_response(mut self, waiting_for_response: bool) -> Self {
        self.waiting_for_response = waiting_for_response;
        self
    }

    /// Sets the timeout for responses
    #[allow(dead_code)]
    pub fn response_timeout(mut self, response_timeout: Option<Instant>) -> Self {
        self.response_timeout = response_timeout;
        self
    }

    /// Sets the players who need to make simultaneous decisions
    #[allow(dead_code)]
    pub fn simultaneous_decision_players(mut self, players: Vec<Entity>) -> Self {
        self.simultaneous_decision_players = players;
        self
    }

    /// Sets the last processed phase
    #[allow(dead_code)]
    pub fn last_processed_phase(mut self, phase: Option<Phase>) -> Self {
        self.last_processed_phase = phase;
        self
    }

    /// Sets the last processed turn number
    #[allow(dead_code)]
    pub fn last_processed_turn(mut self, turn_number: u32) -> Self {
        self.last_processed_turn = turn_number;
        self
    }

    /// Sets decision timeouts
    #[allow(dead_code)]
    pub fn decision_timeouts(mut self, timeouts: HashMap<Entity, std::time::Duration>) -> Self {
        self.decision_timeouts = timeouts;
        self
    }

    /// Builds the PrioritySystem instance
    pub fn build(self) -> PrioritySystem {
        PrioritySystem {
            active_player: self.active_player,
            priority_player: self.priority_player,
            has_priority_passed: self.has_priority_passed,
            all_players_passed: self.all_players_passed,
            player_order: self.player_order,
            priority_index: self.priority_index,
            stack_is_empty: self.stack_is_empty,
            current_phase: self.current_phase,
            waiting_for_response: self.waiting_for_response,
            response_timeout: self.response_timeout,
            simultaneous_decision_players: self.simultaneous_decision_players,
            last_processed_phase: self.last_processed_phase,
            last_processed_turn: self.last_processed_turn,
            decision_timeouts: self.decision_timeouts,
        }
    }
}
