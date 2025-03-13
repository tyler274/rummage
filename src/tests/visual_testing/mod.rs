// Visual Differential Testing system for Rummage
//
// This module provides tools for automated visual regression testing
// by capturing screenshots, comparing them against references, and
// visualizing differences when changes are detected.

pub mod capture;
pub mod comparison;
pub mod config;
pub mod diff;
pub mod examples;
pub mod fixtures;
pub mod utils;

// Re-export the most commonly used types and functions
pub use capture::{
    capture_entity_rendering, capture_screenshot_system, request_screenshot, take_screenshot,
};
pub use comparison::{ComparisonResult, compare_images, save_difference_visualization};
pub use config::{ComparisonMethod, VisualTestConfig, VisualTestingPlugin};
pub use fixtures::{
    generate_reference_images, setup_animation_keyframe, setup_animation_test, setup_card_state,
    setup_test_scene, setup_ui_state, setup_ui_test_scene, setup_visual_test_fixtures,
};
pub use utils::{load_reference_image, save_reference_image};

// Standard test states
pub const CARD_TEST_STATES: &[&str] = &[
    "card_normal",
    "card_tapped",
    "card_highlighted",
    "card_attacking",
    "card_blocking",
    "card_with_counters",
    "card_with_attachments",
    "card_foil",
];

pub const UI_TEST_STATES: &[&str] = &[
    "menu_main",
    "menu_options",
    "game_empty_board",
    "game_complex_board",
    "dialog_confirm",
    "dialog_choose_cards",
    "dialog_stack",
    "dialog_targeting",
];

pub const ANIMATION_TESTS: &[(&str, i32)] = &[
    ("card_draw", 0),
    ("card_draw", 5),
    ("card_draw", 10),
    ("card_play", 0),
    ("card_play", 5),
    ("card_play", 10),
    ("attack", 0),
    ("attack", 5),
    ("attack", 10),
];
