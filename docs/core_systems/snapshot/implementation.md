# Snapshot System Implementation

This document covers the technical implementation details of the Rummage snapshot system.

## Plugin Structure

The snapshot system is implemented as a Bevy plugin that can be added to the application:

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
                trigger_snapshot_on_turn_change,
                trigger_snapshot_on_phase_change,
            ).chain());
    }
}
```

## Core Components

### Configuration

The snapshot system is configured through a resource:

```rust
#[derive(Resource)]
pub struct SnapshotConfig {
    /// Whether to automatically create snapshots on turn changes
    pub auto_snapshot_on_turn: bool,
    /// Whether to automatically create snapshots on phase changes
    pub auto_snapshot_on_phase: bool,
    /// Maximum number of snapshots to process per frame
    pub max_snapshots_per_frame: usize,
    /// Maximum number of snapshots to keep in history
    pub max_history_size: usize,
    /// Whether to compress snapshots
    pub use_compression: bool,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            auto_snapshot_on_turn: true,
            auto_snapshot_on_phase: false,
            max_snapshots_per_frame: 1,
            max_history_size: 100,
            use_compression: true,
        }
    }
}
```

### Event Types

The system communicates through events:

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

## Creating Snapshots

The core snapshot creation logic:

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
    // Get all component types registered for this entity
    let entity_components = world.entity(entity).archetype().components();
    
    // Create a buffer to store the entity's serialized components
    let mut entity_data = Vec::new();
    
    // Write the entity ID
    let entity_id = entity.to_bits();
    entity_data.extend_from_slice(&entity_id.to_le_bytes());
    
    // For each component type
    for component_id in entity_components.iter() {
        // Skip certain component types that don't need to be serialized
        if should_skip_component(component_id) {
            continue;
        }
        
        // Get the component storage
        if let Some(component_info) = world.components().get_info(component_id) {
            // Get the component data for this entity
            if let Some(component_data) = component_info.get_component(world, entity) {
                // Write the component ID
                entity_data.extend_from_slice(&component_id.to_le_bytes());
                
                // Write the component size
                let size = component_data.len();
                entity_data.extend_from_slice(&size.to_le_bytes());
                
                // Write the component data
                entity_data.extend_from_slice(component_data);
            }
        }
    }
    
    // Add the entity's data to the snapshot
    snapshot.game_data.insert(entity.index().to_string(), entity_data);
}
```

## Automatic Snapshot Triggering

Snapshots can be automatically triggered by game events:

```rust
fn trigger_snapshot_on_turn_change(
    mut turn_events: EventReader<TurnChangeEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    config: Res<SnapshotConfig>,
) {
    if !config.auto_snapshot_on_turn {
        return;
    }
    
    for _ in turn_events.iter() {
        // Create a new snapshot event
        snapshot_events.send(SnapshotEvent::Take);
    }
}

fn trigger_snapshot_on_phase_change(
    mut phase_events: EventReader<PhaseChangeEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    config: Res<SnapshotConfig>,
) {
    if !config.auto_snapshot_on_phase {
        return;
    }
    
    for _ in phase_events.iter() {
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
```

## Applying Snapshots

To restore a game state from a snapshot:

```rust
fn apply_snapshot(
    commands: &mut Commands,
    snapshot: &GameSnapshot,
) {
    // Clear existing entities that should be replaced by the snapshot
    clear_snapshotable_entities(commands);
    
    // Restore global game state
    restore_game_state(commands, snapshot);
    
    // For each entity in the snapshot
    for (entity_key, entity_data) in &snapshot.game_data {
        // Create a new entity
        let entity = commands.spawn_empty().id();
        
        // Deserialize and add components
        deserialize_entity_components(commands, entity, entity_data);
    }
}

fn deserialize_entity_components(
    commands: &mut Commands,
    entity: Entity,
    entity_data: &[u8],
) {
    let mut offset = 8; // Skip the entity ID
    
    // While there's more data to read
    while offset < entity_data.len() {
        // Read the component ID
        let component_id_bytes = &entity_data[offset..offset+8];
        let component_id = u64::from_le_bytes(component_id_bytes.try_into().unwrap());
        offset += 8;
        
        // Read the component size
        let size_bytes = &entity_data[offset..offset+8];
        let size = usize::from_le_bytes(size_bytes.try_into().unwrap());
        offset += 8;
        
        // Read the component data
        let component_data = &entity_data[offset..offset+size];
        offset += size;
        
        // Deserialize and add the component
        add_component_from_bytes(commands, entity, component_id, component_data);
    }
}
```

## Snapshot Storage

The system supports saving snapshots to disk and loading them later:

```rust
fn save_snapshot_to_disk(
    snapshot: &GameSnapshot,
    path: &str,
) -> Result<(), std::io::Error> {
    // Serialize the snapshot
    let serialized = bincode::serialize(snapshot)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // Compress if configured
    let final_data = if snapshot.use_compression {
        // Apply compression
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&serialized)?;
        encoder.finish()?
    } else {
        serialized
    };
    
    // Write to file
    std::fs::write(path, final_data)?;
    
    Ok(())
}

fn load_snapshot_from_disk(
    path: &str,
) -> Result<GameSnapshot, std::io::Error> {
    // Read file
    let data = std::fs::read(path)?;
    
    // Check for compression
    let serialized = if is_compressed(&data) {
        // Decompress
        let mut decoder = flate2::read::GzDecoder::new(&data[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        decompressed
    } else {
        data
    };
    
    // Deserialize
    let snapshot = bincode::deserialize(&serialized)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    Ok(snapshot)
}
```

## Performance Considerations

The snapshot system is designed with performance in mind:

1. **Selective Serialization**: Only necessary components are included
2. **Batched Processing**: Limits how many snapshots are processed per frame
3. **Compression Options**: Configurable to balance size vs. speed
4. **Marker Components**: Only entities explicitly marked are included
5. **Queue Management**: Background processing to minimize frame time impact

## Integration Points

The snapshot system integrates with other systems through:

1. **Events**: For triggering and receiving snapshot operations
2. **Component Markers**: To specify what entities should be included
3. **Configuration**: To control behavior based on game requirements
4. **Plugins**: To integrate with other game systems

These integration points provide flexibility while maintaining clean separation of concerns.

## Next Steps

- **[Integration with Networking](networking_integration.md)**: How snapshots work with the networking system
- **[Testing](testing.md)**: How to test snapshot functionality
- **[API Reference](api_reference.md)**: Complete reference documentation 