// Re-export test utilities for use in other modules
pub mod assertions;
pub mod common;
pub mod fixtures;

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
        .add_plugin(crate::game_engine::visual_testing::config::VisualTestingPlugin)
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
pub fn run_test_turn(app: &mut bevy::app::App) {
    // Placeholder for running a full turn cycle
}
