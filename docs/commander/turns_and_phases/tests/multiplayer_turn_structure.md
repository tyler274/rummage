# Multiplayer Turn Structure Tests

## Overview

This document outlines test cases for Commander's multiplayer turn structure, focusing on turn order, extra turns, and phase management in a multiplayer environment. These tests ensure the game properly handles turn-based interactions in a format that can have 3-6 players.

## Test Case: Basic Turn Order

### Test: Turn Order in Multiplayer Game

```rust
#[test]
fn test_multiplayer_turn_order() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_turn_order, next_turn, check_active_player));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn((
        Player { id: 1 },
        PlayerName("Player 1".to_string()),
    )).id();
    
    let player2 = app.world.spawn((
        Player { id: 2 },
        PlayerName("Player 2".to_string()),
    )).id();
    
    let player3 = app.world.spawn((
        Player { id: 3 },
        PlayerName("Player 3".to_string()),
    )).id();
    
    let player4 = app.world.spawn((
        Player { id: 4 },
        PlayerName("Player 4".to_string()),
    )).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player1,
    });
    
    // Validate initial active player
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player1);
    
    // Execute a full turn and move to next player
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Verify active player changed to player 2
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player2);
    
    // Execute another turn
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Verify active player changed to player 3
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player3);
    
    // Simulate a full rotation
    app.world.send_event(EndTurnEvent);
    app.update();
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Should be back to player 1
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player1);
}
```

## Test Case: Player Elimination

### Test: Removing a Player from Turn Order

```rust
#[test]
fn test_player_elimination() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_turn_order, handle_player_elimination, next_turn));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player1,
    });
    
    // Player 2 is eliminated
    app.world.send_event(PlayerEliminatedEvent { player: player2 });
    app.update();
    
    // Verify turn order adjusted
    let turn_order = app.world.resource::<TurnOrder>();
    assert_eq!(turn_order.players, vec![player1, player3, player4]);
    
    // Current player should still be player 1
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player1);
    
    // End turn
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Verify active player skipped player 2 and is now player 3
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player3);
}
```

### Test: Eliminating the Active Player

```rust
#[test]
fn test_active_player_elimination() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_turn_order, handle_player_elimination, next_turn));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player1,
    });
    
    // Active player is eliminated
    app.world.send_event(PlayerEliminatedEvent { player: player1 });
    app.update();
    
    // Verify turn order adjusted and active player changed
    let turn_order = app.world.resource::<TurnOrder>();
    assert_eq!(turn_order.players, vec![player2, player3, player4]);
    
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player2);
}
```

## Test Case: Extra Turns

### Test: Player Takes an Extra Turn

```rust
#[test]
fn test_extra_turn() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_turn_order, handle_extra_turns, next_turn));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player1,
    });
    
    app.insert_resource(ExtraTurns::default());
    
    // Player 1 gets an extra turn
    app.world.send_event(ExtraTurnEvent { 
        player: player1,
        count: 1,
        source: EntitySource { entity: Entity::PLACEHOLDER },
    });
    
    // End current turn
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Verify player 1 gets an extra turn instead of going to player 2
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player1);
    
    // End extra turn
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Now it should go to player 2
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player2);
}
```

### Test: Multiple Extra Turns Across Players

```rust
#[test]
fn test_multiple_extra_turns() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_turn_order, handle_extra_turns, next_turn));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player1,
    });
    
    app.insert_resource(ExtraTurns::default());
    
    // Player 1 grants an extra turn to player 3
    app.world.send_event(ExtraTurnEvent { 
        player: player3,
        count: 1,
        source: EntitySource { entity: Entity::PLACEHOLDER },
    });
    
    // Player 2 grants an extra turn to themselves
    app.world.send_event(ExtraTurnEvent { 
        player: player2,
        count: 1,
        source: EntitySource { entity: Entity::PLACEHOLDER },
    });
    
    // End current turn (player 1)
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Regular turn order goes to player 2
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player2);
    
    // End turn (player 2)
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Player 2's extra turn happens before moving to player 3
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player2);
    
    // End extra turn (player 2)
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Now it's player 3's turn
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player3);
    
    // End turn (player 3)
    app.world.send_event(EndTurnEvent);
    app.update();
    
    // Player 3 gets extra turn
    let turn_manager = app.world.resource::<TurnManager>();
    assert_eq!(turn_manager.active_player, player3);
}
```

## Test Case: Phase Management

### Test: Phase Progression Through a Turn

```rust
#[test]
fn test_phase_progression() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_phase, handle_phase_transitions));
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Set up turn manager
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player,
    });
    
    // Progress through beginning phase
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Beginning(BeginningPhaseStep::Untap));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Beginning(BeginningPhaseStep::Upkeep));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Beginning(BeginningPhaseStep::Draw));
    
    // Move to main phase
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::PreCombatMain);
    
    // Move to combat
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Combat(CombatStep::Beginning));
    
    // Cycle through combat steps
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Combat(CombatStep::DeclareAttackers));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Combat(CombatStep::DeclareBlockers));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Combat(CombatStep::CombatDamage));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Combat(CombatStep::End));
    
    // Move to post-combat main
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::PostCombatMain);
    
    // Move to end step
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Ending(EndingPhaseStep::End));
    
    app.world.send_event(NextPhaseEvent);
    app.update();
    assert_eq!(app.world.resource::<TurnManager>().current_phase, Phase::Ending(EndingPhaseStep::Cleanup));
}
```

### Test: Turn-Based Actions in Each Phase

```rust
#[test]
fn test_turn_based_actions() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_untap_step, handle_draw_step, handle_phase_transitions));
       
    // Create player with permanents
    let player = app.world.spawn((
        Player {},
        Hand::default(),
    )).id();
    
    // Create some tapped permanents
    let permanent1 = app.world.spawn((
        Card { name: "Permanent 1".to_string() },
        Permanent,
        Status { tapped: true },
        Controller { player },
    )).id();
    
    let permanent2 = app.world.spawn((
        Card { name: "Permanent 2".to_string() },
        Permanent,
        Status { tapped: true },
        Controller { player },
    )).id();
    
    // Set up turn manager in untap step
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Untap),
        active_player: player,
    });
    
    // Process untap step
    app.update();
    
    // Verify permanents were untapped
    let status1 = app.world.get::<Status>(permanent1).unwrap();
    let status2 = app.world.get::<Status>(permanent2).unwrap();
    assert!(!status1.tapped);
    assert!(!status2.tapped);
    
    // Move to draw step
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Beginning(BeginningPhaseStep::Draw);
    
    // Create library with cards
    let card1 = app.world.spawn((
        Card { name: "Card 1".to_string() },
        Zone::Library,
        Owner { player },
    )).id();
    
    let library = app.world.spawn((
        Library { owner: player },
        Cards { entities: vec![card1] },
    )).id();
    
    // Process draw step
    app.update();
    
    // Verify card was drawn
    assert_eq!(app.world.get::<Zone>(card1).unwrap(), &Zone::Hand);
    let hand = app.world.get::<Hand>(player).unwrap();
    assert_eq!(hand.cards.len(), 1);
    assert!(hand.cards.contains(&card1));
}
```

## Test Case: Priority System

### Test: Priority Passing in Turn Order

```rust
#[test]
fn test_priority_passing() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_priority, update_turn_order));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::PreCombatMain,
        active_player: player1,
    });
    
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: true,
        all_players_passed: false,
    });
    
    // Active player passes priority
    app.world.send_event(PriorityPassedEvent { player: player1 });
    app.update();
    
    // Check priority passed to next player
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player2);
    
    // Second player passes
    app.world.send_event(PriorityPassedEvent { player: player2 });
    app.update();
    
    // Check priority passed to next player
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player3);
    
    // Third player passes
    app.world.send_event(PriorityPassedEvent { player: player3 });
    app.update();
    
    // Check priority passed to last player
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player4);
    
    // Last player passes
    app.world.send_event(PriorityPassedEvent { player: player4 });
    app.update();
    
    // All players passed, should move to next phase
    let priority = app.world.resource::<PrioritySystem>();
    assert!(priority.all_players_passed);
}
```

### Test: Priority After Casting a Spell

```rust
#[test]
fn test_priority_after_spell() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_priority, handle_spell_cast));
       
    // Create a multiplayer game with 4 players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::PreCombatMain,
        active_player: player1,
    });
    
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: true,
        all_players_passed: false,
    });
    
    // Create a spell
    let spell = app.world.spawn((
        Card { name: "Instant Spell".to_string() },
        Spell,
        Owner { player: player1 },
    )).id();
    
    // Player 1 casts a spell
    app.world.send_event(SpellCastEvent { 
        caster: player1,
        spell: spell,
    });
    app.update();
    
    // Verify spell is on the stack
    let stack = app.world.resource::<Stack>();
    assert!(!stack.is_empty());
    
    // Stack is not empty and priority passed to active player again
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player1);
    assert!(!priority.stack_is_empty);
    
    // All players need to pass again for spell to resolve
    app.world.send_event(PriorityPassedEvent { player: player1 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player2 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player3 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player4 });
    app.update();
    
    // Spell should resolve and priority goes back to active player
    let stack = app.world.resource::<Stack>();
    assert!(stack.is_empty());
    
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player1);
}
```

These test cases ensure the Commander game engine properly handles multiplayer turn structure, including phase management, turn order, and priority passing in multiplayer scenarios. 