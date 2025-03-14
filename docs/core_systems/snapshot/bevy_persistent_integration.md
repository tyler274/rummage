# State Persistence with bevy_persistent

This document details how to use `bevy_persistent` for robust game state persistence and rollback management in Rummage.

## Overview

Game state persistence and rollback functionality are critical for:

- **Save/Load**: Allowing players to save and resume their games
- **Undo Support**: Enabling players to revert mistakes
- **Checkpoints**: Creating automatic save points at important moments
- **Crash Recovery**: Recovering from crashes without losing progress
- **Replay**: Supporting replay functionality

`bevy_persistent` provides an elegant solution for these requirements by automatically handling serialization, deserialization, and file I/O.

## Integration with Snapshot System

The snapshot system can be enhanced with `bevy_persistent` to provide automatic, reliable state persistence:

```rust
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

/// A serializable snapshot of game state
#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct GameSnapshot {
    /// Unique identifier for the snapshot
    pub id: Uuid,
    /// The game turn the snapshot was taken on
    pub turn: u32,
    /// The phase within the turn
    pub phase: Phase,
    /// Active player when snapshot was taken
    pub active_player: usize,
    /// Serialized game entities
    pub entities: Vec<SerializedEntity>,
    /// Timestamp when the snapshot was created
    pub timestamp: f64,
}

/// Collection of game snapshots for rollback support
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct GameSnapshotCollection {
    /// Map of snapshot ID to snapshot data
    pub snapshots: HashMap<Uuid, GameSnapshot>,
    /// Order of snapshots (earliest to latest)
    pub history: Vec<Uuid>,
    /// Maximum number of snapshots to keep
    #[serde(skip)]
    pub max_snapshots: usize,
    /// Current snapshot ID (the active state)
    pub current_snapshot_id: Option<Uuid>,
}

// Set up persistent snapshot system in plugin
fn build_persistent_snapshots(app: &mut App) {
    // Create persistent snapshot collection
    let persistent_snapshots = Persistent::<GameSnapshotCollection>::builder()
        .name("game_snapshots")
        .format(StorageFormat::Bincode) // More efficient for large state
        .path("user://snapshots.bin")
        .default(GameSnapshotCollection {
            snapshots: HashMap::new(),
            history: Vec::new(),
            max_snapshots: 50, // Keep last 50 snapshots
            current_snapshot_id: None,
        })
        .build();

    app.insert_resource(persistent_snapshots)
        .add_systems(Update, auto_save_snapshots)
        .add_systems(Startup, load_snapshot_history);
}
```

## Creating Snapshots

The system for creating snapshots integrates with `bevy_persistent`:

```rust
/// Create a new game state snapshot
fn create_snapshot(
    world: &mut World,
    mut snapshots: ResMut<Persistent<GameSnapshotCollection>>,
) -> Uuid {
    // Get relevant game state information
    let turn = world.resource::<TurnManager>().current_turn;
    let phase = *world.resource::<Phase>();
    let active_player = world.resource::<TurnManager>().active_player;
    
    // Create snapshot ID
    let id = Uuid::new_v4();
    
    // Serialize entities
    let entities = serialize_game_entities(world);
    
    // Create snapshot
    let snapshot = GameSnapshot {
        id,
        turn,
        phase,
        active_player,
        entities,
        timestamp: world.resource::<Time>().elapsed_seconds_f64(),
    };
    
    // Add to collection
    snapshots.snapshots.insert(id, snapshot);
    snapshots.history.push(id);
    snapshots.current_snapshot_id = Some(id);
    
    // Prune old snapshots if needed
    prune_old_snapshots(&mut snapshots);
    
    // Trigger save
    if let Err(err) = snapshots.save() {
        error!("Failed to save game snapshot: {}", err);
    }
    
    id
}
```

## Loading and Applying Snapshots

To restore a game state from a snapshot:

```rust
/// Load a specific snapshot by ID
fn load_snapshot(
    world: &mut World,
    snapshot_id: Uuid,
) -> Result<(), String> {
    // Get snapshot collection
    let snapshots = world.resource::<Persistent<GameSnapshotCollection>>();
    
    // Find the snapshot
    let snapshot = match snapshots.snapshots.get(&snapshot_id) {
        Some(snapshot) => snapshot,
        None => return Err(format!("Snapshot with ID {} not found", snapshot_id)),
    };
    
    // Apply the snapshot to the world
    apply_snapshot_to_world(world, snapshot)?;
    
    // Update current snapshot ID
    let mut snapshots = world.resource_mut::<Persistent<GameSnapshotCollection>>();
    snapshots.current_snapshot_id = Some(snapshot_id);
    
    info!("Loaded snapshot from turn {} (ID: {})", snapshot.turn, snapshot_id);
    
    Ok(())
}

/// Apply a snapshot to the world
fn apply_snapshot_to_world(
    world: &mut World,
    snapshot: &GameSnapshot,
) -> Result<(), String> {
    // First, clean up existing entities that should be replaced
    let entities_to_despawn = get_entities_to_despawn(world);
    for entity in entities_to_despawn {
        world.despawn(entity);
    }
    
    // Restore serialized entities
    for serialized_entity in &snapshot.entities {
        deserialize_and_spawn_entity(world, serialized_entity)?;
    }
    
    // Restore global resources
    let mut turn_manager = world.resource_mut::<TurnManager>();
    turn_manager.current_turn = snapshot.turn;
    turn_manager.active_player = snapshot.active_player;
    
    let mut phase = world.resource_mut::<Phase>();
    *phase = snapshot.phase;
    
    Ok(())
}
```

## Automatic Checkpoint System

Game state can be automatically saved at key moments:

```rust
/// Create checkpoints at key moments in the game
fn checkpoint_system(
    mut commands: Commands,
    world: &mut World,
    mut snapshots: ResMut<Persistent<GameSnapshotCollection>>,
    turn_events: EventReader<TurnStartEvent>,
) {
    // Create checkpoints at the start of each turn
    for event in turn_events.iter() {
        info!("Creating checkpoint at start of turn {}", event.turn);
        let snapshot_id = create_snapshot(world, snapshots.reborrow());
        info!("Created checkpoint with ID: {}", snapshot_id);
    }
}
```

## Rollback System

The rollback system allows reverting to previous states:

```rust
/// Roll back to a previous state
fn rollback_system(
    mut commands: Commands,
    mut world: &mut World,
    rollback_events: EventReader<RollbackEvent>,
    mut snapshots: ResMut<Persistent<GameSnapshotCollection>>,
) {
    for event in rollback_events.iter() {
        match event {
            RollbackEvent::ToTurn(turn) => {
                // Find the snapshot for this turn
                if let Some(snapshot_id) = find_snapshot_for_turn(*turn, &snapshots) {
                    if let Err(err) = load_snapshot(world, snapshot_id) {
                        error!("Failed to roll back to turn {}: {}", turn, err);
                    } else {
                        info!("Successfully rolled back to turn {}", turn);
                    }
                } else {
                    error!("No snapshot found for turn {}", turn);
                }
            },
            RollbackEvent::ToPreviousTurn => {
                // Roll back one turn
                if let Some(current_id) = snapshots.current_snapshot_id {
                    if let Some(prev_id) = get_previous_snapshot_id(current_id, &snapshots) {
                        if let Err(err) = load_snapshot(world, prev_id) {
                            error!("Failed to roll back to previous turn: {}", err);
                        } else {
                            info!("Successfully rolled back to previous turn");
                        }
                    } else {
                        error!("No previous snapshot found");
                    }
                }
            },
            // Other rollback types...
        }
    }
}
```

## Save/Load Game Interface

A user interface for saving and loading games:

```rust
/// Save the current game with a custom name
fn save_game(
    world: &mut World,
    name: &str,
) -> Result<(), String> {
    // Create a snapshot of the current state
    let snapshots = world.resource_mut::<Persistent<GameSnapshotCollection>>();
    let snapshot_id = create_snapshot(world, snapshots);
    
    // Create a named save file
    let save_data = Persistent::<NamedGameSave>::builder()
        .name(name)
        .format(StorageFormat::Bincode)
        .path(format!("user://saves/{}.save", name))
        .default(NamedGameSave {
            name: name.to_string(),
            snapshot_id,
            created_at: chrono::Utc::now(),
            game_info: extract_game_info(world),
        })
        .build();
    
    // Save the file
    if let Err(err) = save_data.save() {
        return Err(format!("Failed to save game: {}", err));
    }
    
    info!("Game saved successfully as '{}'", name);
    Ok(())
}

/// Load a saved game by name
fn load_game(
    world: &mut World,
    name: &str,
) -> Result<(), String> {
    // Load the save file
    let save_data = Persistent::<NamedGameSave>::builder()
        .name(name)
        .format(StorageFormat::Bincode)
        .path(format!("user://saves/{}.save", name))
        .build();
    
    if let Err(err) = save_data.load() {
        return Err(format!("Failed to load save file '{}': {}", name, err));
    }
    
    // Load the snapshot
    load_snapshot(world, save_data.snapshot_id)
}
```

## Crash Recovery

Automatic recovery from crashes using persistent snapshots:

```rust
/// System to check for and recover from crashes
fn crash_recovery_system(
    world: &mut World,
    mut snapshots: ResMut<Persistent<GameSnapshotCollection>>,
) {
    // Check for crash indicator file
    if std::path::Path::new("user://crash_indicator").exists() {
        warn!("Detected previous crash, attempting recovery");
        
        // Try to load snapshots
        if let Err(err) = snapshots.load() {
            error!("Failed to load snapshots during crash recovery: {}", err);
            return;
        }
        
        // Find the most recent valid snapshot
        if let Some(latest_id) = snapshots.history.last().copied() {
            info!("Recovering from snapshot {}", latest_id);
            if let Err(err) = load_snapshot(world, latest_id) {
                error!("Failed to recover from crash: {}", err);
            } else {
                info!("Successfully recovered from crash");
            }
        } else {
            error!("No snapshots available for crash recovery");
        }
        
        // Remove crash indicator
        if let Err(err) = std::fs::remove_file("user://crash_indicator") {
            error!("Failed to remove crash indicator: {}", err);
        }
    }
    
    // Create crash indicator file
    if let Err(err) = std::fs::write("user://crash_indicator", "1") {
        error!("Failed to create crash indicator: {}", err);
    }
}
```

## Hot Reload Support

`bevy_persistent` supports hot reloading for development and testing:

```rust
/// Enable hot reloading of snapshots during development
fn setup_hot_reload(
    app: &mut App,
    snapshots: Persistent<GameSnapshotCollection>,
) {
    #[cfg(debug_assertions)]
    {
        let snapshot_path = snapshots.path().unwrap().to_path_buf();
        app.add_systems(Update, move |world: &mut World| {
            // Check if the file has been modified
            if snapshot_path_modified(&snapshot_path) {
                info!("Detected external changes to snapshot file, reloading");
                let mut snapshots = world.resource_mut::<Persistent<GameSnapshotCollection>>();
                if let Err(err) = snapshots.load() {
                    error!("Failed to hot reload snapshots: {}", err);
                } else {
                    info!("Successfully hot reloaded snapshots");
                }
            }
        });
    }
}
```

## Benefits of bevy_persistent for State Management

Using `bevy_persistent` for state management offers several key advantages:

1. **Atomicity**: Save operations are atomic, reducing the risk of corruption
2. **Error Handling**: Comprehensive error handling for all I/O operations
3. **Versioning**: Support for schema versioning when state structure changes
4. **Format Flexibility**: Support for multiple serialization formats
5. **Hot Reloading**: Ability to detect and reload changes at runtime
6. **Cross-Platform**: Works consistently across all supported platforms
7. **Performance**: Efficient serialization with bincode for large states

## Related Documentation

- [Snapshot Overview](overview.md): Introduction to the snapshot system
- [Implementation](implementation.md): Technical details of snapshot implementation
- [Deck Persistence](../../card_systems/deck_database/persistent_storage.md): Using bevy_persistent for deck storage 