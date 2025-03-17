use bevy::prelude::*;

use crate::game_engine::save::{AutoSaveTracker, SaveConfig};

use crate::game_engine::save::{
    // Remove duplicated imports that cause shadowing
    // AutoSaveTracker, // Shadowed by utils::*
    CheckStateBasedActionsEvent,
    LoadGameEvent,
    // SaveConfig, // Shadowed by utils::*
    SaveGameEvent,
    SaveLoadPlugin,
};

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

/// Resource to track which events were registered for testing
#[derive(Resource, Default)]
pub struct EventVerification {
    pub save_verified: bool,
    pub load_verified: bool,
    pub sba_verified: bool,
}

// Keep the original basic tests for plugin registration
#[test]
fn test_save_load_plugin_registers_systems() {
    // Create a test app with the SaveLoadPlugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Create resources to store verification state
    app.insert_resource(EventVerification {
        save_verified: false,
        load_verified: false,
        sba_verified: false,
    });

    // Add system to track events
    app.add_systems(
        Update,
        |mut save_events: EventReader<SaveGameEvent>,
         mut load_events: EventReader<LoadGameEvent>,
         mut sba_events: EventReader<CheckStateBasedActionsEvent>,
         mut verification: ResMut<EventVerification>| {
            if !save_events.read().collect::<Vec<_>>().is_empty() {
                verification.save_verified = true;
            }
            if !load_events.read().collect::<Vec<_>>().is_empty() {
                verification.load_verified = true;
            }
            if !sba_events.read().collect::<Vec<_>>().is_empty() {
                verification.sba_verified = true;
            }
        },
    );

    // Send test events
    app.world_mut().send_event(SaveGameEvent {
        slot_name: "test".to_string(),
    });
    app.world_mut().send_event(LoadGameEvent {
        slot_name: "test".to_string(),
    });
    app.world_mut().send_event(CheckStateBasedActionsEvent);

    // Run the systems
    app.update();

    // Verify events were registered
    let verification = app.world().resource::<EventVerification>();
    assert!(
        verification.save_verified,
        "SaveGameEvent was not properly registered"
    );
    assert!(
        verification.load_verified,
        "LoadGameEvent was not properly registered"
    );
    assert!(
        verification.sba_verified,
        "CheckStateBasedActionsEvent was not properly registered"
    );
}

// Basic test for auto-save triggers
#[test]
fn test_auto_save_triggers() {
    // Create a test app with the SaveLoadPlugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Since SaveLoadPlugin adds SaveConfig in its setup_save_system startup system,
    // we need to run the app once for the resource to be added
    app.update();

    // Verify the resource was added by the plugin's setup system
    assert!(
        app.world().contains_resource::<SaveConfig>(),
        "SaveConfig resource should be added by the SaveLoadPlugin"
    );
    assert!(
        app.world().contains_resource::<AutoSaveTracker>(),
        "AutoSaveTracker resource should be added by the SaveLoadPlugin"
    );

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
    app.world_mut().send_event(CheckStateBasedActionsEvent);

    // Run the systems
    app.update();
}

fn assert_save_event_triggered(mut reader: EventReader<SaveGameEvent>) {
    let events: Vec<_> = reader.read().collect();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].slot_name, "auto_save");
}

// This test must be run with --test-threads=1 because it modifies the file system
#[test]
#[ignore = "This test modifies the file system and should be run with --test-threads=1"]
fn test_save_load_integration() {
    use crate::game_engine::state::GameState;
    use crate::player::Player;
    use std::collections::VecDeque;
    use std::fs;
    use std::path::Path;

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
        .world_mut()
        .spawn(Player {
            name: "Test Player 1".to_string(),
            life: 40,
            mana_pool: crate::mana::ManaPool::default(),
            player_index: 0,
        })
        .id();

    let player2 = app
        .world_mut()
        .spawn(Player {
            name: "Test Player 2".to_string(),
            life: 35,
            mana_pool: crate::mana::ManaPool::default(),
            player_index: 1,
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
    app.world_mut().send_event(SaveGameEvent {
        slot_name: "test_save".to_string(),
    });

    // Run the systems to process the save event
    app.update();

    // Verify the save file exists
    assert!(Path::new(&format!("{}/test_save.bin", test_save_dir)).exists());

    // Now modify the game state (turn number and active player)
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.turn_number = 10;
        game_state.active_player = player2;
    }

    // Trigger a load
    app.world_mut().send_event(LoadGameEvent {
        slot_name: "test_save".to_string(),
    });

    // Run the systems to process the load event
    app.update();

    // Verify the game state was restored
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.turn_number, 5);

    // Clean up
    fs::remove_dir_all(test_save_dir).unwrap();
}
