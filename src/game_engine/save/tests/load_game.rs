use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::path::Path;

use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::game_engine::state::GameState;
use crate::player::Player;

use super::utils::*;

#[test]
fn test_load_game() {
    // Set up app with the actual plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Set up test environment with real players and game state
    let _player_entities = setup_test_environment(&mut app);

    // Create a save first to load later
    let slot_name = "test_load";

    // Trigger save game event
    app.world().send_event(SaveGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the save event
    app.update();

    // Modify the game state to something different
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.turn_number = 10; // Different from original turn number (3)

        // Modify a player's life total
        let mut player_query = app.world_mut().query::<&mut Player>();
        for mut player in player_query.iter_mut(app.world_mut()) {
            player.life = 20; // Different from original values (40 and 35)
        }
    }

    // Trigger load game event
    app.world().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // Verify game state was restored
    let game_state = app.world().resource::<GameState>();
    assert_eq!(
        game_state.turn_number, 3,
        "Game state turn number was not properly restored"
    );

    // Verify player data was restored
    let player_query = app.world().query::<&Player>();
    let players: Vec<&Player> = player_query.iter(app.world()).collect();

    assert!(
        players.len() >= 2,
        "Expected at least 2 players after loading"
    );

    // Verify that player life totals were restored
    for player in players {
        assert!(
            player.life == 40 || player.life == 35,
            "Player life total was not properly restored"
        );
    }

    // Clean up
    cleanup_test_environment();
}
