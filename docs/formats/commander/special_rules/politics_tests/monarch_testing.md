# The Monarch Mechanic Testing

## Overview

This document details the testing approach for the Monarch game mechanic within the Commander format. The Monarch is a special designation that can be passed between players and provides card advantage through its "draw a card at the beginning of your end step" effect.

## Test Components

### Monarch Assignment Tests

Testing the ways a player can become the Monarch:

```rust
#[test]
fn test_initial_monarch_assignment() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Create a test player
    let player = app.world.spawn(Player::default()).id();
    
    // Assign monarch to player
    app.world.send_event(AssignMonarchEvent { player });
    app.update();
    
    // Verify player is now the monarch
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(player));
}

#[test]
fn test_monarch_assignment_via_card_effect() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Create test players
    let player1 = app.world.spawn(Player::default()).id();
    let player2 = app.world.spawn(Player::default()).id();
    
    // Create a monarch-granting card (e.g., "Court of Grace")
    let monarch_card = app.world.spawn((
        Card::default(),
        MonarchGrantingComponent,
    )).id();
    
    // Assign to player1
    give_card_to_player(&mut app, monarch_card, player1);
    
    // Cast the card
    cast_card(&mut app, player1, monarch_card);
    app.update();
    
    // Verify player1 is now the monarch
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(player1));
    
    // Test changing the monarch to player2
    app.world.send_event(AssignMonarchEvent { player: player2 });
    app.update();
    
    // Verify player2 is now the monarch
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(player2));
}
```

### Combat-Based Monarch Transfer Tests

Test monarch transfer through combat damage:

```rust
#[test]
fn test_monarch_transfer_through_combat() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(CombatPlugin);
       
    // Create test players
    let player1 = app.world.spawn(Player::default()).id();
    let player2 = app.world.spawn(Player::default()).id();
    
    // Make player1 the monarch
    app.world.send_event(AssignMonarchEvent { player: player1 });
    app.update();
    
    // Create creature controlled by player2
    let creature = app.world.spawn((
        Creature::default(),
        Permanent { controller: player2, ..Default::default() },
    )).id();
    
    // Simulate combat damage to player1
    app.world.send_event(CombatDamageEvent {
        source: creature,
        target: player1,
        amount: 2,
    });
    app.update();
    
    // Verify monarch transferred to player2
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(player2));
}
```

## Monarch Effect Tests

Testing the effects of being the monarch:

```rust
#[test]
fn test_monarch_card_draw_effect() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(TurnStructurePlugin);
       
    // Create test player
    let player = app.world.spawn(Player::default()).id();
    
    // Make player the monarch
    app.world.send_event(AssignMonarchEvent { player });
    app.update();
    
    // Get initial hand size
    let initial_hand_size = get_player_hand_size(&app, player);
    
    // Simulate end step for monarch
    app.world.resource_mut::<TurnState>().current_player = player;
    app.world.resource_mut::<TurnState>().current_phase = Phase::End;
    app.update();
    
    // Trigger end step effects
    app.world.send_event(PhaseEndEvent { phase: Phase::End });
    app.update();
    
    // Verify player drew a card
    let new_hand_size = get_player_hand_size(&app, player);
    assert_eq!(new_hand_size, initial_hand_size + 1);
}
```

## Edge Case Tests

Testing edge cases and unusual interactions:

```rust
#[test]
fn test_monarch_persistence_across_turns() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(TurnStructurePlugin);
       
    // Setup multi-player game
    let players = setup_four_player_game(&mut app);
    
    // Make first player the monarch
    app.world.send_event(AssignMonarchEvent { player: players[0] });
    app.update();
    
    // Simulate full turn cycle
    for _ in 0..4 {
        advance_turn(&mut app);
    }
    
    // Verify player is still monarch after full cycle
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(players[0]));
}

#[test]
fn test_monarch_when_player_leaves_game() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup multi-player game
    let players = setup_four_player_game(&mut app);
    
    // Make player 1 the monarch
    app.world.send_event(AssignMonarchEvent { player: players[0] });
    app.update();
    
    // Player leaves game
    app.world.send_event(PlayerLeftGameEvent { player: players[0] });
    app.update();
    
    // Verify monarch is reset
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, None);
    
    // Verify next player can become monarch
    app.world.send_event(AssignMonarchEvent { player: players[1] });
    app.update();
    
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(players[1]));
}
```

## Multiple Monarch-Granting Effect Tests

Testing what happens when multiple effects attempt to assign the monarch:

```rust
#[test]
fn test_simultaneous_monarch_granting_effects() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin);
       
    // Setup players
    let player1 = app.world.spawn(Player::default()).id();
    let player2 = app.world.spawn(Player::default()).id();
    
    // Create stack of effects
    let stack = app.world.resource_mut::<Stack>();
    
    // Add monarch-granting effects to stack
    stack.push(StackItem {
        effect_type: EffectType::GrantMonarch,
        source_player: player1,
        target_player: Some(player1),
        ..Default::default()
    });
    
    stack.push(StackItem {
        effect_type: EffectType::GrantMonarch,
        source_player: player2,
        target_player: Some(player2),
        ..Default::default()
    });
    
    // Resolve stack
    resolve_stack(&mut app);
    
    // Verify last effect resolved made player2 the monarch (LIFO order)
    let monarch = app.world.resource::<MonarchState>();
    assert_eq!(monarch.current_monarch, Some(player2));
}
```

## Integration Tests

Testing monarch interactions with other systems:

```rust
#[test]
fn test_monarch_integration_with_replacement_effects() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(ReplacementEffectsPlugin);
       
    // Setup test
    let player = app.world.spawn(Player::default()).id();
    
    // Create a replacement effect that modifies card draw
    let replacement = app.world.spawn((
        ReplacementEffect {
            effect_type: EffectType::DrawCard,
            replacement: ReplacementType::DrawExtraCard,
            controller: player,
        },
    )).id();
    
    // Make player the monarch
    app.world.send_event(AssignMonarchEvent { player });
    app.update();
    
    // Simulate end step
    let initial_hand_size = get_player_hand_size(&app, player);
    
    app.world.resource_mut::<TurnState>().current_player = player;
    app.world.resource_mut::<TurnState>().current_phase = Phase::End;
    app.world.send_event(PhaseEndEvent { phase: Phase::End });
    app.update();
    
    // Verify player drew 2 cards (1 from monarch + 1 from replacement)
    let new_hand_size = get_player_hand_size(&app, player);
    assert_eq!(new_hand_size, initial_hand_size + 2);
}
```

## UI Testing

Testing the monarch UI representation:

```rust
#[test]
fn test_monarch_ui_representation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(PoliticsPlugin)
       .add_plugin(UiPlugin);
       
    // Setup player
    let player = app.world.spawn(Player::default()).id();
    
    // Make player the monarch
    app.world.send_event(AssignMonarchEvent { player });
    app.update();
    
    // Query for monarch UI elements
    let monarch_indicators = app.world.query_filtered::<Entity, With<MonarchIndicator>>()
        .iter(&app.world)
        .collect::<Vec<_>>();
        
    // Verify monarch indicator exists
    assert!(!monarch_indicators.is_empty());
    
    // Verify monarch indicator is attached to player
    let indicator = monarch_indicators[0];
    let parent = app.world.get::<Parent>(indicator).unwrap();
    
    // Find player entity in UI hierarchy
    let player_ui = find_player_ui_entity(&app, player);
    assert_eq!(parent.get(), player_ui);
}
```

## Conclusion

The Monarch testing suite ensures that this important political mechanic functions correctly in all scenarios. These tests verify that the Monarch designation is properly assigned, transferred, and provides its card advantage benefit consistently. 