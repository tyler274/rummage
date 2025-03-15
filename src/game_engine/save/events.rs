use bevy::prelude::*;

/// Event to trigger saving the game
#[derive(Event)]
pub struct SaveGameEvent {
    pub slot_name: String,
}

/// Event to trigger loading a saved game
#[derive(Event)]
pub struct LoadGameEvent {
    pub slot_name: String,
}

/// Event for checking state-based actions
#[derive(Event)]
pub struct CheckStateBasedActionsEvent;

/// Event to start replaying a game from a saved state
#[derive(Event)]
pub struct StartReplayEvent {
    pub slot_name: String,
}

/// Event to step forward one turn in a replay
#[derive(Event)]
pub struct StepReplayEvent {
    /// Number of steps to advance (default: 1)
    pub steps: usize,
}

/// Event to stop an ongoing replay
#[derive(Event)]
pub struct StopReplayEvent;
