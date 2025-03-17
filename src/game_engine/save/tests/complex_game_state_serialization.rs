use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::game_engine::state::GameState;
use crate::game_engine::zones::{Zone, ZoneManager};
use crate::player::Player;

use super::utils::*;

#[test]
fn test_complex_game_state_serialization() {
    // Set up app with real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Add zone systems
    app.init_resource::<ZoneManager>();
    app.init_resource::<CommandZoneManager>();

    // Create test directory
    let test_dir = Path::new("target/test_saves");
    std::fs::create_dir_all(test_dir).unwrap();

    // Set up a complex game state
    // 1. Create multiple players with different stats
    let player1 = app
        .world()
        .spawn(Player {
            name: "Player 1".to_string(),
            life: 37,
            ..Default::default()
        })
        .id();

    let player2 = app
        .world()
        .spawn(Player {
            name: "Player 2".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player3 = app
        .world()
        .spawn(Player {
            name: "Player 3".to_string(),
            life: 25,
            ..Default::default()
        })
        .id();

    let player4 = app
        .world()
        .spawn(Player {
            name: "Player 4".to_string(),
            life: 31,
            ..Default::default()
        })
        .id();

    // 2. Set up game state with turn order
    let mut turn_order = VecDeque::new();
    turn_order.push_back(player1);
    turn_order.push_back(player2);
    turn_order.push_back(player3);
    turn_order.push_back(player4);

    let game_state = GameState::builder()
        .turn_number(7)
        .active_player(player3)
        .priority_holder(player3)
        .turn_order(turn_order)
        .build();

    app.insert_resource(game_state);

    // 3. Set up complex zone structure
    let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();

    // Create library zones for each player
    let library1 = zone_manager.create_zone(Zone::Library);
    zone_manager.set_zone_owner(library1, player1);

    let library2 = zone_manager.create_zone(Zone::Library);
    zone_manager.set_zone_owner(library2, player2);

    let library3 = zone_manager.create_zone(Zone::Library);
    zone_manager.set_zone_owner(library3, player3);

    let library4 = zone_manager.create_zone(Zone::Library);
    zone_manager.set_zone_owner(library4, player4);

    // Create hand zones for each player
    let hand1 = zone_manager.create_zone(Zone::Hand);
    zone_manager.set_zone_owner(hand1, player1);

    let hand2 = zone_manager.create_zone(Zone::Hand);
    zone_manager.set_zone_owner(hand2, player2);

    let hand3 = zone_manager.create_zone(Zone::Hand);
    zone_manager.set_zone_owner(hand3, player3);

    let hand4 = zone_manager.create_zone(Zone::Hand);
    zone_manager.set_zone_owner(hand4, player4);

    // Add cards to the hands (just entities with a zone marker)
    for i in 0..5 {
        // Player 1's hand
        let card = app.world_mut().spawn_empty().id();
        zone_manager.add_to_zone(hand1, card);

        // Player 2's hand
        let card = app.world_mut().spawn_empty().id();
        zone_manager.add_to_zone(hand2, card);
    }

    for i in 0..3 {
        // Player 3's hand
        let card = app.world_mut().spawn_empty().id();
        zone_manager.add_to_zone(hand3, card);

        // Player 4's hand
        let card = app.world_mut().spawn_empty().id();
        zone_manager.add_to_zone(hand4, card);
    }

    // Create graveyard zones
    let graveyard1 = zone_manager.create_zone(Zone::Graveyard);
    zone_manager.set_zone_owner(graveyard1, player1);

    let graveyard2 = zone_manager.create_zone(Zone::Graveyard);
    zone_manager.set_zone_owner(graveyard2, player2);

    let graveyard3 = zone_manager.create_zone(Zone::Graveyard);
    zone_manager.set_zone_owner(graveyard3, player3);

    let graveyard4 = zone_manager.create_zone(Zone::Graveyard);
    zone_manager.set_zone_owner(graveyard4, player4);

    // Add cards to graveyards
    for i in 0..2 {
        let card = app.world_mut().spawn_empty().id();
        zone_manager.add_to_zone(graveyard1, card);
    }

    // Set up command zone
    let mut command_zone = app.world_mut().resource_mut::<CommandZoneManager>();
    command_zone.add_commander(player1, app.world_mut().spawn_empty().id());
    command_zone.add_commander(player2, app.world_mut().spawn_empty().id());
    command_zone.add_commander(player3, app.world_mut().spawn_empty().id());
    command_zone.add_commander(player4, app.world_mut().spawn_empty().id());

    // Save the complex state
    app.world_mut().send_event(SaveGameEvent {
        slot_name: "complex_state".to_string(),
    });

    app.update();

    // Count entities before modifying
    {
        let entity_count = app.world().entities().len();
        info!("Entities before modification: {}", entity_count);
    }

    // Modify the state
    {
        // Add more cards to various zones
        let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();
        for _ in 0..3 {
            let card = app.world_mut().spawn_empty().id();
            zone_manager.add_to_zone(hand1, card);
        }

        // Add more commanders
        let mut command_zone = app.world_mut().resource_mut::<CommandZoneManager>();
        command_zone.add_commander(player1, app.world_mut().spawn_empty().id());
        command_zone.add_commander(player2, app.world_mut().spawn_empty().id());

        // Add more entities
        for _ in 0..5 {
            app.world_mut().spawn_empty();
        }

        // Verify we added entities
        let player_query = app.world().query::<&Player>();
        info!(
            "Players after modification: {}",
            player_query.iter(app.world()).count()
        );

        // Check zone content
        let zone_manager = app.world().resource::<ZoneManager>();
        let hand1_count = zone_manager.get_zone_contents(hand1).len();
        info!("Hand 1 cards after modification: {}", hand1_count);
        assert!(
            hand1_count > 5,
            "Expected more cards in hand after modification"
        );
    }

    // Load the saved state (should revert modifications)
    app.world_mut().send_event(LoadGameEvent {
        slot_name: "complex_state".to_string(),
    });

    app.update();

    // Verify state was restored correctly
    {
        let game_state = app.world().resource::<GameState>();
        assert_eq!(
            game_state.turn_number, 7,
            "Turn number was not restored correctly"
        );

        // Check restored player count
        let player_query = app.world().query::<&Player>();
        assert_eq!(
            player_query.iter(app.world()).count(),
            4,
            "Player count not restored correctly"
        );

        // Check zone restoration
        let zone_manager = app.world().resource::<ZoneManager>();

        // Verify hand card counts
        assert_eq!(
            zone_manager.get_zone_contents(hand1).len(),
            5,
            "Hand 1 card count not restored correctly"
        );
        assert_eq!(
            zone_manager.get_zone_contents(hand2).len(),
            5,
            "Hand 2 card count not restored correctly"
        );
        assert_eq!(
            zone_manager.get_zone_contents(hand3).len(),
            3,
            "Hand 3 card count not restored correctly"
        );
        assert_eq!(
            zone_manager.get_zone_contents(hand4).len(),
            3,
            "Hand 4 card count not restored correctly"
        );

        // Verify graveyard card count
        assert_eq!(
            zone_manager.get_zone_contents(graveyard1).len(),
            2,
            "Graveyard 1 card count not restored correctly"
        );

        // Verify command zone
        let command_zone = app.world().resource::<CommandZoneManager>();
        assert_eq!(
            command_zone.get_commanders(player1).len(),
            1,
            "Commander count for player 1 not restored correctly"
        );
        assert_eq!(
            command_zone.get_commanders(player2).len(),
            1,
            "Commander count for player 2 not restored correctly"
        );
    }

    // Clean up
    cleanup_test_environment();
}
