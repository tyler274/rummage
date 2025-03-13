use crate::game_engine::GameStack;
use crate::game_engine::Phase;
use crate::game_engine::state::GameState;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// Event for passing priority
#[derive(Event)]
pub struct PassPriorityEvent {
    /// The player passing priority
    pub player: Entity,
}

/// Event for resolving stack items
#[derive(Event)]
pub struct ResolveStackItemEvent {
    /// The stack item to resolve
    pub item: Entity,
}

/// Event for phase transitions
#[derive(Event)]
pub struct NextPhaseEvent;

/// Reason an effect was countered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterReason {
    /// Explicitly countered by a spell or ability
    CounterSpell,
    /// Countered due to invalid targets on resolution
    InvalidTargets,
    /// Countered due to rules (e.g., illegal targets)
    RulesBased,
}

/// Event fired when an effect is countered
#[derive(Event)]
pub struct EffectCounteredEvent {
    /// The item that was countered
    pub item: Entity,
    /// The reason it was countered
    pub reason: CounterReason,
}

/// System for managing priority in MTG
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
    /// Initialize the priority system with the specified players in turn order
    pub fn initialize(&mut self, players: &[Entity], active_player: Entity) {
        // Reset priority tracking
        self.has_priority_passed.clear();
        for &player in players {
            self.has_priority_passed.insert(player, false);
        }

        // Start with the active player
        self.active_player = active_player;
        self.priority_player = active_player;

        // Set up the player order starting with the active player
        self.player_order = Vec::with_capacity(players.len());
        let active_player_index = players
            .iter()
            .position(|&p| p == active_player)
            .unwrap_or(0);

        // Add players to the order in turn order starting with the active player
        for i in 0..players.len() {
            let index = (active_player_index + i) % players.len();
            self.player_order.push(players[index]);
        }

        // Set the current priority index
        self.priority_index = 0;

        // Reset state
        self.all_players_passed = false;
        self.waiting_for_response = false;
        self.response_timeout = None;
        self.simultaneous_decision_players.clear();
        self.decision_timeouts.clear();
    }

    /// Pass priority to the next player in turn order
    pub fn pass_priority(&mut self) {
        if self.player_order.is_empty() {
            return;
        }

        // Mark that the current player has passed priority
        self.has_priority_passed.insert(self.priority_player, true);

        // Check if all players have passed
        let all_passed = self.has_priority_passed.values().all(|&passed| passed);

        if all_passed {
            self.all_players_passed = true;
        }

        // Advance to the next player
        self.priority_index = (self.priority_index + 1) % self.player_order.len();
        self.priority_player = self.player_order[self.priority_index];
    }

    /// Reset the priority system after something has been added to the stack
    pub fn reset_after_stack_action(&mut self, players: &[Entity], active_player: Entity) {
        // After something goes on the stack, priority goes back to the player who put it on the stack
        self.initialize(players, active_player);
        self.all_players_passed = false;
        self.stack_is_empty = false;
    }

    /// Update the stack empty status
    pub fn set_stack_empty(&mut self, is_empty: bool) {
        self.stack_is_empty = is_empty;

        // If the stack becomes empty, reset the all_players_passed flag
        if is_empty {
            self.all_players_passed = false;
        }
    }

    /// Check if we've already processed a particular phase in the current turn
    pub fn has_processed_phase(&self, current_phase: Phase, turn_number: u32) -> bool {
        self.last_processed_phase == Some(current_phase) && self.last_processed_turn == turn_number
    }

    /// Mark a phase as processed for this turn
    pub fn mark_phase_processed(&mut self, current_phase: Phase, turn_number: u32) {
        self.last_processed_phase = Some(current_phase);
        self.last_processed_turn = turn_number;
    }

    /// Check if the current player has priority
    pub fn has_priority(&self, player: Entity) -> bool {
        self.priority_player == player
    }

    /// Reset priority passing status
    pub fn reset_passing_status(&mut self) {
        for (_, passed) in self.has_priority_passed.iter_mut() {
            *passed = false;
        }
        self.all_players_passed = false;
    }

    /// Is a round of priority passing complete
    pub fn priority_round_complete(&self) -> bool {
        self.has_priority_passed.values().all(|&passed| passed)
    }

    /// Check if a player has passed priority
    pub fn has_passed(&self, player: Entity) -> bool {
        self.has_priority_passed
            .get(&player)
            .copied()
            .unwrap_or(false)
    }

    /// Set a timeout for a player's decision
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
        self.simultaneous_decision_players.retain(|p| *p != player);
    }
}

/// System to handle priority passing and checks
pub fn priority_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_state: ResMut<GameState>,
    stack: Res<GameStack>,
    phase: Res<Phase>,
    turn_manager: Res<crate::game_engine::turns::TurnManager>,
) {
    // Update stack empty status
    priority.set_stack_empty(stack.is_empty());

    // Update game state priority holder
    game_state.priority_holder = priority.priority_player;

    // Update current phase
    priority.current_phase = *phase;

    // When all players have passed and the stack is empty, we're ready for phase transition
    if priority.all_players_passed && stack.is_empty() {
        // Mark this phase as processed to prevent duplicate processing
        priority.mark_phase_processed(*phase, turn_manager.turn_number);

        // Trigger phase transition
        commands.spawn(NextPhaseEvent);
    }

    // If in a phase that auto-passes when empty and the stack is empty,
    // automatically pass priority
    if phase.auto_pass_if_empty() && stack.is_empty() {
        // Check if we've already processed this phase in the current turn
        if !priority.has_processed_phase(*phase, turn_manager.turn_number) {
            // Auto-pass for all players
            priority.reset_passing_status();
            for _ in 0..priority.player_order.len() {
                priority.pass_priority();
            }
        }
    }

    // Check for timeouts on waiting for responses
    if priority.waiting_for_response {
        if let Some(timeout) = priority.response_timeout {
            if Instant::now() > timeout {
                priority.waiting_for_response = false;
                // Auto-pass priority for the timed-out player
                priority.pass_priority();
            }
        }
    }
}

/// System to handle priority passing events
pub fn priority_passing_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_stack: ResMut<GameStack>,
    mut pass_events: EventReader<PassPriorityEvent>,
    players: Query<Entity, With<Player>>,
    _time: Res<Time>,
) {
    // Check for priority passes
    for event in pass_events.read() {
        let player = event.player;

        // Verify player has priority
        if player == priority.priority_player {
            // Mark priority as passed
            priority.has_priority_passed.insert(player, true);

            // Move to next player in order
            priority.pass_priority();
        }
    }

    // Check for timeout-based automatic passing
    if let Some(timeout) = priority.response_timeout {
        if Instant::now() > timeout {
            // Auto-pass for timed out player
            let current_player = priority.priority_player;
            priority.has_priority_passed.insert(current_player, true);
            priority.pass_priority();
        }
    }

    // Update stack empty status
    priority.set_stack_empty(game_stack.is_empty());

    // Check if all players have passed
    if priority.priority_round_complete() {
        // If stack is empty, move to next phase
        if priority.stack_is_empty {
            // All passed with empty stack = phase change
            commands.spawn(NextPhaseEvent);
        } else {
            // All passed with non-empty stack = resolve top of stack
            game_stack.resolving = true;
            let top_entity = game_stack.items.last().unwrap().entity;
            commands.spawn(ResolveStackItemEvent { item: top_entity });
        }

        // Reset priority passing tracker
        for player in players.iter() {
            priority.has_priority_passed.insert(player, false);
        }

        // If stack is being resolved, priority goes to active player after resolution
        if !priority.stack_is_empty {
            priority.priority_player = priority.active_player;
            priority.priority_index = priority
                .player_order
                .iter()
                .position(|&p| p == priority.active_player)
                .unwrap_or(0);
        }
    }
}
