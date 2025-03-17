// Internal modules
pub mod data;
pub mod events;
pub mod plugin;
pub mod resources;
pub mod systems;

#[cfg(test)]
pub mod tests;

// Re-export plugin
pub use plugin::SaveLoadPlugin;

// Re-export data types
pub use data::{
    CardData, CommanderData, CommanderPairData, GameSaveData, GameStateData, PlayerData, ZoneData,
};

// Re-export resources
pub use resources::{
    AutoSaveTracker, ReplayAction, ReplayActionType, ReplayState, SaveConfig, SaveMetadata,
};

// Re-export events
pub use events::{
    CheckStateBasedActionsEvent, LoadGameEvent, SaveGameEvent, StartReplayEvent, StepReplayEvent,
    StopReplayEvent,
};
