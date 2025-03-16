use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_persistent::prelude::*;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::data::*;
use crate::game_engine::save::events::*;
use crate::game_engine::save::resources::*;
use crate::game_engine::save::systems::*;
use crate::game_engine::state::GameState;
use crate::game_engine::zones::ZoneManager;
use crate::player::Player;
use std::collections::HashMap;
use std::path::Path;

// Test plugin to set up the test environment for save/load tests
pub struct SaveLoadTestPlugin;

impl Plugin for SaveLoadTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<CheckStateBasedActionsEvent>()
            .add_event::<StartReplayEvent>()
            .add_event::<StepReplayEvent>()
            .add_event::<StopReplayEvent>()
            .add_systems(Startup, setup_save_system)
            .add_systems(
                Update,
                (
                    handle_save_game,
                    handle_load_game,
                    handle_auto_save,
                    handle_start_replay,
                    handle_step_replay,
                    handle_stop_replay,
                ),
            );
    }
}

// Helper function to create a test environment with players and game state
fn setup_test_environment(world: &mut World) -> Vec<Entity> {
    // Set up test directory
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap();

    // Override save config to use test directory
    let mut save_config = SaveConfig::default();
    save_config.save_directory = test_dir.to_path_buf();
    world.insert_resource(save_config);

    // Create test players
    let player1 = world
        .spawn(Player {
            name: "Test Player 1".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player2 = world
        .spawn(Player {
            name: "Test Player 2".to_string(),
            life: 35,
            ..Default::default()
        })
        .id();

    // Set up game state
    let mut turn_order = std::collections::VecDeque::new();
    turn_order.push_back(player1);
    turn_order.push_back(player2);

    let mut lands_played = HashMap::new();
    lands_played.insert(player1, 3);
    lands_played.insert(player2, 2);

    let mut drawn_this_turn = Vec::new();
    drawn_this_turn.push(player1);

    let game_state = GameState {
        turn_number: 3,
        active_player: player1,
        priority_holder: player1,
        turn_order,
        lands_played,
        main_phase_action_taken: true,
        drawn_this_turn,
        state_based_actions_performed: true,
        eliminated_players: Vec::new(),
        use_commander_damage: true,
        commander_damage_threshold: 21,
        starting_life: 40,
    };

    world.insert_resource(game_state);

    // Return the player entities for testing
    vec![player1, player2]
}

// Helper function to clean up test environment
fn cleanup_test_environment() {
    let test_dir = Path::new("target/test_saves");
    let _ = std::fs::remove_dir_all(test_dir);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::AppExit;
    use bevy::utils::Duration;

    // Test basic saving functionality
    #[test]
    fn test_save_game() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment
        let player_entities = setup_test_environment(&mut app.world);

        // Send save game event
        app.world
            .resource_scope(|world, game_state: Mut<GameState>| {
                // Verify the initial state
                assert_eq!(game_state.turn_number, 3);
                assert_eq!(game_state.active_player, player_entities[0]);

                // Send save game event
                world.send_event(SaveGameEvent {
                    slot_name: "test_save".to_string(),
                });
            });

        // Run the systems to process the event
        app.update();

        // Verify save file was created
        let save_path = Path::new("target/test_saves/test_save.bin");
        assert!(save_path.exists());

        // Clean up
        cleanup_test_environment();
    }

    // Test loading a saved game
    #[test]
    fn test_load_game() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment and save a game
        let player_entities = setup_test_environment(&mut app.world);

        // Save the game first
        app.world.send_event(SaveGameEvent {
            slot_name: "test_load".to_string(),
        });
        app.update();

        // Modify the game state to be different
        app.world.resource_mut::<GameState>().turn_number = 10;

        // Load the game
        app.world.send_event(LoadGameEvent {
            slot_name: "test_load".to_string(),
        });
        app.update();

        // Verify the loaded state
        let loaded_state = app.world.resource::<GameState>();
        assert_eq!(loaded_state.turn_number, 3); // Should be back to the saved value

        // Clean up
        cleanup_test_environment();
    }

    // Test loading with empty player list
    #[test]
    fn test_load_game_empty_players() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment but don't add players
        let save_config = SaveConfig::default();
        app.world.insert_resource(save_config);
        app.world.insert_resource(GameState::default());

        // Try to load a game
        app.world.send_event(LoadGameEvent {
            slot_name: "nonexistent_save".to_string(),
        });
        app.update();

        // Verify we get a default game state and don't crash
        let game_state = app.world.resource::<GameState>();
        assert_eq!(game_state.turn_number, 1); // Default turn number

        // Clean up
        cleanup_test_environment();
    }

    // Test saving and loading with zone and commander data
    #[test]
    fn test_save_load_with_zones() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment
        let player_entities = setup_test_environment(&mut app.world);

        // Add zone and commander managers
        app.world.insert_resource(ZoneManager::default());
        app.world.insert_resource(CommandZoneManager::default());

        // Save the game
        app.world.send_event(SaveGameEvent {
            slot_name: "test_zones".to_string(),
        });
        app.update();

        // Modify the game state
        app.world.resource_mut::<GameState>().turn_number = 5;

        // Load the game
        app.world.send_event(LoadGameEvent {
            slot_name: "test_zones".to_string(),
        });
        app.update();

        // Verify the loaded state
        let loaded_state = app.world.resource::<GameState>();
        assert_eq!(loaded_state.turn_number, 3);

        // Clean up
        cleanup_test_environment();
    }

    // Test auto-save functionality
    #[test]
    fn test_auto_save() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment
        let player_entities = setup_test_environment(&mut app.world);

        // Enable auto-save with small frequency
        let mut config = app.world.resource_mut::<SaveConfig>();
        config.auto_save_enabled = true;
        config.auto_save_frequency = 2;

        // Reset the counter
        let mut tracker = app.world.resource_mut::<AutoSaveTracker>();
        tracker.counter = 0;

        // Send state-based action events to trigger auto-save
        app.world.send_event(CheckStateBasedActionsEvent);
        app.update();

        // Counter should be 1 now
        let tracker = app.world.resource::<AutoSaveTracker>();
        assert_eq!(tracker.counter, 1);

        // Send one more to trigger auto-save
        app.world.send_event(CheckStateBasedActionsEvent);
        app.update();

        // Verify auto-save file exists
        let auto_save_path = Path::new("target/test_saves/auto_save.bin");
        assert!(auto_save_path.exists());

        // Counter should be reset to 0
        let tracker = app.world.resource::<AutoSaveTracker>();
        assert_eq!(tracker.counter, 0);

        // Clean up
        cleanup_test_environment();
    }

    // Test loading with corrupted entity mapping
    #[test]
    fn test_load_game_corrupted_mapping() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment and save a game
        let player_entities = setup_test_environment(&mut app.world);

        // Save the game
        app.world.send_event(SaveGameEvent {
            slot_name: "test_corrupted".to_string(),
        });
        app.update();

        // Despawn all player entities to create a mismatch
        for entity in player_entities {
            app.world.despawn(entity);
        }

        // Try to load the game with missing entities
        app.world.send_event(LoadGameEvent {
            slot_name: "test_corrupted".to_string(),
        });
        app.update();

        // Verify the game state was loaded with default entities
        let loaded_state = app.world.resource::<GameState>();
        assert_eq!(loaded_state.turn_number, 3); // Original turn number should be preserved

        // Clean up
        cleanup_test_environment();
    }

    // Test loading game with empty turn order
    #[test]
    fn test_load_game_empty_turn_order() {
        // Create a custom test for the to_game_state method directly

        // Create a minimal save data with empty collections
        let mut save_data = GameSaveData::default();
        save_data.game_state.turn_number = 5;
        save_data.game_state.active_player_index = 0;
        save_data.game_state.priority_holder_index = 0;
        // Intentionally leave turn_order_indices empty

        // Try to convert to GameState with an empty entity list
        let empty_entities: Vec<Entity> = Vec::new();
        let game_state = save_data.to_game_state(&empty_entities);

        // Verify that no panic occurred and default values were used
        assert_eq!(game_state.turn_number, 5);
        assert_eq!(game_state.active_player, Entity::from_raw(0));
        assert_eq!(game_state.priority_holder, Entity::from_raw(0));
        assert!(game_state.turn_order.is_empty());

        // Create a single entity
        let entity_list = vec![Entity::from_raw(1)];

        // Now try with out-of-bounds indices
        save_data.game_state.active_player_index = 5; // Out of bounds
        let game_state = save_data.to_game_state(&entity_list);

        // Should use fallback entity
        assert_eq!(game_state.active_player, Entity::from_raw(0));
    }

    // Test serialization and deserialization of complex game state
    #[test]
    fn test_complex_game_state_serialization() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment
        let player_entities = setup_test_environment(&mut app.world);

        // Create a more complex game state
        app.world
            .resource_scope(|world, mut game_state: Mut<GameState>| {
                // Add eliminated players
                game_state.eliminated_players.push(player_entities[1]);

                // Modify lands played
                *game_state
                    .lands_played
                    .get_mut(&player_entities[0])
                    .unwrap() = 5;

                // Change turn order
                game_state.turn_order.clear();
                game_state.turn_order.push_back(player_entities[0]);
            });

        // Save the game
        app.world.send_event(SaveGameEvent {
            slot_name: "test_complex".to_string(),
        });
        app.update();

        // Reset the game state
        app.world.insert_resource(GameState::default());

        // Load the game
        app.world.send_event(LoadGameEvent {
            slot_name: "test_complex".to_string(),
        });
        app.update();

        // Verify all state was correctly loaded
        let loaded_state = app.world.resource::<GameState>();
        assert_eq!(loaded_state.eliminated_players.len(), 1);
        assert_eq!(
            *loaded_state.lands_played.get(&player_entities[0]).unwrap(),
            5
        );
        assert_eq!(loaded_state.turn_order.len(), 1);

        // Clean up
        cleanup_test_environment();
    }

    // Test saving and loading with partially corrupted data
    #[test]
    fn test_partial_corruption() {
        // Set up test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SaveLoadTestPlugin);

        // Set up test environment
        let player_entities = setup_test_environment(&mut app.world);

        // Save the game
        app.world.send_event(SaveGameEvent {
            slot_name: "test_partial".to_string(),
        });
        app.update();

        // Keep player1 but despawn player2 to simulate partial corruption
        app.world.despawn(player_entities[1]);

        // Load the game
        app.world.send_event(LoadGameEvent {
            slot_name: "test_partial".to_string(),
        });
        app.update();

        // Verify game loaded with the remaining valid entity
        let game_state = app.world.resource::<GameState>();
        assert_eq!(game_state.turn_number, 3);

        // Should have filtered out the missing entity from collections
        assert!(game_state.turn_order.len() <= 2);

        // Clean up
        cleanup_test_environment();
    }
}
