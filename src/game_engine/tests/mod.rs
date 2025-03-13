// Re-export test utilities for use in other modules
pub mod assertions;
pub mod common;
pub mod fixtures;

// Remove the unused imports but keep the modules available
// These modules are meant to be used directly in tests,
// not re-exported by default

/// Sets up a minimal test environment for game engine testing
#[allow(dead_code)]
pub fn setup_test_environment(app: &mut bevy::app::App) {
    use bevy::prelude::*;

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_test_game);
}

/// Sets up a basic test game
#[allow(dead_code)]
fn setup_test_game() {
    // Placeholder for test game setup
}

/// Test states for use in game engine testing
#[allow(dead_code)]
pub const TEST_STATES: &[&str] = &[
    "empty_board",
    "basic_creatures",
    "complex_board",
    "stack_multiple_items",
    "combat_phase",
];

/// Standard test deck for use in tests
#[allow(dead_code)]
pub const TEST_DECK: &str = "test_decks/standard_test_deck.toml";

/// Runs a complete test turn cycle
#[allow(dead_code)]
pub fn run_test_turn(_app: &mut bevy::app::App) {
    use crate::game_engine::phase::{ActivePlayer, MAIN1, MAIN2};
    use crate::game_engine::phase::{BeginningStep, CombatStep, CurrentPhase, EndingStep, Phase};
    use crate::game_engine::tests::common::{
        get_active_player, process_all_events, resolve_stack_completely,
    };

    // Define the turn sequence
    let phases = [
        Phase::Beginning(BeginningStep::Untap),
        MAIN1,
        Phase::Combat(CombatStep::Beginning),
        MAIN2,
        Phase::Ending(EndingStep::End),
    ];

    // Get the active player
    let active_player = get_active_player(_app);

    // Process each phase
    for phase in phases.iter() {
        // Set the current phase
        _app.world_mut().resource_mut::<CurrentPhase>().0 = *phase;

        // Run updates to process the phase
        _app.update();

        // Process any stack effects if needed
        resolve_stack_completely(_app);

        // Make sure all events are processed
        process_all_events(_app);
    }

    // After completing the turn, pass to next player
    // This would access the turn order system to pass to the next player
    let mut players = Vec::new();

    // Collect entities in a way that avoids borrow issues
    for entity in _app.world().iter_entities() {
        players.push(entity.id());
    }

    if !players.is_empty() {
        // Find the current active player index
        let current_idx = players
            .iter()
            .position(|&p| p == active_player)
            .unwrap_or(0);
        let next_idx = (current_idx + 1) % players.len();

        // Remove active player marker from current
        _app.world_mut()
            .entity_mut(active_player)
            .remove::<ActivePlayer>();

        // Add to next player
        _app.world_mut()
            .entity_mut(players[next_idx])
            .insert(ActivePlayer);
    }

    // Reset to beginning phase for next turn
    _app.world_mut().resource_mut::<CurrentPhase>().0 = Phase::Beginning(BeginningStep::Untap);
    _app.update();
}
