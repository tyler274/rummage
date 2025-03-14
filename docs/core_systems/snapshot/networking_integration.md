# Snapshot Integration with Networking

This document explains how the snapshot system integrates with Rummage's networking capabilities to enable multiplayer gameplay.

## Overview

The snapshot system forms a critical part of Rummage's networking architecture, providing:

1. **State Synchronization**: Ensuring all clients have the same game state
2. **Rollback Capability**: Allowing recovery from network disruptions
3. **Deterministic Execution**: Working with deterministic systems for consistent gameplay
4. **Hidden Information Management**: Handling information that should be hidden from certain players

## Integration Architecture

### Core Components

The networking integration uses these key components:

```rust
/// Plugin that integrates the snapshot system with networking
pub struct NetworkSnapshotPlugin;

/// Resource that tracks network-specific snapshot configuration
#[derive(Resource)]
pub struct NetworkSnapshotConfig {
    /// Frequency of network snapshot updates (in seconds)
    pub sync_frequency: f32,
    /// Maximum size of a snapshot packet (in bytes)
    pub max_packet_size: usize,
    /// Whether to compress network snapshots
    pub compress_network_snapshots: bool,
    /// Number of snapshots to keep for rollback purposes
    pub rollback_history_size: usize,
}

/// Component marking entities with network-specific snapshot requirements
#[derive(Component)]
pub struct NetworkSnapshotable {
    /// Which player IDs can see this entity
    pub visible_to: Vec<u64>,
    /// Priority for synchronization (higher values sync first)
    pub sync_priority: u8,
}
```

### Plugin Implementation

The integration plugin adds networking-specific systems:

```rust
impl Plugin for NetworkSnapshotPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<NetworkSnapshotConfig>()
            .init_resource::<PendingNetworkSnapshots>()
            
            // Register custom events
            .add_event::<NetworkSnapshotEvent>()
            
            // Add networking-specific systems
            .add_systems(Update, (
                sync_game_state_with_clients,
                handle_network_snapshot_events,
                process_incoming_snapshots,
            ).chain())
            
            // Add systems to the replicon client and server sets
            .add_systems(RepliconClientSet::Receive, receive_network_snapshot)
            .add_systems(RepliconServerSet::Send, send_network_snapshots);
    }
}
```

## State Synchronization

The core of the networking integration is the state synchronization system:

```rust
fn sync_game_state_with_clients(
    mut commands: Commands,
    time: Res<Time>,
    mut last_sync: Local<f32>,
    config: Res<NetworkSnapshotConfig>,
    game_state: Res<GameState>,
    client_info: Res<ClientRegistry>,
    world: &World,
    mut network_events: EventWriter<NetworkSnapshotEvent>,
) {
    // Check if it's time for a sync
    if time.elapsed_seconds() - *last_sync < config.sync_frequency {
        return;
    }
    
    *last_sync = time.elapsed_seconds();
    
    // Create a base snapshot of the current state
    let base_snapshot = create_game_snapshot(world, &game_state);
    
    // For each connected client, create a tailored snapshot
    for client_id in client_info.connected_clients() {
        let client_snapshot = create_client_specific_snapshot(
            &base_snapshot, 
            client_id, 
            &client_info
        );
        
        // Send the snapshot to the client
        network_events.send(NetworkSnapshotEvent::SendTo(
            client_id, 
            client_snapshot
        ));
    }
}
```

## Client-Specific Snapshots

The system creates tailored snapshots for each client to manage hidden information:

```rust
fn create_client_specific_snapshot(
    base_snapshot: &GameSnapshot,
    client_id: u64,
    client_info: &ClientRegistry,
) -> GameSnapshot {
    // Clone the base snapshot
    let mut client_snapshot = base_snapshot.clone();
    
    // Filter out data the client shouldn't see
    client_snapshot.game_data.retain(|entity_key, _| {
        // Check if this entity is visible to the client
        let entity = Entity::from_bits(
            u64::from_str(entity_key).unwrap_or_default()
        );
        
        is_entity_visible_to_client(entity, client_id, client_info)
    });
    
    // Modify certain data to hide information
    for (_, entity_data) in client_snapshot.game_data.iter_mut() {
        sanitize_hidden_information(entity_data, client_id, client_info);
    }
    
    client_snapshot
}

fn is_entity_visible_to_client(
    entity: Entity,
    client_id: u64,
    client_info: &ClientRegistry,
) -> bool {
    // Logic to determine if an entity should be visible to a client
    // ...
}

fn sanitize_hidden_information(
    entity_data: &mut Vec<u8>,
    client_id: u64,
    client_info: &ClientRegistry,
) {
    // Logic to modify entity data to hide sensitive information
    // ...
}
```

## Network Data Transfer

The system handles sending and receiving snapshots over the network:

```rust
fn send_network_snapshots(
    mut server: ResMut<RenetServer>,
    mut pending: ResMut<PendingNetworkSnapshots>,
    config: Res<NetworkSnapshotConfig>,
) {
    // Process all pending network snapshots
    for (client_id, snapshot) in pending.outgoing.drain(..) {
        // Serialize the snapshot
        let serialized = bincode::serialize(&snapshot)
            .unwrap_or_default();
        
        // Compress if configured
        let final_data = if config.compress_network_snapshots {
            // Compression logic...
            Vec::new()
        } else {
            serialized
        };
        
        // Send to client
        server.send_message(
            client_id,
            NetworkChannel::StateSync as u8,
            final_data
        );
    }
}

fn receive_network_snapshot(
    mut client: ResMut<RenetClient>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
) {
    // Check for incoming snapshot messages
    while let Some(message) = client.receive_message(NetworkChannel::StateSync as u8) {
        // Decompress if needed
        let serialized = if is_compressed(&message) {
            // Decompression logic...
            Vec::new()
        } else {
            message
        };
        
        // Deserialize the snapshot
        if let Ok(snapshot) = bincode::deserialize::<GameSnapshot>(&serialized) {
            // Apply the received snapshot
            snapshot_events.send(SnapshotEvent::Apply(snapshot.id));
        }
    }
}
```

## Rollback System

The integration includes a rollback system for handling network disruptions:

```rust
/// Plugin that provides rollback functionality for network gameplay
pub struct RollbackPlugin;

impl Plugin for RollbackPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RollbackHistory>()
            .add_event::<RollbackEvent>()
            .add_systems(Update, (
                maintain_rollback_history,
                handle_rollback_events,
            ).chain());
    }
}

/// Resource that maintains a history of snapshots for rollback
#[derive(Resource, Default)]
pub struct RollbackHistory {
    /// Historical snapshots indexed by turn and phase
    pub history: HashMap<(u32, Phase), GameSnapshot>,
}

/// Event requesting a rollback to a previous state
#[derive(Event)]
pub struct RollbackEvent {
    /// The turn to roll back to
    pub turn: u32,
    /// The phase within the turn
    pub phase: Option<Phase>,
}

fn maintain_rollback_history(
    mut history: ResMut<RollbackHistory>,
    mut snapshot_events: EventReader<SnapshotProcessedEvent>,
    snapshots: Res<SnapshotRegistry>,
    config: Res<NetworkSnapshotConfig>,
) {
    // For each new snapshot, add it to the history
    for event in snapshot_events.iter() {
        if let Some(snapshot) = snapshots.get(event.id) {
            history.history.insert(
                (snapshot.turn, snapshot.phase.clone()),
                snapshot.clone()
            );
        }
    }
    
    // Prune history to maintain size limits
    if history.history.len() > config.rollback_history_size {
        // Pruning logic...
    }
}

fn handle_rollback_events(
    mut rollback_events: EventReader<RollbackEvent>,
    history: Res<RollbackHistory>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
) {
    for event in rollback_events.iter() {
        // Find the snapshot to roll back to
        let target_snapshot = if let Some(phase) = &event.phase {
            // Find specific phase
            history.history.get(&(event.turn, phase.clone()))
        } else {
            // Find any snapshot from this turn
            history.history.iter()
                .find(|((turn, _), _)| *turn == event.turn)
                .map(|(_, snapshot)| snapshot)
        };
        
        // If found, apply the snapshot
        if let Some(snapshot) = target_snapshot {
            snapshot_events.send(SnapshotEvent::Apply(snapshot.id));
        }
    }
}
```

## Deterministic Random Number Generator

The integration includes special handling for random number generation to ensure deterministic gameplay:

```rust
/// Plugin that integrates deterministic RNG with snapshots for networking
pub struct DeterministicRNGPlugin;

impl Plugin for DeterministicRNGPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NetworkedRngState>()
            .add_systems(Update, (
                capture_rng_in_snapshot,
                restore_rng_from_snapshot,
            ).chain());
    }
}

/// Resource that tracks the state of the deterministic RNG
#[derive(Resource, Serialize, Deserialize)]
pub struct NetworkedRngState {
    /// The current seed
    pub seed: u64,
    /// The number of times the RNG has been used
    pub usage_count: u64,
}

fn capture_rng_in_snapshot(
    rng_state: Res<NetworkedRngState>,
    mut create_snapshot: EventReader<SnapshotEvent>,
    mut snapshots: ResMut<SnapshotRegistry>,
) {
    for event in create_snapshot.iter() {
        if let SnapshotEvent::Take = event {
            // Find the most recent snapshot
            if let Some(snapshot) = snapshots.most_recent() {
                // Add RNG state to the snapshot
                let rng_data = bincode::serialize(&*rng_state).unwrap_or_default();
                snapshot.game_data.insert("rng_state".to_string(), rng_data);
            }
        }
    }
}

fn restore_rng_from_snapshot(
    mut rng_state: ResMut<NetworkedRngState>,
    mut apply_snapshot: EventReader<SnapshotEvent>,
    snapshots: Res<SnapshotRegistry>,
) {
    for event in apply_snapshot.iter() {
        if let SnapshotEvent::Apply(id) = event {
            // Find the snapshot
            if let Some(snapshot) = snapshots.get(*id) {
                // Restore RNG state from snapshot
                if let Some(rng_data) = snapshot.game_data.get("rng_state") {
                    if let Ok(state) = bincode::deserialize::<NetworkedRngState>(rng_data) {
                        *rng_state = state;
                    }
                }
            }
        }
    }
}
```

## Testing Network Integration

Testing the networking integration with snapshots:

```rust
#[test]
fn test_network_snapshot_synchronization() {
    // Set up a test server and client
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        RepliconServerPlugin,
        SnapshotPlugin,
        NetworkSnapshotPlugin,
    ));
    
    // Create a test client
    let client_id = 1;
    let mut client_registry = ClientRegistry::default();
    client_registry.register_client(client_id);
    app.insert_resource(client_registry);
    
    // Set up game state
    setup_test_game_state(&mut app);
    
    // Trigger a snapshot
    app.world.send_event(SnapshotEvent::Take);
    app.update();
    
    // Verify that a network snapshot was created
    let network_events = app.world.resource::<Events<NetworkSnapshotEvent>>();
    let mut reader = network_events.get_reader();
    
    let has_snapshot_event = reader.iter(&network_events).any(|event| {
        matches!(event, NetworkSnapshotEvent::SendTo(id, _) if *id == client_id)
    });
    
    assert!(has_snapshot_event, "Should create a network snapshot for the client");
}
```

## Best Practices

When working with the snapshot and networking integration:

1. **Minimize Snapshot Size**: Use the `NetworkSnapshotable` component to control what gets synchronized
2. **Handle Frequent Updates**: Be mindful of performance impact for frequently changing components
3. **Test Network Conditions**: Use simulated network conditions to test behavior under varying latency
4. **Secure Hidden Information**: Carefully audit what information is sent to each client
5. **Handle Reconnections**: Ensure clients that reconnect receive a complete state update
6. **Monitor Bandwidth**: Keep track of snapshot sizes and network usage
7. **Implement Fallbacks**: Have strategies for when snapshot synchronization fails

## Next Steps

- **[Testing](testing.md)**: How to test snapshot functionality, including network integration
- **[API Reference](api_reference.md)**: Complete reference documentation for the snapshot system 