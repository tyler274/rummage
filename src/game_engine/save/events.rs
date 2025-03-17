use bevy::prelude::*;

/// Event to request saving a game
#[derive(Event, Debug, Clone)]
pub struct SaveGameEvent {
    /// Name of the save slot
    pub slot_name: String,
    /// Optional description
    #[allow(dead_code)]
    pub description: Option<String>,
    /// If set to true, an associated snapshot will be created
    #[allow(dead_code)]
    pub with_snapshot: bool,
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

/// Event to start rewinding a game
#[derive(Event)]
pub struct StartRewindEvent {
    pub steps: usize,
}

/// Event to rewind to a specific turn
#[derive(Event)]
pub struct RewindToTurnEvent {
    pub turn: u32,
}

/// Event to rollback to a previous save or checkpoint
#[derive(Event)]
pub struct RollbackEvent {
    pub checkpoint_name: Option<String>,
}

/// Event to create a new branch from current state
#[derive(Event)]
pub struct CreateBranchEvent {
    /// Optional name for the new branch
    pub name: Option<String>,
}

/// Event to switch to a different branch
#[derive(Event)]
pub struct SwitchBranchEvent {
    pub branch_id: u64,
}

/// Event to capture the current game state into history
#[derive(Event)]
pub struct CaptureHistoryEvent;

/// Event to go forward one step in history
#[derive(Event)]
pub struct HistoryForwardEvent;

/// Event to go backward one step in history
#[derive(Event)]
pub struct HistoryBackwardEvent;
