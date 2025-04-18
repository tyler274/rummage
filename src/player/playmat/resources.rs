// Add necessary bevy imports
use bevy::prelude::*;
// Add project-specific imports
use crate::game_engine::zones::Zone;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Resource for tracking which zone is currently focused
#[derive(Resource, Default)]
pub struct ZoneFocusState {
    /// The entity of the currently focused zone, if any
    pub focused_zone: Option<Entity>,
    /// The type of zone being focused
    pub focused_zone_type: Option<Zone>,
    /// The player entity who owns the focused zone
    pub focused_zone_owner: Option<Entity>,
}

/// Resource to track playmat debug state to prevent log spam
#[derive(Resource, Default)]
pub struct PlaymatDebugState {
    /// A hash of the last logged state for each player to prevent duplicate logs
    last_logged_states: std::collections::HashMap<Entity, u64>,
}

impl PlaymatDebugState {
    /// Check if the state has changed and should be logged
    pub fn should_log(
        &mut self,
        player_entity: Entity,
        player_name: &str,
        player_index: usize,
    ) -> bool {
        // Create a simple hash from the player state
        let mut hasher = DefaultHasher::new();
        player_name.hash(&mut hasher);
        player_index.hash(&mut hasher);
        let new_hash = hasher.finish();

        // Check if state has changed
        let state_changed = self.last_logged_states.get(&player_entity) != Some(&new_hash);

        // Update state if changed
        if state_changed {
            self.last_logged_states.insert(player_entity, new_hash);
        }

        state_changed
    }
}

/// Resource for tracking the current game phase for UI layout purposes
#[derive(Resource, Default)]
pub struct CurrentPhaseLayout {
    /// The current game phase affecting UI layout
    pub phase: GamePhase,
    /// Whether the layout needs to be updated
    pub needs_update: bool,
}

/// Enum representing game phases for UI layout purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GamePhase {
    #[default]
    Main,
    #[allow(dead_code)]
    Combat,
    #[allow(dead_code)]
    Drawing,
    #[allow(dead_code)]
    Searching,
}

// Add a helper method to get the phase name for display
impl GamePhase {
    /// Get a display name for the current phase
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            GamePhase::Main => "Main Phase",
            GamePhase::Combat => "Combat Phase",
            GamePhase::Drawing => "Draw Phase",
            GamePhase::Searching => "Search Phase",
        }
    }
}
