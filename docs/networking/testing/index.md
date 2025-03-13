# Comprehensive Testing Guide for MTG Commander Online

This document serves as the central index for all testing documentation related to our networked MTG Commander implementation. It provides an overview of our testing strategy and links to detailed documentation for specific testing areas.

## Core Principles

Our testing strategy for the online MTG Commander implementation is built on the following core principles:

1. **Comprehensive Coverage**: Testing all aspects of the system, from individual components to full end-to-end gameplay
2. **Realism**: Simulating real-world conditions, including varied network environments and player behaviors
3. **Automation**: Maximizing the use of automated testing to enable frequent regression testing
4. **Game Rule Compliance**: Ensuring the implementation adheres to all Commander format rules
5. **Security**: Verifying that hidden information remains appropriately hidden
6. **Performance**: Validating that the system functions well under various loads and conditions

## Testing Documentation Structure

| Document | Description |
|----------|-------------|
| [Core Testing Strategy](testing.md) | Outlines the fundamental approach to testing the networking implementation |
| [Advanced Testing Strategies](testing_advanced.md) | Covers specialized testing approaches for Commander-specific needs |
| [Integration Testing](integration_testing.md) | Details testing at the boundary between networking and game engine |
| [Security Testing](security_testing.md) | Approaches for testing information hiding and anti-cheat mechanisms |

## Testing Types

### Unit Testing

Unit tests focus on individual components in isolation:

- Networking protocol components
- State synchronization mechanisms
- Game rule implementation
- Command processing

### Integration Testing

Integration tests verify components work together correctly:

- Networking and game state management
- Client/server communication
- Action validation and execution
- Priority and turn handling

### System Testing

System tests examine the complete system's functionality:

- Full game scenarios
- Multiple players
- Complete turn cycles
- Commander-specific rules

### Network Simulation Testing

Tests under various network conditions:

- High latency
- Packet loss
- Jitter
- Bandwidth limitations
- Server/client disconnection and reconnection

## Test Implementation Guidance

When implementing tests, follow these guidelines:

1. **Test Isolation**: Each test should run independently without relying on state from other tests
2. **Determinism**: Tests should produce consistent results when run multiple times with the same inputs
3. **Clear Assertions**: Use descriptive assertion messages that explain what is being tested and why it failed
4. **Comprehensive Verification**: Verify all relevant aspects of state after actions, not just one element
5. **Cleanup**: Tests should clean up after themselves to avoid interfering with other tests

## Test Data Management

Standard test fixtures are available for:

- Player configurations
- Deck compositions
- Board states
- Game scenarios

Use the `TestDataRepository` to access these fixtures:

```rust
// Example of using test fixtures
#[test]
fn test_combat_interaction() {
    let mut app = setup_test_app();
    
    // Load a predefined mid-game state with creatures
    let test_state = TestDataRepository::load_fixture("mid_game_combat_state");
    setup_game_state(&mut app, &test_state);
    
    // Execute test
    // ...
}
```

## Continuous Integration

Our CI pipeline automatically runs the following test suites:

1. **Unit Tests**: On every push and pull request
2. **Integration Tests**: On every push and pull request
3. **System Tests**: On every push to main or develop branches
4. **Security Tests**: Nightly on develop branch
5. **Network Simulation Tests**: Nightly on develop branch

Test results are available in the CI dashboard, including:
- Test pass/fail status
- Performance benchmarks
- Coverage reports
- Network simulation metrics

## Local Testing Workflow

To run tests locally:

```bash
# Run unit tests
cargo test networking::unit

# Run integration tests
cargo test networking::integration

# Run system tests
cargo test networking::system

# Run security tests
cargo test networking::security

# Run network simulation tests
cargo test networking::simulation
```

For more detailed output:

```bash
cargo test networking::integration -- --nocapture --test-threads=1
```

## Additional Testing Resources

- [Bevy Testing Guide](https://bevyengine.org/learn/book/getting-started/testing/)
- [Replicon Network Testing](https://github.com/jakobhellermann/bevy_replicon/wiki/testing)
- [Testing Distributed Systems](https://asatarin.github.io/testing-distributed-systems/)
- [Game Networking Resources](https://github.com/MFatihMAR/Game-Networking-Resources)

## Contributing New Tests

When adding new tests:

1. Identify the appropriate category for your test
2. Follow the existing naming conventions
3. Add detailed comments explaining the test purpose and expected behavior
4. Update test documentation if adding new test categories
5. Ensure tests run within a reasonable timeframe

---

By following this comprehensive testing strategy, we can ensure our networked MTG Commander implementation is robust, performant, and faithful to the rules of the game. Our testing suite provides confidence that the game will work correctly across a variety of real-world conditions and player interactions. 