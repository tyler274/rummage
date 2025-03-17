use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::path::Path;

use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::{GameSaveData, GameStateData, PlayerData, SaveConfig};
use crate::game_engine::state::GameState;

use super::utils::*;

#[test]
fn test_load_game_empty_turn_order() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Run once to initialize resources
    app.update();

    // Set up test environment with players and game state
    let _player_entities = setup_test_environment(&mut app);

    // Create a save file with empty turn order
    let test_dir = Path::new("target/test_saves");

    // Update the save directory in the config
    {
        let mut config = app.world_mut().resource_mut::<SaveConfig>();
        config.save_directory = test_dir.to_path_buf();
    }

    let slot_name = "empty_turn_order";
    let save_path = test_dir.join(format!("{}.bin", slot_name));

    // Create save data with empty turn order
    let save_data = GameSaveData {
        game_state: GameStateData {
            turn_number: 6,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(), // Empty turn order
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
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
        zones: Default::default(),
        commanders: Default::default(),
    };

    // Create a persistent resource and save it
    let persistent_save = Persistent::<GameSaveData>::builder()
        .name("test_empty_turn_order")
        .format(StorageFormat::Bincode)
        .path(save_path.clone())
        .default(save_data.clone())
        .build()
        .expect("Failed to create persistent resource");

    persistent_save
        .persist()
        .expect("Failed to persist save data");

    // Now change the game state
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.turn_number = 1; // Different from what we saved
    }

    // Trigger load game event
    app.world_mut().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();
    app.update(); // One more update to make sure the load completes

    // Verify game state was loaded, despite empty turn order
    let game_state = app.world().resource::<GameState>();

    // Since the save contains turn_number = 6, we expect to see that value after loading
    assert_eq!(
        game_state.turn_number, 6,
        "Turn number was not loaded from empty turn order save"
    );

    // Verify turn order was reconstructed from player entities
    assert!(
        !game_state.turn_order.is_empty(),
        "Turn order should have been reconstructed"
    );

    // Clean up
    cleanup_test_environment();
}
