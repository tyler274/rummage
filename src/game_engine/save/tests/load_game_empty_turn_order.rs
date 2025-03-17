use bevy::prelude::*;
use bincode::{Decode, Encode, config};
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::data::PlayerData;
use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::{GameSaveData, GameStateData, SaveLoadPlugin};
use crate::game_engine::state::GameState;
use crate::player::Player;

use super::utils::*;

#[test]
fn test_load_game_empty_turn_order() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set up test environment with players and game state
    let player_entities = setup_test_environment(&mut app);

    // Create a save file with empty turn order
    let test_dir = Path::new("target/test_saves");
    let slot_name = "empty_turn_order";
    let save_path = test_dir.join(format!("{}.bin", slot_name));

    // Create save data with empty turn order
    let save_data = GameSaveData {
        game_state: GameStateData {
            turn_number: 6,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(), // Empty turn order
            ..Default::default()
        },
        players: vec![
            PlayerData {
                id: 0,
                name: "Player 1".to_string(),
                life: 40,
                mana_pool: Default::default(),
                player_index: 0,
            },
            PlayerData {
                id: 1,
                name: "Player 2".to_string(),
                life: 35,
                mana_pool: Default::default(),
                player_index: 1,
            },
        ],
        save_version: "1.0".to_string(),
        ..Default::default()
    };

    // Serialize and write to file
    let serialized = bincode::encode_to_vec(&save_data, config::standard())
        .expect("Failed to serialize save data");
    std::fs::write(&save_path, serialized).unwrap();

    // Create initial game state with valid turn order using the builder
    let mut turn_order = VecDeque::new();
    turn_order.push_back(player_entities[0]);
    turn_order.push_back(player_entities[1]);

    let initial_game_state = GameState::builder()
        .turn_number(1)
        .active_player(player_entities[0])
        .priority_holder(player_entities[0])
        .turn_order(turn_order)
        .build();

    app.insert_resource(initial_game_state);

    // Trigger load game event
    app.world().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // Verify game state was loaded despite empty turn order
    let game_state = app.world().resource::<GameState>();
    assert_eq!(
        game_state.turn_number, 6,
        "Turn number was not loaded from empty turn order save"
    );

    // The load system should reconstruct a turn order from available players
    assert!(
        !game_state.turn_order.is_empty(),
        "Turn order should be reconstructed"
    );

    // Clean up
    cleanup_test_environment();
}
