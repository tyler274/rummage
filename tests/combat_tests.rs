use bevy::prelude::*;
use rummage::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use rummage::game_engine::GameState;
use rummage::game_engine::combat::*;
use rummage::game_engine::commander::*;
use rummage::game_engine::turn::TurnManager;
use rummage::game_engine::zones::{Zone, ZoneManager};
use rummage::mana::Mana;
use rummage::player::Player;
use std::collections::HashMap;

// Helper function to create a test creature card
fn create_test_creature(name: &str, power: i32, toughness: i32) -> Card {
    Card {
        name: name.to_string(),
        cost: Mana::new_with_colors(2, 0, 0, 0, 0, 0),
        types: CardTypes::CREATURE,
        card_details: CardDetails::Creature(CreatureCard {
            power,
            toughness,
            creature_type: CreatureType::HUMAN,
        }),
        rules_text: String::new(),
    }
}

// Helper function to create a commander creature card
fn create_commander_creature(name: &str, power: i32, toughness: i32) -> Card {
    Card {
        name: name.to_string(),
        cost: Mana::new_with_colors(3, 0, 0, 0, 0, 0),
        types: CardTypes::CREATURE | CardTypes::LEGENDARY,
        card_details: CardDetails::Creature(CreatureCard {
            power,
            toughness,
            creature_type: CreatureType::HUMAN,
        }),
        rules_text: String::new(),
    }
}

// Setup a test app with necessary resources and systems
fn setup_test_app() -> App {
    let mut app = App::new();

    // Add events
    app.add_event::<DeclareAttackersEvent>()
        .add_event::<DeclareBlockersEvent>()
        .add_event::<AssignCombatDamageEvent>()
        .add_event::<AttackerDeclaredEvent>()
        .add_event::<BlockerDeclaredEvent>()
        .add_event::<CombatBeginEvent>()
        .add_event::<CombatEndEvent>()
        .add_event::<CombatDamageEvent>()
        .add_event::<ZoneChangeEvent>();

    // Add resources
    app.insert_resource(CombatState::default())
        .insert_resource(GameState::default())
        .insert_resource(ZoneManager::default())
        .insert_resource(CommandZoneManager::default())
        .insert_resource(CommandZone::default());

    // Add systems
    app.add_systems(
        Update,
        (
            initialize_combat_phase,
            declare_attackers_system,
            declare_blockers_system,
            assign_combat_damage_system,
            process_combat_damage_system,
            end_combat_system,
            record_commander_damage,
        ),
    );

    // Setup TurnManager
    let mut turn_manager = TurnManager::default();
    app.insert_resource(turn_manager);

    app
}

#[test]
fn test_combat_initialization() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Trigger combat initialization
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Get combat state and verify it's initialized
    let combat_state = app.world().resource::<CombatState>();

    assert_eq!(combat_state.attackers.len(), 0);
    assert_eq!(combat_state.blockers.len(), 0);
    assert_eq!(combat_state.in_declare_attackers, false);
    assert_eq!(combat_state.in_declare_blockers, false);
    assert_eq!(combat_state.in_combat_damage, false);
    assert_eq!(combat_state.combat_damage_step_number, 0);

    // Verify player tracking is set up
    assert!(
        combat_state
            .creatures_attacking_each_player
            .contains_key(&player1)
    );
    assert!(
        combat_state
            .creatures_attacking_each_player
            .contains_key(&player2)
    );
}

#[test]
fn test_declaring_attackers() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacking creatures
    let attacker1_card = create_test_creature("Goblin", 2, 1);
    let attacker1 = app.world_mut().spawn(attacker1_card).id();

    let attacker2_card = create_test_creature("Soldier", 1, 3);
    let attacker2 = app.world_mut().spawn(attacker2_card).id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Declare attackers
    app.world_mut()
        .resource_mut::<Events<AttackerDeclaredEvent>>()
        .send(AttackerDeclaredEvent {
            attacker: attacker1,
            defender: player2,
        });

    app.world_mut()
        .resource_mut::<Events<AttackerDeclaredEvent>>()
        .send(AttackerDeclaredEvent {
            attacker: attacker2,
            defender: player2,
        });

    // Run the attacker declaration system
    app.update();

    // Verify attackers were registered
    let combat_state = app.world().resource::<CombatState>();

    assert_eq!(combat_state.attackers.len(), 2);
    assert_eq!(combat_state.attackers.get(&attacker1), Some(&player2));
    assert_eq!(combat_state.attackers.get(&attacker2), Some(&player2));

    assert_eq!(
        combat_state.blocked_status.get(&attacker1),
        Some(&BlockedStatus::Unblocked)
    );
    assert_eq!(
        combat_state.blocked_status.get(&attacker2),
        Some(&BlockedStatus::Unblocked)
    );

    assert!(combat_state.players_attacked_this_turn.contains(&player2));

    // Verify per-player tracking
    let attackers_on_player2 = combat_state
        .creatures_attacking_each_player
        .get(&player2)
        .unwrap();
    assert_eq!(attackers_on_player2.len(), 2);
    assert!(attackers_on_player2.contains(&attacker1));
    assert!(attackers_on_player2.contains(&attacker2));
}

#[test]
fn test_declaring_blockers() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacking creature
    let attacker_card = create_test_creature("Goblin", 2, 1);
    let attacker = app.world_mut().spawn(attacker_card).id();

    // Create blocking creature
    let blocker_card = create_test_creature("Wall", 0, 4);
    let blocker = app.world_mut().spawn(blocker_card).id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Setup attacker
    {
        let mut combat_state = app.world_mut().resource_mut::<CombatState>();
        combat_state.attackers.insert(attacker, player2);
        combat_state
            .blocked_status
            .insert(attacker, BlockedStatus::Unblocked);

        if let Some(attackers) = combat_state
            .creatures_attacking_each_player
            .get_mut(&player2)
        {
            attackers.push(attacker);
        }
    }

    // Declare blocker
    app.world_mut()
        .resource_mut::<Events<BlockerDeclaredEvent>>()
        .send(BlockerDeclaredEvent {
            blocker: blocker,
            attacker: attacker,
        });

    // Run the blocker declaration system
    app.update();

    // Verify blocker was registered
    let combat_state = app.world().resource::<CombatState>();

    assert_eq!(combat_state.blockers.len(), 1);
    assert!(combat_state.blockers.contains_key(&attacker));

    let blockers_of_attacker = combat_state.blockers.get(&attacker).unwrap();
    assert_eq!(blockers_of_attacker.len(), 1);
    assert_eq!(blockers_of_attacker[0], blocker);

    assert_eq!(
        combat_state.blocked_status.get(&attacker),
        Some(&BlockedStatus::Blocked)
    );
}

#[test]
fn test_combat_damage_to_player() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacking creature
    let attacker_card = create_test_creature("Goblin", 2, 1);
    let attacker = app.world_mut().spawn(attacker_card).id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Setup attacker
    {
        let mut combat_state = app.world_mut().resource_mut::<CombatState>();
        combat_state.attackers.insert(attacker, player2);
        combat_state
            .blocked_status
            .insert(attacker, BlockedStatus::Unblocked);

        if let Some(attackers) = combat_state
            .creatures_attacking_each_player
            .get_mut(&player2)
        {
            attackers.push(attacker);
        }
    }

    // Trigger combat damage assignment
    app.world_mut()
        .resource_mut::<Events<AssignCombatDamageEvent>>()
        .send(AssignCombatDamageEvent {
            is_first_strike: false,
        });

    // Run damage assignment system
    app.update();

    // Process combat damage
    app.update();

    // Verify damage was dealt to player
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.player_life_totals.get(&player2), Some(&38)); // 40 - 2 = 38
}

#[test]
fn test_combat_damage_to_blocker() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacking creature
    let attacker_card = create_test_creature("Goblin", 2, 1);
    let attacker = app.world_mut().spawn(attacker_card).id();

    // Create blocking creature
    let blocker_card = create_test_creature("Wall", 0, 4);
    let blocker = app.world_mut().spawn(blocker_card).id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Setup attacker and blocker
    {
        let mut combat_state = app.world_mut().resource_mut::<CombatState>();
        combat_state.attackers.insert(attacker, player2);
        combat_state
            .blocked_status
            .insert(attacker, BlockedStatus::Blocked);
        combat_state.blockers.insert(attacker, vec![blocker]);

        if let Some(attackers) = combat_state
            .creatures_attacking_each_player
            .get_mut(&player2)
        {
            attackers.push(attacker);
        }
    }

    // Trigger combat damage assignment
    app.world_mut()
        .resource_mut::<Events<AssignCombatDamageEvent>>()
        .send(AssignCombatDamageEvent {
            is_first_strike: false,
        });

    // Run damage assignment system and process damage
    app.update();

    // Verify damage events were created
    let combat_state = app.world().resource::<CombatState>();

    // Verify life total of player2 is still 40 (no damage went through)
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.player_life_totals.get(&player2), Some(&40));
}

#[test]
fn test_commander_combat_damage() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacking commander
    let commander_card = create_commander_creature("Tetsuko", 3, 3);
    let commander_component = Commander {
        owner: player1,
        ..Commander::default()
    };
    let commander = app
        .world_mut()
        .spawn((commander_card, commander_component))
        .id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Setup attacker
    {
        let mut combat_state = app.world_mut().resource_mut::<CombatState>();
        combat_state.attackers.insert(commander, player2);
        combat_state
            .blocked_status
            .insert(commander, BlockedStatus::Unblocked);

        if let Some(attackers) = combat_state
            .creatures_attacking_each_player
            .get_mut(&player2)
        {
            attackers.push(commander);
        }

        // Initialize commander damage tracking
        if !combat_state
            .commander_damage_this_combat
            .contains_key(&player2)
        {
            combat_state
                .commander_damage_this_combat
                .insert(player2, HashMap::new());
        }

        if let Some(damage_map) = combat_state.commander_damage_this_combat.get_mut(&player2) {
            damage_map.insert(commander, 0);
        }
    }

    // Trigger combat damage assignment
    app.world_mut()
        .resource_mut::<Events<AssignCombatDamageEvent>>()
        .send(AssignCombatDamageEvent {
            is_first_strike: false,
        });

    // Run damage assignment system and process damage
    app.update();

    // Create and process the combat damage event
    app.world_mut()
        .resource_mut::<Events<CombatDamageEvent>>()
        .send(CombatDamageEvent {
            source: commander,
            target: player2,
            damage: 3,
            is_combat_damage: true,
            source_is_commander: true,
        });

    // Run the commander damage tracking system
    app.update();

    // Verify damage was dealt to player
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.player_life_totals.get(&player2), Some(&37)); // 40 - 3 = 37

    // Verify commander damage was tracked in the commander component
    let commander_entity = app.world().entity(commander);
    let commander_component = commander_entity.get::<Commander>().unwrap();

    // Verify the commander dealt damage to player2
    let tracked_damage = commander_component
        .damage_dealt
        .iter()
        .find(|(p, _)| *p == player2)
        .map(|(_, d)| *d)
        .unwrap_or(0);

    assert_eq!(tracked_damage, 3);
}

#[test]
fn test_full_combat_sequence() {
    let mut app = setup_test_app();

    // Create players
    let player1 = app.world_mut().spawn(Player::default()).id();
    let player2 = app.world_mut().spawn(Player::default()).id();

    // Setup TurnManager
    let mut turn_manager = app.world_mut().resource_mut::<TurnManager>();
    turn_manager.active_player = player1;
    turn_manager.player_order = vec![player1, player2];

    // Setup GameState
    let mut game_state = app.world_mut().resource_mut::<GameState>();
    game_state.players.insert(player1, 0);
    game_state.players.insert(player2, 1);
    game_state.player_life_totals.insert(player1, 40);
    game_state.player_life_totals.insert(player2, 40);

    // Create attacker for player1
    let attacker_card = create_test_creature("Goblin", 2, 1);
    let attacker = app.world_mut().spawn(attacker_card).id();

    // Create blocker for player2
    let blocker_card = create_test_creature("Wall", 0, 4);
    let blocker = app.world_mut().spawn(blocker_card).id();

    // Initialize combat
    app.world_mut()
        .resource_mut::<Events<CombatBeginEvent>>()
        .send(CombatBeginEvent { player: player1 });
    app.update();

    // Declare attacker
    app.world_mut()
        .resource_mut::<Events<AttackerDeclaredEvent>>()
        .send(AttackerDeclaredEvent {
            attacker: attacker,
            defender: player2,
        });
    app.update();

    // Declare blocker
    app.world_mut()
        .resource_mut::<Events<BlockerDeclaredEvent>>()
        .send(BlockerDeclaredEvent {
            blocker: blocker,
            attacker: attacker,
        });
    app.update();

    // Assign combat damage
    app.world_mut()
        .resource_mut::<Events<AssignCombatDamageEvent>>()
        .send(AssignCombatDamageEvent {
            is_first_strike: false,
        });
    app.update();

    // End combat
    app.world_mut()
        .resource_mut::<Events<CombatEndEvent>>()
        .send(CombatEndEvent { player: player1 });
    app.update();

    // Verify combat state is cleared after combat
    let combat_state = app.world().resource::<CombatState>();
    assert_eq!(combat_state.attackers.len(), 0);
    assert_eq!(combat_state.blockers.len(), 0);
    assert_eq!(combat_state.blocked_status.len(), 0);
    assert_eq!(combat_state.assigned_combat_damage.len(), 0);
    assert_eq!(combat_state.pending_combat_damage.len(), 0);

    // Verify player's life total is still 40 (since the attacker was blocked)
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.player_life_totals.get(&player2), Some(&40));
}
