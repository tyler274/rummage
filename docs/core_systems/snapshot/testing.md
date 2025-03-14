# Snapshot System Testing

This document covers testing approaches and strategies for the Rummage snapshot system. For a general overview of testing in Rummage, see the [Testing Overview](../../testing/index.md).

# Testing the Snapshot System

This document outlines approaches and best practices for testing the Snapshot system in Rummage.

## Types of Tests

The snapshot system should be tested at several levels:

1. **Unit Tests**: Isolated tests of individual snapshot components and functions
2. **Integration Tests**: Tests of snapshot system interaction with other game systems
3. **End-to-End Tests**: Tests of complete game scenarios using snapshots
4. **Performance Tests**: Tests of snapshot system performance characteristics
5. **Network Tests**: Tests of snapshot integration with networking

## Unit Testing

Unit tests focus on individual components of the snapshot system:

```rust
#[test]
fn test_snapshot_creation() {
    // Set up a minimal app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(SnapshotPlugin);
    
    // Add some test entities
    let entity1 = app.world.spawn((Snapshotable, TestComponent { value: 42 })).id();
    let entity2 = app.world.spawn((Snapshotable, TestComponent { value: 123 })).id();
    
    // Create a game state
    let game_state = GameState {
        turn: 1,
        phase: Phase::Main1,
        active_player: 0,
    };
    app.insert_resource(game_state);
    
    // Create a snapshot
    app.world.send_event(SnapshotEvent::Take);
    app.update();
    
    // Verify the snapshot was created
    let snapshot_registry = app.world.resource::<SnapshotRegistry>();
    assert_eq!(snapshot_registry.snapshots.len(), 1, "Should create one snapshot");
    
    // Verify the snapshot contents
    let snapshot = snapshot_registry.most_recent().unwrap();
    assert_eq!(snapshot.turn, 1, "Snapshot should have the correct turn");
    assert_eq!(snapshot.phase, Phase::Main1, "Snapshot should have the correct phase");
    assert_eq!(snapshot.active_player, 0, "Snapshot should have the correct active player");
    
    // Verify entities were captured
    assert_eq!(snapshot.game_data.len(), 2, "Snapshot should include 2 entities");
}

#[test]
fn test_snapshot_application() {
    // Set up a minimal app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(SnapshotPlugin);
    
    // Add some test entities
    let entity1 = app.world.spawn((Snapshotable, TestComponent { value: 42 })).id();
    
    // Create a game state
    let game_state = GameState {
        turn: 1,
        phase: Phase::Main1,
        active_player: 0,
    };
    app.insert_resource(game_state);
    
    // Create a snapshot
    app.world.send_event(SnapshotEvent::Take);
    app.update();
    
    // Get the snapshot ID
    let snapshot_id = app.world.resource::<SnapshotRegistry>()
                             .most_recent().unwrap().id;
    
    // Modify the entity
    if let Some(mut test_comp) = app.world.get_mut::<TestComponent>(entity1) {
        test_comp.value = 99;
    }
    
    // Apply the snapshot
    app.world.send_event(SnapshotEvent::Apply(snapshot_id));
    app.update();
    
    // Verify the entity was restored to its original state
    let test_comp = app.world.get::<TestComponent>(entity1).unwrap();
    assert_eq!(test_comp.value, 42, "Component should be restored to original value");
}
```

## Integration Testing

Integration tests verify how snapshots interact with other game systems:

```rust
#[test]
fn test_snapshot_with_turn_system() {
    // Set up a test app with relevant plugins
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        SnapshotPlugin,
        TurnSystemPlugin,
    ));
    
    // Configure auto-snapshots
    let mut config = SnapshotConfig::default();
    config.auto_snapshot_on_turn = true;
    app.insert_resource(config);
    
    // Set up initial game state
    app.insert_resource(GameState {
        turn: 1,
        phase: Phase::Main1,
        active_player: 0,
    });
    
    // Add some test entities
    app.world.spawn((Snapshotable, TestComponent { value: 42 }));
    
    // Advance the turn
    app.world.send_event(AdvanceTurnEvent);
    app.update();
    
    // Verify a snapshot was automatically created
    let snapshot_registry = app.world.resource::<SnapshotRegistry>();
    assert_eq!(snapshot_registry.snapshots.len(), 1, "Should create one snapshot on turn change");
    
    // Verify the snapshot has the correct turn number
    let snapshot = snapshot_registry.most_recent().unwrap();
    assert_eq!(snapshot.turn, 2, "Snapshot should capture the new turn number");
}
```

## End-to-End Testing

End-to-end tests verify complete game scenarios:

```rust
#[test]
fn test_full_game_with_snapshots() {
    // Set up a complete game environment
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        GameEnginePlugin,
        SnapshotPlugin,
    ));
    
    // Set up a test game
    setup_test_game(&mut app);
    
    // Play through multiple turns, taking snapshots
    for turn in 1..5 {
        // Play through a turn
        play_turn(&mut app);
        
        // Take a snapshot
        app.world.send_event(SnapshotEvent::Take);
        app.update();
    }
    
    // Verify we have snapshots for each turn
    let snapshot_registry = app.world.resource::<SnapshotRegistry>();
    assert_eq!(snapshot_registry.snapshots.len(), 4, "Should have 4 snapshots");
    
    // Go back to turn 2
    let turn_2_snapshot = snapshot_registry.snapshots.values()
                                          .find(|s| s.turn == 2)
                                          .unwrap();
    app.world.send_event(SnapshotEvent::Apply(turn_2_snapshot.id));
    app.update();
    
    // Verify game state was restored correctly
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.turn, 2, "Game should be restored to turn 2");
    
    // Continue playing from this restored state
    play_turn(&mut app);
    
    // Verify the game progressed correctly from the restored state
    let game_state = app.world.resource::<GameState>();
    assert_eq!(game_state.turn, 3, "Game should advance to turn 3");
}
```

## Performance Testing

Performance tests measure the impact of snapshots:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn snapshot_creation_benchmark(c: &mut Criterion) {
    c.bench_function("create snapshot with 100 entities", |b| {
        // Set up a test app
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, SnapshotPlugin));
        
        // Add 100 test entities
        for i in 0..100 {
            app.world.spawn((Snapshotable, TestComponent { value: i }));
        }
        
        b.iter(|| {
            // Create a snapshot
            app.world.send_event(SnapshotEvent::Take);
            app.update();
            
            // Clear the snapshots for the next iteration
            app.world.resource_mut::<SnapshotRegistry>().snapshots.clear();
        });
    });
}

fn snapshot_application_benchmark(c: &mut Criterion) {
    c.bench_function("apply snapshot with 100 entities", |b| {
        // Set up a test app
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, SnapshotPlugin));
        
        // Add 100 test entities
        for i in 0..100 {
            app.world.spawn((Snapshotable, TestComponent { value: i }));
        }
        
        // Create a snapshot
        app.world.send_event(SnapshotEvent::Take);
        app.update();
        
        // Get the snapshot ID
        let snapshot_id = app.world.resource::<SnapshotRegistry>()
                               .most_recent().unwrap().id;
        
        b.iter(|| {
            // Apply the snapshot
            app.world.send_event(SnapshotEvent::Apply(snapshot_id));
            app.update();
        });
    });
}

criterion_group!(
    snapshot_benches,
    snapshot_creation_benchmark,
    snapshot_application_benchmark
);
criterion_main!(snapshot_benches);
```

## Network Testing

Testing snapshot integration with networking:

```rust
#[test]
fn test_network_snapshot_sync() {
    // Set up a server app
    let mut server_app = App::new();
    server_app.add_plugins((
        MinimalPlugins,
        RepliconServerPlugin,
        SnapshotPlugin,
        NetworkSnapshotPlugin,
    ));
    
    // Set up a client app
    let mut client_app = App::new();
    client_app.add_plugins((
        MinimalPlugins,
        RepliconClientPlugin,
        SnapshotPlugin,
        NetworkSnapshotPlugin,
    ));
    
    // Connect the client to the server
    let client_id = connect_client_to_server(&mut server_app, &mut client_app);
    
    // Set up game state on the server
    setup_test_game(&mut server_app);
    
    // Trigger a snapshot and network sync
    server_app.world.send_event(SnapshotEvent::Take);
    server_app.update();
    
    // Run multiple updates to allow for network processing
    for _ in 0..10 {
        server_app.update();
        client_app.update();
    }
    
    // Verify the client received and applied the snapshot
    let client_state = client_app.world.resource::<GameState>();
    let server_state = server_app.world.resource::<GameState>();
    
    assert_eq!(client_state.turn, server_state.turn, "Client turn should match server");
    assert_eq!(client_state.phase, server_state.phase, "Client phase should match server");
}
```

## Testing Deterministic RNG

Tests for RNG integration with snapshots:

```rust
#[test]
fn test_rng_snapshot_determinism() {
    // Set up a test app
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        SnapshotPlugin,
        DeterministicRNGPlugin,
    ));
    
    // Initialize RNG with a known seed
    app.insert_resource(NetworkedRngState {
        seed: 12345,
        usage_count: 0,
    });
    
    // Create an RNG and generate some random numbers
    let mut rng = app.world.resource_mut::<NetworkedRngState>().create_rng();
    let first_values: Vec<u32> = (0..10).map(|_| rng.gen::<u32>()).collect();
    
    // Take a snapshot
    app.world.send_event(SnapshotEvent::Take);
    app.update();
    
    // Get the snapshot ID
    let snapshot_id = app.world.resource::<SnapshotRegistry>()
                           .most_recent().unwrap().id;
    
    // Generate more random numbers (changing the RNG state)
    let mut rng = app.world.resource_mut::<NetworkedRngState>().create_rng();
    for _ in 0..20 {
        rng.gen::<u32>();
    }
    
    // Apply the snapshot to restore the RNG state
    app.world.send_event(SnapshotEvent::Apply(snapshot_id));
    app.update();
    
    // Generate random numbers again from the restored state
    let mut rng = app.world.resource_mut::<NetworkedRngState>().create_rng();
    let restored_values: Vec<u32> = (0..10).map(|_| rng.gen::<u32>()).collect();
    
    // Verify the sequences are identical
    assert_eq!(first_values, restored_values, "RNG sequences should be identical after snapshot restoration");
}
```

## Test Fixtures

Creating reusable test fixtures:

```rust
/// Sets up a basic test environment for snapshot testing
fn setup_snapshot_test_environment() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        SnapshotPlugin,
    ));
    
    // Add test entities
    app.world.spawn((Snapshotable, TestComponent { value: 1 }));
    app.world.spawn((Snapshotable, TestComponent { value: 2 }));
    app.world.spawn((Snapshotable, TestComponent { value: 3 }));
    
    // Add game state
    app.insert_resource(GameState {
        turn: 1,
        phase: Phase::Main1,
        active_player: 0,
    });
    
    app
}

/// Creates a snapshot and returns the snapshot ID
fn create_test_snapshot(app: &mut App) -> Uuid {
    app.world.send_event(SnapshotEvent::Take);
    app.update();
    
    app.world.resource::<SnapshotRegistry>()
             .most_recent()
             .unwrap()
             .id
}

/// Test component for snapshot tests
#[derive(Component, Clone, PartialEq, Debug, Serialize, Deserialize)]
struct TestComponent {
    value: i32,
}
```

## Mocking Dependencies

Using mocks for testing:

```rust
/// Mock game state for testing
#[derive(Resource, Clone, Debug, Default)]
struct MockGameState {
    turn: u32,
    phase: Phase,
    active_player: usize,
}

/// Mock event for testing auto-snapshots
#[derive(Event)]
struct MockTurnChangeEvent;

/// Test that snapshots are triggered by events using mocks
#[test]
fn test_snapshot_event_triggers() {
    // Set up a test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(SnapshotPlugin)
       .add_event::<MockTurnChangeEvent>();
    
    // Add a system that listens for MockTurnChangeEvent and triggers snapshots
    app.add_systems(Update, |
        mut turn_events: EventReader<MockTurnChangeEvent>,
        mut snapshot_events: EventWriter<SnapshotEvent>,
    | {
        for _ in turn_events.iter() {
            snapshot_events.send(SnapshotEvent::Take);
        }
    });
    
    // Send a mock turn change event
    app.world.send_event(MockTurnChangeEvent);
    app.update();
    
    // Verify a snapshot was created
    let snapshot_registry = app.world.resource::<SnapshotRegistry>();
    assert_eq!(snapshot_registry.snapshots.len(), 1, "Should create a snapshot in response to the event");
}
```

## Best Practices

When testing the snapshot system:

1. **Isolate Tests**: Each test should focus on a specific aspect of the snapshot system
2. **Use Test Fixtures**: Create reusable setups for snapshot testing
3. **Test Error Handling**: Verify behavior when snapshots fail or are corrupted
4. **Measure Performance**: Track performance metrics for snapshot operations
5. **Test Edge Cases**:
   - Empty snapshots
   - Very large snapshots
   - Concurrent snapshot operations
   - Snapshot application during state changes
6. **Test Integration Points**: Verify all systems that interact with snapshots
7. **Use Mocks**: Create mock implementations of dependencies for focused testing

## Next Steps

- **[API Reference](api_reference.md)**: Complete reference documentation for the snapshot system 