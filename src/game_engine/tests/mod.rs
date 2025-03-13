// Re-export test utilities for use in other modules
pub mod assertions;
pub mod common;
pub mod fixtures;
pub mod visual_diff;

pub use assertions::*;
pub use common::*;
pub use fixtures::*;
pub use visual_diff::{
    ComparisonMethod, ComparisonResult, VisualTestConfig, VisualTestingPlugin,
    capture_entity_rendering, compare_images, load_reference_image, save_difference_visualization,
    save_reference_image, take_screenshot,
};

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
        .add_plugin(visual_diff::VisualTestingPlugin)
        .add_systems(Startup, setup_visual_test_scene);
}

/// Sets up a visual test scene with standard elements
fn setup_visual_test_scene() {
    // Placeholder for visual test scene setup
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
