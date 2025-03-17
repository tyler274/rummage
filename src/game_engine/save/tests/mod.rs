use bevy::prelude::*;
use bevy::utils::Duration;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::{
    AutoSaveTracker, CheckStateBasedActionsEvent, LoadGameEvent, SaveConfig, SaveGameEvent,
    SaveLoadPlugin, SaveMetadata,
};
use crate::game_engine::state::GameState;
use crate::player::Player;

mod auto_save;
mod complex_game_state_serialization;
mod load_game;
mod load_game_corrupted_mapping;
mod load_game_empty_players;
mod load_game_empty_turn_order;
mod partial_corruption;
mod save_game;
mod save_load_with_zones;
mod utils;

// Re-export test utilities
pub use utils::*;

// Keep the original basic tests for plugin registration
#[test]
fn test_save_load_plugin_registers_systems() {
    // Create a test app with the SaveLoadPlugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Verify systems were registered by checking for the event types
    assert!(app.is_event_added::<SaveGameEvent>());
    assert!(app.is_event_added::<LoadGameEvent>());
    assert!(app.is_event_added::<CheckStateBasedActionsEvent>());
}

// Basic test for auto-save triggers
#[test]
fn test_auto_save_triggers() {
    // Create a test app with the SaveLoadPlugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Verify default configuration
    assert!(app.world.contains_resource::<SaveConfig>());
    assert!(app.world.contains_resource::<AutoSaveTracker>());

    // Set auto-save to trigger on every check
    app.insert_resource(SaveConfig {
        save_directory: "test_saves".into(),
        auto_save_enabled: true,
        auto_save_frequency: 1,
    });

    // Reset counter
    app.insert_resource(AutoSaveTracker { counter: 0 });

    // Add save event reader for verification
    app.add_systems(Update, assert_save_event_triggered);

    // Trigger the state-based action check
    app.world.send_event(CheckStateBasedActionsEvent);

    // Run the systems
    app.update();
}

fn assert_save_event_triggered(mut reader: EventReader<SaveGameEvent>) {
    let events: Vec<_> = reader.read().collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].slot_name, "auto_save");
}

// This test must be run with --test-threads=1 because it modifies the file system
#[cfg(feature = "integration_tests")]
#[test]
fn test_save_load_integration() {
    use std::fs;

    // Create a test app
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set save directory for tests
    let test_save_dir = "test_save_integration";
    app.insert_resource(SaveConfig {
        save_directory: test_save_dir.into(),
        auto_save_enabled: false,
        auto_save_frequency: 999, // Disable auto-save
    });

    // Clean up test directory if it exists
    if Path::new(test_save_dir).exists() {
        fs::remove_dir_all(test_save_dir).unwrap();
    }

    // Create fake game state and players
    let player1 = app
        .world
        .spawn(Player {
            name: "Test Player 1".to_string(),
            life: 40,
        })
        .id();

    let player2 = app
        .world
        .spawn(Player {
            name: "Test Player 2".to_string(),
            life: 35,
        })
        .id();

    let mut turn_order = VecDeque::new();
    turn_order.push_back(player1);
    turn_order.push_back(player2);

    let game_state = GameState {
        turn_number: 5,
        active_player: player1,
        priority_holder: player1,
        turn_order,
        lands_played: vec![(player1, 2), (player2, 1)],
        main_phase_action_taken: true,
        drawn_this_turn: vec![player1, player2],
        state_based_actions_performed: false,
        eliminated_players: vec![],
        use_commander_damage: true,
        commander_damage_threshold: 21,
        starting_life: 40,
    };

    app.insert_resource(game_state);

    // Trigger a save
    app.world.send_event(SaveGameEvent {
        slot_name: "test_save".to_string(),
    });

    // Run the systems to process the save event
    app.update();

    // Verify the save file exists
    assert!(Path::new(&format!("{}/test_save.bin", test_save_dir)).exists());

    // Now modify the game state (turn number and active player)
    let mut game_state = app.world.resource_mut::<GameState>();
    game_state.turn_number = 10;
    game_state.active_player = player2;

    // Trigger a load
    app.world.send_event(LoadGameEvent {
        slot_name: "test_save".to_string(),
    });

    // Run the systems to process the load event
    app.update();

    // Verify the game state was restored
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.turn_number, 5);

    // Clean up
    fs::remove_dir_all(test_save_dir).unwrap();
}
