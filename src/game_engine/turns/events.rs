use bevy::prelude::*;

/// Event fired at the beginning of a player's turn
#[derive(Event, Debug)]
pub struct TurnStartEvent {
    /// The player whose turn is starting
    /// TODO: Use this field when implementing turn start handling
    #[allow(dead_code)]
    pub player: Entity,
    /// The turn number that is starting
    /// TODO: Use this field when implementing turn tracking
    #[allow(dead_code)]
    pub turn_number: u32,
}

impl TurnStartEvent {
    /// Creates a new turn start event
    /// TODO: Use this constructor when implementing turn start events
    #[allow(dead_code)]
    pub fn new(player: Entity, turn_number: u32) -> Self {
        Self {
            player,
            turn_number,
        }
    }
}

/// Event fired at the end of a player's turn
#[derive(Event, Debug)]
pub struct TurnEndEvent {
    /// The player whose turn is ending
    /// TODO: Use this field when implementing turn end handling
    #[allow(dead_code)]
    pub player: Entity,
    /// The turn number that is ending
    /// TODO: Use this field when implementing turn tracking
    #[allow(dead_code)]
    pub turn_number: u32,
}

impl TurnEndEvent {
    /// Creates a new turn end event
    /// TODO: Use this constructor when implementing turn end events
    #[allow(dead_code)]
    pub fn new(player: Entity, turn_number: u32) -> Self {
        Self {
            player,
            turn_number,
        }
    }
}

/// Local resource to track turn event processing to prevent duplicate events
#[derive(Debug, Default)]
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

impl TurnEventTracker {
    /// Creates a new TurnEventTrackerBuilder for chainable construction
    /// TODO: Implement this when turn event tracking is needed
    #[allow(dead_code)]
    pub fn builder() -> crate::game_engine::turns::builder::TurnEventTrackerBuilder {
        crate::game_engine::turns::builder::TurnEventTrackerBuilder::new()
    }

    /// Reset all event processing flags
    /// TODO: Implement this when turn event tracking is needed
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.turn_start_processed = false;
        self.turn_end_processed = false;
        self.untap_step_processed = false;
    }

    /// Update the turn number and reset flags if it's a new turn
    /// TODO: Implement this when turn tracking is needed
    #[allow(dead_code)]
    pub fn update_turn(&mut self, turn_number: u32) {
        if turn_number != self.last_processed_turn {
            self.reset();
            self.last_processed_turn = turn_number;
        }
    }
}
