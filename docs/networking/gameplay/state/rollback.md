# State Rollback and Recovery

This document outlines the implementation of state rollback and recovery mechanisms in our MTG Commander game engine, addressing network disruptions and maintaining gameplay integrity despite unstable connections.

## Table of Contents

1. [Overview](#overview)
2. [Rollback Architecture](#rollback-architecture)
3. [State Snapshots](#state-snapshots)
4. [Deterministic Replay](#deterministic-replay)
5. [RNG Synchronization for Rollbacks](#rng-synchronization-for-rollbacks)
6. [Client-Side Prediction](#client-side-prediction)
7. [Recovery Processes](#recovery-processes)
8. [Implementation Example](#implementation-example)

## Overview

In networked gameplay, unstable connections can lead to state inconsistencies between the server and clients. The state rollback system allows the game to:

1. Detect state deviations
2. Revert to a previous valid state
3. Deterministically replay actions to catch up
4. Resume normal play without disrupting the game flow

This approach is particularly important for turn-based games like MTG Commander where the integrity of game state is critical.

## Rollback Architecture

Our rollback architecture follows these principles:

1. **Server Authority**: The server maintains the authoritative game state
2. **State History**: Both server and clients maintain a history of game states
3. **Deterministic Replay**: Actions can be replayed deterministically to reconstruct state
4. **Input Buffering**: Client inputs are buffered to handle resynchronization
5. **Minimal Disruption**: Rollbacks should be as seamless as possible to players

### Component Integration

```rust
// src/networking/state/rollback.rs
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use crate::networking::server::resources::GameServer;
use crate::game_engine::state::GameState;

/// Plugin for handling state rollbacks in networked games
pub struct StateRollbackPlugin;

impl Plugin for StateRollbackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StateHistory>()
           .init_resource::<ClientInputBuffer>()
           .add_systems(Update, (
               create_state_snapshots,
               detect_state_deviations,
               handle_rollback_requests,
               apply_rollbacks,
           ));
    }
}
```

## State Snapshots

The core of our rollback system is the ability to capture and restore game state snapshots:

```rust
/// Resource for tracking game state history
#[derive(Resource)]
pub struct StateHistory {
    /// Timestamped state snapshots
    pub snapshots: Vec<StateSnapshot>,
    /// Maximum number of snapshots to retain
    pub max_snapshots: usize,
    /// Time between state snapshots (in seconds)
    pub snapshot_interval: f32,
    /// Last snapshot time
    pub last_snapshot_time: f32,
}

impl Default for StateHistory {
    fn default() -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots: 20, // Store up to 20 snapshots (~1 minute of gameplay at 3s intervals)
            snapshot_interval: 3.0, // Take a snapshot every 3 seconds
            last_snapshot_time: 0.0,
        }
    }
}

/// A complete snapshot of game state at a point in time
#[derive(Clone, Debug)]
pub struct StateSnapshot {
    /// Timestamp when this snapshot was created
    pub timestamp: f32,
    /// Unique sequence number
    pub sequence_id: u64,
    /// Serialized game state
    pub game_state: Vec<u8>,
    /// Serialized RNG state
    pub rng_state: Vec<u8>,
    /// Action sequence that led to this state
    pub action_sequence: Vec<ActionRecord>,
}
```

### Creating Snapshots

```rust
/// System to periodically create game state snapshots
pub fn create_state_snapshots(
    mut state_history: ResMut<StateHistory>,
    game_state: Res<GameState>,
    global_rng: Res<GlobalEntropy<WyRand>>,
    time: Res<Time>,
    sequence_tracker: Res<ActionSequence>,
) {
    // Check if it's time for a new snapshot
    if time.elapsed_seconds() - state_history.last_snapshot_time >= state_history.snapshot_interval {
        // Create new snapshot
        let snapshot = StateSnapshot {
            timestamp: time.elapsed_seconds(),
            sequence_id: sequence_tracker.current_sequence_id,
            game_state: serialize_game_state(&game_state),
            rng_state: global_rng.try_serialize_state().unwrap_or_default(),
            action_sequence: sequence_tracker.recent_actions.clone(),
        };
        
        // Add to history
        state_history.snapshots.push(snapshot);
        state_history.last_snapshot_time = time.elapsed_seconds();
        
        // Trim history if needed
        if state_history.snapshots.len() > state_history.max_snapshots {
            state_history.snapshots.remove(0);
        }
    }
}
```

## Deterministic Replay

To ensure consistent rollback behavior, all game actions must be deterministic and replayable:

```rust
/// Record of a game action for replay purposes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionRecord {
    /// Unique sequence ID for this action
    pub sequence_id: u64,
    /// Player who initiated the action
    pub player_id: Entity,
    /// Timestamp when the action occurred
    pub timestamp: f32,
    /// The actual action
    pub action: GameAction,
}

/// System to replay actions after a rollback
pub fn replay_actions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    rollback_event: Res<RollbackEvent>,
    actions: Vec<ActionRecord>,
) {
    // Restore the game state and RNG to the rollback point
    deserialize_game_state(&mut game_state, &rollback_event.snapshot.game_state);
    global_rng.deserialize_state(&rollback_event.snapshot.rng_state).expect("Failed to restore RNG state");
    
    // Replay all actions that occurred after the rollback point
    for action in actions {
        // Process each action in sequence
        apply_action(&mut commands, &mut game_state, &mut global_rng, action);
    }
}
```

## RNG Synchronization for Rollbacks

The RNG state is critical for deterministic rollbacks. We extend our existing RNG synchronization to support rollbacks:

```rust
/// Resource to track RNG snapshots for rollback
#[derive(Resource)]
pub struct RngSnapshotHistory {
    /// History of RNG states indexed by sequence ID
    pub snapshots: HashMap<u64, Vec<u8>>,
    /// Maximum number of RNG snapshots to keep
    pub max_snapshots: usize,
}

impl Default for RngSnapshotHistory {
    fn default() -> Self {
        Self {
            snapshots: HashMap::new(),
            max_snapshots: 100,
        }
    }
}

/// System to capture RNG state before randomized actions
pub fn capture_rng_before_randomized_action(
    sequence_tracker: Res<ActionSequence>,
    global_rng: Res<GlobalEntropy<WyRand>>,
    mut rng_history: ResMut<RngSnapshotHistory>,
) {
    // Save the current RNG state before a randomized action
    if let Some(serialized_state) = global_rng.try_serialize_state() {
        rng_history.snapshots.insert(sequence_tracker.current_sequence_id, serialized_state);
        
        // Clean up old snapshots if needed
        if rng_history.snapshots.len() > rng_history.max_snapshots {
            // Find and remove oldest snapshot
            if let Some(oldest_key) = rng_history.snapshots.keys()
                .min()
                .copied() {
                rng_history.snapshots.remove(&oldest_key);
            }
        }
    }
}
```

## Client-Side Prediction

To minimize the perception of network issues, clients can implement prediction:

```rust
/// Resource to track client-side prediction state
#[derive(Resource)]
pub struct PredictionState {
    /// Actions predicted but not yet confirmed
    pub pending_actions: Vec<ActionRecord>,
    /// Whether prediction is currently active
    pub is_predicting: bool,
    /// Last confirmed server sequence ID
    pub last_confirmed_sequence: u64,
}

/// System to apply client-side prediction
pub fn apply_client_prediction(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut prediction: ResMut<PredictionState>,
    input: Res<Input<KeyCode>>,
    client: Res<GameClient>,
) {
    // Only predict for local player actions
    if let Some(local_player) = client.local_player {
        // Check if a new action was input
        if input.just_pressed(KeyCode::Space) {
            // Example: Predict a "pass turn" action
            let action = GameAction::PassTurn { player: local_player };
            
            // Apply prediction locally
            apply_action_local(&mut commands, &mut game_state, action.clone());
            
            // Record the prediction
            prediction.pending_actions.push(ActionRecord {
                sequence_id: prediction.last_confirmed_sequence + prediction.pending_actions.len() as u64 + 1,
                player_id: local_player,
                timestamp: 0.0, // Will be filled by server
                action,
            });
            
            // Send to server
            // ...
        }
    }
}
```

## Recovery Processes

When a network issue is detected, the recovery process begins:

```rust
/// Event triggered when a rollback is needed
#[derive(Event)]
pub struct RollbackEvent {
    /// The snapshot to roll back to
    pub snapshot: StateSnapshot,
    /// Reason for the rollback
    pub reason: RollbackReason,
    /// Clients affected by this rollback
    pub affected_clients: Vec<ClientId>,
}

/// Reasons for triggering a rollback
#[derive(Debug, Clone, Copy)]
pub enum RollbackReason {
    /// State divergence detected
    StateDivergence,
    /// Client reconnected after disconnect
    ClientReconnection,
    /// Server-forced rollback
    ServerForced,
    /// Desync in randomized outcome
    RandomizationDesync,
}

/// System to handle client reconnection with state recovery
pub fn handle_client_reconnection(
    mut commands: Commands,
    mut server: ResMut<GameServer>,
    mut server_events: EventReader<ServerEvent>,
    state_history: Res<StateHistory>,
    mut rollback_events: EventWriter<RollbackEvent>,
    client_states: Res<ClientStateTracker>,
) {
    for event in server_events.read() {
        if let ServerEvent::ClientConnected { client_id } = event {
            // Check if this is a reconnection
            if let Some(player_entity) = server.client_player_map.get(client_id) {
                // Find last known state for this client
                if let Some(last_known_sequence) = client_states.get_last_sequence(*client_id) {
                    // Find appropriate snapshot to roll back to
                    if let Some(snapshot) = find_appropriate_snapshot(&state_history, last_known_sequence) {
                        // Trigger rollback just for this client
                        rollback_events.send(RollbackEvent {
                            snapshot: snapshot.clone(),
                            reason: RollbackReason::ClientReconnection,
                            affected_clients: vec![*client_id],
                        });
                    }
                }
            }
        }
    }
}
```

## Implementation Example

### Complete Rollback Process

This example shows a complete rollback process after detecting a state divergence:

```rust
/// System to detect and handle state divergences
pub fn detect_state_divergences(
    mut commands: Commands,
    mut state_checksums: EventReader<StateChecksumEvent>,
    state_history: Res<StateHistory>,
    server: Option<Res<GameServer>>,
    mut rollback_events: EventWriter<RollbackEvent>,
) {
    // Only run on server
    if server.is_none() {
        return;
    }
    
    for checksum_event in state_checksums.read() {
        // Compare client checksum with server's expected checksum
        if checksum_event.client_checksum != checksum_event.expected_checksum {
            info!("State divergence detected for client {:?} at sequence {}",
                  checksum_event.client_id, checksum_event.sequence_id);
            
            // Find appropriate snapshot to roll back to
            if let Some(snapshot) = find_rollback_snapshot(&state_history, checksum_event.sequence_id) {
                // Trigger rollback for the affected client
                rollback_events.send(RollbackEvent {
                    snapshot: snapshot.clone(),
                    reason: RollbackReason::StateDivergence,
                    affected_clients: vec![checksum_event.client_id],
                });
                
                // Log the rollback event
                info!("Initiating rollback to sequence {} for client {:?}",
                      snapshot.sequence_id, checksum_event.client_id);
            }
        }
    }
}

/// Find an appropriate snapshot for rollback
fn find_rollback_snapshot(history: &StateHistory, divergence_sequence: u64) -> Option<&StateSnapshot> {
    // Find the most recent snapshot before the divergence
    history.snapshots
        .iter()
        .rev()
        .find(|snapshot| snapshot.sequence_id < divergence_sequence)
}

/// Apply a rollback
pub fn apply_rollback(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut rollback_events: EventReader<RollbackEvent>,
    action_history: Res<ActionHistory>,
) {
    for event in rollback_events.read() {
        // 1. Restore game state from snapshot
        deserialize_game_state(&mut game_state, &event.snapshot.game_state);
        
        // 2. Restore RNG state
        global_rng.deserialize_state(&event.snapshot.rng_state)
            .expect("Failed to restore RNG state");
        
        // 3. Find actions that need to be replayed
        let actions_to_replay = action_history.get_actions_after(event.snapshot.sequence_id);
        
        // 4. Replay actions
        for action in actions_to_replay {
            apply_action(&mut commands, &mut game_state, &mut global_rng, action.clone());
        }
        
        // 5. Notify clients of the rollback
        for client_id in &event.affected_clients {
            commands.add(SendRollbackNotification {
                client_id: *client_id,
                snapshot: event.snapshot.clone(),
                reason: event.reason,
            });
        }
    }
}
```

### Handling Randomized Actions During Rollback

Special consideration for randomized actions like card shuffling:

```rust
/// Apply an action during rollback replay
fn apply_action(
    commands: &mut Commands,
    game_state: &mut GameState,
    global_rng: &mut GlobalEntropy<WyRand>,
    action: ActionRecord,
) {
    match &action.action {
        GameAction::ShuffleLibrary { player, library } => {
            // For randomized actions, we need to ensure deterministic outcomes
            if let Ok(mut player_rng) = players.get_mut(action.player_id) {
                // Important: Use the RNG in a consistent way
                let mut library_entity = *library;
                let mut library_comp = game_state.get_library_mut(library_entity);
                
                // Deterministic shuffle using the player's RNG component
                library_comp.shuffle_with_rng(&mut player_rng.rng);
            }
        },
        GameAction::FlipCoin { player } => {
            // Another example of randomized action
            if let Ok(mut player_rng) = players.get_mut(action.player_id) {
                // The random result will be the same as the original action
                // if the RNG state is properly restored
                let result = player_rng.rng.gen_bool(0.5);
                
                // Apply the result
                game_state.record_coin_flip(*player, result);
            }
        },
        // Handle other action types
        _ => {
            // Apply non-randomized actions normally
            game_state.apply_action(&action.action);
        }
    }
}
```

## Real-World Considerations

In practice, a rollback system needs to balance several considerations:

1. **Snapshot Frequency**: More frequent snapshots use more memory but allow more precise rollbacks
2. **Rollback Visibility**: How visible should rollbacks be to players?
3. **Partial vs. Full Rollbacks**: Sometimes only a portion of the state needs rollback
4. **Action Batching**: Batch multiple actions to minimize rollback frequency
5. **Bandwidth Costs**: State synchronization requires bandwidth - optimize it

### Optimizing for MTG Commander

For MTG Commander specifically:

1. Take snapshots at natural game boundaries (turn changes, phase changes)
2. Use incremental state updates between major decision points
3. Maintain separate RNG state for "hidden information" actions like shuffling
4. Prioritize server authority for rule enforcement and dispute resolution
5. Enable client prediction for responsive UI during network hiccups 