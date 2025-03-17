use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::path::Path;

use crate::game_engine::save::events::LoadGameEvent;
use crate::game_engine::save::{
    GameSaveData, GameStateData, SaveConfig, SaveLoadPlugin, SaveMetadata,
};
use crate::game_engine::state::GameState;
use crate::player::Player;

use super::utils::*;

#[test]
fn test_load_game_empty_players() {
    // Set up app with the real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Run once to initialize resources
    app.update();

    // Set up basic resources
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap();

    // Update the save directory in the config
    {
        let mut config = app.world_mut().resource_mut::<SaveConfig>();
        config.save_directory = test_dir.to_path_buf();
    }

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
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        },
        players: Vec::new(), // Empty player list
        save_version: "1.0".to_string(),
        zones: Default::default(),
        commanders: Default::default(),
    };

    // Create a persistent resource and save it
    let persistent_save = Persistent::<GameSaveData>::builder()
        .name("test_empty_players")
        .format(StorageFormat::Bincode)
        .path(save_path.clone())
        .default(save_data.clone())
        .build()
        .expect("Failed to create persistent resource");

    persistent_save
        .persist()
        .expect("Failed to persist save data");

    // Create a simple game state
    let game_state = GameState::builder().turn_number(1).build();

    app.insert_resource(game_state);

    // Trigger load game event
    app.world_mut().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();
    app.update(); // Run another update to ensure the load completes

    // Verify game state
    let game_state = app.world().resource::<GameState>();

    // Since the save contains turn_number = 5, we expect to see that value after loading
    assert_eq!(
        game_state.turn_number, 5,
        "Game state turn number was not loaded from empty players save"
    );

    // Verify there are no players - loading a save with no players
    // should not generate any new players
    let player_count = {
        let mut world = app.world_mut();
        let mut player_query = world.query::<&Player>();
        player_query.iter(&world).count()
    };
    assert_eq!(
        player_count, 0,
        "There should be no players after loading an empty player save"
    );

    // Clean up
    cleanup_test_environment();
}
