use bevy::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::{GameSaveData, GameStateData, SaveLoadPlugin};
use crate::game_engine::state::GameState;
use crate::player::Player;

use super::utils::*;

#[test]
fn test_load_game_empty_players() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set up basic resources
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap();

    // Create a save file with empty player list
    let slot_name = "empty_players";
    let save_path = test_dir.join(format!("{}.bin", slot_name));

    // Create empty save data
    let save_data = GameSaveData {
        game_state: GameStateData {
            turn_number: 5,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(),
            ..Default::default()
        },
        players: Vec::new(), // Empty player list
        save_version: "1.0".to_string(),
        ..Default::default()
    };

    // Serialize and write to file
    let serialized = bincode::serialize(&save_data).expect("Failed to serialize save data");
    std::fs::write(&save_path, serialized).unwrap();

    // Create a simple game state in the app using the builder
    let game_state = GameState::builder()
        .turn_number(1)
        .active_player(Entity::PLACEHOLDER)
        .priority_holder(Entity::PLACEHOLDER)
        .turn_order(VecDeque::new())
        .build();

    app.insert_resource(game_state);

    // Trigger load game event
    app.world().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // Verify game state was loaded despite empty players
    let game_state = app.world().resource::<GameState>();
    assert_eq!(
        game_state.turn_number, 5,
        "Game state turn number was not loaded from empty players save"
    );

    // Verify no player entities were created from the empty save
    let player_query = app.world().query::<&Player>();
    let players_count = player_query.iter(app.world()).count();
    assert_eq!(
        players_count, 0,
        "Expected no players after loading empty players save"
    );

    // Clean up
    cleanup_test_environment();
}
