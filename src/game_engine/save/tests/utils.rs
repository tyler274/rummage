use bevy::prelude::*;
use bevy_persistent::prelude::*;
use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::events::*;
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
        auto_save_enabled: true,
        auto_save_frequency: 2,
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
    if !config.auto_save_enabled {
        return;
    }

    // Only increment counter when state-based actions are checked
    for _ in event_reader.read() {
        auto_save_tracker.counter += 1;

        // Check if it's time to auto-save
        if auto_save_tracker.counter >= config.auto_save_frequency {
            info!("Auto-saving game...");
            event_writer.send(SaveGameEvent {
                slot_name: "auto_save".to_string(),
            });
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

// Helper function to create a test environment with players and game state
pub fn setup_test_environment(app: &mut App) -> Vec<Entity> {
    // Set up test directory
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap();

    // Remove any existing metadata file to prevent deserialization errors
    let metadata_path = test_dir.join("metadata.bin");
    if metadata_path.exists() {
        std::fs::remove_file(&metadata_path).unwrap();
    }

    // Clear old save files from test directory
    for entry in std::fs::read_dir(test_dir).unwrap() {
        if let Ok(entry) = entry {
            std::fs::remove_file(entry.path()).unwrap_or_else(|_| {});
        }
    }

    // Override save config to use test directory
    let save_config = SaveConfig {
        save_directory: test_dir.to_path_buf(),
        auto_save_enabled: true,
        auto_save_frequency: 2,
    };
    app.insert_resource(save_config);

    // Insert other required resources
    app.insert_resource(AutoSaveTracker::default());
    app.insert_resource(ReplayState::default());
    app.insert_resource(SaveMetadata::default());

    // Create test players
    let player1 = app
        .world()
        .spawn(Player {
            name: "Test Player 1".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player2 = app
        .world()
        .spawn(Player {
            name: "Test Player 2".to_string(),
            life: 35,
            ..Default::default()
        })
        .id();

    // Set up game state using the builder pattern
    let mut turn_order = VecDeque::new();
    turn_order.push_back(player1);
    turn_order.push_back(player2);

    let mut lands_played = Vec::new();
    lands_played.push((player1, 3));
    lands_played.push((player2, 2));

    let mut drawn_this_turn = Vec::new();
    drawn_this_turn.push(player1);

    // Use the builder pattern to create game state
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

// Helper function to clean up test environment
pub fn cleanup_test_environment() {
    let test_dir = Path::new("target/test_saves");
    let _ = std::fs::remove_dir_all(test_dir);
}

// Local SaveConfig struct for testing
#[derive(Resource)]
pub struct SaveConfig {
    pub save_directory: std::path::PathBuf,
    pub auto_save_enabled: bool,
    pub auto_save_frequency: usize,
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            save_directory: std::path::PathBuf::from("target/test_saves"),
            auto_save_enabled: true,
            auto_save_frequency: 2,
        }
    }
}

// Local AutoSaveTracker struct for testing
#[derive(Resource)]
pub struct AutoSaveTracker {
    pub counter: usize,
}

impl Default for AutoSaveTracker {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

// Local ReplayState struct for testing
#[derive(Resource)]
pub struct ReplayState {
    pub active: bool,
}

impl Default for ReplayState {
    fn default() -> Self {
        Self { active: false }
    }
}

// Local SaveInfo struct for testing
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SaveInfo {
    pub slot_name: String,
    pub timestamp: u64,
    pub description: String,
    pub turn_number: u32,
    pub player_count: usize,
}

// Local SaveMetadata struct for testing
#[derive(Resource, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SaveMetadata {
    pub saves: Vec<SaveInfo>,
}

// Mock save handler for tests using bevy_persistent
pub fn handle_save_game_for_tests(
    mut event_reader: EventReader<SaveGameEvent>,
    game_state: Res<GameState>,
    query_players: Query<(Entity, &Player)>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        info!("Saving game to slot: {}", event.slot_name);

        let save_path = Path::new("target/test_saves").join(format!("{}.bin", event.slot_name));

        // Create test save data
        let save_data = GameSaveData {
            game_state: GameStateData {
                turn_number: game_state.turn_number,
                active_player_index: 0,
                priority_holder_index: 0,
                turn_order_indices: vec![0, 1],
            },
            players: query_players
                .iter()
                .enumerate()
                .map(|(idx, (_, player))| PlayerData {
                    id: idx,
                    name: player.name.clone(),
                    life: player.life,
                    player_index: idx,
                })
                .collect(),
            save_version: "1.0".to_string(),
        };

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
    query_players: Query<(Entity, &mut Player)>,
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

            if let Some(mut game_state) = game_state.as_mut() {
                game_state.turn_number = save_data.game_state.turn_number;
            }

            // Update players
            for player_data in save_data.players {
                for (_, mut player) in query_players.iter() {
                    if player.name == player_data.name {
                        // In a real implementation, we would update player state here
                    }
                }
            }
        }
    }
}

// Mock game save data structures
#[derive(Debug, Serialize, Deserialize, Default, Clone, Encode, Decode)]
pub struct GameSaveData {
    pub game_state: GameStateData,
    pub players: Vec<PlayerData>,
    pub save_version: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Encode, Decode)]
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Encode, Decode)]
pub struct PlayerData {
    pub id: usize,
    pub name: String,
    pub life: i32,
    pub player_index: usize,
}

impl GameSaveData {
    // Convert saved data back to game state
    pub fn to_game_state(&self, entities: &[Entity]) -> GameState {
        let mut turn_order = VecDeque::new();

        for &idx in &self.game_state.turn_order_indices {
            if idx < entities.len() {
                turn_order.push_back(entities[idx]);
            }
        }

        GameState::builder()
            .turn_number(self.game_state.turn_number)
            .active_player(if self.game_state.active_player_index < entities.len() {
                entities[self.game_state.active_player_index]
            } else {
                Entity::PLACEHOLDER
            })
            .priority_holder(if self.game_state.priority_holder_index < entities.len() {
                entities[self.game_state.priority_holder_index]
            } else {
                Entity::PLACEHOLDER
            })
            .turn_order(turn_order)
            .build()
    }
}
