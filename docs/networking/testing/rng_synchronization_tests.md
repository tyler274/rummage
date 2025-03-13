# Testing RNG Synchronization in Networked Gameplay

This document outlines testing strategies for ensuring deterministic and synchronized random number generation across network boundaries in our MTG Commander game engine.

## Table of Contents

1. [Testing Goals](#testing-goals)
2. [Test Categories](#test-categories)
3. [Test Fixtures](#test-fixtures)
4. [Automated Tests](#automated-tests)
5. [Manual Testing](#manual-testing)
6. [Performance Considerations](#performance-considerations)

## Testing Goals

Testing RNG synchronization focuses on these key goals:

1. **Determinism**: Verify that identical RNG seeds produce identical random sequences on all clients
2. **State Preservation**: Ensure RNG state is properly serialized, transmitted, and restored
3. **Resilience**: Test recovery from network disruptions or client reconnections
4. **Sequence Integrity**: Confirm that game actions using randomness always produce the same results

## Test Categories

### Basic Determinism Tests

These tests verify that the underlying RNG components work deterministically:

```rust
#[test]
fn test_rng_determinism() {
    // Create two RNGs with the same seed
    let seed = 12345u64;
    let mut rng1 = WyRand::seed_from_u64(seed);
    let mut rng2 = WyRand::seed_from_u64(seed);
    
    // Generate sequences from both RNGs
    let sequence1: Vec<u32> = (0..100).map(|_| rng1.next_u32()).collect();
    let sequence2: Vec<u32> = (0..100).map(|_| rng2.next_u32()).collect();
    
    // Verify sequences are identical
    assert_eq!(sequence1, sequence2);
}
```

### Serialization Tests

These tests verify that RNG state can be properly serialized and deserialized:

```rust
#[test]
fn test_rng_serialization() {
    // Create an RNG and use it to generate some values
    let mut original_rng = GlobalEntropy::<WyRand>::from_entropy();
    let original_values: Vec<u32> = (0..10).map(|_| original_rng.next_u32()).collect();
    
    // Serialize the RNG state
    let serialized_state = original_rng.try_serialize_state().expect("Failed to serialize RNG state");
    
    // Create a new RNG and deserialize the state into it
    let mut new_rng = GlobalEntropy::<WyRand>::from_entropy();
    new_rng.deserialize_state(&serialized_state).expect("Failed to deserialize RNG state");
    
    // Generate the same number of values from the new RNG
    let new_values: Vec<u32> = (0..10).map(|_| new_rng.next_u32()).collect();
    
    // The values should be the same, since the states were synchronized
    assert_eq!(original_values, new_values);
}
```

### Network Transmission Tests

These tests verify that RNG state can be properly transmitted across the network:

```rust
#[test]
fn test_rng_network_transmission() {
    // Setup server and client apps
    let mut server_app = App::new();
    let mut client_app = App::new();
    
    // Configure apps for network testing
    setup_network_test(&mut server_app, true, false);
    setup_network_test(&mut client_app, false, true);
    
    // Run updates to establish connection
    for _ in 0..5 {
        server_app.update();
        client_app.update();
    }
    
    // Generate some random values on the server
    let server_values = {
        let mut rng = server_app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.next_u32()).collect::<Vec<_>>()
    };
    
    // Serialize and send RNG state from server to client
    let serialized_state = {
        let rng = server_app.world.resource::<GlobalEntropy<WyRand>>();
        rng.try_serialize_state().expect("Failed to serialize RNG state")
    };
    
    // Simulate network transmission
    let rng_message = RngStateMessage {
        state: serialized_state,
        timestamp: server_app.world.resource::<Time>().elapsed_seconds(),
    };
    
    // Apply the RNG state to the client
    {
        let mut client_rng = client_app.world.resource_mut::<GlobalEntropy<WyRand>>();
        client_rng.deserialize_state(&rng_message.state).expect("Failed to deserialize RNG state");
    }
    
    // Generate the same number of values on the client
    let client_values = {
        let mut rng = client_app.world.resource_mut::<GlobalEntropy<WyRand>>();
        (0..10).map(|_| rng.next_u32()).collect::<Vec<_>>()
    };
    
    // Verify the values match
    assert_eq!(server_values, client_values);
}
```

### Player-Specific RNG Tests

These tests verify that player-specific RNGs maintain determinism:

```rust
#[test]
fn test_player_rng_forking() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .init_resource::<PlayerRegistry>();
    
    // Add test systems
    app.add_systems(Startup, setup_test_players)
        .add_systems(Update, fork_player_rngs);
    
    // Run the app to set up players and fork RNGs
    app.update();
    
    // Get player entities
    let player_registry = app.world.resource::<PlayerRegistry>();
    let player1 = player_registry.get_player(1);
    let player2 = player_registry.get_player(2);
    
    // Generate random sequences for each player
    let player1_values = generate_random_sequence(&mut app.world, player1);
    let player2_values = generate_random_sequence(&mut app.world, player2);
    
    // Values should be different because they were forked from the global RNG
    assert_ne!(player1_values, player2_values);
    
    // Save player RNG states
    save_player_rng_states(&mut app.world);
    
    // Create a new app and restore the states
    let mut new_app = App::new();
    // Configure new app
    // ...
    
    // Restore the player RNG states
    restore_player_rng_states(&mut new_app.world);
    
    // Generate new sequences
    let new_player1_values = generate_random_sequence(&mut new_app.world, player1);
    let new_player2_values = generate_random_sequence(&mut new_app.world, player2);
    
    // New sequences should match the original ones
    assert_eq!(player1_values, new_player1_values);
    assert_eq!(player2_values, new_player2_values);
}
```

### Game Action Tests

These tests verify that game actions involving randomness produce consistent results:

```rust
#[test]
fn test_shuffle_library_determinism() {
    // Setup test environment
    let mut app = setup_multiplayer_test_app();
    
    // Create a deck with known order
    let cards = (1..53).collect::<Vec<_>>();
    
    // Setup player and library
    let player_entity = spawn_test_player(&mut app.world);
    let library_entity = spawn_test_library(&mut app.world, player_entity, cards.clone());
    
    // Seed the player's RNG
    seed_player_rng(&mut app.world, player_entity, 12345u64);
    
    // First shuffle
    app.world.send_event(ShuffleLibraryEvent { library_entity });
    app.update();
    
    // Get shuffled order
    let first_shuffle = get_library_order(&app.world, library_entity);
    
    // Reset library and RNG to original state
    reset_library(&mut app.world, library_entity, cards.clone());
    seed_player_rng(&mut app.world, player_entity, 12345u64);
    
    // Second shuffle
    app.world.send_event(ShuffleLibraryEvent { library_entity });
    app.update();
    
    // Get shuffled order
    let second_shuffle = get_library_order(&app.world, library_entity);
    
    // Both shuffles should result in the same order
    assert_eq!(first_shuffle, second_shuffle);
}
```

## Test Fixtures

Common test fixtures for RNG testing:

```rust
/// Sets up a test environment with multiple clients
pub fn setup_multiplayer_rng_test() -> TestHarness {
    let mut harness = TestHarness::new();
    
    // Setup server
    harness.create_server_app();
    
    // Setup multiple clients
    for i in 0..4 {
        harness.create_client_app(i);
    }
    
    // Initialize RNG with a fixed seed
    harness.seed_global_rng(12345u64);
    
    // Connect clients to server
    harness.connect_all_clients();
    
    harness
}

/// Executes a randomized game action on all clients and verifies consistency
pub fn verify_random_action_consistency(harness: &mut TestHarness, action: RandomizedAction) {
    // Execute action on server
    harness.execute_on_server(action.clone());
    
    // Synchronize RNG state to clients
    harness.sync_rng_state();
    
    // Execute same action on all clients
    let results = harness.execute_on_all_clients(action);
    
    // All results should be identical
    let first_result = &results[0];
    for result in &results[1..] {
        assert_eq!(first_result, result);
    }
}
```

## Automated Tests

### Integration with CI Pipeline

Include these RNG synchronization tests in the CI pipeline:

```yaml
# .github/workflows/rng-tests.yml
name: RNG Synchronization Tests

on:
  push:
    branches: [ main ]
    paths:
      - 'src/networking/rng/**'
      - 'src/game_engine/actions/**'
  pull_request:
    branches: [ main ]
    paths:
      - 'src/networking/rng/**'
      - 'src/game_engine/actions/**'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      - name: Run RNG tests
        run: cargo test --package rummage --lib networking::rng
```

### Test Coverage Requirements

Aim for these coverage targets for RNG synchronization code:

1. **Line Coverage**: At least 90%
2. **Branch Coverage**: At least 85%
3. **Function Coverage**: 100% 

## Manual Testing

Some aspects of RNG synchronization should be manually tested:

1. **Disconnection Recovery**: Test that clients reconnecting receive correct RNG state
2. **High Latency Scenarios**: Test with artificially high network latency
3. **Packet Loss**: Test with simulated packet loss to verify recovery
4. **Cross-Platform Consistency**: Verify RNG consistency between different operating systems

## Performance Considerations

When testing RNG synchronization, monitor these performance metrics:

1. **Serialization Size**: RNG state should be compact
2. **Synchronization Frequency**: Balance consistency vs. network overhead
3. **CPU Overhead**: Monitor CPU usage during RNG-heavy operations
4. **Memory Usage**: Track memory usage when many player-specific RNGs are active

## Documentation and Logging

Implement thorough logging for RNG synchronization to aid in debugging:

```rust
pub fn log_rng_sync(
    rng_state: Res<RngStateTracker>,
    client_id: Option<Res<ClientId>>,
) {
    if let Some(client_id) = client_id {
        info!(
            "RNG sync: Client {} received state of size {} bytes (timestamp: {})",
            client_id.0,
            rng_state.global_state.len(),
            rng_state.last_sync
        );
    } else {
        info!(
            "RNG sync: Server updated state of size {} bytes (timestamp: {})",
            rng_state.global_state.len(),
            rng_state.last_sync
        );
    }
}
``` 