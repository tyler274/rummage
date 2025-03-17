# Save/Load System

The save/load system provides functionality to persist and restore game state in the MTG Commander game engine. This system uses `bevy_persistent` for efficient and reliable persistence of game state.

## Components

### Plugin

The `SaveLoadPlugin` handles initialization and registration of necessary systems for the save/load functionality:

```rust
// To add save/load functionality to your app
app.add_plugins(SaveLoadPlugin);
```

### Events

Two primary events drive the save/load system:

- `SaveGameEvent`: Triggers saving the game state to a specified slot
- `LoadGameEvent`: Triggers loading a game state from a specified slot

Example usage:

```rust
// To trigger a save
commands.event_writer::<SaveGameEvent>().send(SaveGameEvent {
    slot_name: "my_save".to_string(),
});

// To load a saved game
commands.event_writer::<LoadGameEvent>().send(LoadGameEvent {
    slot_name: "my_save".to_string(),
});
```

### Configuration

The `SaveConfig` resource provides configuration options for the save system:

```rust
// Example of customizing save configuration
commands.insert_resource(SaveConfig {
    save_directory: PathBuf::from("custom_saves"),
    auto_save_enabled: true,
    auto_save_frequency: 20, // Auto-save every 20 state-based action checks
});
```

Key configuration options:
- `save_directory`: Path where save files will be stored
- `auto_save_enabled`: Whether automatic saving is enabled
- `auto_save_frequency`: How often auto-saves occur (in state-based action checks)

## Auto-Save System

The save system includes automatic saving functionality, which:

1. Increments a counter each time state-based actions are checked
2. When the counter reaches the configured threshold, creates an auto-save
3. Resets the counter after each auto-save

Auto-saves are stored in the "auto_save" slot by default.

## Save Data Structure

The system serializes game state into a `GameSaveData` structure which includes:

- `game_state`: Core game state information (turn number, active player, etc.)
- `players`: Player data including life totals and other player-specific info
- `zones`: Serialized zone contents (battlefield, graveyards, etc.)
- `commanders`: Commander-specific data
- `save_version`: Game version for compatibility checking

## Save Metadata

The system maintains metadata about all save files in a `SaveMetadata` resource, which includes:
- Slot names
- Timestamps
- Descriptions
- Turn numbers
- Player counts

This metadata is persisted using `bevy_persistent` and is automatically kept in sync when saves are created.

## Implementation Notes

### Entity Mapping

Since Bevy entities are not stable across game sessions, the save system:
1. Converts entity references to indices during saving
2. Recreates entities during loading
3. Maps indices back to entities for reconstructing relationships

### Save File Format

Save files are stored using `bevy_persistent` with the `Bincode` format. This provides:
- Space-efficient storage
- Fast serialization/deserialization
- Automatic handling of save/load operations

## Usage Examples

### Manual Save

```rust
// Trigger a manual save
world.send_event(SaveGameEvent {
    slot_name: "my_checkpoint".to_string(),
});
```

### Loading a Game

```rust
// Load from a saved slot
world.send_event(LoadGameEvent {
    slot_name: "my_checkpoint".to_string(),
});
```

### Accessing Save Metadata

```rust
// Display available saves
fn display_saves(save_metadata: Res<Persistent<SaveMetadata>>) {
    for save in &save_metadata.saves {
        info!(
            "Save: {}, Turn: {}, Players: {}",
            save.slot_name, save.turn_number, save.player_count
        );
    }
}
```

### Creating and Using Snapshots

```rust
// Creating a snapshot of the current state
fn create_game_snapshot(mut commands: Commands, game_state: Res<GameState>) {
    let snapshot = Persistent::<GameSaveData>::builder()
        .name("snapshot")
        .format(StorageFormat::Bincode)
        .path("user://snapshots/latest.save")
        .default(GameSaveData::default())
        .build()
        .expect("Failed to create snapshot");
    
    // Set snapshot data from current state and persist
    // actual implementation would convert game state to save data
    if let Err(e) = snapshot.persist() {
        error!("Failed to save snapshot: {}", e);
    }
}
``` 