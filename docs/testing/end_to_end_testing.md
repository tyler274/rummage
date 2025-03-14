# End-to-End Testing

End-to-End (E2E) testing in Rummage validates the entire game system by simulating complete games from start to finish. This approach ensures that all components work together correctly in realistic gameplay scenarios.

## Overview

E2E tests in Rummage:
- Test complete game flows
- Simulate realistic gameplay scenarios
- Verify high-level game rules
- Validate the entire system integration

## Testing Approach

E2E testing focuses on validating the complete game experience:

1. **Full Game Simulations**: Play through entire games automatically
2. **Scenario Testing**: Test specific game scenarios and edge cases
3. **System Integration**: Verify all systems work together cohesively
4. **Format Rules**: Validate compliance with format-specific rules

## Example: Basic Game Simulation

```rust
#[test]
fn test_basic_commander_game() {
    // Setup complete game environment
    let mut app = App::new();
    app.add_plugins(GamePlugins);
    
    // Create players with decks
    let player1 = setup_player_with_deck(&mut app, "test_decks/mono_red.json");
    let player2 = setup_player_with_deck(&mut app, "test_decks/mono_blue.json");
    
    // Set up game state
    app.insert_resource(GameState {
        active_player: player1,
        phase: Phase::Beginning,
        step: Step::Untap,
        turn_number: 1,
    });
    
    // Create test script with sequence of actions
    let test_script = TestScript::new()
        .mulligan(player1, 7)
        .mulligan(player2, 7)
        .play_land(player1, 0)  // Play first land in hand
        .pass_turn(player1)
        .play_land(player2, 0)
        .pass_turn(player2)
        .play_land(player1, 0)
        .cast_spell(player1, 1, vec![player2])  // Cast spell targeting opponent
        .pass_turn(player1)
        .play_land(player2, 0)
        .cast_spell(player2, 2, vec![]);  // Cast a non-targeting spell
    
    app.insert_resource(test_script);
    
    // Run simulation for 5 turns or until game ends
    for _ in 0..5 {
        if app.world.resource::<GameState>().is_game_over() {
            break;
        }
        
        // Process all actions for current turn
        while app.world.resource::<TestScript>().has_pending_actions() {
            app.update();
        }
        
        // Advance to next turn
        app.world.send_event(AdvanceTurnEvent);
        app.update();
    }
    
    // Verify game state after simulation
    let player1_data = app.world.get::<PlayerData>(player1).unwrap();
    let player2_data = app.world.get::<PlayerData>(player2).unwrap();
    
    // Check life totals
    assert!(player1_data.life < 20 || player2_data.life < 20, 
           "Game simulation should result in damage being dealt");
    
    // Check battlefield state
    let battlefield = app.world.resource::<Zones>().battlefield;
    let permanents = app.world.query::<(Entity, &InZone, &CardType)>()
                         .iter(&app.world)
                         .filter(|(_, zone, _)| zone.zone == battlefield)
                         .count();
    
    assert!(permanents > 0, "Battlefield should have permanents after simulation");
}
```

## Scripted Scenario Testing

Testing specific game scenarios allows verification of complex interactions:

```rust
#[test]
fn test_commander_tax_scenario() {
    // Setup environment
    let mut app = App::new();
    app.add_plugins(CommanderGamePlugins);
    
    // Setup players
    let player1 = setup_player_with_commander(&mut app, "Atraxa, Praetors' Voice");
    let player2 = setup_player_with_commander(&mut app, "Muldrotha, the Gravetide");
    
    // Give player1 enough mana to cast commander multiple times
    app.world.send_event(AddManaEvent {
        player: player1,
        mana: ManaAmount {
            white: 10,
            blue: 10,
            black: 10,
            green: 10,
            ..Default::default()
        },
    });
    
    // Get commander entity
    let commander = app.world.resource::<PlayerData>().get_commander(player1);
    
    // Cast commander once
    app.world.send_event(CastCommanderEvent { player: player1 });
    app.update();
    
    // Resolve and put in play
    app.world.send_event(ResolveTopOfStackEvent);
    app.update();
    
    // Verify commander tax is 0 for first cast
    let tax = app.world.get::<CommanderTax>(commander).unwrap();
    assert_eq!(tax.value, 0, "First cast should have no tax");
    
    // Destroy commander to send it to command zone
    app.world.send_event(DestroyPermanentEvent { permanent: commander });
    app.update();
    
    // Choose to put in command zone
    app.world.send_event(CommandZoneChoiceEvent { 
        commander,
        choice: CommanderZoneChoice::CommandZone,
    });
    app.update();
    
    // Cast commander again
    app.world.send_event(CastCommanderEvent { player: player1 });
    app.update();
    
    // Verify tax increased
    let tax = app.world.get::<CommanderTax>(commander).unwrap();
    assert_eq!(tax.value, 2, "Commander tax should be 2 after first recast");
    
    // Verify player paid correct amount of mana
    let player_data = app.world.get::<PlayerData>(player1).unwrap();
    assert!(player_data.mana_pool.is_empty(), "Player should have paid commander cost plus tax");
}
```

## Deck Testing

E2E tests can validate that complete decks function correctly:

```rust
#[test]
fn test_deck_gameplay() {
    // Setup
    let mut app = App::new();
    app.add_plugins(GameTestPlugins);
    
    // Load test deck from file
    let deck_list = load_deck_from_file("test_decks/combo_deck.json");
    
    // Create player with deck
    let player = setup_player_with_decklist(&mut app, deck_list);
    let opponent = setup_basic_opponent(&mut app);
    
    // Play out first 10 turns
    for _ in 0..10 {
        // Execute optimal play for test deck
        execute_optimal_turn(&mut app, player);
        
        // Let opponent take a basic turn
        execute_basic_turn(&mut app, opponent);
    }
    
    // Verify deck executed its strategy
    let combo_pieces_in_play = count_combo_pieces_in_play(&app, player);
    assert!(combo_pieces_in_play >= 2, "Deck should assemble at least part of its combo");
    
    // Check if player won or made significant progress
    let game_state = app.world.resource::<GameState>();
    let player_data = app.world.get::<PlayerData>(player).unwrap();
    let opponent_data = app.world.get::<PlayerData>(opponent).unwrap();
    
    let player_progress = calculate_game_progress(player_data, opponent_data);
    assert!(player_progress > 0.5, "Player should make significant game progress");
}
```

## Format Compliance Testing

E2E tests can verify compliance with format-specific rules:

```rust
#[test]
fn test_commander_format_rules() {
    // Setup
    let mut app = App::new();
    app.add_plugins(CommanderGamePlugins);
    
    // Test cases for format rules
    let test_cases = vec![
        // Deck with invalid color identity
        ("invalid_color_identity.json", false, "Color identity rule"),
        
        // Deck with cards banned in Commander
        ("banned_cards.json", false, "Banned list rule"),
        
        // Valid Commander deck
        ("valid_commander.json", true, "Valid deck"),
        
        // Deck with duplicate cards (except basic lands)
        ("duplicate_cards.json", false, "Singleton rule"),
    ];
    
    for (deck_file, should_be_valid, rule_name) in test_cases {
        // Attempt to create game with deck
        let result = try_create_commander_game(&mut app, deck_file);
        
        // Verify result matches expectation
        assert_eq!(result.is_ok(), should_be_valid, 
                  "Commander format rule check failed for: {}", rule_name);
    }
}
```

## Best Practices

For effective E2E testing in Rummage:

1. **Use Realistic Data**: Test with real deck lists and game scenarios
2. **Automate Gameplay**: Create helper functions for common game actions
3. **Test Multiple Paths**: Verify different game outcomes and decision paths
4. **Validate Format Rules**: Ensure compliance with format-specific regulations
5. **Isolate Test Cases**: Create focused scenarios for specific interactions
6. **Simulate Tournament Play**: Test rules used in competitive environments

## Related Documentation

For more information on testing in Rummage, see:

- [Unit Testing](unit_testing.md)
- [Integration Testing](integration_testing.md)
- [Snapshot Testing](../core_systems/snapshot/testing.md) 