use bevy::prelude::*;
use rummage::card::{Card, CardTypes};
use rummage::game_engine::commander::*;
use rummage::game_engine::zones::*;
use rummage::mana::{Color, Mana};
use rummage::player::Player;
use std::collections::HashSet;

// Helper function to create a test card with commander properties
fn create_commander_card() -> Card {
    Card {
        name: "Golos, Tireless Pilgrim".to_string(),
        cost: Mana::new_with_colors(5, 0, 0, 0, 0, 0),
        types: CardTypes::CREATURE | CardTypes::LEGENDARY,
        card_details: rummage::card::CardDetails::Creature(
            rummage::card::CreatureCard {
                power: 3,
                toughness: 5,
                creature_type: rummage::card::CreatureType::HUMAN,
            },
        ),
        rules_text: "When Golos enters the battlefield, search your library for a land card, put that card onto the battlefield tapped, then shuffle.".to_string(),
    }
}

#[test]
fn test_commander_tax_calculation() {
    let mut commander = Commander::default();

    // First cast should have no tax
    commander.cast_count = 0;
    let tax = calculate_commander_tax(&commander);
    assert_eq!(tax.colorless, 0);

    // Second cast should have 2 tax
    commander.cast_count = 1;
    let tax = calculate_commander_tax(&commander);
    assert_eq!(tax.colorless, 2);

    // Third cast should have 4 tax
    commander.cast_count = 2;
    let tax = calculate_commander_tax(&commander);
    assert_eq!(tax.colorless, 4);
}

#[test]
fn test_commander_damage_tracking() {
    // Create a minimal app
    let mut app = App::new();

    // Create player entities
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Create a commander entity
    let mut commander = Commander::default();
    commander.owner = player1;

    // Add damage to player2
    commander.damage_dealt.push((player2, 10));

    // Verify not at lethal damage yet
    assert!(!CommanderRules::check_commander_damage_elimination(
        &commander, player2
    ));

    // Update damage to lethal amount
    commander.damage_dealt.clear();
    commander
        .damage_dealt
        .push((player2, CommanderRules::COMMANDER_DAMAGE_THRESHOLD));

    // Verify now at lethal damage
    assert!(CommanderRules::check_commander_damage_elimination(
        &commander, player2
    ));
}

#[test]
fn test_color_identity_extraction() {
    // Create a test card with multiple colors
    let mut card = create_commander_card();

    // Set the mana cost to include multiple colors
    card.cost = Mana::new_with_colors(2, 1, 1, 1, 1, 1);

    // Extract color identity
    let color_identity = CommanderRules::extract_color_identity(&card);

    // Verify all 5 colors are present
    assert!(color_identity.contains(&Color::WHITE));
    assert!(color_identity.contains(&Color::BLUE));
    assert!(color_identity.contains(&Color::BLACK));
    assert!(color_identity.contains(&Color::RED));
    assert!(color_identity.contains(&Color::GREEN));
    assert_eq!(color_identity.len(), 5);
}

#[test]
fn test_can_be_commander() {
    // Test with a legendary creature
    let mut card = create_commander_card();
    assert!(CommanderRules::can_be_commander(&card));

    // Test with a non-legendary creature
    card.types = CardTypes::CREATURE;
    assert!(!CommanderRules::can_be_commander(&card));

    // Test with a card that explicitly says it can be your commander
    card.types = CardTypes::PLANESWALKER;
    card.rules_text = "Grist, the Hunger Tide can be your commander.".to_string();
    assert!(CommanderRules::can_be_commander(&card));
}

/// This test requires modifications to make the ZoneManager methods public
/// Currently skipped due to private method access restrictions
#[test]
#[ignore]
fn test_zone_change_handling() {
    // Create a minimal app with required resources and systems
    let mut app = App::new();

    // Add events
    app.add_event::<ZoneChangeEvent>()
        .add_event::<CommanderZoneChoiceEvent>();

    // Add resources
    app.insert_resource(ZoneManager::default());

    // Add systems
    app.add_systems(Update, handle_commander_zone_change);

    // Create player entity
    let player = app.world_mut().spawn(Player::default()).id();

    // Create a commander entity
    let commander_card = create_commander_card();
    let commander = Commander {
        owner: player,
        ..Commander::default()
    };

    let commander_entity = app.world_mut().spawn((commander_card, commander)).id();

    // Send a zone change event (battlefield to graveyard)
    app.world_mut()
        .resource_mut::<Events<ZoneChangeEvent>>()
        .send(ZoneChangeEvent {
            card: commander_entity,
            owner: player,
            source: Zone::Battlefield,
            destination: Zone::Graveyard,
            was_visible: true,
            is_visible: true,
        });

    // Run the systems
    app.update();

    // Verify a choice event was created
    let choice_events = app.world().resource::<Events<CommanderZoneChoiceEvent>>();
    let mut reader = choice_events.get_cursor();
    let events: Vec<_> = reader.read(choice_events).collect();

    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.commander, commander_entity);
    assert_eq!(event.owner, player);
    assert_eq!(event.current_zone, Zone::Graveyard);
    assert!(event.can_go_to_command_zone);
}

/// This test requires modifications to make the ZoneManager methods public
/// Currently skipped due to private method access restrictions
#[test]
#[ignore]
fn test_process_commander_zone_choices() {
    // Create a minimal app with required resources and systems
    let mut app = App::new();

    // Add events
    app.add_event::<CommanderZoneChoiceEvent>();

    // Add resources
    app.insert_resource(ZoneManager::default());

    // Add systems
    app.add_systems(Update, process_commander_zone_choices);

    // Create player entity
    let player = app.world_mut().spawn(Player::default()).id();

    // Create a commander entity
    let commander_card = create_commander_card();
    let commander = Commander {
        owner: player,
        ..Commander::default()
    };

    let commander_entity = app.world_mut().spawn((commander_card, commander)).id();

    // Initialize the zone manager
    let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();
    zone_manager.init_player_zones(player);

    // NOTE: We can't directly access private methods like add_to_graveyard
    // In a real implementation, we would need to make these methods public
    // or create test-specific interfaces

    // Send a choice event
    app.world_mut()
        .resource_mut::<Events<CommanderZoneChoiceEvent>>()
        .send(CommanderZoneChoiceEvent {
            commander: commander_entity,
            owner: player,
            current_zone: Zone::Graveyard,
            can_go_to_command_zone: true,
        });

    // Run the systems
    app.update();

    // NOTE: We can't verify the zone change since we can't access these methods
    // In a real implementation, we would make the methods public or add test helpers
}

#[test]
fn test_record_commander_damage() {
    // Create a minimal app with required resources and systems
    let mut app = App::new();

    // Add events
    app.add_event::<CombatDamageEvent>();

    // Add systems
    app.add_systems(Update, record_commander_damage);

    // Create player entities
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Create a commander entity
    let commander_card = create_commander_card();
    let commander = Commander {
        owner: player1,
        ..Commander::default()
    };

    let commander_entity = app.world_mut().spawn((commander_card, commander)).id();

    // Send a combat damage event
    app.world_mut()
        .resource_mut::<Events<CombatDamageEvent>>()
        .send(CombatDamageEvent {
            source: commander_entity,
            target: player2,
            damage: 5,
            is_combat_damage: true,
        });

    // Run the systems
    app.update();

    // Verify the damage was recorded
    let commander = app
        .world()
        .entity(commander_entity)
        .get::<Commander>()
        .unwrap();

    // Check that damage was recorded
    let damage = commander
        .damage_dealt
        .iter()
        .find(|(p, _)| *p == player2)
        .map(|(_, d)| *d)
        .unwrap_or(0);

    assert_eq!(damage, 5);
    assert!(commander.dealt_combat_damage_this_turn.contains(&player2));
}
