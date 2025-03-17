use bevy::prelude::*;
use std::path::Path;

use crate::game_engine::commander::CommandZoneManager;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::save::events::{LoadGameEvent, SaveGameEvent};
use crate::game_engine::state::GameState;
use crate::game_engine::zones::{CardLocation, Zone, ZoneId, ZoneManager};
use crate::player::Player;

use super::utils::*;

#[test]
fn test_save_load_with_zones() {
    // Set up app with real plugin
    let mut app = App::new();
    app.add_plugins(SaveLoadPlugin);

    // Add zone systems
    app.init_resource::<ZoneManager>();
    app.init_resource::<CommandZoneManager>();

    // Set up test environment with players and game state
    let player_entities = setup_test_environment(&mut app);

    // Set up zones for testing
    let player1 = player_entities[0];
    let player2 = player_entities[1];

    let mut zone_manager = app.world.resource_mut::<ZoneManager>();

    // Create library zones for each player
    let library1_id = zone_manager.create_zone(Zone::Library(player1));
    let library2_id = zone_manager.create_zone(Zone::Library(player2));

    // Create hand zones for each player
    let hand1_id = zone_manager.create_zone(Zone::Hand(player1));
    let hand2_id = zone_manager.create_zone(Zone::Hand(player2));

    // Create battlefield zone
    let battlefield_id = zone_manager.create_zone(Zone::Battlefield);

    // Add some cards to zones
    // Card 1 in player 1's hand
    let card1 = CardLocation {
        zone_id: hand1_id,
        position: 0,
    };
    zone_manager.add_card_to_zone(card1.zone_id, "Card 1".to_string());

    // Card 2 in player 2's hand
    let card2 = CardLocation {
        zone_id: hand2_id,
        position: 0,
    };
    zone_manager.add_card_to_zone(card2.zone_id, "Card 2".to_string());

    // Card 3 on battlefield
    let card3 = CardLocation {
        zone_id: battlefield_id,
        position: 0,
    };
    zone_manager.add_card_to_zone(card3.zone_id, "Card 3".to_string());

    // Add commander for player 1
    let mut command_zone = app.world.resource_mut::<CommandZoneManager>();
    command_zone.set_commander(player1, "Commander 1".to_string());

    // Save the game state with zones
    let slot_name = "test_zones";

    // Trigger save game event
    app.world.send_event(SaveGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the save event
    app.update();

    // Clear zones completely to verify they get restored
    {
        let mut zone_manager = app.world.resource_mut::<ZoneManager>();
        zone_manager.clear();
    }

    // Verify zones are empty
    {
        let zone_manager = app.world.resource::<ZoneManager>();
        assert_eq!(
            zone_manager.get_all_zone_ids().len(),
            0,
            "Zones were not cleared"
        );
    }

    // Trigger load game event
    app.world.send_event(LoadGameEvent {
        slot_name: slot_name.to_string(),
    });

    // Run systems to process the load event
    app.update();

    // Verify zones were restored
    let zone_manager = app.world.resource::<ZoneManager>();

    // Check zones were recreated
    assert!(
        zone_manager.get_all_zone_ids().len() > 0,
        "Zones were not restored"
    );

    // Find the hand zones for each player
    let hand_zones: Vec<ZoneId> = zone_manager
        .get_all_zone_ids()
        .into_iter()
        .filter(|&id| {
            if let Some(Zone::Hand(_)) = zone_manager.get_zone(id) {
                true
            } else {
                false
            }
        })
        .collect();

    assert!(
        hand_zones.len() >= 2,
        "Expected at least 2 hand zones after loading"
    );

    // Check at least one card in player hands
    let hand_cards_count: usize = hand_zones
        .iter()
        .map(|&id| zone_manager.get_cards_in_zone(id).len())
        .sum();

    assert!(
        hand_cards_count > 0,
        "Expected cards in hand zones after loading"
    );

    // Check battlefield zone
    let battlefield_zones: Vec<ZoneId> = zone_manager
        .get_all_zone_ids()
        .into_iter()
        .filter(|&id| {
            if let Some(Zone::Battlefield) = zone_manager.get_zone(id) {
                true
            } else {
                false
            }
        })
        .collect();

    assert!(
        !battlefield_zones.is_empty(),
        "Battlefield zone was not restored"
    );

    if !battlefield_zones.is_empty() {
        let battlefield_id = battlefield_zones[0];
        let battlefield_cards = zone_manager.get_cards_in_zone(battlefield_id);
        assert!(
            !battlefield_cards.is_empty(),
            "Expected cards on battlefield after loading"
        );
    }

    // Check commander zone
    let command_zone = app.world.resource::<CommandZoneManager>();
    assert!(
        command_zone.get_commander(player1).is_some(),
        "Commander for player 1 was not restored"
    );

    // Clean up
    cleanup_test_environment();
}
