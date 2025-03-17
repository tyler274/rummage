use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::data::PlayerData;
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

    // Create valid players but corrupted game state using the builder pattern
    let save_data = GameSaveData::builder()
        .game_state(
            GameStateData::builder()
                .turn_number(999999999) // Potentially suspicious value
                .active_player_index(0) // Valid index
                .priority_holder_index(0) // Valid index
                .turn_order_indices(vec![0, 1]) // Valid indices
                .build(),
        )
        .players(vec![
            PlayerData::builder()
                .id(0)
                .name("Valid Player 1".to_string())
                .life(40)
                .mana_pool(Default::default())
                .player_index(0)
                .build(),
            PlayerData::builder()
                .id(1)
                .name("Valid Player 2".to_string())
                .life(-999999) // Invalid negative life
                .mana_pool(Default::default())
                .player_index(1)
                .build(),
        ])
        .save_version("corrupted".to_string()) // Invalid version
        .build();

    // Create a persistent resource and set its value to our save data
    let mut persistent_save = Persistent::<GameSaveData>::builder()
        .name("test_partial_corruption")
        .format(StorageFormat::Bincode)
        .path(save_path.clone())
        .default(GameSaveData::default())
        .build()
        .expect("Failed to create persistent resource");

    // Handle the Result return
    let _ = persistent_save.set(save_data);

    // Ensure directory exists
    if let Some(parent) = save_path.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|e| {
            panic!("Failed to create test directory: {}", e);
        });
    }

    persistent_save
        .persist()
        .expect("Failed to save persistent data");

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
    app.world_mut().send_event(LoadGameEvent {
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
    let mut player_query = app.world_mut().query::<&Player>();
    let players: Vec<&Player> = player_query.iter(app.world_mut()).collect();

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
    cleanup_test_environment_compat();
}
