use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;

use crate::game_engine::save::data::{GameSaveData, SaveInfo};

/// Configuration for the save system
#[derive(Resource)]
pub struct SaveConfig {
    pub save_directory: PathBuf,
    pub auto_save_enabled: bool,
    pub auto_save_frequency: usize, // How often to auto-save (in state-based action checks)
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            save_directory: PathBuf::from("saves"),
            auto_save_enabled: true,
            auto_save_frequency: 10, // Auto-save every 10 state-based action checks
        }
    }
}

/// Tracker for auto-saving
#[derive(Resource)]
pub struct AutoSaveTracker {
    pub counter: usize,
}

impl Default for AutoSaveTracker {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

/// Metadata about all saved games
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub saves: Vec<SaveInfo>,
}

impl Default for SaveMetadata {
    fn default() -> Self {
        Self { saves: Vec::new() }
    }
}

/// Replay state for game replay functionality
#[derive(Resource)]
pub struct ReplayState {
    /// Whether a replay is currently in progress
    pub active: bool,

    /// Original saved game data
    pub original_save: Option<GameSaveData>,

    /// Current game state in the replay
    pub current_game_state: Option<GameSaveData>,

    /// Queue of action records to apply during replay
    pub action_queue: VecDeque<ReplayAction>,

    /// Current step in the replay
    pub current_step: usize,
}

impl Default for ReplayState {
    fn default() -> Self {
        Self {
            active: false,
            original_save: None,
            current_game_state: None,
            action_queue: VecDeque::new(),
            current_step: 0,
        }
    }
}

/// A record of a game action for replay purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayAction {
    /// Type of action
    pub action_type: ReplayActionType,

    /// Player who performed the action (as index)
    pub player_index: usize,

    /// Any additional data needed to replay the action
    pub data: String,

    /// Turn number when action occurred
    pub turn: u32,

    /// Phase when action occurred
    pub phase: String,
}

/// Types of actions that can be replayed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplayActionType {
    PlayCard,
    DeclareAttackers,
    DeclareBlockers,
    ActivateAbility,
    ResolveEffect,
    DrawCard,
    PassPriority,
    CastSpell,
    EndTurn,
}
