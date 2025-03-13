# Bevy Replicon Integration for Rollback with RNG State Management

This document details the integration of bevy_replicon with our rollback system, focusing on maintaining RNG state consistency across the network.

## Table of Contents

1. [Introduction](#introduction)
2. [Replicon and RNG Integration](#replicon-and-rng-integration)
3. [Resources and Components](#resources-and-components)
4. [Systems Integration](#systems-integration)
5. [State Preservation and Recovery](#state-preservation-and-recovery)
6. [Implementation Examples](#implementation-examples)
7. [Performance Considerations](#performance-considerations)
8. [Testing Guidelines](#testing-guidelines)

## Introduction

bevy_replicon is a lightweight, ECS-friendly networking library that provides replication for Bevy games. While it handles much of the complexity of network synchronization, maintaining deterministic RNG state during rollbacks requires additional mechanisms.

This document outlines how we extend bevy_replicon to handle RNG state management during network disruptions, ensuring all clients maintain identical random number sequences after recovery.

## Replicon and RNG Integration

The key challenge is integrating bevy_replicon's entity replication with our RNG management system, particularly when:

1. Replicating randomized game actions
2. Handling rollbacks after connection interruptions
3. Ensuring newly connected clients receive the correct RNG state
4. Maintaining determinism during complex game scenarios

Our solution uses bevy_replicon's server-authoritative model but adds RNG state tracking and distribution mechanisms.

### Architectural Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           SERVER                                         │
│                                                                         │
│  ┌───────────────┐     ┌──────────────────┐     ┌────────────────────┐  │
│  │               │     │                  │     │                    │  │
│  │  REPLICON     │────▶│  RNG STATE       │────▶│  GAME STATE       │  │
│  │  SERVER       │     │  MANAGER         │     │  MANAGER          │  │
│  │               │     │                  │     │                    │  │
│  └───────┬───────┘     └─────────┬────────┘     └────────┬───────────┘  │
│          │                       │                       │              │
│          │                       │                       │              │
│          ▼                       ▼                       ▼              │
│  ┌───────────────┐     ┌──────────────────┐     ┌────────────────────┐  │
│  │               │     │                  │     │                    │  │
│  │  REPLICON     │────▶│  ROLLBACK       │◀────│  SEQUENCE          │  │
│  │  REPLICATION  │     │  COORDINATOR    │     │  TRACKER           │  │
│  │               │     │                  │     │                    │  │
│  └───────────────┘     └──────────────────┘     └────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           CLIENT                                         │
│                                                                         │
│  ┌───────────────┐     ┌──────────────────┐     ┌────────────────────┐  │
│  │               │     │                  │     │                    │  │
│  │  REPLICON     │────▶│  RNG STATE       │────▶│  GAME STATE       │  │
│  │  CLIENT       │     │  APPLIER         │     │  RECEIVER         │  │
│  │               │     │                  │     │                    │  │
│  └───────┬───────┘     └─────────┬────────┘     └────────┬───────────┘  │
│          │                       │                       │              │
│          │                       │                       │              │
│          ▼                       ▼                       ▼              │
│  ┌───────────────┐     ┌──────────────────┐     ┌────────────────────┐  │
│  │               │     │                  │     │                    │  │
│  │  LOCAL        │────▶│  PREDICTION     │◀────│  HISTORY           │  │
│  │  RNG MANAGER  │     │  RECONCILIATION │     │  TRACKER           │  │
│  │               │     │                  │     │                    │  │
│  └───────────────┘     └──────────────────┘     └────────────────────┘  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Resources and Components

### Core Resources

```rust
/// Resource that tracks RNG state for replication
#[derive(Resource)]
pub struct RngReplicationState {
    /// Current global RNG state
    pub global_state: Vec<u8>,
    /// Player-specific RNG states
    pub player_states: HashMap<Entity, Vec<u8>>,
    /// Sequence number for the latest RNG state update
    pub sequence: u64,
    /// Timestamp of the last update
    pub last_update: f32,
    /// Flag indicating the state has changed
    pub dirty: bool,
}

/// Resource for rollback checkpoints with RNG state
#[derive(Resource)]
pub struct RollbackCheckpoints {
    /// Checkpoints with sequence IDs as keys
    pub checkpoints: BTreeMap<u64, RollbackCheckpoint>,
    /// Maximum number of checkpoints to maintain
    pub max_checkpoints: usize,
}

/// Structure for a single rollback checkpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RollbackCheckpoint {
    /// Checkpoint sequence ID
    pub sequence_id: u64,
    /// Timestamp of the checkpoint
    pub timestamp: f32,
    /// Global RNG state
    pub global_rng_state: Vec<u8>,
    /// Player-specific RNG states
    pub player_rng_states: HashMap<Entity, Vec<u8>>,
    /// Replicated entities snapshot
    pub replicated_entities: Vec<EntityData>,
}

/// Replicon channel for RNG synchronization
#[derive(Default)]
pub struct RngSyncChannel;

/// Extension for RepliconServer to handle RNG state
impl RepliconServerExt for RepliconServer {
    /// Send RNG state to a specific client
    fn send_rng_state(&mut self, client_id: ClientId, rng_state: &RngReplicationState) {
        let message = RngStateMessage {
            global_state: rng_state.global_state.clone(),
            player_states: rng_state.player_states.clone(),
            sequence: rng_state.sequence,
            timestamp: rng_state.last_update,
        };
        
        self.send_message(client_id, RngSyncChannel, bincode::serialize(&message).unwrap());
    }
    
    /// Broadcast RNG state to all clients
    fn broadcast_rng_state(&mut self, rng_state: &RngReplicationState) {
        let message = RngStateMessage {
            global_state: rng_state.global_state.clone(),
            player_states: rng_state.player_states.clone(),
            sequence: rng_state.sequence,
            timestamp: rng_state.last_update,
        };
        
        self.broadcast_message(RngSyncChannel, bincode::serialize(&message).unwrap());
    }
}
```

### Components for Entity Tracking

```rust
/// Component to flag an entity as having randomized behavior
#[derive(Component, Reflect, Default)]
pub struct RandomizedBehavior {
    /// The last RNG sequence ID used for this entity
    pub last_rng_sequence: u64,
    /// Whether this entity has pending randomized actions
    pub has_pending_actions: bool,
}

/// Component for player-specific RNG
#[derive(Component, Reflect)]
pub struct PlayerRng {
    /// Sequence of the last RNG state
    pub sequence: u64,
    /// Whether this RNG is remote (on another client)
    pub is_remote: bool,
}
```

## Systems Integration

### Server-Side Systems

```rust
/// Plugin that integrates bevy_replicon with our RNG and rollback systems
pub struct RepliconRngRollbackPlugin;

impl Plugin for RepliconRngRollbackPlugin {
    fn build(&self, app: &mut App) {
        // Register network channel
        app.register_network_channel::<RngSyncChannel>(ChannelConfig {
            channel_id: 100, // Use a unique channel ID
            mode: ChannelMode::Unreliable,
        });
        
        // Add resources
        app.init_resource::<RngReplicationState>()
            .init_resource::<RollbackCheckpoints>();
            
        // Server systems
        app.add_systems(Update, (
            capture_rng_state,
            replicate_rng_state,
            create_rollback_checkpoints,
        ).run_if(resource_exists::<RepliconServer>()));
        
        // Client systems
        app.add_systems(Update, (
            apply_rng_state_updates,
            handle_rollback_requests,
        ).run_if(resource_exists::<RepliconClient>()));
    }
}

/// System to capture RNG state for replication
pub fn capture_rng_state(
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    player_rngs: Query<(Entity, &PlayerRng)>,
    mut rng_state: ResMut<RngReplicationState>,
    time: Res<Time>,
    sequence: Res<SequenceTracker>,
) {
    // Don't update too frequently
    if time.elapsed_seconds() - rng_state.last_update < 1.0 {
        return;
    }
    
    // Capture global RNG state
    if let Some(state) = global_rng.try_serialize_state() {
        rng_state.global_state = state;
        rng_state.dirty = true;
    }
    
    // Capture player RNG states
    for (entity, _) in player_rngs.iter() {
        if let Some(player_rng) = player_rngs.get_component::<Entropy<WyRand>>(entity).ok() {
            if let Some(state) = player_rng.try_serialize_state() {
                rng_state.player_states.insert(entity, state);
                rng_state.dirty = true;
            }
        }
    }
    
    if rng_state.dirty {
        rng_state.sequence = sequence.current_sequence;
        rng_state.last_update = time.elapsed_seconds();
    }
}

/// System to replicate RNG state to clients
pub fn replicate_rng_state(
    mut server: ResMut<RepliconServer>,
    rng_state: Res<RngReplicationState>,
) {
    if rng_state.dirty {
        server.broadcast_rng_state(&rng_state);
    }
}

/// System to create rollback checkpoints
pub fn create_rollback_checkpoints(
    mut checkpoints: ResMut<RollbackCheckpoints>,
    rng_state: Res<RngReplicationState>,
    time: Res<Time>,
    replicated_query: Query<Entity, With<Replication>>,
    entity_data: Res<EntityData>,
) {
    // Create a new checkpoint every few seconds
    if time.elapsed_seconds() % 5.0 < 0.1 {
        // Collect replicated entity data
        let mut entities = Vec::new();
        for entity in replicated_query.iter() {
            if let Some(data) = entity_data.get_entity_data(entity) {
                entities.push(data.clone());
            }
        }
        
        // Create checkpoint
        let checkpoint = RollbackCheckpoint {
            sequence_id: rng_state.sequence,
            timestamp: time.elapsed_seconds(),
            global_rng_state: rng_state.global_state.clone(),
            player_rng_states: rng_state.player_states.clone(),
            replicated_entities: entities,
        };
        
        // Add to checkpoints
        checkpoints.checkpoints.insert(rng_state.sequence, checkpoint);
        
        // Prune old checkpoints
        while checkpoints.checkpoints.len() > checkpoints.max_checkpoints {
            if let Some((&oldest_key, _)) = checkpoints.checkpoints.iter().next() {
                checkpoints.checkpoints.remove(&oldest_key);
            }
        }
    }
}
```

### Client-Side Systems

```rust
/// System to apply RNG state updates from server
pub fn apply_rng_state_updates(
    mut client: ResMut<RepliconClient>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut player_rngs: Query<(Entity, &mut PlayerRng)>,
    mut events: EventReader<NetworkEvent>,
) {
    for event in events.read() {
        if let NetworkEvent::Message(_, RngSyncChannel, data) = event {
            // Deserialize the RNG state message
            if let Ok(message) = bincode::deserialize::<RngStateMessage>(data) {
                // Apply global RNG state
                if !message.global_state.is_empty() {
                    global_rng.deserialize_state(&message.global_state)
                        .expect("Failed to deserialize global RNG state");
                }
                
                // Apply player-specific RNG states
                for (entity, mut player_rng) in player_rngs.iter_mut() {
                    if let Some(state) = message.player_states.get(&entity) {
                        if let Some(rng) = player_rngs.get_component_mut::<Entropy<WyRand>>(entity).ok() {
                            rng.deserialize_state(state).expect("Failed to deserialize player RNG state");
                            player_rng.sequence = message.sequence;
                        }
                    }
                }
            }
        }
    }
}

/// System to handle rollback requests
pub fn handle_rollback_requests(
    mut client: ResMut<RepliconClient>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut player_rngs: Query<(Entity, &mut PlayerRng)>,
    mut events: EventReader<NetworkEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let NetworkEvent::Message(_, RollbackChannel, data) = event {
            // Deserialize the rollback message
            if let Ok(message) = bincode::deserialize::<RollbackMessage>(data) {
                // Apply global RNG state from the checkpoint
                if !message.checkpoint.global_rng_state.is_empty() {
                    global_rng.deserialize_state(&message.checkpoint.global_rng_state)
                        .expect("Failed to deserialize checkpoint RNG state");
                }
                
                // Apply player-specific RNG states from the checkpoint
                for (entity, mut player_rng) in player_rngs.iter_mut() {
                    if let Some(state) = message.checkpoint.player_rng_states.get(&entity) {
                        if let Some(rng) = player_rngs.get_component_mut::<Entropy<WyRand>>(entity).ok() {
                            rng.deserialize_state(state).expect("Failed to deserialize player RNG state");
                            player_rng.sequence = message.checkpoint.sequence_id;
                        }
                    }
                }
                
                // Restore entity state from checkpoint
                for entity_data in &message.checkpoint.replicated_entities {
                    // Restore entity or spawn if it doesn't exist
                    // ...
                }
                
                info!("Applied rollback to sequence {}", message.checkpoint.sequence_id);
            }
        }
    }
}
```

## State Preservation and Recovery

The rollback process occurs in these steps:

1. **Detection**: Server detects desynchronization (via mismatch in action results)
2. **Checkpoint Selection**: Server selects appropriate rollback checkpoint
3. **Notification**: Server notifies affected clients of rollback
4. **State Restoration**: Both server and clients:
   - Restore game state 
   - Restore RNG state
   - Replay necessary actions
5. **Verification**: Server verifies all clients are synchronized

### Rollback Protocol

```rust
/// Enum for rollback types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RollbackType {
    /// Full rollback with complete state restoration
    Full,
    /// Partial rollback for specific entities only
    Partial,
    /// RNG-only rollback for randomization issues
    RngOnly,
}

/// Message for rollback requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackMessage {
    /// Type of rollback
    pub rollback_type: RollbackType,
    /// Rollback checkpoint
    pub checkpoint: RollbackCheckpoint,
    /// Reason for rollback
    pub reason: String,
}
```

## Implementation Examples

### Example 1: Rollback After Network Interruption

```rust
/// System to detect and handle network interruptions
pub fn handle_network_interruption(
    mut server: ResMut<RepliconServer>,
    checkpoints: Res<RollbackCheckpoints>,
    clients: Query<(Entity, &ClientConnection)>,
    time: Res<Time>,
) {
    // Check for clients with high latency or disconnection
    for (entity, connection) in clients.iter() {
        if connection.latency > 1.0 || !connection.connected {
            // Find most recent valid checkpoint
            if let Some((_, checkpoint)) = checkpoints.checkpoints.iter().rev().next() {
                // Initiate rollback for all clients
                let rollback_message = RollbackMessage {
                    rollback_type: RollbackType::Full,
                    checkpoint: checkpoint.clone(),
                    reason: "Network interruption detected".to_string(),
                };
                
                // Send to all clients
                server.broadcast_message(RollbackChannel, bincode::serialize(&rollback_message).unwrap());
                
                // Apply rollback on server too
                apply_rollback_on_server(&rollback_message);
                
                info!("Initiated rollback due to network interruption");
            }
            break;
        }
    }
}
```

### Example 2: Handling Card Shuffle During Rollback

```rust
/// System to handle card shuffling during or after a rollback
pub fn handle_shuffle_during_rollback(
    mut commands: Commands,
    mut shuffle_events: EventReader<ShuffleLibraryEvent>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    player_rngs: Query<(Entity, &PlayerRng)>,
    libraries: Query<(Entity, &Library, &Parent)>,
) {
    for event in shuffle_events.read() {
        if let Ok((library_entity, library, parent)) = libraries.get(event.library_entity) {
            // Get the player entity (parent)
            let player_entity = parent.get();
            
            // Get player's RNG
            if let Ok((_, player_rng)) = player_rngs.get(player_entity) {
                // Use the appropriate RNG for deterministic shuffle
                let mut card_indices: Vec<usize> = (0..library.cards.len()).collect();
                
                if player_rng.is_remote {
                    // Use global RNG for remote player to ensure consistency
                    for i in (1..card_indices.len()).rev() {
                        let j = global_rng.gen_range(0..=i);
                        card_indices.swap(i, j);
                    }
                } else {
                    // Use player-specific RNG for local player
                    if let Some(rng) = player_rngs.get_component::<Entropy<WyRand>>(player_entity).ok() {
                        for i in (1..card_indices.len()).rev() {
                            let j = rng.gen_range(0..=i);
                            card_indices.swap(i, j);
                        }
                    }
                }
                
                // Apply shuffle result
                // ...
                
                info!("Performed deterministic shuffle during/after rollback");
            }
        }
    }
}
```

## Performance Considerations

When implementing RNG state management with bevy_replicon and rollbacks, consider these performance factors:

1. **RNG State Size**: 
   - WyRand has a compact 8-byte state, ideal for frequent replication
   - More complex PRNGs may have larger states, increasing network overhead

2. **Checkpoint Frequency**:
   - More frequent checkpoints = better recovery granularity but higher overhead
   - Recommended: 5-10 second intervals for most games

3. **Selective Replication**:
   - Only replicate RNG state when it changes significantly
   - Consider checksums to detect state changes efficiently

4. **Bandwidth Usage**:
   - Use the appropriate channel mode (reliable for critical RNG updates)
   - Batch RNG updates with other state replication when possible

5. **Memory Overhead**:
   - Limit maximum checkpoints based on available memory (10-20 is reasonable)
   - Use sliding window approach to discard old checkpoints

## Testing Guidelines

For effective testing of replicon-based RNG rollback, follow these approaches:

1. **Determinism Tests**:
   - Verify identical seeds produce identical sequences on all clients
   - Test saving and restoring RNG state produces identical future values

2. **Network Disruption Tests**:
   - Simulate connection drops to trigger rollback
   - Verify game state remains consistent after recovery

3. **Performance Tests**:
   - Measure impact of RNG state replication on bandwidth
   - Profile checkpoint creation and restoration overhead

4. **Integration Tests**:
   - Test complex game scenarios like multi-player card shuffling
   - Verify random outcomes remain consistent across network boundaries

For detailed testing examples, see the [RNG Synchronization Tests](../../testing/rng_synchronization_tests.md) document.

---

By following these guidelines, you can create a robust integration between bevy_replicon, our rollback system, and RNG state management that maintains deterministic behavior even during network disruptions. 