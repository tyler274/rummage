# Save/Load System Implementation

This document provides technical details about the save/load system implementation in Rummage.

## Architecture

The save/load system consists of several interconnected components:

1. **Plugin**: `SaveLoadPlugin` handles registration of all events, resources, and systems.
2. **Events**: Events like `SaveGameEvent` and `LoadGameEvent` trigger save and load operations.
3. **Resources**: Configuration and state tracking resources like `SaveConfig` and `ReplayState`.
4. **Data Structures**: Serializable data representations in the `data.rs` module.
5. **Systems**: Bevy systems for handling operations defined in `systems.rs`.

## Data Model

The save/load system uses a comprehensive data model to capture the game state:

### GameSaveData

The main structure that holds all serialized game data:

```rust
pub struct GameSaveData {
    pub game_state: GameStateData,
    pub players: Vec<PlayerData>,
    pub zones: ZoneData,
    pub commanders: CommanderData,
    pub save_version: String,
}
```

### GameStateData

Core game state information:

```rust
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
    pub lands_played: Vec<(usize, u32)>,
    pub main_phase_action_taken: bool,
    pub drawn_this_turn: Vec<usize>,
    pub eliminated_players: Vec<usize>,
    pub use_commander_damage: bool,
    pub commander_damage_threshold: u32,
    pub starting_life: i32,
}
```

### PlayerData

Player-specific information:

```rust
pub struct PlayerData {
    pub id: usize,
    pub name: String,
    pub life: i32,
    pub mana_pool: ManaPool,
    pub player_index: usize,
}
```

### ZoneData

Information about card zones and contents:

```rust
pub struct ZoneData {
    // Maps player indices to their libraries
    pub libraries: std::collections::HashMap<usize, Vec<usize>>,
    // Maps player indices to their hands
    pub hands: std::collections::HashMap<usize, Vec<usize>>,
    // Shared battlefield
    pub battlefield: Vec<usize>,
    // Maps player indices to their graveyards
    pub graveyards: std::collections::HashMap<usize, Vec<usize>>,
    // Shared exile zone
    pub exile: Vec<usize>,
    // Command zone
    pub command_zone: Vec<usize>,
    // Maps card indices to their current zone
    pub card_zone_map: std::collections::HashMap<usize, Zone>,
}
```

### CommanderData

Commander-specific data:

```rust
pub struct CommanderData {
    // Maps player indices to their commander indices
    pub player_commanders: std::collections::HashMap<usize, Vec<usize>>,
    // Maps commander indices to their current zone
    pub commander_zone_status: std::collections::HashMap<usize, CommanderZoneLocation>,
    // Tracks how many times a commander has moved zones
    pub zone_transition_count: std::collections::HashMap<usize, u32>,
}
```

## bevy_persistent Integration

The save/load system uses `bevy_persistent` for robust persistence. This implementation provides:

1. **Format Selection**: Currently uses `Bincode` for efficient binary serialization.
2. **Path Selection**: Appropriate paths based on platform (native or web) and user configuration.
3. **Error Handling**: Robust handling of failures during save/load operations with graceful fallbacks.
4. **Resource Management**: Automatic resource persistence and loading.

Example integration from the `setup_save_system` function with improved error handling:

```rust
// Create save directory if it doesn't exist
let config = SaveConfig::default();

// Only try to create directory on native platforms
#[cfg(not(target_arch = "wasm32"))]
if let Err(e) = std::fs::create_dir_all(&config.save_directory) {
    error!("Failed to create save directory: {}", e);
    // Continue anyway - the directory might already exist
}

// Determine the appropriate base path for persistence based on platform
let metadata_path = get_storage_path(&config, "metadata.bin");

// Initialize persistent save metadata with fallback options
let save_metadata = match Persistent::builder()
    .name("save_metadata")
    .format(StorageFormat::Bincode)
    .path(metadata_path)
    .default(SaveMetadata::default())
    .build()
{
    Ok(metadata) => metadata,
    Err(e) => {
        error!("Failed to create persistent save metadata: {}", e);
        // Create a fallback in-memory resource
        Persistent::builder()
            .name("save_metadata")
            .format(StorageFormat::Bincode)
            .path(PathBuf::from("metadata.bin"))
            .default(SaveMetadata::default())
            .build()
            .expect("Failed to create even basic metadata")
    }
};

commands.insert_resource(config.clone());
commands.insert_resource(save_metadata);
```

## Configuration

The save system is configured through the `SaveConfig` resource:

```rust
#[derive(Resource, Clone, Debug)]
pub struct SaveConfig {
    pub save_directory: PathBuf,
    pub auto_save_enabled: bool,
    pub auto_save_frequency: usize,
}
```

This resource allows customizing:
- The directory where save files are stored
- Whether auto-saving is enabled
- How frequently auto-saves occur

## Entity Mapping

One of the challenges in serializing Bevy's ECS is handling entity references. The save/load system solves this by:

1. **During Save**: Converting entity references to indices using a mapping
2. **During Load**: Recreating entities and building a reverse mapping
3. **After Load**: Reconstructing relationships using the new entity handles

This approach ensures entity references remain valid across save/load cycles, even though the actual entity IDs change.

## Replay System

The replay system extends save/load functionality by:

1. Loading a saved game state
2. Recording actions in a `ReplayAction` queue
3. Allowing step-by-step playback of recorded actions
4. Providing controls to start, step through, and stop replays

## Error Handling

The save/load system employs several error handling strategies:

1. **Corrupted Data**: Graceful handling of corrupted saves with fallbacks to default values
2. **Missing Entities**: Safe handling when mapped entities don't exist, including placeholder entities when needed
3. **Empty Player Lists**: Special handling for saves with no players, preserving game state data
4. **Version Compatibility**: Checking save version compatibility
5. **File System Errors**: Robust handling of IO and persistence errors with appropriate error messages
6. **Directory Creation**: Automatic creation of save directories with error handling and verification
7. **Save Verification**: Verification that save files were actually created with appropriate delays
8. **Filesystem Synchronization**: Added delays to ensure filesystem operations complete before verification

Example of handling corrupted entity mappings:

```rust
// If there's a corrupted mapping, fall back to basic properties
if index_to_entity.is_empty() || index_to_entity.contains(&Entity::PLACEHOLDER) {
    // At minimum, restore basic properties not tied to player entities
    game_state.turn_number = save_data.game_state.turn_number;
    
    // For empty player list, set reasonable defaults for player-related fields
    if save_data.game_state.turn_order_indices.is_empty() {
        // Create a fallback turn order
        game_state.turn_order = VecDeque::new();
    }
} else {
    // Full restore with valid player entities
    **game_state = save_data.to_game_state(&index_to_entity);
}
```

Example of improved directory creation and save verification:

```rust
// Ensure save directory exists for native platforms
#[cfg(not(target_arch = "wasm32"))]
{
    if !config.save_directory.exists() {
        match std::fs::create_dir_all(&config.save_directory) {
            Ok(_) => info!("Created save directory: {:?}", config.save_directory),
            Err(e) => {
                error!("Failed to create save directory: {}", e);
                continue; // Skip this save attempt
            }
        }
    }
}

// ... saving process ...

// Verify save file was created for native platforms
#[cfg(not(target_arch = "wasm32"))]
{
    // Wait a short time to ensure filesystem operations complete
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    if !save_path.exists() {
        error!("Save file was not created at: {:?}", save_path);
        continue;
    } else {
        info!("Verified save file exists at: {:?}", save_path);
    }
}
```

## Testing

The save/load system includes comprehensive tests:

1. **Unit Tests**: Testing individual components and functions
2. **Integration Tests**: Testing full save/load cycles
3. **Edge Cases**: Testing corrupted saves, empty data, etc.
4. **Platform-Specific Tests**: Special considerations for WebAssembly

## WebAssembly Support

For web builds, the save/load system:

1. Uses browser local storage instead of the file system
2. Handles storage limitations and permissions
3. Uses appropriate path prefixes for the storage backend

See [WebAssembly Local Storage](web_storage.md) for more details.

## Performance Considerations

The save/load system is designed with performance in mind:

1. Uses efficient binary serialization (Bincode)
2. Avoids unnecessary re-serialization of unchanged data
3. Performs heavy operations outside of critical game loops
4. Uses compact data representations where possible

## Future Improvements

Potential future enhancements:

1. **Incremental Saves**: Only saving changes since the last save
2. **Save Compression**: Optional compression for large save files
3. **Save Verification**: Checksums or other validation of save integrity
4. **Multiple Save Formats**: Support for JSON or other human-readable formats
5. **Cloud Integration**: Syncing saves to cloud storage 