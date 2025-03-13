use crate::game_engine::phase::types::Phase;
use bevy::prelude::*;

/// Resource that manages turn order and the active player
#[derive(Resource, Debug)]
pub struct TurnManager {
    /// The currently active player taking their turn
    pub active_player: Entity,

    /// The order of players in the game
    pub player_order: Vec<Entity>,

    /// The index in player_order of the active player
    pub active_player_index: usize,

    /// The current turn number
    pub turn_number: u32,

    /// Players eliminated from the game
    pub eliminated_players: Vec<Entity>,

    /// The current game phase/step
    /// TODO: Implement phase tracking and transitions
    #[allow(dead_code)]
    pub current_phase: Phase,
}

impl Default for TurnManager {
    fn default() -> Self {
        crate::game_engine::turns::builder::TurnManagerBuilder::new().build()
    }
}

impl TurnManager {
    /// Creates a new TurnManagerBuilder for chainable construction
    /// TODO: Implement this when turn management is fully implemented
    #[allow(dead_code)]
    pub fn builder() -> crate::game_engine::turns::builder::TurnManagerBuilder {
        crate::game_engine::turns::builder::TurnManagerBuilder::new()
    }

    /// Initialize the turn manager with the list of players
    pub fn initialize(&mut self, players: Vec<Entity>) {
        self.player_order = players.clone();
        if !players.is_empty() {
            self.active_player = players[0];
            self.active_player_index = 0;
        }
    }

    /// Move to the next player's turn
    pub fn advance_turn(&mut self) {
        if self.player_order.is_empty() {
            return;
        }

        // Increment turn number if we've gone through all players
        if self.active_player_index >= self.player_order.len() - 1 {
            self.turn_number += 1;
        }

        // Move to the next player, skipping eliminated players
        loop {
            self.active_player_index = (self.active_player_index + 1) % self.player_order.len();
            self.active_player = self.player_order[self.active_player_index];

            // If player is not eliminated, break the loop
            if !self.eliminated_players.contains(&self.active_player) {
                break;
            }

            // Safety check to avoid infinite loop if all players are eliminated
            if self.eliminated_players.len() >= self.player_order.len() {
                break;
            }
        }
    }

    /// Mark a player as eliminated
    /// TODO: Implement player elimination mechanics
    #[allow(dead_code)]
    pub fn eliminate_player(&mut self, player: Entity) {
        if !self.eliminated_players.contains(&player) {
            self.eliminated_players.push(player);
        }
    }

    /// Check if all players but one are eliminated
    /// TODO: Implement game end condition checking
    #[allow(dead_code)]
    pub fn is_game_over(&self) -> bool {
        let active_players = self.player_order.len() - self.eliminated_players.len();
        active_players <= 1
    }

    /// Get the index of a player in the turn order
    /// TODO: Use this when implementing turn-based effects
    #[allow(dead_code)]
    pub fn get_player_index(&self, player: Entity) -> Option<usize> {
        self.player_order.iter().position(|&p| p == player)
    }

    /// Get the currently active player
    #[allow(dead_code)]
    pub fn get_active_player(&self) -> Entity {
        self.active_player
    }
}
