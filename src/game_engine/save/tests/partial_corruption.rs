use bevy::prelude::*;
use bincode;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::{GameSaveData, GameStateData, SaveLoadPlugin};
use crate::game_engine::state::GameState;
use crate::player::Player;

use super::utils::*;

#[test]
fn test_partial_corruption() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set up test environment with players and game state
    let player_entities = setup_test_environment(&mut app);

    // Create a save file with partially corrupted data
    let test_dir = Path::new("target/test_saves");
    let slot_name = "partial_corruption";
    let save_path = test_dir.join(format!("{}.bin", slot_name));

    // Create valid players but corrupted game state
    let save_data = GameSaveData {
        game_state: GameStateData {
            turn_number: 999999999,         // Potentially suspicious value
            active_player_index: 0,         // Valid index
            priority_holder_index: 0,       // Valid index
            turn_order_indices: vec![0, 1], // Valid indices
            ..Default::default()
        },
        players: vec![
            super::utils::PlayerData {
                id: 0,
                name: "Valid Player 1".to_string(),
                life: 40,
                player_index: 0,
            },
            super::utils::PlayerData {
                id: 1,
                name: "Valid Player 2".to_string(),
                life: -999999, // Invalid negative life
                player_index: 1,
            },
        ],
        save_version: "corrupted".to_string(), // Invalid version
        ..Default::default()
    };

    // Serialize and write to file
    let serialized = bincode::serialize(&save_data).expect("Failed to serialize save data");
    std::fs::write(&save_path, serialized).unwrap();

    // Create initial game state
    let mut turn_order = VecDeque::new();
    for entity in &player_entities {
        turn_order.push_back(*entity);
    }

    let mut lands_played = Vec::new();
    lands_played.push((player_entities[0], 3));
    lands_played.push((player_entities[1], 2));

    let mut drawn_this_turn = Vec::new();
    drawn_this_turn.push(player_entities[0]);

    let initial_game_state = GameState::builder()
        .turn_number(3)
        .active_player(player_entities[0])
        .priority_holder(player_entities[0])
        .turn_order(turn_order)
        .lands_played(lands_played)
        .main_phase_action_taken(true)
        .drawn_this_turn(drawn_this_turn)
        .state_based_actions_performed(true)
        .build();

    app.insert_resource(initial_game_state);

    // Trigger load game event for the corrupted save
    app.world().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // The loading system should handle partial corruption gracefully
    // Verify game state was loaded with reasonable fallbacks
    let game_state = app.world().resource::<GameState>();

    // Turn number was corrupted, but should be loaded with some fallback or sanitized value
    assert!(
        game_state.turn_number <= 100,
        "Turn number was not sanitized from corrupted value"
    );

    // Verify player data was loaded with reasonable fallbacks
    let player_query = app.world().query::<&Player>();
    let players: Vec<&Player> = player_query.iter(app.world()).collect();

    assert!(
        players.len() >= 2,
        "Expected at least 2 players after loading partially corrupted save"
    );

    // Check no player has negative life (should be sanitized)
    for player in &players {
        assert!(
            player.life >= 0,
            "Player life should not be negative after loading corrupted save"
        );
    }

    // Clean up
    cleanup_test_environment();
}
