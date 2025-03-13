# Networking Testing Strategy

This document outlines the testing strategy for the multiplayer networking implementation in our MTG Commander game engine.

## Table of Contents

1. [Testing Goals](#testing-goals)
2. [Testing Environments](#testing-environments)
3. [Test Types](#test-types)
4. [Game-Specific Tests](#game-specific-tests)
5. [Network Condition Simulation](#network-condition-simulation)
6. [Performance Testing](#performance-testing)
7. [Security Testing](#security-testing)
8. [Continuous Integration](#continuous-integration)

## Testing Goals

Our networking testing aims to ensure:

1. **Correctness**: Game state is properly synchronized between server and clients
2. **Consistency**: All clients see the same game state with appropriate visibility rules
3. **Performance**: Networking adds minimal overhead to gameplay
4. **Robustness**: System handles network disruptions, edge cases, and high loads
5. **Security**: Hidden information stays hidden and cheating is prevented

## Testing Environments

We'll use several testing environments:

### Local Development Environment

- Single process running both server and client
- Fast development cycle
- No real network latency or issues

```rust
// Example setup for local development testing
pub fn setup_local_test_environment(app: &mut App) {
    app.insert_resource(NetworkingConfig {
        is_server: true,
        is_client: true,
        server_address: Some("127.0.0.1".to_string()),
        port: 5000,
        max_clients: 4,
    });

    app.add_plugins(NetworkingPlugin);
}
```

### Simulated Network Environment

- Multiple processes communicating over loopback interface
- Artificial latency, packet loss, and jitter
- Closer to real-world conditions while remaining deterministic

```rust
pub fn setup_simulated_network_test() {
    // Start server process
    let server_process = std::process::Command::new("cargo")
        .args(&["run", "--", "--server"])
        .spawn()
        .expect("Failed to start server");

    // Give server time to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Start client processes
    let mut client_processes = Vec::new();
    for i in 0..4 {
        let client = std::process::Command::new("cargo")
            .args(&["run", "--", "--client", "--id", &i.to_string()])
            .spawn()
            .expect("Failed to start client");
        client_processes.push(client);
    }

    // Run test scenario
    // ...

    // Cleanup processes
    server_process.kill().expect("Failed to kill server");
    for mut client in client_processes {
        client.kill().expect("Failed to kill client");
    }
}
```

### Distributed Test Environment

- Multiple machines or containers on a real network
- Real-world network conditions
- Final validation of the networking implementation

## Test Types

### Unit Tests

Focus on testing individual networking components in isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_message_serialization() {
        let message = NetworkMessage {
            message_type: MessageType::Action,
            payload: "Test payload".to_string(),
            sequence: 1,
            timestamp: 1234.5678,
        };

        let serialized = bincode::serialize(&message).unwrap();
        let deserialized: NetworkMessage<String> = bincode::deserialize(&serialized).unwrap();

        assert_eq!(message.message_type, deserialized.message_type);
        assert_eq!(message.payload, deserialized.payload);
        assert_eq!(message.sequence, deserialized.sequence);
        assert_eq!(message.timestamp, deserialized.timestamp);
    }

    #[test]
    fn test_client_connection() {
        // Setup minimal test app
        let mut app = App::new();
        
        // Add required plugins and systems
        app.add_plugins(MinimalPlugins)
            .add_plugins(bevy_replicon::prelude::ClientPlugin::default())
            .add_systems(Update, handle_connection_status);
        
        // Initialize resources
        app.insert_resource(GameClient::default());
        
        // Update client status and run systems
        let mut client_status = RepliconClientStatus::Connecting;
        app.insert_resource(client_status);
        app.update();
        
        // Check the result
        let client = app.world.resource::<GameClient>();
        assert_eq!(client.connection_status, ConnectionStatus::Connecting);
        
        // Test connection established
        client_status = RepliconClientStatus::Connected;
        app.insert_resource(client_status);
        app.update();
        
        let client = app.world.resource::<GameClient>();
        assert_eq!(client.connection_status, ConnectionStatus::Connected);
    }
}
```

### Integration Tests

Test how networking components work together:

```rust
#[test]
fn test_client_server_communication() {
    // Create test app with both server and client
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy_replicon::prelude::ServerPlugin::default())
        .add_plugins(bevy_replicon::prelude::ClientPlugin::default())
        .add_systems(Startup, setup_test)
        .add_systems(Update, (
            send_test_message,
            receive_test_message,
            verify_communication,
        ));
    
    // Run enough updates to allow for communication
    for _ in 0..10 {
        app.update();
    }
    
    // Verify communication occurred
    let test_results = app.world.resource::<TestResults>();
    assert!(test_results.message_received);
}
```

### End-to-End Tests

Test the entire networking stack in a game scenario:

```rust
#[test]
fn test_multiplayer_game_flow() {
    // Setup complete game environment
    let mut app = setup_multiplayer_test_app();
    
    // Simulate game actions
    app.world.resource_mut::<TestController>().queue_action(
        TestAction::ConnectPlayers(4)
    );
    
    // Run updates to process actions
    app.update();
    
    // Verify all players connected
    let connected_clients = app.world.resource::<ConnectedClients>();
    assert_eq!(connected_clients.len(), 4);
    
    // Simulate a full game turn
    app.world.resource_mut::<TestController>().queue_action(
        TestAction::ExecuteGameTurn
    );
    
    // Run enough updates for a full turn
    for _ in 0..100 {
        app.update();
    }
    
    // Verify game state consistency across clients
    verify_client_consistency(&app);
}
```

## Game-Specific Tests

### Hidden Information Tests

Verify that cards in hidden zones are only visible to appropriate players:

```rust
#[test]
fn test_hidden_information() {
    let mut app = setup_multiplayer_test_app();
    
    // Setup test scenario with 4 players
    // ...
    
    // Player 1 draws a card
    app.world.resource_mut::<TestController>()
        .execute_action_for_player(1, TestAction::DrawCard);
    app.update();
    
    // Verify only player 1 can see the card
    let player1_client = app.world.resource::<ClientRegistry>().get_client(1);
    let other_clients = app.world.resource::<ClientRegistry>()
        .get_all_except(1);
    
    assert!(player1_client.can_see_card(card_id));
    
    for client in other_clients {
        assert!(!client.can_see_card(card_id));
    }
}
```

### Game Rules Tests

Verify that networked game rules are enforced correctly:

```rust
#[test]
fn test_priority_passing() {
    let mut app = setup_multiplayer_test_app();
    
    // Setup test with 4 players
    // ...
    
    // Active player attempts to pass priority
    app.world.resource_mut::<TestController>()
        .execute_action_for_player(
            1, 
            TestAction::PlayerAction(NetworkedActionType::PassPriority)
        );
    app.update();
    
    // Verify priority passed to the next player
    let priority_system = app.world.resource::<PrioritySystem>();
    assert_eq!(priority_system.current_player, 2);
    
    // Non-priority player attempts to cast spell (should fail)
    let result = app.world.resource_mut::<TestController>()
        .execute_action_for_player(
            3, 
            TestAction::PlayerAction(NetworkedActionType::CastSpell)
        );
    app.update();
    
    assert_eq!(result, ActionResult::Rejected(ErrorCode::NotYourPriority));
}
```

### State Synchronization Tests

Verify that all clients maintain the same game state:

```rust
#[test]
fn test_game_state_synchronization() {
    let mut app = setup_multiplayer_test_app();
    
    // Setup test scenario
    // ...
    
    // Execute complex game actions
    let actions = [
        TestAction::PlayLand(player_id: 1, card_id: 101),
        TestAction::CastSpell(player_id: 1, card_id: 102, targets: vec![201]),
        TestAction::PassPriority(player_id: 1),
        TestAction::PassPriority(player_id: 2),
        TestAction::CastSpell(player_id: 3, card_id: 103, targets: vec![102]),
        // ...
    ];
    
    for action in actions {
        app.world.resource_mut::<TestController>().execute_action(action);
        app.update_n_times(5); // Allow time for replication
    }
    
    // Verify all client states match the server state for public information
    let server_state = extract_server_game_state(&app);
    let client_states = extract_all_client_game_states(&app);
    
    for (client_id, client_state) in client_states {
        assert_eq!(
            server_state.public_state, 
            client_state.public_state,
            "Client {} state does not match server state", 
            client_id
        );
    }
}
```

## Network Condition Simulation

Test how the game behaves under various network conditions:

```rust
pub fn test_under_network_conditions(
    latency_ms: u64,
    packet_loss_percent: f32,
    jitter_ms: u64,
) {
    // Setup test app with network condition simulation
    let mut app = setup_multiplayer_test_app();
    
    // Configure network simulation
    app.insert_resource(NetworkSimulation {
        latency: Some(latency_ms),
        packet_loss: Some(packet_loss_percent / 100.0),
        jitter: Some(jitter_ms),
    });
    
    // Run standard test scenario
    // ...
    
    // Verify game functions correctly despite network conditions
    // ...
}

#[test]
fn test_high_latency() {
    test_under_network_conditions(200, 0.0, 0);
}

#[test]
fn test_packet_loss() {
    test_under_network_conditions(50, 5.0, 0);
}

#[test]
fn test_jitter() {
    test_under_network_conditions(50, 0.0, 50);
}

#[test]
fn test_terrible_connection() {
    test_under_network_conditions(300, 10.0, 100);
}
```

## Performance Testing

Measure networking performance:

```rust
#[test]
fn test_networking_performance() {
    let mut app = setup_multiplayer_test_app();
    
    // Add performance metrics collection
    app.add_systems(Update, collect_network_metrics);
    
    // Run a standard game scenario
    // ...
    
    // Collect metrics
    let metrics = app.world.resource::<NetworkMetrics>();
    
    // Assert performance meets requirements
    assert!(metrics.average_message_size < 1024); // Average message under 1KB
    assert!(metrics.messages_per_second < 100);  // Under 100 messages per second
    assert!(metrics.average_processing_time < 5.0); // Under 5ms processing time
}
```

## Security Testing

Test for security vulnerabilities:

```rust
#[test]
fn test_hidden_information_leak() {
    let mut app = setup_multiplayer_test_app();
    
    // Setup game with known cards in player's hand
    // ...
    
    // Try to exploit by sending specially crafted messages
    send_malicious_request(&mut app, MaliciousRequest::RevealHand(other_player_id));
    
    // Verify other player's hand remains hidden
    let client = app.world.resource::<ClientRegistry>().get_client(1);
    let other_player_hand = app.world.resource::<GameState>().get_player_hand(2);
    
    for card in other_player_hand {
        assert!(!client.can_see_card(card));
    }
}

#[test]
fn test_action_validation() {
    let mut app = setup_multiplayer_test_app();
    
    // Setup game state
    // ...
    
    // Try to perform illegal actions
    let illegal_actions = [
        TestAction::PlayExtraLand,
        TestAction::CastWithoutMana,
        TestAction::TargetInvalid,
        // ...
    ];
    
    for action in illegal_actions {
        let result = app.world.resource_mut::<TestController>()
            .execute_action(action);
        app.update();
        
        // Verify action was rejected
        assert!(matches!(result, ActionResult::Rejected(_)));
    }
}
```

## Continuous Integration

Automated tests in CI pipeline:

```yaml
# .github/workflows/networking-tests.yml
name: Networking Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Build
      run: cargo build --verbose
    - name: Run networking unit tests
      run: cargo test --verbose networking::unit_tests
    - name: Run networking integration tests
      run: cargo test --verbose networking::integration_tests
    - name: Run simulated network tests
      run: cargo test --verbose networking::simulation
```

---

This testing strategy provides a comprehensive approach to verifying the correctness, performance, and security of our multiplayer networking implementation. By covering different types of tests across various environments, we can ensure a robust and enjoyable multiplayer experience for our MTG Commander game engine. 