# Game Engine Testing Guide

## Overview

This guide outlines the comprehensive testing approach for the Rummage Magic: The Gathering Commander game engine. Testing a complex game engine requires multiple layers of validation to ensure rules are correctly implemented and interactions work as expected.

## Testing Philosophies

The Rummage testing strategy follows these core philosophies:

1. **Rules-First Testing**: Test cases are derived directly from the MTG Comprehensive Rules
2. **Isolation and Integration**: Test components in isolation before testing their interactions
3. **Edge Case Coverage**: Explicitly test corner cases and unusual card interactions
4. **Performance Validation**: Ensure the engine performs well under various game conditions
5. **Reproducibility**: All tests should be deterministic and repeatable
6. **Visual Consistency**: Ensure consistent rendering across platforms and game states

## Testing Layers

The game engine testing is structured in layers:

### 1. Unit Tests

Unit tests validate individual components in isolation. Each module should have comprehensive unit tests covering:

- Basic functionality
- Edge cases
- Error handling
- Public interface contracts

Example for the stack system:

```rust
#[test]
fn test_stack_push_and_resolve() {
    let mut app = App::new();
    // Setup minimal test environment
    app.add_plugins(MinimalPlugins)
       .add_plugin(StackPlugin);
    
    // Add a spell to the stack
    let spell_entity = app.world.spawn_empty().id();
    app.world.send_event(StackPushEvent {
        entity: spell_entity,
        source: None,
    });
    
    // Run systems
    app.update();
    
    // Verify stack state
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    
    // Resolve the top item
    app.world.send_event(StackResolveTopEvent);
    app.update();
    
    // Verify stack is empty
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 0);
}
```

### 2. Integration Tests

Integration tests verify the interaction between multiple components. Key integration scenarios include:

- Turn structure and phase progression
- Combat resolution
- Stack interaction with permanents
- Zone transitions

Example integration test:

```rust
#[test]
fn test_spell_cast_and_resolve_integration() {
    let mut app = App::new();
    // Setup more complete environment
    app.add_plugins(GameEngineTestPlugins)
       .add_systems(Startup, setup_test_game);
    
    // Create test player and card
    let (player, card) = setup_test_player_with_card(&mut app);
    
    // Cast the spell
    app.world.send_event(CastSpellEvent {
        player,
        card,
        targets: Vec::new(),
        mode: CastMode::Normal,
    });
    
    // Run systems to process the cast
    app.update();
    
    // Verify card moved to stack
    let stack = app.world.resource::<Stack>();
    assert!(stack.contains(card));
    
    // Resolve the stack
    resolve_stack_completely(&mut app);
    
    // Verify expected outcome based on card type
    let card_type = app.world.get::<CardType>(card).unwrap();
    match *card_type {
        CardType::Creature => {
            // Verify creature entered battlefield
            let battlefield = app.world.resource::<Battlefield>();
            assert!(battlefield.contains(card));
        },
        CardType::Sorcery => {
            // Verify sorcery went to graveyard
            let graveyard = get_player_graveyard(&app, player);
            assert!(graveyard.contains(card));
        },
        // Handle other card types
        _ => {}
    }
}
```

### 3. End-to-End Tests

End-to-end tests simulate complete game scenarios to validate the engine as a whole:

```rust
#[test]
fn test_complete_game_scenario() {
    let mut app = App::new();
    // Setup full game environment
    app.add_plugins(FullGameTestPlugins)
       .add_systems(Startup, setup_full_test_game);
    
    // Load predefined scenario
    let scenario = TestScenario::load("scenarios/two_player_creature_combat.json");
    scenario.apply_to_app(&mut app);
    
    // Run a fixed number of turns
    run_turns(&mut app, 3);
    
    // Verify expected game state
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.active_player_index, 1);
    
    // Verify player life totals
    let players = app.world.query::<&Player>().iter(&app.world).collect::<Vec<_>>();
    assert_eq!(players[0].life_total, 35);
    assert_eq!(players[1].life_total, 38);
    
    // Verify battlefield state
    let battlefield = app.world.resource::<Battlefield>();
    assert_eq!(battlefield.creatures_for_player(players[0].entity).count(), 2);
    assert_eq!(battlefield.creatures_for_player(players[1].entity).count(), 1);
}
```

### 4. Visual Differential Testing

Visual differential testing ensures consistent rendering across platforms and updates:

```rust
#[test]
fn test_card_rendering_consistency() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(VisualTestingPlugin)
       .add_systems(Startup, setup_card_rendering_test);
    
    // Configure test environment
    let card_states = [
        "in_hand", 
        "on_battlefield", 
        "tapped", 
        "with_counters"
    ];
    
    for state in &card_states {
        // Configure card state
        setup_card_state(&mut app, state);
        app.update();
        
        // Capture rendering
        if let Some(screenshot) = take_screenshot(&app) {
            // Compare with reference
            match load_reference_image(&format!("card_{}.png", state)) {
                Ok(reference) => {
                    let result = compare_images(&screenshot, &reference);
                    assert!(
                        result.similarity_score > 0.99, 
                        "Card rendering for state '{}' differs from reference", 
                        state
                    );
                },
                Err(_) => {
                    // Generate reference if not exists
                    let _ = save_reference_image(screenshot, &format!("card_{}.png", state));
                }
            }
        }
    }
}
```

## Test Data Management

### Card Test Database

A specialized test card database simplifies testing of specific interactions:

```rust
// Access test cards by specific properties
let board_wipe = test_cards::get_card("board_wipe");
let counter_spell = test_cards::get_card("counter_spell");
let indestructible_creature = test_cards::get_card("indestructible_creature");

// Test interaction
test_interaction(board_wipe, indestructible_creature);
```

### Scenario Files

Predefined test scenarios enable reproducible complex game states:

```json
{
  "players": [
    {
      "name": "Player 1",
      "life": 40,
      "battlefield": ["test_cards/serra_angel", "test_cards/sol_ring"],
      "hand": ["test_cards/counterspell", "test_cards/lightning_bolt"],
      "graveyard": ["test_cards/llanowar_elves"]
    },
    {
      "name": "Player 2",
      "life": 36,
      "battlefield": ["test_cards/goblin_guide", "test_cards/birds_of_paradise"],
      "hand": ["test_cards/wrath_of_god"],
      "graveyard": []
    }
  ],
  "turn": {
    "active_player": 0,
    "phase": "main1",
    "priority_player": 0
  }
}
```

## Testing Particular Systems

### Mana System Testing

The mana system requires specific testing for:

1. **Mana Production**: Test ability to produce mana from various sources
2. **Mana Payment**: Test payment for spells and abilities
3. **Mana Restrictions**: Test "spend only on X" restrictions
4. **Color Identity**: Test commander color identity rules

```rust
#[test]
fn test_mana_restrictions() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(ManaPlugin);
    
    // Create a player with a mana pool
    let player = setup_test_player(&mut app);
    
    // Add mana with "spend this mana only on creature spells" restriction
    add_restricted_mana(&mut app, player, ManaColor::Green, 3, ManaRestriction::CreatureSpells);
    
    // Test allowed payment
    let creature_card = spawn_test_card(&mut app, "Grizzly Bears");
    let result = try_pay_mana_cost(&mut app, player, creature_card);
    assert!(result.is_success);
    
    // Test restricted payment
    let noncreature_card = spawn_test_card(&mut app, "Giant Growth");
    let result = try_pay_mana_cost(&mut app, player, noncreature_card);
    assert!(result.is_failure);
    assert_eq!(result.failure_reason, ManaPaymentFailure::RestrictionViolation);
}
```

### Combat System Testing

Combat testing should validate:

1. **Attack Declaration**: Rules about who can attack
2. **Blocker Declaration**: Valid blocking assignments
3. **Combat Damage**: Correct damage assignment and processing
4. **Combat Effects**: Triggers that happen during combat

```rust
#[test]
fn test_combat_damage_assignment() {
    let mut app = App::new();
    app.add_plugins(GameEngineTestPlugins);
    
    // Setup attacker with 4/4 stats
    let attacker = spawn_test_creature(&mut app, 4, 4);
    
    // Setup two blockers: 2/2 and 1/1
    let blocker1 = spawn_test_creature(&mut app, 2, 2);
    let blocker2 = spawn_test_creature(&mut app, 1, 1);
    
    // Declare attack
    declare_attacker(&mut app, attacker);
    
    // Declare blockers
    declare_blockers(&mut app, vec![blocker1, blocker2], attacker);
    
    // Assign damage: 2 to first blocker, 2 to second blocker
    assign_combat_damage(&mut app, attacker, vec![(blocker1, 2), (blocker2, 2)]);
    
    // Process damage
    process_combat_damage(&mut app);
    
    // Verify results
    assert!(is_creature_dead(&app, blocker1));
    assert!(is_creature_dead(&app, blocker2));
    assert!(!is_creature_dead(&app, attacker));
}
```

### Stack and Priority Testing

Testing stack interactions requires:

1. **Proper Sequencing**: Items resolve in LIFO order
2. **Priority Passing**: Correct priority assignment during resolution
3. **Interruption**: Ability to respond to items on the stack
4. **Special Actions**: Actions that don't use the stack

```rust
#[test]
fn test_stack_priority_and_responses() {
    let mut app = App::new();
    app.add_plugins(GameEngineTestPlugins);
    
    // Setup players
    let player1 = setup_test_player(&mut app);
    let player2 = setup_test_player(&mut app);
    
    // Setup cards
    let lightning_bolt = spawn_test_card(&mut app, "Lightning Bolt");
    let counterspell = spawn_test_card(&mut app, "Counterspell");
    
    // Give cards to players
    give_card_to_player(&mut app, lightning_bolt, player1);
    give_card_to_player(&mut app, counterspell, player2);
    
    // Player 1 casts Lightning Bolt
    cast_spell(&mut app, player1, lightning_bolt, Some(player2));
    
    // Verify Lightning Bolt is on the stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    
    // Player 2 responds with Counterspell
    cast_spell(&mut app, player2, counterspell, Some(lightning_bolt));
    
    // Verify both spells are on the stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 2);
    assert_eq!(stack.items[0].card, counterspell);
    assert_eq!(stack.items[1].card, lightning_bolt);
    
    // Resolve stack
    resolve_stack_completely(&mut app);
    
    // Verify both cards went to graveyard and Lightning Bolt didn't deal damage
    assert!(is_card_in_graveyard(&app, counterspell, player2));
    assert!(is_card_in_graveyard(&app, lightning_bolt, player1));
    
    let player2_resource = app.world.get::<Player>(player2).unwrap();
    assert_eq!(player2_resource.life_total, 40); // Unchanged
}
```

## Testing Best Practices

### Arrange-Act-Assert Pattern

Follow the Arrange-Act-Assert pattern in test implementation:

```rust
#[test]
fn test_some_functionality() {
    // ARRANGE: Set up the test environment
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(SystemUnderTest);
    
    let test_entity = setup_test_entity(&mut app);
    
    // ACT: Perform the action being tested
    perform_action(&mut app, test_entity);
    app.update(); // Run systems
    
    // ASSERT: Verify expected outcomes
    let result = get_result(&app, test_entity);
    assert_eq!(result, expected_value);
}
```

### Use Test Fixtures

Create reusable test fixtures to simplify test implementation:

```rust
// Fixture for tests involving combat
fn setup_combat_fixture(app: &mut App) -> CombatFixture {
    app.add_plugins(MinimalPlugins)
       .add_plugin(CombatPlugin);
       
    let player1 = app.world.spawn((Player { life_total: 40, ..Default::default() })).id();
    let player2 = app.world.spawn((Player { life_total: 40, ..Default::default() })).id();
    
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, ..Default::default() },
        Permanent { controller: player1, ..Default::default() },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2, ..Default::default() },
        Permanent { controller: player2, ..Default::default() },
    )).id();
    
    CombatFixture {
        player1,
        player2,
        attacker,
        blocker,
    }
}
```

### Focused Test Cases

Keep test cases focused on a single behavior or requirement:

```rust
// GOOD: Focused test
#[test]
fn creatures_with_deathtouch_destroy_blockers() {
    // Test setup
    let creature_with_deathtouch = setup_deathtouch_creature();
    let normal_creature = setup_normal_creature();
    
    // Combat interaction
    simulate_combat(creature_with_deathtouch, normal_creature);
    
    // Verification
    assert!(normal_creature.is_destroyed());
}

// BAD: Unfocused test
#[test]
fn test_deathtouch_and_trample_and_first_strike() {
    // Too many interactions being tested at once
    // Makes it hard to understand test failures
}
```

### Property-Based Testing

Use property-based testing for rules that should hold across many inputs:

```rust
#[test]
fn test_mana_payment_properties() {
    proptest!(|(cost: ManaCost, mana_pool: ManaPool)| {
        // Property: If payment succeeds, the mana pool should decrease by exactly the cost
        let initial_total = mana_pool.total_mana();
        let result = pay_mana_cost(cost, &mut mana_pool.clone());
        
        if result.is_success {
            let new_total = mana_pool.total_mana();
            let used_mana = initial_total - new_total;
            
            // The mana used should equal the cost
            prop_assert_eq!(used_mana, cost.total_mana());
        }
    });
}
```

## Test Debugging Tools

### Logging in Tests

Use descriptive logging to help debug test failures:

```rust
#[test]
fn test_with_detailed_logging() {
    // Set up the test
    info!("Setting up test with player 1 having 3 creatures and player 2 having 2 enchantments");
    
    // Configure logging level for test
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(LogPlugin {
            level: Level::DEBUG,
            filter: "test=debug,game_engine=debug".to_string(),
       });
    
    // Detailed action description
    debug!("Player 1 casting board wipe spell");
    
    // Log outcome for debugging
    info!("Expected: All creatures destroyed, enchantments remain. Got: {} creatures, {} enchantments",
          remaining_creatures, remaining_enchantments);
}
```

### State Snapshots

Create snapshots of game state for easier debugging:

```rust
#[test]
fn test_complex_interaction() {
    let mut app = App::new();
    // Setup test
    
    // Take snapshot before action
    let before_snapshot = take_game_state_snapshot(&app);
    save_snapshot("before_action.json", &before_snapshot);
    
    // Perform action
    perform_complex_action(&mut app);
    
    // Take snapshot after action
    let after_snapshot = take_game_state_snapshot(&app);
    save_snapshot("after_action.json", &after_snapshot);
    
    // Verify expected changes
    verify_state_changes(&before_snapshot, &after_snapshot);
}
```

## Performance Testing

Include performance testing as part of your test suite:

```rust
#[test]
fn benchmark_large_board_state() {
    let mut app = App::new();
    app.add_plugins(GameEngineTestPlugins);
    
    // Create a large board state
    setup_large_board_state(&mut app, 100); // 100 permanents per player
    
    // Measure time for operations
    let start = std::time::Instant::now();
    process_turn_cycle(&mut app);
    let duration = start.elapsed();
    
    // Log performance results
    info!("Processing turn with large board took: {:?}", duration);
    
    // Assert performance requirements
    assert!(duration < std::time::Duration::from_millis(100),
            "Turn processing too slow: {:?}", duration);
}
```

## Continuous Integration

Ensure your tests run in CI:

```yaml
# .github/workflows/tests.yml
name: Game Engine Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      
      - name: Run unit tests
        run: cargo test --lib
        
      - name: Run integration tests
        run: cargo test --test integration
        
      - name: Run visual tests
        run: cargo test --test visual
        
      - name: Run performance tests
        run: cargo test --test performance
```

## Common Testing Patterns

### Singleton Resource Validation

```rust
#[test]
fn test_resource_update() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Add initial resource
    app.insert_resource(GameState {
        turn_count: 0,
        active_player: 0,
    });
    
    // Add system that increments turn count
    app.add_systems(Update, increment_turn_system);
    
    // Run system
    app.update();
    
    // Verify resource was updated
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.turn_count, 1);
}
```

### Event Testing

```rust
#[test]
fn test_event_handling() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Add event and system
    app.add_event::<CardDrawEvent>();
    app.add_systems(Update, handle_card_draw);
    
    // Send test event
    app.world.resource_mut::<Events<CardDrawEvent>>().send(CardDrawEvent {
        player: Entity::from_raw(1),
        count: 3,
    });
    
    // Run system
    app.update();
    
    // Verify event was handled (check side effects)
    // ...
}
```

### Component Addition/Removal

```rust
#[test]
fn test_component_addition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // System that adds components
    app.add_systems(Update, add_damage_component_system);
    
    // Create test entity
    let entity = app.world.spawn(Creature {
        power: 2,
        toughness: 2,
    }).id();
    
    // Trigger damage
    app.world.resource_mut::<Events<DamageEvent>>().send(DamageEvent {
        target: entity,
        amount: 1,
    });
    
    // Run system
    app.update();
    
    // Verify component was added
    assert!(app.world.get::<Damaged>(entity).is_some());
    assert_eq!(app.world.get::<Damaged>(entity).unwrap().amount, 1);
}
```

## Conclusion

A comprehensive testing strategy is essential for the Rummage MTG Commander game engine. By combining unit tests, integration tests, end-to-end tests, and visual tests, we can ensure that the engine correctly implements the MTG rules and provides a consistent player experience.

Remember, an untested game engine is a source of bugs and inconsistencies. Invest time in creating a robust test suite, and it will pay dividends in reduced debugging time and improved game quality. 