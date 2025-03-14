# Testing Overview

## Introduction

The Rummage MTG Commander game engine employs a comprehensive testing strategy to ensure correctness, reliability, and performance across all game mechanics and user interactions.

## Core Testing Principles

Our testing framework is built on these foundational principles:

| Principle | Description | Implementation |
|-----------|-------------|----------------|
| **Rules Correctness** | All MTG rule implementations must be verified against official rules | Rule-specific test cases with expected outcomes |
| **Determinism** | Game states must evolve consistently with the same inputs | Seeded random tests, state verification |
| **Cross-Platform Consistency** | Behavior and visuals must be identical across platforms | Visual differential testing, behavior validation |
| **Performance** | System must maintain responsiveness under various conditions | Load testing, benchmarking key operations |
| **Accessibility** | Features must work with assistive technologies | Screen reader testing, keyboard navigation tests |

## Testing Pyramid

We implement a comprehensive testing pyramid with increasing scope and integration:

```
    ┌─────────────┐
    │   E2E &     │
    │  Visual     │
    │  Testing    │
    ├─────────────┤
    │Integration  │
    │  Testing    │
    ├─────────────┤
    │    Unit     │
    │  Testing    │
    └─────────────┘
```

### [Unit Testing](unit_testing.md)

Unit tests verify isolated components, focusing on correctness at the smallest levels:

```rust
#[test]
fn test_mana_cost_parsing() {
    // Given a mana cost string
    let cost_string = "{2}{W}{W}";
    
    // When we parse it
    let cost = ManaCost::from_string(cost_string);
    
    // Then the components should be correctly parsed
    assert_eq!(cost.generic, 2);
    assert_eq!(cost.white, 2);
    assert_eq!(cost.blue, 0);
    assert_eq!(cost.black, 0);
    assert_eq!(cost.red, 0);
    assert_eq!(cost.green, 0);
}
```

Key unit test categories:
- **Component Tests**: Verify individual ECS components
- **System Tests**: Test isolated ECS systems
- **Rules Tests**: Verify specific rule implementations
- **Parser Tests**: Test card text and effect parsing
- **Utility Tests**: Validate helper functions

### [Integration Testing](integration_testing.md)

Integration tests verify interactions between multiple systems:

```rust
#[test]
fn test_creature_etb_effects() {
    // Create a test world with necessary systems
    let mut app = App::new();
    app.add_plugins(TestingPlugins)
       .add_systems(Update, (cast_spell_system, resolve_etb_effects));
    
    // Set up a creature card with an ETB effect
    let creature_entity = setup_test_creature(&mut app, "Mulldrifter");
    
    // Cast the creature spell
    let player = setup_test_player(&mut app);
    app.world.send_event(CastSpellEvent {
        card: creature_entity,
        controller: player,
        targets: Vec::new(),
    });
    
    // Run systems to resolve the spell
    app.update();
    
    // Verify the ETB effect (draw 2 cards) occurred
    let player_data = app.world.get::<PlayerData>(player).unwrap();
    assert_eq!(player_data.hand.len(), 2, "Player should have drawn 2 cards from ETB effect");
}
```

Key integration test categories:
- **Card Interactions**: Test how cards affect each other
- **Game State Transitions**: Verify phase and turn changes
- **Player Actions**: Test sequences of player actions
- **Zone Transitions**: Validate card movement between zones

### [End-to-End Testing](end_to_end_testing.md)

E2E tests validate complete game scenarios from start to finish:

```rust
#[test]
fn test_basic_commander_game() {
    // Initialize complete game environment
    let mut app = App::new();
    app.add_plugins(CommanderGamePlugins);
    
    // Set up two players with predefined decks
    let player1 = setup_player(&mut app, "Player1", "Atraxa_Deck.json");
    let player2 = setup_player(&mut app, "Player2", "Muldrotha_Deck.json");
    
    // Define automated sequence of player actions
    let game_script = GameScript::from_file("test_scripts/basic_commander_game.yaml");
    app.insert_resource(game_script);
    
    // Run game simulation
    while !app.world.resource::<GameState>().is_game_over {
        app.update();
    }
    
    // Verify final game state
    let game_result = app.world.resource::<GameResult>();
    assert_eq!(game_result.winner, Some(player1), "Player 1 should win this scripted game");
    assert_eq!(game_result.turn_count, 12, "Game should last 12 turns");
}
```

Key E2E test categories:
- **Game Completion**: Test full games through to completion
- **Scenario Tests**: Verify specific game scenarios
- **Multiplayer Tests**: Validate multiplayer dynamics
- **Tournament Rules**: Test format-specific rules

### [Visual Testing](visual_testing.md)

Visual tests ensure consistent UI representation across platforms:

```rust
#[test]
fn test_card_rendering() {
    // Initialize app with rendering plugins
    let mut app = App::new();
    app.add_plugins(RenderingTestPlugins);
    
    // Create test card
    let card = setup_test_card(&mut app, "Lightning Bolt");
    
    // Render card to texture
    let texture = render_card_to_texture(&mut app, card);
    
    // Compare with reference image with tolerance for slight rendering differences
    let reference = load_reference_image("cards/lightning_bolt.png");
    let comparison = compare_images(texture, reference);
    
    assert!(comparison.similarity > 0.99, 
           "Card rendering should match reference image");
}
```

Key visual test categories:
- **Card Rendering**: Verify cards appear correctly
- **UI Components**: Test UI element appearance
- **Animations**: Validate animation correctness
- **Layout Tests**: Ensure responsive layouts work

### [Performance Testing](performance_testing.md)

Performance tests measure system responsiveness and resource usage:

```rust
#[test]
fn benchmark_large_board_state() {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn benchmark(c: &mut Criterion) {
        c.bench_function("100 card battlefield update", |b| {
            b.iter_batched(
                || setup_large_battlefield(100), // Setup 100 cards
                |mut app| {
                    // Measure the time for a complete update cycle
                    black_box(app.update());
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
    
    criterion_group!(benches, benchmark);
    criterion_main!(benches);
}
```

Key performance test areas:
- **Frame Rate**: Measure FPS under varying load
- **Memory Usage**: Track memory consumption
- **CPU Utilization**: Monitor processing requirements
- **Load Scaling**: Test with increasing entity counts

### Snapshot Testing

Snapshot testing allows us to capture and verify game state at specific points in time, providing a powerful tool for validating complex interactions and game state transitions.

Rather than duplicating the extensive snapshot testing documentation here, please refer to the comprehensive [Snapshot System Testing Documentation](../core_systems/snapshot/testing.md).

Key snapshot testing uses:
- **State Verification**: Validate game state correctness
- **Regression Testing**: Detect unintended changes to game behavior
- **Cross-System Testing**: Verify components work together correctly
- **Replay Validation**: Ensure replay system correctly reproduces game states

### Network Testing

Our network testing verifies the integrity of multiplayer functionality:

- **State Synchronization**: Verify game states remain synchronized across clients
- **Latency Simulation**: Test behavior under varying network conditions
- **Disconnection Handling**: Validate reconnection and recovery
- **Deterministic RNG**: Ensure random events produce identical results across clients

### Rules Compliance

Rules compliance testing verifies correct implementation of MTG rules:

- **Comprehensive Rules Coverage**: Tests mapped to official rules
- **Judge Corner Cases**: Special scenarios from tournament rulings
- **Rule Interactions**: Tests for complex rule interactions
- **Oracle Text Tests**: Validation against official card rulings

## Testing Infrastructure

### [CI/CD Pipeline](ci_cd_pipeline.md)

Our continuous integration pipeline ensures ongoing quality:

1. **Pull Request Checks**:
   - Fast unit tests run on every PR
   - Linting and formatting checks
   - Build verification

2. **Main Branch Validation**:
   - Full test suite runs
   - Performance regression checks
   - Cross-platform test matrix

3. **Release Preparation**:
   - Complete E2E and integration tests
   - Visual regression testing
   - Performance benchmarking

### Test Data Management

We maintain structured test datasets:

- **Card Database**: Test card definitions with expected behaviors
- **Game Scenarios**: Predefined game states for testing
- **Board States**: Complex battlefield configurations
- **Performance Benchmarks**: Standard scenarios for consistency

## Contributing Tests

To contribute new tests:

1. **Identify Testing Gap**: Find an untested feature or edge case
2. **Determine Test Level**: Choose appropriate test level (unit, integration, etc.)
3. **Write Test**: Follow test pattern for that level
4. **Verify Coverage**: Ensure test increases coverage metrics
5. **Submit PR**: Include tests with implementation changes

See our [contribution guidelines](../CONTRIBUTING.md) for more details.

## Testing Best Practices

Follow these best practices when writing tests for Rummage:

1. **Test Focused Behavior**: Each test should verify one specific behavior
2. **Use Clear Assertions**: Make assertion messages descriptive 
3. **Create Minimal Setup**: Use only what's necessary for the test
4. **Use Test Abstractions**: Share setup code between similar tests
5. **Test Edge Cases**: Include boundary conditions and error scenarios

## Testing Metrics and Goals

We track these key metrics for our test suite:

- **Code Coverage**: Maintain >90% coverage for core game logic
- **Rules Coverage**: Document percentage of MTG rules with dedicated tests
- **Test Performance**: Keep test suite execution time under 5 minutes
- **Failure Rate**: Maintain <1% flaky test ratio

## Next Steps

To dive deeper into our testing approach:

- [Unit Testing](unit_testing.md): Component-level testing guides
- [Integration Testing](integration_testing.md): System interaction testing
- [End-to-End Testing](end_to_end_testing.md): Complete gameplay testing
- [Visual Testing](visual_testing.md): UI consistency testing
- [Performance Testing](performance_testing.md): System performance validation
- [CI/CD Pipeline](ci_cd_pipeline.md): Automated testing infrastructure 