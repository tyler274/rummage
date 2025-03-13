# RNG Synchronization and Rollback Integration

This document explains how the random number generator (RNG) synchronization system integrates with the state rollback mechanism to maintain consistent game state across network disruptions.

## Table of Contents

1. [Overview](#overview)
2. [Challenges](#challenges)
3. [Integration Architecture](#integration-architecture)
4. [Implementation Details](#implementation-details)
5. [Example Scenarios](#example-scenarios)
6. [Performance Considerations](#performance-considerations)

## Overview

Card games like Magic: The Gathering require randomization for shuffling libraries, coin flips, dice rolls, and "random target" selection. In a networked environment, these random operations must produce identical results across all clients despite network disruptions. Our solution combines the RNG synchronization system with the state rollback mechanism to ensure consistent gameplay.

## Challenges

Several challenges must be addressed:

1. **Deterministic Recovery**: After a network disruption, random operations must produce the same results as before
2. **Hidden Information**: RNG state must be preserved without revealing hidden information (like library order)
3. **Partial Rollbacks**: Some clients may need to roll back while others do not
4. **Performance**: RNG state serialization and transmission must be efficient
5. **Cheating Prevention**: The system must prevent manipulation of random outcomes

## Integration Architecture

The integration of RNG synchronization with the rollback system follows this architecture:

```
┌────────────────────┐     ┌───────────────────┐     ┌────────────────────┐
│                    │     │                   │     │                    │
│  RNG STATE SYSTEM  │◄────┤  ROLLBACK SYSTEM  │────►│  GAME STATE SYSTEM │
│                    │     │                   │     │                    │
└───────┬────────────┘     └─────────┬─────────┘     └─────────┬──────────┘
        │                            │                         │
        │                            │                         │
        ▼                            ▼                         ▼
┌────────────────────┐     ┌───────────────────┐     ┌────────────────────┐
│                    │     │                   │     │                    │
│  HISTORY TRACKER   │◄────┤ SEQUENCE TRACKER  │────►│  ACTION PROCESSOR  │
│                    │     │                   │     │                    │
└────────────────────┘     └───────────────────┘     └────────────────────┘
```

### Key Components

1. **RNG State System**: Manages the global and player-specific RNG states
2. **Rollback System**: Handles state rollbacks due to network disruptions
3. **Game State System**: Maintains the authoritative game state
4. **History Tracker**: Records RNG states at key sequence points
5. **Sequence Tracker**: Assigns sequence IDs to game actions
6. **Action Processor**: Applies game actions deterministically

## Implementation Details

### RNG State Snapshots

For every game state snapshot, we also capture the corresponding RNG state:

```rust
/// System to capture RNG state with game state snapshots
pub fn capture_rng_with_state_snapshot(
    mut state_history: ResMut<StateHistory>,
    global_rng: Res<GlobalEntropy<WyRand>>,
    players: Query<(Entity, &PlayerRng)>,
) {
    if let Some(current_snapshot) = state_history.snapshots.last_mut() {
        // Save global RNG state
        if let Some(global_state) = global_rng.try_serialize_state() {
            current_snapshot.rng_state = global_state;
        }
        
        // Save player-specific RNG states
        let mut player_rng_states = HashMap::new();
        for (entity, player_rng) in players.iter() {
            if let Some(state) = player_rng.rng.try_serialize_state() {
                player_rng_states.insert(entity, state);
            }
        }
        current_snapshot.player_rng_states = player_rng_states;
    }
}
```

### Randomized Action Handling

Randomized actions require special attention during rollbacks:

```rust
/// Apply a randomized action with RNG state consistency
pub fn apply_randomized_action(
    action: &GameAction,
    rng_state: Option<&[u8]>,
    global_rng: &mut GlobalEntropy<WyRand>,
    player_rngs: &mut Query<&mut PlayerRng>,
) -> ActionResult {
    // If we have a saved RNG state for this action, restore it first
    if let Some(state) = rng_state {
        global_rng.deserialize_state(state).expect("Failed to restore RNG state");
    }
    
    match action {
        GameAction::ShuffleLibrary { player, library } => {
            // Get the player's RNG component
            if let Ok(mut player_rng) = player_rngs.get_mut(*player) {
                // Perform the shuffle using the player's RNG
                // This will produce the same result if the RNG state is the same
                // ...
                
                ActionResult::Success
            } else {
                ActionResult::PlayerNotFound
            }
        },
        // Handle other randomized actions...
        _ => ActionResult::NotRandomized,
    }
}
```

### Rollback With RNG Recovery

During a rollback, both game state and RNG state are restored:

```rust
/// System to perform a rollback with RNG state recovery
pub fn perform_rollback_with_rng(
    mut game_state: ResMut<GameState>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut player_rngs: Query<(Entity, &mut PlayerRng)>,
    rollback_event: Res<RollbackEvent>,
) {
    // Restore game state
    deserialize_game_state(&mut game_state, &rollback_event.snapshot.game_state);
    
    // Restore global RNG state
    global_rng.deserialize_state(&rollback_event.snapshot.rng_state)
        .expect("Failed to restore global RNG state");
    
    // Restore player-specific RNG states
    for (entity, mut player_rng) in player_rngs.iter_mut() {
        if let Some(state) = rollback_event.snapshot.player_rng_states.get(&entity) {
            player_rng.rng.deserialize_state(state)
                .expect("Failed to restore player RNG state");
        }
    }
    
    // Log the rollback
    info!("Performed rollback to sequence {} with RNG state recovery", 
          rollback_event.snapshot.sequence_id);
}
```

## Example Scenarios

### Scenario 1: Library Shuffle During Network Disruption

1. Player A initiates a library shuffle
2. Network disruption occurs during processing
3. System detects the disruption and initiates rollback
4. RNG state from before the shuffle is restored
5. Shuffle action is replayed with identical RNG state
6. All clients observe the same shuffle outcome despite the disruption

```rust
// Handling a shuffle during network disruption
pub fn handle_shuffle_during_disruption(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut player_rngs: Query<&mut PlayerRng>,
    mut rollback_events: EventReader<RollbackEvent>,
    action_history: Res<ActionHistory>,
) {
    for event in rollback_events.read() {
        // Restore game and RNG state
        perform_rollback_with_rng(&mut game_state, &mut global_rng, &mut player_rngs, event);
        
        // Get actions to replay
        let actions = action_history.get_actions_after(event.snapshot.sequence_id);
        
        // Replay actions deterministically
        for action in actions {
            // If this is a shuffle action, it will produce the same result
            // because the RNG state has been restored
            apply_action(&mut commands, &mut game_state, &mut global_rng, &mut player_rngs, &action);
        }
    }
}
```

### Scenario 2: Client Reconnection After Multiple Random Events

1. Client disconnects during a game with several random actions
2. Random actions continue to occur (coin flips, shuffles, etc.)
3. Client reconnects after several actions
4. System identifies the last confirmed sequence point for the client
5. Rollback state is sent with corresponding RNG state
6. Client replays all actions, producing identical random results
7. Client's game state is now synchronized with the server

## Performance Considerations

Integrating RNG with rollbacks introduces performance considerations:

1. **RNG State Size**: RNG state serialization should be compact
   - WyRand RNG state is typically only 8 bytes
   - Avoid large RNG algorithms for frequent serialization

2. **Selective Snapshots**: Not every action needs RNG state preservation
   - Only save RNG state before randomized actions
   - Use sequence IDs to correlate RNG states with actions

3. **Batched Updates**: Group randomized actions to minimize state snapshots
   - Example: When shuffling multiple permanents, capture RNG once

4. **Compressed History**: Use a sliding window approach for history
   - Discard old RNG states when they're no longer needed
   - Keep only enough history for realistic rollback scenarios

5. **Optimized Serialization**: Use efficient binary serialization
   - Consider custom serialization for RNG state if needed
   - Avoid JSON or other verbose formats for RNG state

## Implementation Notes

1. The RNG synchronization system must be initialized before the rollback system
2. All random operations must use the synchronized RNG, never `thread_rng()` or other sources
3. Player-specific operations should use player-specific RNG components
4. RNG state should be included in regular network synchronization
5. Client reconnection should always include RNG state restoration 