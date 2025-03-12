use crate::game_engine::GameStack;
use crate::game_engine::Phase;
use crate::game_engine::state::GameState;
use bevy::prelude::*;
use std::collections::VecDeque;

/// System for managing priority in MTG
#[derive(Resource)]
pub struct PrioritySystem {
    /// The player who currently has priority
    pub active_player: Entity,

    /// Whether a player currently has priority
    pub has_priority: bool,

    /// Queue of players who will get priority
    pub priority_queue: VecDeque<Entity>,

    /// Whether the stack is currently empty
    pub stack_is_empty: bool,

    /// Whether all players have passed priority in succession
    pub all_players_passed: bool,

    /// Tracks the last phase we completed a full priority round for
    pub last_processed_phase: Option<Phase>,

    /// Tracks the last turn number we processed
    pub last_processed_turn: u32,
}

impl Default for PrioritySystem {
    fn default() -> Self {
        Self {
            active_player: Entity::PLACEHOLDER,
            has_priority: false,
            priority_queue: VecDeque::new(),
            stack_is_empty: true,
            all_players_passed: false,
            last_processed_phase: None,
            last_processed_turn: 0,
        }
    }
}

impl PrioritySystem {
    /// Initialize the priority system with the specified players in turn order
    pub fn initialize(&mut self, players: &[Entity], active_player: Entity) {
        self.priority_queue.clear();

        // Start with the active player
        self.active_player = active_player;

        // Set up the priority order starting with the active player
        let active_player_index = players
            .iter()
            .position(|&p| p == active_player)
            .unwrap_or(0);

        // Add players to the queue in turn order starting with the active player
        for i in 0..players.len() {
            let index = (active_player_index + i) % players.len();
            self.priority_queue.push_back(players[index]);
        }

        // Set initial priority state
        self.has_priority = true;
        self.all_players_passed = false;
    }

    /// Pass priority to the next player in the queue
    pub fn pass_priority(&mut self) {
        if !self.priority_queue.is_empty() {
            // Move the current player to the back of the queue
            let current_player = self.priority_queue.pop_front().unwrap();
            self.priority_queue.push_back(current_player);

            // Check if we've gone all the way around
            if current_player == self.active_player {
                // If we're back to the active player and the stack is still empty,
                // this means everyone has passed in succession
                if self.stack_is_empty {
                    self.all_players_passed = true;
                }
            }

            // Set the new player with priority
            if let Some(&next_player) = self.priority_queue.front() {
                self.active_player = next_player;
            }
        }
    }

    /// Reset the priority system after something has been added to the stack
    pub fn reset_after_stack_action(&mut self, players: &[Entity], active_player: Entity) {
        // After something goes on the stack, priority goes back to the active player
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
}

/// System to handle priority passing and checks
pub fn priority_system(
    _commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_state: ResMut<GameState>,
    stack: Res<GameStack>,
    phase: Res<Phase>,
    turn_manager: Res<crate::game_engine::turns::TurnManager>,
    // This would also interact with any pending game actions
) {
    // Update stack empty status
    priority.set_stack_empty(stack.is_empty());

    // Update game state priority holder
    game_state.priority_holder = priority.active_player;

    // When all players have passed and the stack is empty, we're ready for phase transition
    if priority.all_players_passed && stack.is_empty() {
        // Mark this phase as processed to prevent duplicate processing
        priority.mark_phase_processed(*phase, turn_manager.turn_number);
    }

    // If in a phase that auto-passes when empty and the stack is empty,
    // automatically pass priority
    if phase.auto_pass_if_empty() && stack.is_empty() {
        // Check if we've already processed this phase in the current turn
        if !priority.has_processed_phase(*phase, turn_manager.turn_number) {
            // Auto-pass for all players
            for _ in 0..priority.priority_queue.len() {
                priority.pass_priority();
            }
        }
    }

    // For now, this is a placeholder for the full system, which would:
    // - Handle player input for passing priority
    // - Process priority passing after spell casts or ability activations
    // - Handle automatic priority passing for certain game situations
}
