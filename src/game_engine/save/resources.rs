use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;

use crate::game_engine::save::data::GameSaveData;
use crate::game_engine::save::data::SaveInfo;
use crate::game_engine::save::events::SaveGameEvent;

/// Resource for tracking when to perform auto-saves
#[derive(Debug, Resource)]
pub struct AutoSaveTracker {
    /// Time in seconds since last auto-save
    pub time_since_last_save: f32,
    /// Last turn number that was checkpointed
    pub last_turn_checkpoint: u32,
}

impl Default for AutoSaveTracker {
    fn default() -> Self {
        Self {
            time_since_last_save: 0.0,
            last_turn_checkpoint: 0,
        }
    }
}

/// Configuration settings for the save system
#[derive(Debug, Resource, Clone)]
pub struct SaveConfig {
    /// Directory where saves are stored
    pub save_directory: PathBuf,
    /// Whether auto-save is enabled
    pub auto_save_enabled: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval_seconds: f32,
    /// Maximum number of save slots to keep (0 for unlimited)
    #[allow(dead_code)]
    pub max_save_slots: usize,
    /// Whether to capture snapshots with saves
    #[allow(dead_code)]
    pub capture_snapshots: bool,
}

impl Default for SaveConfig {
    fn default() -> Self {
        let mut save_path = PathBuf::new();
        save_path.push("saves");

        // For native platforms, create relative to current directory
        #[cfg(not(target_arch = "wasm32"))]
        {
            save_path = std::env::current_dir().unwrap_or_default().join(save_path);
        }

        Self {
            save_directory: save_path,
            auto_save_enabled: true,
            auto_save_interval_seconds: 60.0, // Save every minute by default
            max_save_slots: 10,
            capture_snapshots: true,
        }
    }
}

/// Metadata about all saved games
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveMetadata {
    pub saves: Vec<SaveInfo>,
    pub checkpoints: Vec<SaveInfo>,
}

/// Replay state for game replay functionality
#[derive(Resource, Default)]
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

/// Represents a branch point in game history
#[derive(Debug, Clone)]
pub struct GameBranch {
    /// Unique ID for this branch
    pub id: u64,

    /// Name of the branch (optional, for UI)
    pub name: Option<String>,

    /// History of game states in this branch
    pub states: VecDeque<GameSaveData>,

    /// Current position in this branch's history
    pub current_index: usize,

    /// Parent branch ID, if this is a branch from another timeline
    pub parent_branch_id: Option<u64>,

    /// Turn number where this branch was created
    pub branch_point_turn: u32,
}

impl GameBranch {
    /// Create a new branch with a given ID
    pub fn new(id: u64) -> Self {
        Self {
            id,
            name: None,
            states: VecDeque::new(),
            current_index: 0,
            parent_branch_id: None,
            branch_point_turn: 0,
        }
    }

    /// Create a branch from an existing branch
    pub fn branch_from(parent: &GameBranch, id: u64, state: GameSaveData) -> Self {
        let mut branch = Self::new(id);
        branch.parent_branch_id = Some(parent.id);
        branch.branch_point_turn = state.game_state.turn_number;
        branch.states.push_back(state);
        branch
    }

    /// Add a state to this branch
    pub fn add_state(&mut self, state: GameSaveData) {
        // If we have existing states and are not at the end of history, truncate
        if !self.states.is_empty() && self.current_index < self.states.len() - 1 {
            let new_len = self.current_index + 1;
            self.states.truncate(new_len);
        }

        self.states.push_back(state);
        // Safely calculate the new index (should always be len - 1 after push_back)
        self.current_index = self.states.len().saturating_sub(1);
    }

    /// Get the current state
    pub fn current_state(&self) -> Option<&GameSaveData> {
        self.states.get(self.current_index)
    }

    /// Move to previous state if possible
    pub fn go_backward(&mut self) -> bool {
        if self.current_index > 0 {
            self.current_index -= 1;
            true
        } else {
            false
        }
    }

    /// Move to next state if possible
    pub fn go_forward(&mut self) -> bool {
        if self.current_index < self.states.len() - 1 {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Jump to a specific turn in this branch
    pub fn go_to_turn(&mut self, turn: u32) -> bool {
        for (i, state) in self.states.iter().enumerate() {
            if state.game_state.turn_number == turn {
                self.current_index = i;
                return true;
            }
        }
        false
    }
}

/// Game history for rewinding and rollback with branching support
#[derive(Resource)]
pub struct GameHistory {
    /// All branches in the game history
    pub branches: Vec<GameBranch>,

    /// ID of the active branch
    pub active_branch_id: u64,

    /// ID counter for generating unique branch IDs
    pub next_branch_id: u64,

    /// Whether history navigation is active
    pub is_navigating: bool,

    /// Maximum number of states to keep per branch
    pub max_states_per_branch: usize,
}

impl Default for GameHistory {
    fn default() -> Self {
        let mut history = Self {
            branches: Vec::new(),
            active_branch_id: 0,
            next_branch_id: 1,
            is_navigating: false,
            max_states_per_branch: 50,
        };

        // Create initial main branch
        let main_branch = GameBranch::new(0);
        history.branches.push(main_branch);

        history
    }
}

impl GameHistory {
    /// Get the active branch
    pub fn active_branch(&self) -> Option<&GameBranch> {
        self.branches.iter().find(|b| b.id == self.active_branch_id)
    }

    /// Get a mutable reference to the active branch
    pub fn active_branch_mut(&mut self) -> Option<&mut GameBranch> {
        self.branches
            .iter_mut()
            .find(|b| b.id == self.active_branch_id)
    }

    /// Add a state to the active branch
    pub fn add_state(&mut self, state: GameSaveData) {
        // Store the max states value locally before borrowing self mutably
        let max_states = self.max_states_per_branch;

        if let Some(branch) = self.active_branch_mut() {
            branch.add_state(state);

            // Ensure we don't exceed max history size
            let states_to_remove = branch.states.len().saturating_sub(max_states);
            if states_to_remove > 0 {
                for _ in 0..states_to_remove {
                    branch.states.pop_front();
                }
                // Adjust the current_index, ensuring it doesn't underflow
                branch.current_index = branch.current_index.saturating_sub(states_to_remove);
            }
        }
    }

    /// Create a new branch from the current state
    pub fn create_branch(&mut self, state: GameSaveData) -> u64 {
        let new_branch_id = self.next_branch_id;
        self.next_branch_id += 1;

        if let Some(current_branch) = self.active_branch() {
            let new_branch = GameBranch::branch_from(current_branch, new_branch_id, state);
            self.branches.push(new_branch);
        } else {
            // If no active branch (should never happen), create a new main branch
            let mut new_branch = GameBranch::new(new_branch_id);
            new_branch.add_state(state);
            self.branches.push(new_branch);
        }

        // Switch to the new branch
        self.active_branch_id = new_branch_id;

        new_branch_id
    }

    /// Switch to a specific branch
    pub fn switch_to_branch(&mut self, branch_id: u64) -> bool {
        if self.branches.iter().any(|b| b.id == branch_id) {
            self.active_branch_id = branch_id;
            true
        } else {
            false
        }
    }

    /// Get the current game state
    pub fn current_state(&self) -> Option<&GameSaveData> {
        self.active_branch().and_then(|b| b.current_state())
    }

    /// Rewind one step back in the current branch
    pub fn rewind(&mut self) -> Option<&GameSaveData> {
        if let Some(branch) = self.active_branch_mut() {
            if branch.go_backward() {
                return branch.current_state();
            }
        }
        None
    }

    /// Fast forward one step in the current branch
    pub fn fast_forward(&mut self) -> Option<&GameSaveData> {
        if let Some(branch) = self.active_branch_mut() {
            if branch.go_forward() {
                return branch.current_state();
            }
        }
        None
    }

    /// Jump to a specific turn in the current branch
    pub fn go_to_turn(&mut self, turn: u32) -> Option<&GameSaveData> {
        if let Some(branch) = self.active_branch_mut() {
            if branch.go_to_turn(turn) {
                return branch.current_state();
            }
        }
        None
    }

    /// Find available parent branches to return to
    #[allow(dead_code)]
    pub fn available_parent_branches(&self) -> Vec<u64> {
        let mut result = Vec::new();
        if let Some(branch) = self.active_branch() {
            if let Some(parent_id) = branch.parent_branch_id {
                result.push(parent_id);

                // Also include grandparents
                if let Some(parent) = self.branches.iter().find(|b| b.id == parent_id) {
                    if let Some(grandparent_id) = parent.parent_branch_id {
                        result.push(grandparent_id);
                    }
                }
            }
        }
        result
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

impl ReplayAction {
    /// Create a new ReplayAction with default values
    pub fn new(action_type: ReplayActionType) -> Self {
        Self {
            action_type,
            player_index: 0,
            data: String::new(),
            turn: 0,
            phase: String::new(),
        }
    }

    /// Set the player index
    pub fn with_player(mut self, player_index: usize) -> Self {
        self.player_index = player_index;
        self
    }

    /// Set the data string
    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }

    /// Set the turn number
    pub fn with_turn(mut self, turn: u32) -> Self {
        self.turn = turn;
        self
    }

    /// Set the phase
    pub fn with_phase(mut self, phase: String) -> Self {
        self.phase = phase;
        self
    }
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

/// Resource to store queued save events
#[derive(Resource, Default)]
pub struct SaveEvents {
    /// List of save events to process
    pub events: Vec<SaveGameEvent>,
}
