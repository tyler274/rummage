use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::save::data::{GameSaveData, GameStateData, PlayerData};
use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::state::GameState;

use super::utils::*;

#[test]
fn test_load_game_corrupted_mapping() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set up test environment with game state
    let player_entities = setup_test_environment(&mut app);

    // Create a save file with corrupted entity mapping (indices out of bounds)
    let test_dir = Path::new("target/test_saves");
    let slot_name = "corrupted_mapping";
    let save_path = test_dir.join(format!("{}.bin", slot_name));

    // Create save data with invalid entity indices
    let save_data = GameSaveData {
        game_state: GameStateData {
            turn_number: 7,
            active_player_index: 999,                  // Invalid index
            priority_holder_index: 999,                // Invalid index
            turn_order_indices: vec![999, 1000, 1001], // Invalid indices
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        },
        players: vec![PlayerData {
            id: 999,
            name: "Corrupted Player".to_string(),
            life: 40,
            mana_pool: Default::default(),
            player_index: 999,
        }],
        save_version: "1.0".to_string(),
        zones: Default::default(),
        commanders: Default::default(),
    };

    // Create a persistent resource and set its value to our save data
    let mut persistent_save = Persistent::<GameSaveData>::builder()
        .name("test_corrupted_mapping")
        .format(StorageFormat::Bincode)
        .path(save_path.clone())
        .default(GameSaveData::default())
        .build()
        .expect("Failed to create persistent resource");

    // Handle the Result return
    let _ = persistent_save.set(save_data);

    // Set the value and save it to disk
    persistent_save
        .persist()
        .expect("Failed to save persistent data");

    // Create initial game state
    let mut turn_order = VecDeque::new();
    for entity in &player_entities {
        turn_order.push_back(*entity);
    }

    let initial_game_state = GameState::builder()
        .turn_number(1)
        .active_player(player_entities[0])
        .priority_holder(player_entities[0])
        .turn_order(turn_order)
        .build();

    app.insert_resource(initial_game_state);

    // Trigger load game event
    app.world_mut().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // Verify that the game state was handled gracefully despite corruption
    let game_state = app.world().resource::<GameState>();

    // The load system should have maintained valid entity references
    for entity in &game_state.turn_order {
        assert!(
            app.world().get_entity(*entity).is_ok(),
            "Invalid entity in turn order"
        );
    }

    assert!(
        app.world().get_entity(game_state.active_player).is_ok(),
        "Invalid active player entity"
    );

    assert!(
        app.world().get_entity(game_state.priority_holder).is_ok(),
        "Invalid priority holder entity"
    );

    // Clean up
    cleanup_test_environment_compat();
}
