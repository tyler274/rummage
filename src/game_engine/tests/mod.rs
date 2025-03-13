// Re-export test utilities for use in other modules
pub mod assertions;
pub mod common;
pub mod fixtures;

use bevy::prelude::Entity;

pub use assertions::*;
pub use common::*;
pub use fixtures::*;

// Re-export visual testing functionality
pub use crate::game_engine::visual_testing::capture::{capture_entity_rendering, take_screenshot};
pub use crate::game_engine::visual_testing::comparison::{
    ComparisonResult, compare_images, save_difference_visualization,
};
pub use crate::game_engine::visual_testing::config::{
    ComparisonMethod, VisualTestConfig, VisualTestingPlugin,
};
pub use crate::game_engine::visual_testing::fixtures::{
    generate_reference_images, setup_animation_keyframe, setup_animation_test, setup_card_state,
    setup_test_scene, setup_ui_state, setup_ui_test_scene, setup_visual_test_fixtures,
};
pub use crate::game_engine::visual_testing::utils::{load_reference_image, save_reference_image};

/// Sets up a minimal test environment for game engine testing
pub fn setup_test_environment(app: &mut bevy::app::App) {
    use bevy::prelude::*;

    app.add_plugins(MinimalPlugins)
        .add_systems(Startup, setup_test_game);
}

/// Sets up a basic test game
fn setup_test_game() {
    // Placeholder for test game setup
}

/// Sets up a test environment with visual testing support
pub fn setup_visual_test_environment(app: &mut bevy::app::App) {
    use bevy::prelude::*;

    app.add_plugins(MinimalPlugins)
        .add_plugins(crate::game_engine::visual_testing::config::VisualTestingPlugin)
        .add_systems(
            Startup,
            crate::game_engine::visual_testing::fixtures::setup_test_scene,
        );
}

/// Test states for use in game engine testing
pub const TEST_STATES: &[&str] = &[
    "empty_board",
    "basic_creatures",
    "complex_board",
    "stack_multiple_items",
    "combat_phase",
];

/// Standard test deck for use in tests
pub const TEST_DECK: &str = "test_decks/standard_test_deck.toml";

/// Runs a complete test turn cycle
pub fn run_test_turn(_app: &mut bevy::app::App) {
    use crate::game_engine::phase::{ActivePlayer, MAIN1, MAIN2};
    use crate::game_engine::phase::{BeginningStep, CombatStep, CurrentPhase, EndingStep, Phase};

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
