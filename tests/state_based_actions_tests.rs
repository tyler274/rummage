use bevy::prelude::*;
use rummage::card::{Card, CardTypes, CreatureOnField};
use rummage::game_engine::commander::{Commander, EliminationReason, PlayerEliminatedEvent};
use rummage::game_engine::state::{GameState, state_based_actions_system};
use rummage::game_engine::zones::{Zone, ZoneManager};
use rummage::mana::Mana;
use rummage::player::Player;

// Helper function to set up a test app with Commander-specific resources
fn setup_commander_test_app() -> (App, Vec<Entity>) {
    let mut app = App::new();

    // Register relevant events
    app.add_event::<PlayerEliminatedEvent>();

    // Create player entities
    let mut world = app.world_mut();
    let player1 = world
        .spawn(Player {
            name: "Player 1".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player2 = world
        .spawn(Player {
            name: "Player 2".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player3 = world
        .spawn(Player {
            name: "Player 3".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let player4 = world
        .spawn(Player {
            name: "Player 4".to_string(),
            life: 40,
            ..Default::default()
        })
        .id();

    let players = vec![player1, player2, player3, player4];

    // Initialize resources
    let mut game_state = GameState::default();
    game_state.set_turn_order(players.clone());

    // Initialize zone manager
    let mut zone_manager = ZoneManager::default();
    for player in &players {
        zone_manager.init_player_zones(*player);
    }

    // Add resources to the app
    app.insert_resource(game_state)
        .insert_resource(zone_manager);

    (app, players)
}

// Helper function to create a test commander
fn create_test_commander(owner: Entity) -> (Card, Commander) {
    let card = Card {
        name: "Test Commander".to_string(),
        cost: Mana::new_with_colors(5, 0, 0, 0, 0, 0),
        types: CardTypes::CREATURE | CardTypes::LEGENDARY,
        card_details: rummage::card::CardDetails::Creature(rummage::card::CreatureCard {
            power: 5,
            toughness: 5,
            creature_type: rummage::card::CreatureType::HUMAN,
        }),
        rules_text: "".to_string(),
    };

    let commander = Commander {
        owner,
        cast_count: 0,
        damage_dealt: Vec::new(),
        color_identity: Default::default(),
        is_partner: false,
        is_background: false,
        dealt_combat_damage_this_turn: Default::default(),
    };

    (card, commander)
}

// Helper function to create a test creature
fn create_test_creature(power: i32, toughness: i32, damage: u32) -> (Card, CreatureOnField) {
    let card = Card {
        name: "Test Creature".to_string(),
        cost: Mana::default(),
        types: CardTypes::CREATURE,
        card_details: rummage::card::CardDetails::Creature(rummage::card::CreatureCard {
            power,
            toughness, // Base toughness
            creature_type: rummage::card::CreatureType::HUMAN,
        }),
        rules_text: "".to_string(),
    };

    let creature = CreatureOnField {
        power_modifier: 0,
        toughness_modifier: 0,
        battle_damage: damage as u64, // Convert damage to battle_damage
        token: false,
    };

    (card, creature)
}

#[test]
fn test_player_life_loss_elimination() {
    let (mut app, players) = setup_commander_test_app();

    // Set player 1 to 0 life
    let mut player1 = app.world_mut().get_mut::<Player>(players[0]).unwrap();
    player1.life = 0;

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system
    app.update();

    // Verify player was eliminated
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.eliminated_players.contains(&players[0]));
    assert!(game_state.state_based_actions_performed);
}

#[test]
fn test_commander_damage_elimination() {
    let (mut app, players) = setup_commander_test_app();

    // Create a commander for player 1
    let (card, mut commander) = create_test_commander(players[0]);

    // Add lethal commander damage to player 2
    commander.damage_dealt.push((players[1], 21));

    let commander_entity = app.world_mut().spawn((card, commander)).id();

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system
    app.update();

    // Verify player was eliminated
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.eliminated_players.contains(&players[1]));
    assert!(game_state.state_based_actions_performed);
}

#[test]
fn test_empty_library_elimination() {
    let (mut app, players) = setup_commander_test_app();

    // Mark player as having drawn this turn
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.drawn_this_turn.push(players[0]);
    }

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system (library is already empty by default)
    app.update();

    // Verify player was eliminated
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.eliminated_players.contains(&players[0]));
    assert!(game_state.state_based_actions_performed);
}

#[test]
fn test_lethal_damage_on_creature() {
    let (mut app, players) = setup_commander_test_app();

    // Create a creature with lethal damage
    let (card, creature) = create_test_creature(3, 3, 3);

    // Spawn the creature to the battlefield
    let creature_entity = app.world_mut().spawn((card, creature)).id();

    // Add to the battlefield zone
    {
        let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();
        zone_manager.add_to_battlefield(players[0], creature_entity);
    }

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system
    app.update();

    // Check that state-based actions were performed
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.state_based_actions_performed);
}

#[test]
fn test_zero_toughness_creature() {
    let (mut app, players) = setup_commander_test_app();

    // Create a creature with 0 toughness
    let (card, creature) = create_test_creature(2, 0, 0);

    // Spawn the creature to the battlefield
    let creature_entity = app.world_mut().spawn((card, creature)).id();

    // Add to the battlefield zone
    {
        let mut zone_manager = app.world_mut().resource_mut::<ZoneManager>();
        zone_manager.add_to_battlefield(players[0], creature_entity);
    }

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system
    app.update();

    // Check that state-based actions were performed
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.state_based_actions_performed);
}

#[test]
fn test_game_over_detection() {
    let (mut app, players) = setup_commander_test_app();

    // Eliminate all but one player
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.eliminate_player(players[1], EliminationReason::LifeLoss);
        game_state.eliminate_player(players[2], EliminationReason::LifeLoss);
        game_state.eliminate_player(players[3], EliminationReason::LifeLoss);
    }

    // Add the state-based actions system
    app.add_systems(Update, state_based_actions_system);

    // Run the system
    app.update();

    // Check that the game is over with the correct winner
    let game_state = app.world().resource::<GameState>();
    assert!(game_state.is_game_over());
    assert_eq!(game_state.get_winner(), Some(players[0]));
}

#[test]
fn test_advancing_active_player_skips_eliminated() {
    let (mut app, players) = setup_commander_test_app();

    // Eliminate player 2
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.eliminate_player(players[1], EliminationReason::LifeLoss);
    }

    // Advance active player
    {
        let mut game_state = app.world_mut().resource_mut::<GameState>();
        game_state.advance_active_player();
    }

    // Check that we skipped player 2 and went to player 3
    let game_state = app.world().resource::<GameState>();
    assert_eq!(game_state.active_player, players[2]);
}
