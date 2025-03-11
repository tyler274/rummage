use crate::game_engine::state::GameState;
use crate::game_engine::{BeginningStep, EndingStep, Phase, PhaseState, Step};
use crate::menu::state::GameMenuState;
use crate::player::Player;
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
}

impl Default for TurnManager {
    fn default() -> Self {
        Self {
            active_player: Entity::PLACEHOLDER,
            player_order: Vec::new(),
            active_player_index: 0,
            turn_number: 1,
            eliminated_players: Vec::new(),
        }
    }
}

impl TurnManager {
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
    pub fn eliminate_player(&mut self, player: Entity) {
        if !self.eliminated_players.contains(&player) {
            self.eliminated_players.push(player);
        }
    }

    /// Check if all players but one are eliminated
    pub fn is_game_over(&self) -> bool {
        let active_players = self.player_order.len() - self.eliminated_players.len();
        active_players <= 1
    }

    /// Get the index of a player in the turn order
    pub fn get_player_index(&self, player: Entity) -> Option<usize> {
        self.player_order.iter().position(|&p| p == player)
    }
}

/// Event triggered at the start of a turn
#[derive(Event)]
pub struct TurnStartEvent {
    pub player: Entity,
    pub turn_number: u32,
}

/// Event triggered at the end of a turn
#[derive(Event)]
pub struct TurnEndEvent {
    pub player: Entity,
    pub turn_number: u32,
}

/// System to handle the start of a new turn
pub fn turn_start_system(
    mut commands: Commands,
    phase_state: Res<PhaseState>,
    player_query: Query<&Player>,
    mut turn_start_events: EventWriter<TurnStartEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Only trigger at the beginning of the untap step
    if phase_state.current_phase == Phase::Beginning(BeginningStep::Untap) {
        // Create a turn start event
        turn_start_events.send(TurnStartEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        info!(
            "Turn {} started for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    }
}

/// System to handle the end of a turn
pub fn turn_end_system(
    mut commands: Commands,
    phase_state: Res<PhaseState>,
    player_query: Query<&Player>,
    mut turn_end_events: EventWriter<TurnEndEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Only trigger at the beginning of the end step
    if phase_state.current_phase == Phase::Ending(EndingStep::End) {
        // Create a turn end event
        turn_end_events.send(TurnEndEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        info!(
            "Turn {} ended for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    }
}

/// System to handle untapping permanents
pub fn untap_system(
    mut commands: Commands,
    phase_state: Res<PhaseState>,
    turn_manager: Res<TurnManager>,
    // We would need queries for permanents to untap them
) {
    // Only trigger at the beginning of the untap step
    if phase_state.current_phase == Phase::Beginning(BeginningStep::Untap) {
        info!(
            "Untapping permanents for player {:?}",
            turn_manager.active_player
        );
        // Here we would untap all permanents for the active player
    }
}

/// Register all turn-related systems and events
pub fn register_turn_systems(app: &mut App) {
    app.add_event::<TurnStartEvent>()
        .add_event::<TurnEndEvent>()
        .insert_resource(TurnManager::default())
        .add_systems(
            Update,
            (turn_start_system, turn_end_system, untap_system)
                .run_if(in_state(GameMenuState::InGame)),
        );
}
