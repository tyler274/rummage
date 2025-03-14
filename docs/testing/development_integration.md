# Development Integration

This document describes how testing is integrated with the development workflow in the Rummage project.

## Overview

Testing in Rummage is not a separate phase but an integral part of the development process. This integration ensures that code quality is maintained from the earliest stages of development, reducing bugs, preventing regressions, and improving overall code quality.

## Test-Driven Development

Rummage follows a test-driven development (TDD) approach for implementing MTG rules and game mechanics:

1. **Write Tests First**: Before implementing a feature, write tests that define the expected behavior
2. **Run Tests (Fail)**: Run the tests to confirm they fail as expected
3. **Implement Feature**: Write the minimal code needed to pass the tests
4. **Run Tests (Pass)**: Verify the tests now pass
5. **Refactor**: Clean up the code while maintaining passing tests

### Example TDD Workflow for MTG Rules

When implementing a new MTG rule, such as the "Legend Rule":

```rust
// 1. First, write the test
#[test]
fn test_legend_rule() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(TestingPlugins);
    
    // Create a player
    let player = app.world.spawn(PlayerMarker).id();
    let battlefield = app.world.spawn(BattlefieldMarker).id();
    
    // Create first legendary creature
    let legend1 = app.world.spawn((
        CardMarker,
        Creature { power: 3, toughness: 3 },
        Legendary,
        Name("Tarmogoyf".to_string()),
        InZone { zone: battlefield },
        Controller { player },
    )).id();
    
    // Create second legendary creature with same name
    let legend2 = app.world.spawn((
        CardMarker,
        Creature { power: 3, toughness: 3 },
        Legendary,
        Name("Tarmogoyf".to_string()),
        InZone { zone: battlefield },
        Controller { player },
    )).id();
    
    // Apply state-based actions
    app.world.send_event(CheckStateBasedActions);
    app.update();
    
    // Verify only one legendary creature remains on battlefield
    let legend_count = app.world.query_filtered::<(), (With<Legendary>, With<CardMarker>)>()
                       .iter(&app.world)
                       .count();
    
    assert_eq!(legend_count, 1, "Only one legendary creature should remain after state-based actions");
}

// 2. Implement the rule
fn apply_legend_rule(
    mut commands: Commands,
    players: Query<(Entity, &PlayerZones)>,
    legends: Query<(Entity, &Controller, &Name), (With<Legendary>, With<CardMarker>)>,
) {
    // Group legends by controller and name
    let mut legend_groups = HashMap::new();
    
    for (legend_entity, controller, name) in legends.iter() {
        legend_groups
            .entry((controller.player, name.0.clone()))
            .or_insert_with(Vec::new)
            .push(legend_entity);
    }
    
    // For each group with more than one legend, keep only the newest
    for (_, legends) in legend_groups.iter() {
        if legends.len() <= 1 {
            continue;
        }
        
        // Keep the first one, sacrifice the rest
        for &legend_entity in legends.iter().skip(1) {
            // Move to graveyard
            // ... implementation details ...
        }
    }
}
```

## Continuous Integration Hooks

Development is integrated with the CI/CD pipeline through:

1. **Pre-commit Hooks**: Automatically run tests before allowing commits
2. **PR Validation**: Enforce passing tests and code standards on PR submission
3. **Integration Gates**: Prevent merges that would break existing functionality

See the [CI/CD Pipeline](ci_cd_pipeline.md) documentation for details on these integration points.

## Development Environments

Test integration in different development contexts:

### Local Development

For local development:

1. **Watch Mode**: Tests run automatically when files change
   ```bash
   cargo watch -x "test --lib"
   ```

2. **Test Filters**: Run specific tests during focused development
   ```bash
   cargo test combat -- --nocapture
   ```

3. **Debug Tests**: Run tests with debugging enabled
   ```bash
   rust-lldb target/debug/deps/rummage-1234abcd
   ```

### IDE Integration

Integration with common development environments:

1. **VS Code**:
   - Run/debug tests from within the editor
   - Visualize test coverage
   - Code lens for test navigation

2. **IntelliJ/CLion**:
   - Run tests from gutter icons
   - Debug test failures
   - View test history

## Test Fixtures and Helpers

To streamline the development process, Rummage provides:

1. **Test Fixtures**: Common test setups for frequently tested scenarios
   ```rust
   // Use a fixture for standard game setup
   let (mut app, player1, player2) = setup_two_player_game();
   ```

2. **Test Utilities**: Helper functions for common test operations
   ```rust
   // Utility to simplify card creation
   let lightning_bolt = create_test_card(&mut app, "Lightning Bolt");
   ```

3. **Mock Systems**: Simplified systems for testing in isolation
   ```rust
   // Replace network systems with mock implementation
   app.add_plugin(MockNetworkPlugin);
   ```

## Development Tools

Tools that integrate testing into development:

1. **Snapshot Review Tool**: Visual interface for reviewing snapshot tests
2. **Coverage Reports**: Interactive coverage visualization during development
3. **Performance Monitors**: Real-time performance metrics during testing

## Best Practices

Guidelines for integrating testing with development:

1. **Write Tests Alongside Code**: Tests should be in the same PR as implementation
2. **Maintain Test Coverage**: Don't let coverage drop as code grows
3. **Test First for Bug Fixes**: Always reproduce bugs with tests before fixing
4. **Run Full Suite Regularly**: Don't rely only on focused tests
5. **Document Test Limitations**: Make clear what aspects aren't covered by tests

## Related Documentation

- [Testing Overview](index.md)
- [Unit Testing](unit_testing.md)
- [Integration Testing](integration_testing.md)
- [End-to-End Testing](end_to_end_testing.md)
- [CI/CD Pipeline](ci_cd_pipeline.md) 