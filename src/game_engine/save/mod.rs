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
#[allow(unused_imports)]
pub use data::{GameSaveData, GameStateData, PlayerData};

// Re-export resources
#[allow(unused_imports)]
pub use resources::{AutoSaveTracker, ReplayState, SaveConfig, SaveMetadata};

// Re-export events
#[allow(unused_imports)]
pub use events::{
    CheckStateBasedActionsEvent, LoadGameEvent, SaveGameEvent, StartReplayEvent, StepReplayEvent,
};
