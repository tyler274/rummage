# Save/Load System

The save/load system provides functionality to persist and restore game state in the MTG Commander game engine. This system uses binary serialization via `bincode` for efficient and compact save files.

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

Save files are stored as binary files with the `.bin` extension using the `bincode` serialization library. This format is:
- Space-efficient
- Fast to serialize/deserialize
- Not human-readable (unlike JSON)

The save metadata is also stored in binary format at `saves/metadata.bin`.

## Usage Examples

### Manual Save

```rust
fn save_button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    mut save_event: EventWriter<SaveGameEvent>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            save_event.send(SaveGameEvent {
                slot_name: "manual_save".to_string(),
            });
        }
    }
}
```

### Loading a Save

```rust
fn load_button_system(
    mut interaction_query: Query<(&Interaction, &SaveSlot), (Changed<Interaction>, With<LoadButton>)>,
    mut load_event: EventWriter<LoadGameEvent>,
) {
    for (interaction, save_slot) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            load_event.send(LoadGameEvent {
                slot_name: save_slot.name.clone(),
            });
        }
    }
}
```

### Displaying Save Metadata

```rust
fn display_saves_system(
    save_metadata: Res<Persistent<SaveMetadata>>,
    mut query: Query<&mut Text, With<SaveListText>>,
) {
    for mut text in &mut query {
        let mut save_text = String::new();
        
        for save in &save_metadata.saves {
            let time = chrono::DateTime::<chrono::Utc>::from_timestamp(save.timestamp as i64, 0)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S");
                
            save_text.push_str(&format!(
                "Save: {}\nTime: {}\nTurn: {}\nPlayers: {}\n\n",
                save.slot_name, time, save.turn_number, save.player_count
            ));
        }
        
        text.sections[0].value = save_text;
    }
}
``` 