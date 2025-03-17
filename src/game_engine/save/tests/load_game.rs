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

    // Set up test environment with real players and game state
    let _player_entities = setup_test_environment(&mut app);

    // Create a save first to load later
    let slot_name = "test_load";

    // Trigger save game event
    app.world_mut().send_event(SaveGameEvent {
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
    app.world_mut().send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();
    app.update(); // One more update to ensure load completes

    // Verify game state was restored
    let game_state = app.world().resource::<GameState>();

    // The save data has turn_number = 3, so after loading we expect the game_state to have turn_number = 3
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

    // The save had players with life 40 and 35
    assert_eq!(
        player_life_values,
        vec![35, 40],
        "Player life totals were not properly restored"
    );

    // Clean up
    cleanup_test_environment_compat();
}
