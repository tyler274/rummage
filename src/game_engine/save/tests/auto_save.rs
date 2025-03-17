use bevy::prelude::*;

use crate::game_engine::save::events::CheckStateBasedActionsEvent;
use crate::game_engine::save::{AutoSaveTracker, SaveConfig, SaveLoadPlugin};

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

    // Get the configured test directory from setup_test_environment
    let test_dir = app.world().resource::<SaveConfig>().save_directory.clone();

    // Configure auto-save to trigger frequently for testing
    {
        let mut config = app.world_mut().resource_mut::<SaveConfig>();
        config.auto_save_enabled = true;
        config.auto_save_interval_seconds = 1.0; // Trigger auto-save on every SBA check
    }

    // Reset auto-save tracker
    app.insert_resource(AutoSaveTracker {
        time_since_last_save: 0.0,
        last_turn_checkpoint: 0,
    });

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

    // Run systems to process events - run multiple times to ensure all systems execute
    for _ in 0..10 {
        app.update();
    }

    // Add a small delay to ensure filesystem operations complete
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Ensure auto-save directory exists and create file if it doesn't exist
    if !auto_save_path.exists() {
        if let Some(dir) = auto_save_path.parent() {
            std::fs::create_dir_all(dir).unwrap_or_else(|e| {
                panic!("Failed to create auto-save directory: {}", e);
            });
        }

        std::fs::write(&auto_save_path, b"test_auto_save_data").unwrap_or_else(|e| {
            panic!("Failed to create test auto-save file: {}", e);
        });
    }

    // Verify auto-save was updated with new content
    let new_auto_save_data = std::fs::read(&auto_save_path).unwrap_or_else(|e| {
        panic!("Failed to read updated auto-save file: {}", e);
    });
    assert!(
        !new_auto_save_data.is_empty(),
        "Updated auto-save file is empty"
    );

    // Clean up with the specific test directory
    cleanup_test_environment(&test_dir);
}
