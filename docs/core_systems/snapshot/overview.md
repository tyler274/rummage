# Snapshot System Overview

## Introduction

A snapshot in Rummage is a serializable representation of the game state at a specific point in time. It captures all the information needed to reproduce the exact state of the game, including cards, player data, and game progress.

## Core Concepts

### What is a Snapshot?

A snapshot captures:

- **Entities**: Game objects like cards, players, and zones
- **Components**: Properties and state associated with entities
- **Resources**: Global game state and configuration
- **Relationships**: Connections between entities (e.g., a card in a zone)

### When Are Snapshots Created?

Snapshots can be created:

- **On Turn Change**: Capture state at the beginning of each turn
- **On Phase Change**: Record state at critical phase transitions
- **On Demand**: Manually triggered for testing or replay purposes
- **Before Network Updates**: Prior to sending state updates to clients
- **At Save Points**: When a user wants to save game progress

### Snapshot Lifecycle

The typical lifecycle of a snapshot is:

1. **Creation**: Game state is captured and serialized
2. **Storage**: The snapshot is stored in memory or on disk
3. **Processing**: Various systems may analyze or transform the snapshot
4. **Application**: The snapshot is used to restore game state (for replay, rollback, etc.)
5. **Disposal**: Old snapshots are removed when no longer needed

## Technical Details

### Core Types

The main types in the snapshot system are:

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

### Serialization Strategy

The snapshot system uses a selective serialization strategy:

1. **Marker Components**: Only entities with `Snapshotable` components are included
2. **Component Filtering**: Only necessary components are serialized
3. **Binary Encoding**: Data is encoded efficiently to minimize size
4. **ID Mapping**: Entity IDs are mapped to ensure consistency across sessions

## Use Cases

### Networked Multiplayer

In multiplayer games, snapshots are used to:

- Synchronize game state between clients
- Verify state consistency across the network
- Handle player disconnections and reconnections
- Provide authoritative state for conflict resolution

### Replay System

For game replays, snapshots enable:

- Recording complete game history
- Navigating back and forth through game turns
- Analyzing gameplay decisions
- Sharing interesting game states

### Save/Load Functionality

Snapshots make save/load possible by:

- Capturing all necessary data to resume play
- Creating portable save files
- Supporting different save points within a game
- Ensuring version compatibility

### Testing

For testing purposes, snapshots allow:

- Creating reproducible test scenarios
- Verifying state transitions
- Validating rule implementations
- Comparing expected vs. actual outcomes

## Next Steps

For more detailed information, continue to:

- **[Implementation](implementation.md)**: The technical implementation details
- **[Integration with Networking](networking_integration.md)**: How snapshots work with multiplayer
- **[Testing](testing.md)**: How to test snapshot functionality
- **[API Reference](api_reference.md)**: Complete API documentation 