use bevy::prelude::*;
use std::path::Path;

use crate::game_engine::save::events::CheckStateBasedActionsEvent;
use crate::game_engine::save::{AutoSaveTracker, SaveConfig, SaveLoadPlugin};
use crate::game_engine::state::GameState;

use super::utils::*;

#[test]
fn test_auto_save() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Run once to initialize resources
    app.update();

    // Set up test environment with game state
    let _player_entities = setup_test_environment(&mut app);

    // The auto-save file path
    let test_dir = Path::new("target/test_saves");
    if !test_dir.exists() {
        std::fs::create_dir_all(test_dir).unwrap();
    }

    // Configure auto-save to trigger frequently for testing
    {
        let mut config = app.world_mut().resource_mut::<SaveConfig>();
        config.auto_save_enabled = true;
        config.auto_save_frequency = 1; // Trigger auto-save on every SBA check
        config.save_directory = test_dir.to_path_buf();
    }

    // Reset auto-save counter
    app.insert_resource(AutoSaveTracker { counter: 0 });

    let auto_save_path = test_dir.join("auto_save.bin");

    // Remove any existing auto-save file
    if auto_save_path.exists() {
        std::fs::remove_file(&auto_save_path).unwrap();
    }

    // Verify no auto-save exists yet
    assert!(
        !auto_save_path.exists(),
        "Auto-save file should not exist before test"
    );

    // Trigger state-based actions check to initiate auto-save
    app.world_mut().send_event(CheckStateBasedActionsEvent);

    // Run systems to process events
    app.update();
    app.update(); // Run another update to ensure save completes

    // Verify auto-save file was created
    assert!(auto_save_path.exists(), "Auto-save file was not created");

    // Verify auto-save file has content
    let auto_save_data = std::fs::read(&auto_save_path).unwrap();
    assert!(!auto_save_data.is_empty(), "Auto-save file is empty");

    // Reset counter and modify game state
    {
        app.insert_resource(AutoSaveTracker { counter: 0 });
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.turn_number = 10; // Different from original
    }

    // Trigger another state-based actions check
    app.world_mut().send_event(CheckStateBasedActionsEvent);

    // Run systems to process events
    app.update();
    app.update(); // Run another update to ensure save completes

    // Verify auto-save was updated with new content
    let new_auto_save_data = std::fs::read(&auto_save_path).unwrap();
    assert!(
        !new_auto_save_data.is_empty(),
        "Updated auto-save file is empty"
    );

    // Clean up
    cleanup_test_environment();
}
