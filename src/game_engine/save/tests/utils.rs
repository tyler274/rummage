use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::game_engine::save::events::*;
use crate::game_engine::save::{
    AutoSaveTracker, GameSaveData, PlayerData, ReplayState, SaveConfig, SaveMetadata,
};
use crate::game_engine::state::GameState;
use crate::player::Player;

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
            .add_event::<crate::snapshot::SnapshotEvent>()
            .add_systems(Startup, setup_save_system_for_tests)
            .add_systems(
                Update,
                (
                    handle_save_game_for_tests,
                    handle_load_game_for_tests,
                    handle_auto_save_for_tests,
                    handle_start_replay,
                    handle_step_replay,
                    handle_stop_replay,
                ),
            );
    }
}

// Custom setup for tests to avoid file system conflicts with real save system
pub fn setup_save_system_for_tests(mut commands: Commands) {
    // Create test save directory if it doesn't exist
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap_or_else(|e| {
        error!("Failed to create test save directory: {}", e);
    });

    let config = SaveConfig {
        save_directory: test_dir.to_path_buf(),
        auto_save_enabled: false,
        auto_save_frequency: 999, // Set very high to prevent auto-saving during tests
        checkpoint_frequency: 5,
        history_size: 50,
    };

    commands.insert_resource(config);
    commands.insert_resource(AutoSaveTracker::default());
    commands.insert_resource(ReplayState::default());

    // Create and initialize save metadata
    commands.insert_resource(SaveMetadata::default());
}

// Custom auto-save handler for tests
pub fn handle_auto_save_for_tests(
    mut event_reader: EventReader<CheckStateBasedActionsEvent>,
    mut event_writer: EventWriter<SaveGameEvent>,
    mut auto_save_tracker: ResMut<AutoSaveTracker>,
    config: Res<SaveConfig>,
) {
    for _ in event_reader.read() {
        auto_save_tracker.counter += 1;

        if config.auto_save_enabled && auto_save_tracker.counter >= config.auto_save_frequency {
            // Trigger a save event
            event_writer.send(SaveGameEvent {
                slot_name: "auto_save".to_string(),
            });

            // Reset counter
            auto_save_tracker.counter = 0;
        }
    }
}

// Stub implementations for replay handlers
pub fn handle_start_replay(
    mut event_reader: EventReader<StartReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
) {
    for event in event_reader.read() {
        info!("Starting replay from slot: {}", event.slot_name);
        replay_state.active = true;
    }
}

pub fn handle_step_replay(
    mut event_reader: EventReader<StepReplayEvent>,
    replay_state: Res<ReplayState>,
) {
    for _ in event_reader.read() {
        if replay_state.active {
            info!("Stepping replay to next action");
        }
    }
}

pub fn handle_stop_replay(
    mut event_reader: EventReader<StopReplayEvent>,
    mut replay_state: ResMut<ReplayState>,
) {
    for _ in event_reader.read() {
        info!("Stopping replay");
        replay_state.active = false;
    }
}

// Common test environment setup for save/load tests
pub fn setup_test_environment(app: &mut App) -> Vec<Entity> {
    // Create a unique test directory name using process ID and timestamp
    // This ensures each test run (even in parallel) gets its own directory
    let unique_id = std::process::id();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    // Create a PathBuf directly instead of using Path::new on a temporary string
    let test_dir_name = format!("target/test_saves_{}_{}", unique_id, timestamp);
    let test_dir = PathBuf::from(test_dir_name);

    std::fs::create_dir_all(&test_dir).unwrap_or_else(|e| {
        error!("Failed to create test save directory: {}", e);
    });

    // Set up SaveConfig for tests
    let config = SaveConfig {
        save_directory: test_dir,
        auto_save_enabled: false,
        auto_save_frequency: 999, // Set very high to prevent auto-saving during tests
        checkpoint_frequency: 5,
        history_size: 50,
    };
    app.insert_resource(config);
    app.insert_resource(AutoSaveTracker::default());
    app.insert_resource(ReplayState::default());
    app.insert_resource(SaveMetadata::default());

    // Register SnapshotEvent for testing
    app.add_event::<crate::snapshot::SnapshotEvent>();

    // Create test players
    let player1 = app
        .world_mut()
        .spawn(Player {
            name: "Player 1".to_string(),
            life: 40,
            mana_pool: crate::mana::ManaPool::default(),
            player_index: 0,
        })
        .id();

    let player2 = app
        .world_mut()
        .spawn(Player {
            name: "Player 2".to_string(),
            life: 35,
            mana_pool: crate::mana::ManaPool::default(),
            player_index: 1,
        })
        .id();

    // Create turn order
    let mut turn_order = VecDeque::new();
    turn_order.push_back(player1);
    turn_order.push_back(player2);

    // Create initial game state data
    let lands_played = vec![(player1, 1), (player2, 1)];
    let drawn_this_turn = vec![player1, player2];

    // GameState using builder pattern
    let game_state = GameState::builder()
        .turn_number(3)
        .active_player(player1)
        .priority_holder(player1)
        .turn_order(turn_order)
        .lands_played(lands_played)
        .main_phase_action_taken(true)
        .drawn_this_turn(drawn_this_turn)
        .state_based_actions_performed(true)
        .eliminated_players(Vec::new())
        .use_commander_damage(true)
        .commander_damage_threshold(21)
        .starting_life(40)
        .build();

    app.insert_resource(game_state);

    // Return the player entities for testing
    vec![player1, player2]
}

// Helper function to clean up test environment with the specific directory
pub fn cleanup_test_environment(test_dir: &Path) {
    if test_dir.exists() {
        let _ = std::fs::remove_dir_all(test_dir);
        info!("Cleaned up test directory: {:?}", test_dir);
    }
}

// Legacy version for backward compatibility with existing tests
pub fn cleanup_default_test_environment() {
    let test_dir = Path::new("target/test_saves");
    let _ = std::fs::remove_dir_all(test_dir);
    info!("Cleaned up default test directory: {:?}", test_dir);
}

// // Provides a shared function for backward compatibility to get the default test dir
// pub fn get_default_test_dir() -> &'static Path {
//     Path::new("target/test_saves")
// }

// Alias for the old cleanup function signature to handle existing tests
pub fn cleanup_test_environment_compat() {
    cleanup_default_test_environment();
}

// Mock game save data structures for testing
#[derive(Debug, Serialize, Deserialize, Default, Clone, Resource)]
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
}

// Mock save handler for tests using bevy_persistent
pub fn handle_save_game_for_tests(
    mut event_reader: EventReader<SaveGameEvent>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    _commands: Commands,
) {
    for event in event_reader.read() {
        info!("Saving game to slot: {}", event.slot_name);

        let save_path = Path::new("target/test_saves").join(format!("{}.bin", event.slot_name));

        // Create test save data using the builder pattern
        let save_data = GameSaveData::builder()
            .game_state(
                crate::game_engine::save::GameStateData::builder()
                    .turn_number(game_state.turn_number)
                    .active_player_index(0)
                    .priority_holder_index(0)
                    .turn_order_indices(vec![0, 1])
                    .lands_played(vec![(0, 1), (1, 1)])
                    .main_phase_action_taken(true)
                    .drawn_this_turn(vec![0, 1])
                    .eliminated_players(vec![])
                    .use_commander_damage(true)
                    .commander_damage_threshold(21)
                    .starting_life(40)
                    .build(),
            )
            .players(
                query_players
                    .iter()
                    .enumerate()
                    .map(|(idx, (_, player))| {
                        PlayerData::builder()
                            .id(idx)
                            .name(player.name.clone())
                            .life(player.life)
                            .mana_pool(player.mana_pool.clone())
                            .player_index(idx)
                            .build()
                    })
                    .collect(),
            )
            .save_version("1.0".to_string())
            .zones(Default::default())
            .commanders(Default::default())
            .build();

        // Create persistent resource for this save
        let persistent_save = Persistent::builder()
            .name(&format!("test_save_{}", event.slot_name))
            .format(StorageFormat::Bincode)
            .path(save_path)
            .default(save_data)
            .build()
            .expect("Failed to create persistent save");

        // Persist immediately
        if let Err(e) = persistent_save.persist() {
            error!("Test save failed: {}", e);
        }
    }
}

// Mock load handler for testing using bevy_persistent
pub fn handle_load_game_for_tests(
    mut event_reader: EventReader<LoadGameEvent>,
    mut game_state: Option<ResMut<GameState>>,
    _query_players: Query<(Entity, &mut Player)>,
) {
    for event in event_reader.read() {
        info!("Loading game from slot: {}", event.slot_name);

        let save_path = Path::new("target/test_saves").join(format!("{}.bin", event.slot_name));

        if save_path.exists() {
            // Create persistent resource to load the save
            let persistent_save = Persistent::<GameSaveData>::builder()
                .name(&format!("test_load_{}", event.slot_name))
                .format(StorageFormat::Bincode)
                .path(save_path)
                .default(GameSaveData::default())
                .build()
                .expect("Failed to create persistent load");

            let save_data = persistent_save.clone();

            if let Some(game_state) = game_state.as_mut() {
                game_state.turn_number = save_data.game_state.turn_number;
            }

            // Update players - in a real implementation, we would convert
            // the save data back to player entities
        }
    }
}

// Helper function to add a game camera for tests
pub fn add_test_game_camera(app: &mut App) -> Entity {
    let camera_entity = app
        .world_mut()
        .spawn((
            Camera2d::default(),
            crate::camera::components::GameCamera,
            Name::new("Test Game Camera"),
        ))
        .id();

    info!("Added test game camera: {:?}", camera_entity);
    camera_entity
}
