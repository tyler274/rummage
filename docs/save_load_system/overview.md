# Save/Load System Overview

This document provides a high-level overview of the save/load system in Rummage, explaining its core concepts and usage.

## Basic Usage

### Saving a Game

To save the current game state, send a `SaveGameEvent`:

```rust
// Save to a specific slot
world.send_event(SaveGameEvent {
    slot_name: "my_save".to_string(),
});
```

The save system will:
1. Collect all necessary game state information
2. Serialize game zones, cards, and commanders
3. Use `bevy_persistent` to persist the game state
4. Write it to the designated save file
5. Update metadata with information about the save

### Loading a Game

To load a previously saved game, send a `LoadGameEvent`:

```rust
// Load from a specific slot
world.send_event(LoadGameEvent {
    slot_name: "my_save".to_string(),
});
```

The load system will:
1. Use `bevy_persistent` to load the saved game state
2. Deserialize the game state data
3. Recreate all necessary entities and resources
4. Restore the game state, zone contents, and commander data
5. Restore all entity relationships and card positions

### Automatic Saving

The system includes an automatic save feature that triggers during state-based action checks:

```rust
// Configure auto-save behavior
commands.insert_resource(SaveConfig {
    save_directory: PathBuf::from("saves"),
    auto_save_enabled: true,
    auto_save_frequency: 10, // Save every 10 state-based action checks
});
```

## Saved Data

The save system captures and restores the following data:

1. **Game State**: Turn number, active player, priority holder, turn order, etc.
2. **Player Data**: Life totals, mana pools, and other player-specific information
3. **Zone Data**: Contents of all game zones (libraries, hands, battlefield, graveyard, etc.)
4. **Card Positions**: Where each card is located in the game state
5. **Commander Information**: Commander assignments, cast counts, and zone locations

## Replay Functionality

The replay system allows stepping through a saved game:

```rust
// Start a replay from a save file
world.send_event(StartReplayEvent {
    slot_name: "my_save".to_string(),
});

// Step forward in the replay (multiple steps possible)
world.send_event(StepReplayEvent { steps: 1 });

// Stop the current replay
world.send_event(StopReplayEvent);
```

During replay, the system:
1. Loads the initial game state
2. Applies recorded actions in sequence
3. Updates the visual state of the game
4. Allows stepping forward at the user's pace

## Save Metadata

The system maintains metadata about all saves in the `SaveMetadata` resource using `bevy_persistent`:

```rust
// Access save metadata
fn display_saves_system(save_metadata: Res<Persistent<SaveMetadata>>) {
    for save in &save_metadata.saves {
        println!("Save: {}, Turn: {}, Time: {}", 
            save.slot_name, 
            save.turn_number, 
            save.timestamp);
    }
}
```

## Configuration

The save system can be configured via the `SaveConfig` resource:

```rust
let config = SaveConfig {
    // Directory where save files are stored
    save_directory: PathBuf::from("custom_saves"),
    
    // Whether auto-save is enabled
    auto_save_enabled: true,
    
    // How often auto-saves occur (in state-based action checks)
    auto_save_frequency: 20,
};
```

## Entity Serialization

The save/load system handles entity references by converting them to indices during serialization and rebuilding entities during deserialization. This preserves all relationships between entities despite the fact that entity IDs will change between sessions.

## Next Steps

- See [Implementation](implementation.md) for technical details
- Check the [API Reference](api_reference.md) for a complete list of types and functions
- Look at [Testing](testing.md) for how to test save/load functionality 