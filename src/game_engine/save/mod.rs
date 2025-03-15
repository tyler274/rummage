use bevy::prelude::*;
use bevy_persistent::Storage;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Internal modules
mod data;
mod events;
mod plugin;
mod resources;
mod systems;

// Re-export public API
pub use data::*;
pub use events::*;
pub use plugin::SaveLoadPlugin;
pub use resources::*;
pub use systems::*;

// Import the components we need for type references
use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::mana::ManaPool;
use crate::player::Player;
