use crate::game_engine::phase::{ActivePlayer, CurrentPhase, Phase};
use crate::game_engine::stack::GameStack;
use crate::game_engine::zones::{Zone, ZoneMarker};
use bevy::prelude::*;

/// Common test utilities for game engine testing

/// Setup test logging
pub fn setup_test_logger() -> () {
    // Configure test-specific logging
}

/// Test game configuration
#[derive(Debug, Clone)]
pub struct TestGameConfig {
    /// Number of players
    pub player_count: usize,
    /// Starting life total
    pub starting_life: i32,
    /// Starting hand size
    pub starting_hand_size: usize,
    /// Whether to enable politics features
    pub politics_enabled: bool,
    /// Test seed for RNG
    pub test_seed: u64,
}

impl Default for TestGameConfig {
    fn default() -> Self {
        Self {
            player_count: 4,
            starting_life: 40,
            starting_hand_size: 7,
            politics_enabled: true,
            test_seed: 12345, // Deterministic seed for tests
        }
    }
}

/// Utility to get active player
pub fn get_active_player(_app: &App) -> Entity {
    // Default fallback entity
    let default_entity = Entity::from_raw(0);

    // Use a simpler approach - just find the first entity with ActivePlayer
    let mut found = None;
    let world = _app.world();

    // Manual iteration to avoid borrowing issues
    for entity in world.iter_entities() {
        if world.get::<ActivePlayer>(entity.id()).is_some() {
            found = Some(entity.id());
            break;
        }
    }

    found.unwrap_or(default_entity)
}

/// Progress game to specific phase
pub fn progress_to_phase(_app: &mut App, _phase: Phase) {
    // Set the desired phase
    _app.world_mut().resource_mut::<CurrentPhase>().0 = _phase;

    // Run a game update to process the phase change
    _app.update();
}

/// Waits for all events to be processed
pub fn process_all_events(app: &mut App) {
    // Run extra update to ensure all events are processed
    app.update();
}

/// Resolves the entire stack
pub fn resolve_stack_completely(_app: &mut App) {
    // Access the stack resource
    let stack_empty = _app.world().resource::<GameStack>().is_empty();

    if !stack_empty {
        // Keep resolving until stack is empty
        while !_app.world().resource::<GameStack>().is_empty() {
            // Trigger stack resolution
            _app.update();
        }
    }
}

/// Gets a player's graveyard
pub fn get_player_graveyard(_app: &App, _player: Entity) -> Vec<Entity> {
    // Query the graveyard zone for the player
    let mut graveyard = Vec::new();
    let world = _app.world();

    // Manual iteration to avoid borrowing issues
    for entity in world.iter_entities() {
        if let Some(zone) = world.get::<ZoneMarker>(entity.id()) {
            if zone.zone_type == Zone::Graveyard && zone.owner == Some(_player) {
                graveyard.push(entity.id());
            }
        }
    }

    graveyard
}
