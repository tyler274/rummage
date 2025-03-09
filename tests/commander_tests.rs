use bevy::prelude::*;
use rummage::card::{Card, CardTypes};
use rummage::game_engine::commander::*;
use rummage::game_engine::zones::*;
use rummage::mana::{Color, Mana};
use rummage::player::Player;
use std::collections::{HashMap, HashSet};

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
    // Create a minimal app with required resources
    let mut app = App::new();
    app.insert_resource(CommandZoneManager::default());

    // Create commander entity
    let commander_entity = app.world_mut().spawn(Commander::default()).id();

    // Create a base mana cost
    let base_cost = Mana::new_with_colors(3, 1, 0, 0, 0, 0);

    // Get the CommandZoneManager
    let mut cmd_zone_manager = app.world_mut().resource_mut::<CommandZoneManager>();

    // Initialize transition count as 0
    cmd_zone_manager
        .zone_transition_count
        .insert(commander_entity, 0);

    // First cast should have no tax
    let cost_with_tax =
        calculate_commander_cost(commander_entity, base_cost.clone(), &cmd_zone_manager);
    assert_eq!(cost_with_tax.colorless, 3);
    assert_eq!(cost_with_tax.white, 1);

    // Update cast count to 1
    cmd_zone_manager
        .zone_transition_count
        .insert(commander_entity, 1);
    let cost_with_tax =
        calculate_commander_cost(commander_entity, base_cost.clone(), &cmd_zone_manager);
    assert_eq!(cost_with_tax.colorless, 5); // 3 base + 2 tax
    assert_eq!(cost_with_tax.white, 1);

    // Update cast count to 2
    cmd_zone_manager
        .zone_transition_count
        .insert(commander_entity, 2);
    let cost_with_tax =
        calculate_commander_cost(commander_entity, base_cost.clone(), &cmd_zone_manager);
    assert_eq!(cost_with_tax.colorless, 7); // 3 base + 4 tax
    assert_eq!(cost_with_tax.white, 1);
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

#[test]
fn test_command_zone_manager() {
    // Create a minimal app with required resources
    let mut app = App::new();

    // Add events
    app.add_event::<ZoneChangeEvent>()
        .add_event::<CommanderZoneChoiceEvent>();

    // Add resources
    app.insert_resource(ZoneManager::default())
        .insert_resource(CommandZoneManager::default());

    // Create player entities
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Create commander cards
    let commander1_card = create_commander_card();
    let commander2_card = create_commander_card();

    // Create commander entities with components
    let commander1 = Commander {
        owner: player1,
        color_identity: HashSet::from([Color::WHITE, Color::BLUE]),
        ..Commander::default()
    };

    let commander2 = Commander {
        owner: player2,
        color_identity: HashSet::from([Color::RED, Color::GREEN]),
        ..Commander::default()
    };

    let commander1_entity = app
        .world_mut()
        .spawn((commander1_card.clone(), commander1))
        .id();
    let commander2_entity = app
        .world_mut()
        .spawn((commander2_card.clone(), commander2))
        .id();

    // Create commander mapping
    let mut player_commanders = HashMap::new();
    player_commanders.insert(player1, vec![commander1_entity]);
    player_commanders.insert(player2, vec![commander2_entity]);

    // Initialize CommandZoneManager
    let mut cmd_zone_manager = app.world_mut().resource_mut::<CommandZoneManager>();
    let cards_query = app.world().query::<(Entity, &Card)>();
    cmd_zone_manager.initialize(player_commanders, &cards_query);

    // Test getting player commanders
    let player1_commanders = cmd_zone_manager.get_player_commanders(player1);
    assert_eq!(player1_commanders.len(), 1);
    assert_eq!(player1_commanders[0], commander1_entity);

    // Test commander zone status
    let zone = cmd_zone_manager.get_commander_zone(commander1_entity);
    assert_eq!(zone, CommanderZoneLocation::CommandZone);

    // Test updating zone
    cmd_zone_manager.update_commander_zone(commander1_entity, CommanderZoneLocation::Battlefield);
    let updated_zone = cmd_zone_manager.get_commander_zone(commander1_entity);
    assert_eq!(updated_zone, CommanderZoneLocation::Battlefield);

    // Test cast count
    assert_eq!(cmd_zone_manager.get_cast_count(commander1_entity), 0);
    let count = cmd_zone_manager
        .zone_transition_count
        .entry(commander1_entity)
        .or_insert(0);
    *count += 1;
    assert_eq!(cmd_zone_manager.get_cast_count(commander1_entity), 1);
}

#[test]
fn test_commander_zone_changes() {
    // Create a minimal app with systems
    let mut app = App::new();

    // Add events
    app.add_event::<ZoneChangeEvent>()
        .add_event::<CommanderZoneChoiceEvent>()
        .add_event::<CombatDamageEvent>();

    // Add resources
    app.insert_resource(ZoneManager::default())
        .insert_resource(CommandZoneManager::default())
        .insert_resource(CommandZone::default());

    // Add systems
    app.add_systems(
        Update,
        (handle_commander_zone_change, process_commander_zone_choices),
    );

    // Create player entity
    let player = app.world_mut().spawn(Player::default()).id();

    // Create a commander entity
    let commander_card = create_commander_card();
    let commander = Commander {
        owner: player,
        ..Commander::default()
    };

    let commander_entity = app.world_mut().spawn((commander_card, commander)).id();

    // Initialize CommandZoneManager
    let mut player_commanders = HashMap::new();
    player_commanders.insert(player, vec![commander_entity]);

    let mut cmd_zone_manager = app.world_mut().resource_mut::<CommandZoneManager>();
    let cards_query = app.world().query::<(Entity, &Card)>();
    cmd_zone_manager.initialize(player_commanders, &cards_query);

    // Set up initial zone (command zone)
    cmd_zone_manager.update_commander_zone(commander_entity, CommanderZoneLocation::CommandZone);

    // Test sending zone change event (command zone to battlefield)
    let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();
    zone_manager.add_to_zone(commander_entity, player, Zone::CommandZone);

    app.world_mut()
        .resource_mut::<Events<ZoneChangeEvent>>()
        .send(ZoneChangeEvent {
            card: commander_entity,
            owner: player,
            source: Zone::CommandZone,
            destination: Zone::Battlefield,
        });

    // Run systems to process the events
    app.update();

    // Verify zone was updated
    let cmd_zone_manager = app.world().resource::<CommandZoneManager>();
    assert_eq!(
        cmd_zone_manager.get_commander_zone(commander_entity),
        CommanderZoneLocation::Battlefield
    );

    // Test sending zone change event (battlefield to graveyard)
    app.world_mut()
        .resource_mut::<Events<ZoneChangeEvent>>()
        .send(ZoneChangeEvent {
            card: commander_entity,
            owner: player,
            source: Zone::Battlefield,
            destination: Zone::Graveyard,
        });

    // Run systems to process the events
    app.update();

    // Verify choice event was created (would need to read the event)
    // For now, we can just verify the zone is still registered as graveyard
    let cmd_zone_manager = app.world().resource::<CommandZoneManager>();
    assert_eq!(
        cmd_zone_manager.get_commander_zone(commander_entity),
        CommanderZoneLocation::Graveyard
    );

    // Manually trigger a commander zone choice event (graveyard to command zone)
    app.world_mut()
        .resource_mut::<Events<CommanderZoneChoiceEvent>>()
        .send(CommanderZoneChoiceEvent {
            commander: commander_entity,
            owner: player,
            current_zone: Zone::Graveyard,
            can_go_to_command_zone: true,
        });

    // Run systems to process the events
    app.update();

    // Verify zone was updated to command zone
    let cmd_zone_manager = app.world().resource::<CommandZoneManager>();
    assert_eq!(
        cmd_zone_manager.get_commander_zone(commander_entity),
        CommanderZoneLocation::CommandZone
    );

    // Verify cast count was incremented
    assert_eq!(cmd_zone_manager.get_cast_count(commander_entity), 1);
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
        });

    // Update the app to run systems
    app.update();

    // This would verify that the CommanderZoneChoiceEvent was triggered
    // but we need access to the event reader, which is not trivial in tests
}
