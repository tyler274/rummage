use bevy::prelude::*;

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

    // Add the test plugin to handle the save/load events
    app.add_plugins(super::utils::SaveLoadTestPlugin);

    // Set up test environment with real players and game state
    let _player_entities = setup_test_environment(&mut app);

    // Create a save first to load later
    let slot_name = "test_load";

    // Trigger save game event
    app.world_mut().send_event(SaveGameEvent {
        slot_name: slot_name.to_string(),
        description: None,
        with_snapshot: false,
    });

    // Run systems to process the save event
    app.update();

    // Modify the game state to something different
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        let original_turn = game_state.turn_number;
        game_state.turn_number = 10; // Different from original turn number (3)
        info!(
            "Modified turn number from {} to {}",
            original_turn, game_state.turn_number
        );

        // Modify a player's life total
        let mut player_query = app.world_mut().query::<&mut Player>();
        for mut player in player_query.iter_mut(app.world_mut()) {
            player.life = 20; // Different from original values (40 and 35)
        }
    }

    // Trigger load game event
    app.world_mut().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event - run multiple times to ensure all systems execute
    for _ in 0..5 {
        app.update();
    }

    // Verify game state was restored
    let game_state = app.world().resource::<GameState>();
    info!("After load, turn number is: {}", game_state.turn_number);

    // The turn number should match the value set in setup_test_environment
    assert_eq!(
        game_state.turn_number, 3,
        "Game state turn number was not properly restored"
    );

    // Collect and verify player life totals were restored
    let mut player_life_values = Vec::new();

    // Get player life values using app.world()
    {
        let world = app.world_mut();
        let mut player_query = world.query::<&Player>();

        for player in player_query.iter(&world) {
            player_life_values.push(player.life);
        }
    }

    // Sort to make comparison reliable
    player_life_values.sort();

    // The life values are restored to original values in our test handler
    assert_eq!(
        player_life_values,
        vec![35, 40],
        "Player life totals were not properly restored"
    );

    // Clean up
    cleanup_test_environment_compat();
}
