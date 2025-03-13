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

# Networking Testing Documentation

This section provides comprehensive documentation on testing methodologies for our networked MTG Commander game engine.

## Testing Overview

Testing networked applications presents unique challenges due to:

1. **Variable Network Conditions**: Latency, packet loss, and disconnections
2. **State Synchronization**: Ensuring all clients see the same game state
3. **Randomization Consistency**: Maintaining deterministic behavior across network boundaries
4. **Security Concerns**: Preventing cheating and unauthorized access

Our testing approach addresses these challenges through a multi-layered strategy, combining unit tests, integration tests, and end-to-end tests with specialized tools for network simulation.

## Testing Categories

### Unit Tests

Unit tests verify individual components and systems in isolation:

- [Basic Network Component Tests](overview.md#unit-tests)
- [RNG Synchronization Unit Tests](rng_synchronization_tests.md#unit-tests)
- [Replicon RNG Integration Tests](replicon_rng_tests.md#unit-tests)

### Integration Tests

Integration tests verify that multiple components work together correctly:

- [Client-Server Integration](integration/strategy.md#client-server-integration)
- [Game State Synchronization](overview.md#game-state-synchronization-tests)
- [RNG Integration Tests](rng_synchronization_tests.md#integration-tests)
- [Replicon RNG Integration Tests](replicon_rng_tests.md#integration-tests)

### End-to-End Tests

End-to-end tests verify complete game scenarios from start to finish:

- [Full Game Scenarios](overview.md#full-game-scenarios)
- [Network Disruption Tests](advanced_techniques.md#network-disruption-testing)
- [Card Interactions Over Network](overview.md#card-interaction-tests)
- [Replicon Rollback Tests](replicon_rng_tests.md#end-to-end-tests)

### Performance Tests

Performance tests measure the efficiency and scalability of our networking code:

- [Bandwidth Utilization](advanced_techniques.md#bandwidth-testing)
- [Latency Impact](advanced_techniques.md#latency-testing)
- [Scaling Tests](advanced_techniques.md#scaling-tests)
- [RNG State Replication Performance](replicon_rng_tests.md#performance-tests)

### Security Tests

Security tests verify that our game is resistant to cheating and unauthorized access:

- [Authentication Testing](security/strategy.md#authentication-tests)
- [Authorization Tests](security/strategy.md#authorization-tests)
- [Anti-Cheat Verification](security/strategy.md#anti-cheat-tests)
- [Hidden Information Protection](security/strategy.md#hidden-information-tests)

## Test Implementation Guide

When implementing tests for our networked MTG Commander game, follow these guidelines:

1. **Test Each Layer**: Test network communication, state synchronization, and game logic separately
2. **Simulate Real Conditions**: Use network simulators to test under realistic conditions
3. **Automation**: Automate as many tests as possible for continuous integration
4. **Determinism**: Ensure tests are deterministic and repeatable
5. **RNG Testing**: Pay special attention to randomized game actions

## Testing Tools

Our testing infrastructure includes these specialized tools:

1. **Network Simulators**: Tools to simulate various network conditions
2. **Test Harnesses**: Specialized test environments for network testing
3. **RNG Test Utilities**: Tools for verifying random number determinism
4. **Benchmarking Tools**: Performance measurement utilities

## Key Test Scenarios

Ensure these critical scenarios are thoroughly tested:

1. **Client Connection/Disconnection**: Test proper handling of clients joining and leaving
2. **State Synchronization**: Verify all clients see the same game state
3. **Randomized Actions**: Test that shuffling, coin flips, etc. are deterministic
4. **Network Disruption**: Test recovery after connection issues
5. **Latency Compensation**: Test playability under various latency conditions

## Testing RNG with Replicon

Our new approach using bevy_replicon for RNG state management requires specialized testing:

- [Replicon RNG Testing Overview](replicon_rng_tests.md)
- [RNG State Serialization Tests](replicon_rng_tests.md#testing-rng-state-serialization-and-deserialization)
- [Checkpoint Testing](replicon_rng_tests.md#testing-checkpoint-creation-and-restoration)
- [Network Disruption Recovery](replicon_rng_tests.md#testing-rollback-due-to-network-disruption)
- [Card Shuffling Tests](replicon_rng_tests.md#testing-card-shuffling-during-network-disruption)

## Test Fixtures and Harnesses

We provide several test fixtures to simplify test implementation:

- [Basic Network Test Fixture](overview.md#test-fixtures)
- [RNG Test Harness](rng_synchronization_tests.md#test-fixtures)
- [Replicon RNG Test Harness](replicon_rng_tests.md#test-environment-setup)

---

For more detailed information on specific testing areas, refer to the corresponding documentation links above. 