# Integration Testing

Integration testing in Rummage verifies that different components and systems work correctly together. This level of testing focuses on interactions between systems and provides confidence that our isolated components function properly when combined.

## Overview

Integration tests in Rummage:
- Test multiple systems working together
- Verify cross-component interactions
- Validate complex game mechanics
- Ensure subsystems combine correctly

## Testing Approach

Unlike unit tests that focus on isolated components, integration tests examine how multiple parts of the system interact:

1. **System Combination**: Test multiple systems operating together
2. **Multi-Step Processes**: Validate sequences of game actions
3. **Cross-Component Interactions**: Verify components correctly modify each other
4. **State Transitions**: Ensure state changes proceed correctly across systems

## Example: Card Casting and Resolution

```rust
#[test]
fn test_cast_and_resolve_instant() {
    // Setup test environment with multiple systems
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        ManaPlugins,
        CardPlugins,
        StackPlugins,
    ));
    
    // Add relevant systems
    app.add_systems(Update, (
        cast_spell_system,
        pay_mana_costs_system,
        resolve_spells_system,
        apply_effects_system,
    ));
    
    // Create test entities
    let player = setup_test_player(&mut app);
    let opponent = setup_test_player(&mut app);
    
    // Give the player a Lightning Bolt card in hand
    let lightning_bolt = app.world.spawn((
        CardMarker,
        InstantType,
        LightningBoltCard,
        InZone { zone: app.world.resource::<PlayerData>().get_hand(player) },
        Controller { player },
        ManaCost::from_string("{R}"),
    )).id();
    
    // Give player mana
    app.world.send_event(AddManaEvent {
        player,
        mana: ManaAmount { red: 1, ..Default::default() },
    });
    
    // Cast the Lightning Bolt targeting opponent
    app.world.send_event(CastSpellEvent {
        card: lightning_bolt,
        controller: player,
        targets: vec![opponent],
    });
    app.update();
    
    // Verify card is on the stack
    let stack = app.world.resource::<GameStack>();
    assert!(stack.contains(lightning_bolt), "Lightning Bolt should be on the stack");
    
    // Resolve the spell
    app.world.send_event(ResolveTopOfStackEvent);
    app.update();
    
    // Verify opponent took 3 damage
    let opponent_data = app.world.get::<PlayerData>(opponent).unwrap();
    assert_eq!(opponent_data.life, 17, "Opponent should take 3 damage");
    
    // Verify card is now in graveyard
    let card_zone = app.world.get::<InZone>(lightning_bolt).unwrap();
    let graveyard = app.world.resource::<PlayerData>().get_graveyard(player);
    assert_eq!(card_zone.zone, graveyard, "Card should be in graveyard after resolution");
}
```

## Testing Card Interactions

Integration testing is crucial for verifying complex card interactions:

```rust
#[test]
fn test_counterspell_interaction() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(GameTestPlugins);
    
    // Create players and initial game state
    let player1 = setup_test_player(&mut app);
    let player2 = setup_test_player(&mut app);
    
    // Player 1 has a Lightning Bolt
    let lightning_bolt = create_card(&mut app, "Lightning Bolt", player1);
    
    // Player 2 has a Counterspell
    let counterspell = create_card(&mut app, "Counterspell", player2);
    
    // Give both players appropriate mana
    app.world.send_event(AddManaEvent {
        player: player1,
        mana: ManaAmount { red: 1, ..Default::default() },
    });
    
    app.world.send_event(AddManaEvent {
        player: player2,
        mana: ManaAmount { blue: 2, ..Default::default() },
    });
    
    // Player 1 casts Lightning Bolt targeting Player 2
    app.world.send_event(CastSpellEvent {
        card: lightning_bolt,
        controller: player1,
        targets: vec![player2],
    });
    app.update();
    
    // Player 2 casts Counterspell targeting Lightning Bolt
    app.world.send_event(CastSpellEvent {
        card: counterspell,
        controller: player2,
        targets: vec![lightning_bolt],
    });
    app.update();
    
    // Resolve Counterspell (top of stack)
    app.world.send_event(ResolveTopOfStackEvent);
    app.update();
    
    // Verify Lightning Bolt is countered (not on stack, in graveyard)
    let stack = app.world.resource::<GameStack>();
    assert!(!stack.contains(lightning_bolt), "Lightning Bolt should not be on stack");
    
    let card_zone = app.world.get::<InZone>(lightning_bolt).unwrap();
    let graveyard = app.world.resource::<PlayerData>().get_graveyard(player1);
    assert_eq!(card_zone.zone, graveyard, "Lightning Bolt should be in graveyard");
    
    // Verify player 2 didn't take damage
    let player2_data = app.world.get::<PlayerData>(player2).unwrap();
    assert_eq!(player2_data.life, 20, "Player 2 should not have taken damage");
}
```

## Testing Game Phases

Integration tests verify correct progression through game phases:

```rust
#[test]
fn test_turn_structure() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(GameTestPlugins);
    
    // Create players
    let player1 = setup_test_player(&mut app);
    let player2 = setup_test_player(&mut app);
    
    // Set up turn system
    app.insert_resource(GameState {
        active_player: player1,
        phase: Phase::Beginning,
        step: Step::Untap,
        turn_number: 1,
    });
    
    // Progress through phases
    let phases_to_test = vec![
        Phase::Beginning,
        Phase::PreCombatMain,
        Phase::Combat,
        Phase::PostCombatMain,
        Phase::Ending,
    ];
    
    for phase in phases_to_test {
        // Verify current phase
        let game_state = app.world.resource::<GameState>();
        assert_eq!(game_state.phase, phase, "Game should be in correct phase");
        
        // Advance phase
        app.world.send_event(AdvancePhaseEvent);
        app.update();
    }
    
    // Verify turn passed to next player
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.active_player, player2, "Active player should now be player 2");
    assert_eq!(game_state.turn_number, 2, "Turn number should be incremented");
}
```

## Best Practices

For effective integration testing in Rummage:

1. **Use Game Test Plugins**: Create specialized plugin sets that include required functionality
2. **Test End-to-End Flows**: Validate complete gameplay sequences
3. **Focus on Boundaries**: Pay special attention to where systems interact
4. **Test Priority Systems**: Verify systems execute in correct order
5. **Validate Game Rules**: Ensure rule implementations work correctly together
6. **Set Up Complete States**: Create realistic game states for testing

## Related Documentation

For more information on testing in Rummage, see:

- [Unit Testing](unit_testing.md)
- [End-to-End Testing](end_to_end_testing.md)
- [Snapshot Testing](../core_systems/snapshot/testing.md) 