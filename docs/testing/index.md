# Testing Overview

## Introduction to Rummage Testing

The Rummage MTG Commander game engine employs a comprehensive testing strategy to ensure correctness, reliability, and performance. Our testing approach combines traditional software testing methodologies with specialized techniques for game engines, card game rules, and networked multiplayer interactions.

## Testing Philosophy

Our testing philosophy is built on these core principles:

1. **Rules Correctness**: The MTG rule system is complex and precise. Our tests ensure that all rules are implemented correctly according to official MTG Comprehensive Rules.

2. **Determinism**: Game states must evolve deterministically based on inputs, especially critical for networked play.

3. **Cross-Platform Consistency**: Visual and behavioral consistency across different platforms and hardware.

4. **Performance Under Load**: Responsiveness and stability under various game conditions and player counts.

5. **Accessibility**: Testing of accessibility features and compatibility with assistive technologies.

## Testing Layers

Our testing strategy is organized in layers, from isolated component testing to comprehensive end-to-end validation:

### [Unit Testing](unit_testing.md)

Unit tests verify individual game engine components in isolation:

- Rules implementations for specific card mechanics
- Game state transitions and validations
- Core game engine primitives

```rust
#[test]
fn test_mana_cost_parsing() {
    let cost = ManaCost::from_string("{2}{W}{W}");
    assert_eq!(cost.generic, 2);
    assert_eq!(cost.white, 2);
    assert_eq!(cost.blue, 0);
    // ...and so on
}
```

### [Integration Testing](integration_testing.md)

Integration tests verify interactions between multiple game systems:

- Card effects interacting with game state
- Multi-step game processes (casting spells, combat)
- Player action sequences

```rust
#[test]
fn test_creature_cast_and_etb_trigger() {
    // Test casting a creature and its ETB trigger resolving properly
}
```

### [End-to-End Testing](end_to_end_testing.md)

End-to-end tests validate complete game scenarios:

- Full game simulations from start to finish
- Player interaction sequences
- Complex game state evolutions

```rust
#[test]
fn test_four_player_commander_game() {
    // Simulate a complete 4-player game with predefined actions
}
```

### [Visual Differential Testing](visual_differential_testing.md)

Visual differential testing ensures consistent rendering across platforms:

- Card rendering consistency
- UI component appearance
- Animation smoothness and correctness
- Visual feedback for game actions

```rust
#[test]
fn test_card_rendering_across_platforms() {
    // Compare card rendering against reference images
}
```

### [Performance Testing](performance_testing.md)

Performance tests measure and validate system responsiveness:

- Frame rate under varying conditions
- Memory usage during extended play
- Network bandwidth requirements
- Load time benchmarks

```rust
#[test]
fn benchmark_large_board_state_performance() {
    // Measure FPS and memory usage with 100+ permanents
}
```

## Specialized Testing Areas

### Network Testing

Network testing focuses on multiplayer functionality:

- Synchronization of game state
- Latency handling and prediction
- Reconnection and state recovery
- Deterministic RNG across clients

### Rules Compliance Testing

Rules testing verifies correct implementation of MTG rules:

- Comprehensive rules coverage
- Corner cases and rule interactions
- Official rulings validation
- Tournament scenario validation

### Accessibility Testing

Accessibility testing ensures the game is usable by all players:

- Screen reader compatibility
- Keyboard navigation
- Color contrast requirements
- Text scaling support

## Testing Infrastructure

Our testing infrastructure includes:

### [CI/CD Pipeline](ci_cd_pipeline.md)

Continuous integration ensures consistent quality:

- Automated test execution on each commit
- Nightly full test suite runs
- Performance regression detection
- Cross-platform test matrix

### Test Data Management

Structured test data supports comprehensive testing:

- Card database with test metadata
- Predefined game scenarios
- Recorded game sessions
- Performance benchmarking datasets

## Contributing to Testing

We welcome contributions to our testing suite:

1. **Adding Tests**: Particularly for new cards, mechanics, or edge cases
2. **Test Infrastructure**: Improvements to testing tools and frameworks
3. **Bug Reproduction**: Tests that reproduce reported issues
4. **Performance Benchmarks**: New performance test scenarios

See our [contribution guidelines](../CONTRIBUTING.md) for more details on how to contribute.

## Test-Driven Development

Rummage uses test-driven development for new features:

1. Write tests that define expected behavior
2. Implement the feature to pass the tests
3. Refactor while maintaining test coverage
4. Document both implementation and tests

## Testing Metrics

We track the following testing metrics:

- **Code Coverage**: Aiming for >90% code coverage
- **Rules Coverage**: Percentage of MTG rules with dedicated tests
- **Visual Coverage**: Percentage of UI components with visual tests
- **Performance Baselines**: Key performance indicators and thresholds

## Next Steps

To dive deeper into our testing approach, explore these sections:

- [Unit Testing](unit_testing.md) - For component-level testing
- [Integration Testing](integration_testing.md) - For system interaction testing
- [End-to-End Testing](end_to_end_testing.md) - For complete gameplay testing
- [Visual Differential Testing](visual_differential_testing.md) - For rendering consistency
- [Performance Testing](performance_testing.md) - For system performance validation
- [CI/CD Pipeline](ci_cd_pipeline.md) - For automated testing infrastructure 