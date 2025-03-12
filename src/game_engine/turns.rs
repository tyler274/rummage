use crate::game_engine::{BeginningStep, EndingStep, Phase};
use crate::menu::GameMenuState;
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

    /// The current game phase/step
    pub current_phase: Phase,
}

impl Default for TurnManager {
    fn default() -> Self {
        Self {
            active_player: Entity::PLACEHOLDER,
            player_order: Vec::new(),
            active_player_index: 0,
            turn_number: 1,
            eliminated_players: Vec::new(),
            current_phase: Phase::Beginning(BeginningStep::Untap),
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

    /// Get the currently active player
    pub fn get_active_player(&self) -> Entity {
        self.active_player
    }
}

/// Event triggered at the start of a turn
#[derive(Event, Debug)]
pub struct TurnStartEvent {
    /// The player whose turn is starting
    pub player: Entity,
    /// The turn number that is starting
    pub turn_number: u32,
}

/// Event triggered at the end of a turn
#[derive(Event, Debug)]
pub struct TurnEndEvent {
    /// The player whose turn is ending
    pub player: Entity,
    /// The turn number that is ending
    pub turn_number: u32,
}

/// Resource to track turn event state to prevent duplicate events
#[derive(Resource, Default, Debug)]
pub struct TurnEventTracker {
    /// Whether a turn start event has been sent for the current turn
    pub turn_start_processed: bool,
    /// Whether a turn end event has been sent for the current turn
    pub turn_end_processed: bool,
    /// Whether the untap step has been processed for the current turn
    pub untap_step_processed: bool,
    /// The last turn number that was processed
    pub last_processed_turn: u32,
}

/// System to handle the start of a new turn
pub fn turn_start_system(
    _commands: Commands,
    phase: Res<Phase>,
    _player_query: Query<&Player>,
    mut turn_start_events: EventWriter<TurnStartEvent>,
    turn_manager: Res<TurnManager>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only trigger at the beginning of the untap step
    if *phase == Phase::Beginning(BeginningStep::Untap) {
        // Check if we've already processed this turn
        if event_tracker.turn_start_processed
            && event_tracker.last_processed_turn == turn_manager.turn_number
        {
            return;
        }

        // Create a turn start event
        turn_start_events.send(TurnStartEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        // Mark as processed
        event_tracker.turn_start_processed = true;
        event_tracker.last_processed_turn = turn_manager.turn_number;

        info!(
            "Turn {} started for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    } else {
        // If we're not in the untap step, reset the tracker for the next turn
        event_tracker.turn_start_processed = false;
    }
}

/// System to handle the end of a turn
pub fn turn_end_system(
    _commands: Commands,
    phase: Res<Phase>,
    _player_query: Query<&Player>,
    mut turn_end_events: EventWriter<TurnEndEvent>,
    turn_manager: Res<TurnManager>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only trigger at the beginning of the end step
    if *phase == Phase::Ending(EndingStep::End) {
        // Check if we've already processed the end of this turn
        if event_tracker.turn_end_processed
            && event_tracker.last_processed_turn == turn_manager.turn_number
        {
            return;
        }

        // Create a turn end event
        turn_end_events.send(TurnEndEvent {
            player: turn_manager.active_player,
            turn_number: turn_manager.turn_number,
        });

        // Mark as processed
        event_tracker.turn_end_processed = true;
        event_tracker.last_processed_turn = turn_manager.turn_number;

        info!(
            "Turn {} ended for player {:?}",
            turn_manager.turn_number, turn_manager.active_player
        );
    } else {
        // If we're not in the end step, reset the tracker for the next turn
        event_tracker.turn_end_processed = false;
    }
}

/// System that handles untapping permanents during the untap step
/// This system considers special effects that prevent untapping, like NoUntapEffect
pub fn handle_untap_step(
    mut card_query: Query<
        (
            Entity,
            &mut crate::card::PermanentState,
            Option<&crate::card::NoUntapEffect>,
        ),
        With<crate::card::Card>,
    >,
    turn_manager: Res<TurnManager>,
    phase: Res<Phase>,
    _player_query: Query<&mut crate::player::Player>,
    mut event_tracker: Local<TurnEventTracker>,
) {
    // Only process during untap step
    if *phase != Phase::Beginning(BeginningStep::Untap) {
        // Reset the untap step tracker if we're not in the untap step
        event_tracker.untap_step_processed = false;
        return;
    }

    // Check if we've already processed the untap step for this turn
    if event_tracker.untap_step_processed
        && event_tracker.last_processed_turn == turn_manager.turn_number
    {
        return;
    }

    let active_player_entity = turn_manager.get_active_player();
    info!(
        "Processing untap step for player: {:?}",
        active_player_entity
    );

    // Get the current turn number
    let current_turn = turn_manager.turn_number;

    for (entity, mut permanent_state, no_untap_effect) in card_query.iter_mut() {
        // Update summoning sickness regardless of untap restrictions
        permanent_state.update_summoning_sickness(current_turn);

        // Check if there's a no-untap effect
        if let Some(no_untap) = no_untap_effect {
            let should_skip_untap = match &no_untap.condition {
                // Skip untap if we're in the next untap step after the effect was applied
                Some(crate::card::NoUntapCondition::NextUntapStep) => true,

                // Other conditions would be checked here, but we won't implement them for now
                // as they would require more complex state tracking
                _ => true, // Default to skipping untap if any other condition exists
            };

            if should_skip_untap {
                info!("Permanent {:?} does not untap due to NoUntapEffect", entity);
                continue;
            }
        }

        // Untap the permanent if it's tapped
        if permanent_state.is_tapped {
            let untapped = permanent_state.untap();
            if untapped {
                info!("Untapped permanent: {:?}", entity);
            }
        }
    }

    // Mark the untap step as processed for this turn
    event_tracker.untap_step_processed = true;
    event_tracker.last_processed_turn = turn_manager.turn_number;
}

/// Register all turn-related systems with the app
pub fn register_turn_systems(app: &mut App) {
    app.add_systems(
        Update,
        (turn_start_system, turn_end_system, handle_untap_step)
            .run_if(in_state(crate::menu::GameMenuState::InGame)),
    );
}
