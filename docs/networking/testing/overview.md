# Networking Testing Overview

This document provides a comprehensive overview of the testing approach for the Rummage MTG Commander game engine's networking functionality.

## Table of Contents

1. [Introduction](#introduction)
2. [Testing Principles](#testing-principles)
3. [Testing Levels](#testing-levels)
4. [Test Fixtures](#test-fixtures)
5. [Network Simulation](#network-simulation)
6. [Automation Approach](#automation-approach)

## Introduction

Networking code is inherently complex due to its asynchronous nature, potential for race conditions, and sensitivity to network conditions. Our testing approach is designed to address these challenges by employing a combination of unit tests, integration tests, and end-to-end tests, with a focus on simulating real-world network conditions.

## Testing Principles

Our networking testing follows these core principles:

1. **Deterministic Tests**: Tests should be repeatable and produce the same results given the same inputs
2. **Isolation**: Individual tests should run independently without relying on state from other tests
3. **Real-World Conditions**: Tests should simulate various network conditions including latency, packet loss, and disconnections
4. **Comprehensive Coverage**: Tests should cover all networking components and their interactions
5. **Performance Validation**: Tests should validate that networking performs adequately under expected loads

## Testing Levels

### Unit Tests

Unit tests focus on individual networking components in isolation:

```rust
#[test]
fn test_message_serialization() {
    // Create a test message
    let message = NetworkMessage::GameAction {
        sequence_id: 123,
        player_id: 456,
        action: GameAction::DrawCard { player_id: 456, count: 1 },
    };
    
    // Serialize the message
    let serialized = bincode::serialize(&message).expect("Serialization failed");
    
    // Deserialize the message
    let deserialized: NetworkMessage = bincode::deserialize(&serialized).expect("Deserialization failed");
    
    // Verify the deserialized message matches the original
    assert_eq!(message, deserialized);
}
```

### Integration Tests

Integration tests verify that multiple components work together correctly:

```rust
#[test]
fn test_client_server_connection() {
    // Set up a server
    let mut server_app = App::new();
    server_app.add_plugins(MinimalPlugins)
        .add_plugins(RepliconServerPlugin::default());
    
    // Set up a client
    let mut client_app = App::new();
    client_app.add_plugins(MinimalPlugins)
        .add_plugins(RepliconClientPlugin::default());
    
    // Start the server
    server_app.world.resource_mut::<RepliconServer>()
        .start_endpoint(ServerEndpoint::new(8080));
    
    // Connect the client
    client_app.world.resource_mut::<RepliconClient>()
        .connect_endpoint(ClientEndpoint::new("127.0.0.1", 8080));
    
    // Run updates to establish connection
    for _ in 0..10 {
        server_app.update();
        client_app.update();
    }
    
    // Verify the client is connected
    let client = client_app.world.resource::<RepliconClient>();
    assert!(client.is_connected());
}
```

### End-to-End Tests

End-to-end tests verify complete game scenarios:

```rust
#[test]
fn test_multiplayer_game_flow() {
    // Set up a server with a game
    let mut server_app = setup_server_with_game();
    
    // Set up clients for multiple players
    let mut client_apps = vec![
        setup_client_app(0),
        setup_client_app(1),
        setup_client_app(2),
        setup_client_app(3),
    ];
    
    // Connect all clients
    connect_all_clients(&mut server_app, &mut client_apps);
    
    // Run a full game turn cycle
    run_game_turn_cycle(&mut server_app, &mut client_apps);
    
    // Verify game state is consistent across all clients
    verify_consistent_game_state(&server_app, &client_apps);
}
```

## Test Fixtures

### Basic Network Test Fixture

```rust
/// Sets up a standard network test environment with server and clients
pub fn setup_network_test(app: &mut App, is_server: bool, is_client: bool) {
    // Add required plugins
    app.add_plugins(MinimalPlugins);
    
    // Add either server or client plugins
    if is_server {
        app.add_plugins(RepliconServerPlugin::default());
    }
    
    if is_client {
        app.add_plugins(RepliconClientPlugin::default());
    }
    
    // Add networking resources
    app.init_resource::<NetworkConfig>()
        .init_resource::<ConnectionStatus>();
    
    // Add core networking systems
    app.add_systems(Update, network_connection_status_update);
}
```

### Game State Test Fixture

```rust
/// Sets up a test environment with a standard game state
pub fn setup_test_game_state(app: &mut App) {
    // Add game state
    app.init_resource::<GameState>();
    
    // Set up players
    let player_entities = spawn_test_players(app);
    
    // Set up initial game board
    setup_test_board_state(app, &player_entities);
    
    // Initialize game systems
    app.add_systems(Update, (
        update_game_state,
        process_game_actions,
        sync_game_state,
    ));
}
```

## Network Simulation

To test under various network conditions, we use a network condition simulator:

```rust
/// Simulates network conditions for testing
pub struct NetworkConditionSimulator {
    /// Simulated latency in milliseconds
    pub latency: u32,
    /// Packet loss percentage (0-100)
    pub packet_loss: u8,
    /// Jitter in milliseconds
    pub jitter: u32,
    /// Bandwidth cap in KB/s
    pub bandwidth: u32,
}

impl NetworkConditionSimulator {
    /// Applies network conditions to a packet
    pub fn process_packet(&self, packet: &mut Packet) {
        // Apply packet loss
        if rand::random::<u8>() < self.packet_loss {
            packet.dropped = true;
            return;
        }
        
        // Apply latency with jitter
        let jitter_amount = if self.jitter > 0 {
            rand::thread_rng().gen_range(0..self.jitter)
        } else {
            0
        };
        
        packet.delay = Duration::from_millis((self.latency + jitter_amount) as u64);
        
        // Apply bandwidth limitation
        if self.bandwidth > 0 {
            packet.throttled = packet.size > self.bandwidth;
        }
    }
}
```

## Automation Approach

Our testing automation strategy focuses on:

1. **Continuous Integration**: All networking tests run on every PR and merge to main
2. **Matrix Testing**: Tests run against multiple configurations (OS, Bevy version, etc.)
3. **Performance Benchmarks**: Regular testing of networking performance metrics
4. **Stress Testing**: Load tests to verify behavior under heavy usage
5. **Long-running Tests**: Tests that run for extended periods to catch time-dependent issues

## Key Test Scenarios

The following critical scenarios must pass for all networking changes:

1. **Connection Handling**: Establishing connections, handling disconnections, and reconnections
2. **State Synchronization**: Ensuring all clients see the same game state
3. **Latency Compensation**: Verifying the game remains playable under various latency conditions
4. **Error Recovery**: Testing recovery from network errors and disruptions
5. **Security**: Validating that security measures work as expected

---

For more detailed testing information, see the [RNG Synchronization Tests](rng_synchronization_tests.md) and [Replicon RNG Tests](replicon_rng_tests.md) documents. 