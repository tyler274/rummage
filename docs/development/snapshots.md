# Snapshot System

This guide explains Rummage's snapshot system, which is used for game state serialization, replay, and networked multiplayer.

## Table of Contents

1. [Introduction to Snapshots](#introduction-to-snapshots)
2. [Snapshot Architecture](#snapshot-architecture)
3. [Creating Snapshots](#creating-snapshots)
4. [Processing Snapshots](#processing-snapshots)
5. [Snapshot Events](#snapshot-events)
6. [Integration with Networking](#integration-with-networking)
7. [Testing Snapshots](#testing-snapshots)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)

## Introduction to Snapshots

The snapshot system in Rummage provides a way to capture, serialize, and later restore the state of the game. This is essential for several critical features:

- **Networked Multiplayer**: Synchronizing game state between players
- **Replay System**: Recording and replaying games
- **Undo Functionality**: Allowing players to revert to previous game states
- **Save/Load**: Persisting game state between sessions

A snapshot is a serializable representation of the game state at a specific point in time. It captures all relevant entity data needed to recreate the exact game state.

## Snapshot Architecture

The snapshot system uses the following key components:

### Core Types

```rust
/// A serializable snapshot of the game state
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameSnapshot {
    /// Unique identifier for the snapshot
    pub id: Uuid,
    /// The game turn the snapshot was taken on
    pub turn: u32,
    /// The phase within the turn
    pub phase: Phase,
    /// Active player when snapshot was taken
    pub active_player: usize,
    /// Serialized game data
    pub game_data: HashMap<String, Vec<u8>>,
    /// Timestamp when the snapshot was created
    pub timestamp: f64,
}

/// Tracks pending snapshot operations
#[derive(Resource, Default)]
pub struct PendingSnapshots {
    /// Snapshots waiting to be processed
    pub queue: VecDeque<GameSnapshot>,
    /// Whether snapshot processing is paused
    pub paused: bool,
}

/// Marker for entities that should be included in snapshots
#[derive(Component)]
pub struct Snapshotable;
```

### Plugin Structure

The snapshot system is implemented as a Bevy plugin:

```rust
pub struct SnapshotPlugin;

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<PendingSnapshots>()
            .init_resource::<SnapshotConfig>()
            
            // Register snapshot events
            .add_event::<SnapshotEvent>()
            .add_event::<SnapshotProcessedEvent>()
            
            // Add snapshot systems to the appropriate schedule
            .add_systems(Update, (
                handle_snapshot_events,
                process_pending_snapshots,
            ).chain());
    }
}
```

## Creating Snapshots

Snapshots are created by capturing the relevant components from entities marked as `Snapshotable`:

```rust
fn create_game_snapshot(
    world: &World,
    game_state: &GameState,
) -> GameSnapshot {
    // Create a new empty snapshot
    let mut snapshot = GameSnapshot {
        id: Uuid::new_v4(),
        turn: game_state.turn,
        phase: game_state.phase.clone(),
        active_player: game_state.active_player,
        game_data: HashMap::new(),
        timestamp: world.resource::<Time>().elapsed_seconds(),
    };
    
    // Find all snapshotable entities
    let mut snapshotable_query = world.query_filtered::<Entity, With<Snapshotable>>();
    
    // For each snapshotable entity, serialize its components
    for entity in snapshotable_query.iter(world) {
        serialize_entity_to_snapshot(world, entity, &mut snapshot);
    }
    
    snapshot
}

fn serialize_entity_to_snapshot(
    world: &World,
    entity: Entity,
    snapshot: &mut GameSnapshot,
) {
    // Entity serialization logic...
}
```

Snapshots are typically triggered by game events:

```rust
fn trigger_snapshot_on_turn_change(
    mut turn_events: EventReader<TurnChangeEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    config: Res<SnapshotConfig>,
) {
    if !config.auto_snapshot_on_turn {
        return;
    }
    
    for event in turn_events.iter() {
        // Create a new snapshot event
        snapshot_events.send(SnapshotEvent::Take);
    }
}
```

## Processing Snapshots

The `process_pending_snapshots` system handles snapshots in the pending queue:

```rust
/// Process any pending snapshots in the queue
pub fn process_pending_snapshots(
    mut commands: Commands,
    mut pending: ResMut<PendingSnapshots>,
    mut processed_events: EventWriter<SnapshotProcessedEvent>,
    config: Res<SnapshotConfig>,
) {
    // Skip if processing is paused
    if pending.paused {
        return;
    }
    
    // Process up to config.max_snapshots_per_frame
    let to_process = pending.queue.len().min(config.max_snapshots_per_frame);
    
    for _ in 0..to_process {
        if let Some(snapshot) = pending.queue.pop_front() {
            // Apply the snapshot to the game state
            apply_snapshot(&mut commands, &snapshot);
            
            // Notify that a snapshot was processed
            processed_events.send(SnapshotProcessedEvent {
                id: snapshot.id,
                success: true,
            });
        }
    }
}

fn apply_snapshot(
    commands: &mut Commands,
    snapshot: &GameSnapshot,
) {
    // Snapshot application logic...
}
```

## Snapshot Events

The system uses events to communicate snapshot operations:

```rust
/// Events for snapshot operations
#[derive(Event)]
pub enum SnapshotEvent {
    /// Take a new snapshot of the current state
    Take,
    /// Apply a specific snapshot
    Apply(Uuid),
    /// Save the current snapshot to disk
    Save(String),
    /// Load a snapshot from disk
    Load(String),
}

/// Event fired when a snapshot has been processed
#[derive(Event)]
pub struct SnapshotProcessedEvent {
    /// The ID of the processed snapshot
    pub id: Uuid,
    /// Whether processing succeeded
    pub success: bool,
}
```

The `handle_snapshot_events` system processes these events:

```rust
pub fn handle_snapshot_events(
    mut commands: Commands,
    mut snapshot_events: EventReader<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
    game_state: Res<GameState>,
    world: &World,
) {
    for event in snapshot_events.iter() {
        match event {
            SnapshotEvent::Take => {
                // Create a new snapshot
                let snapshot = create_game_snapshot(world, &game_state);
                // Add to pending queue
                pending.queue.push_back(snapshot);
            },
            SnapshotEvent::Apply(id) => {
                // Find and apply the snapshot by ID
                // ...
            },
            SnapshotEvent::Save(path) => {
                // Save the current snapshot to disk
                // ...
            },
            SnapshotEvent::Load(path) => {
                // Load a snapshot from disk
                // ...
            }
        }
    }
}
```

## Integration with Networking

The snapshot system integrates with bevy_replicon for networked multiplayer:

```rust
/// Network-related snapshot events
#[derive(Event, Serialize, Deserialize)]
pub enum NetworkSnapshotEvent {
    /// Request a snapshot from the server
    Request,
    /// Send a snapshot to clients
    Provide(GameSnapshot),
    /// Acknowledge receipt of a snapshot
    Acknowledge(Uuid),
}

fn handle_network_snapshot_events(
    mut network_events: EventReader<NetworkSnapshotEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
    network: Res<NetworkResource>,
    is_server: Res<IsServer>,
) {
    for event in network_events.iter() {
        match event {
            NetworkSnapshotEvent::Request => {
                if is_server.0 {
                    // Server creates a snapshot to send to client
                    snapshot_events.send(SnapshotEvent::Take);
                    // Logic to send the snapshot...
                }
            },
            NetworkSnapshotEvent::Provide(snapshot) => {
                if !is_server.0 {
                    // Client receives a snapshot from server
                    pending.queue.push_back(snapshot.clone());
                }
            },
            NetworkSnapshotEvent::Acknowledge(id) => {
                // Handle acknowledgment...
            }
        }
    }
}
```

## Testing Snapshots

Testing snapshot functionality is crucial for stability. Rummage includes several test utilities:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_snapshot_serialization() {
        // Set up a test game state
        let mut app = App::new();
        app.add_plugin(SnapshotPlugin);
        
        // Add some test entities
        app.world.spawn((
            Snapshotable,
            TestComponent { value: 42 },
        ));
        
        // Take a snapshot
        let snapshot = create_game_snapshot(&app.world, &GameState::default());
        
        // Verify snapshot data
        assert!(snapshot.game_data.contains_key("TestComponent"));
        // ...
    }
    
    #[test]
    fn test_snapshot_application() {
        // Test applying a snapshot to restore state
        // ...
    }
    
    #[test]
    fn test_snapshot_roundtrip() {
        // Test full roundtrip: create -> serialize -> deserialize -> apply
        // ...
    }
}
```

## Best Practices

When working with the snapshot system, follow these best practices:

### Component Design

1. **Implement Serialization**: Make sure all components that need to be included in snapshots implement `Serialize` and `Deserialize`
2. **Mark Entities**: Use the `Snapshotable` component for entities that should be included
3. **Minimize Data**: Only include necessary data in snapshots to reduce size and processing time

### System Design

1. **Snapshot Frequency**: Balance frequency of snapshots with performance considerations
2. **Batched Processing**: Process snapshots in batches to avoid frame drops
3. **Error Handling**: Gracefully handle snapshot errors without affecting gameplay

### Networking Considerations

1. **Bandwidth Management**: Send only delta snapshots when possible
2. **Verification**: Include checksums or version information to verify snapshot integrity
3. **Fallbacks**: Have fallback mechanisms when snapshots fail to apply

## Troubleshooting

### Common Snapshot Issues

#### Deserialization Errors

If you see deserialization errors in the logs:

```
ERROR Failed to deserialize snapshot component: Error("missing field `position`", line: 0, column: 0)
```

Ensure all serialized components maintain backward compatibility when changed.

#### Performance Issues

If snapshot processing causes frame rate drops:

```
WARN Snapshot processing took 35ms, exceeding target frame time
```

Consider:
- Reducing snapshot frequency
- Decreasing `max_snapshots_per_frame` in the `SnapshotConfig`
- Optimizing serialization of large components

#### Divergent Game State

If game states diverge between clients:

1. Ensure deterministic behavior for all systems that modify snapshotable entities
2. Check for race conditions in snapshot application
3. Verify that all clients have the same version of game code and data

---

For further information on using the snapshot system, see the [Networking Guide](networking/index.md) for details on network integration. 