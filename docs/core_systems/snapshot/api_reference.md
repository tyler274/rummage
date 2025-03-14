# Snapshot System API Reference

This document provides a comprehensive reference for the Snapshot System API in Rummage.

## Core Types

### GameSnapshot

The primary data structure for storing game state:

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

impl GameSnapshot {
    /// Creates a new empty snapshot
    pub fn new(turn: u32, phase: Phase, active_player: usize) -> Self;
    
    /// Returns the size of the snapshot in bytes
    pub fn size_bytes(&self) -> usize;
    
    /// Checks if the snapshot contains a specific entity
    pub fn contains_entity(&self, entity: Entity) -> bool;
    
    /// Gets the serialized data for a specific entity
    pub fn get_entity_data(&self, entity: Entity) -> Option<&Vec<u8>>;
}
```

### SnapshotRegistry

Resource for managing snapshots:

```rust
/// Manages snapshots in memory
#[derive(Resource, Default)]
pub struct SnapshotRegistry {
    /// All available snapshots indexed by ID
    pub snapshots: HashMap<Uuid, GameSnapshot>,
}

impl SnapshotRegistry {
    /// Returns the most recent snapshot
    pub fn most_recent(&self) -> Option<&GameSnapshot>;
    
    /// Returns a snapshot by ID
    pub fn get(&self, id: Uuid) -> Option<&GameSnapshot>;
    
    /// Adds a snapshot to the registry
    pub fn add(&mut self, snapshot: GameSnapshot);
    
    /// Removes a snapshot from the registry
    pub fn remove(&mut self, id: Uuid) -> Option<GameSnapshot>;
    
    /// Clears all snapshots
    pub fn clear(&mut self);
    
    /// Returns all snapshots sorted by timestamp
    pub fn all_sorted_by_time(&self) -> Vec<&GameSnapshot>;
    
    /// Finds snapshots for a specific turn
    pub fn find_by_turn(&self, turn: u32) -> Vec<&GameSnapshot>;
}
```

### PendingSnapshots

Resource for tracking snapshot operations:

```rust
/// Tracks pending snapshot operations
#[derive(Resource, Default)]
pub struct PendingSnapshots {
    /// Snapshots waiting to be processed
    pub queue: VecDeque<GameSnapshot>,
    /// Whether snapshot processing is paused
    pub paused: bool,
}

impl PendingSnapshots {
    /// Adds a snapshot to the queue
    pub fn enqueue(&mut self, snapshot: GameSnapshot);
    
    /// Gets the next snapshot from the queue
    pub fn dequeue(&mut self) -> Option<GameSnapshot>;
    
    /// Pauses snapshot processing
    pub fn pause(&mut self);
    
    /// Resumes snapshot processing
    pub fn resume(&mut self);
    
    /// Clears the queue
    pub fn clear(&mut self);
}
```

### SnapshotConfig

Configuration for the snapshot system:

```rust
/// Configuration for the snapshot system
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

## Components

### Snapshotable

Marker component for entities that should be included in snapshots:

```rust
/// Marker for entities that should be included in snapshots
#[derive(Component)]
pub struct Snapshotable;
```

### SnapshotExcluded

Marker component for components that should not be included in snapshots:

```rust
/// Marker for components that should be excluded from snapshots
#[derive(Component)]
pub struct SnapshotExcluded;
```

## Events

### SnapshotEvent

Events for controlling snapshot operations:

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
```

### SnapshotProcessedEvent

Event for snapshot processing completion:

```rust
/// Event fired when a snapshot has been processed
#[derive(Event)]
pub struct SnapshotProcessedEvent {
    /// The ID of the processed snapshot
    pub id: Uuid,
    /// Whether processing succeeded
    pub success: bool,
    /// Error message if processing failed
    pub error: Option<String>,
}
```

## Plugin

### SnapshotPlugin

Main plugin for the snapshot system:

```rust
pub struct SnapshotPlugin;

impl Plugin for SnapshotPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<PendingSnapshots>()
            .init_resource::<SnapshotConfig>()
            .init_resource::<SnapshotRegistry>()
            
            // Register snapshot events
            .add_event::<SnapshotEvent>()
            .add_event::<SnapshotProcessedEvent>()
            
            // Add snapshot systems to the appropriate schedule
            .add_systems(Update, (
                handle_snapshot_events,
                process_pending_snapshots,
                trigger_snapshot_on_turn_change,
                trigger_snapshot_on_phase_change,
                cleanup_old_snapshots,
            ).chain());
    }
}
```

## Systems

### Handle Snapshot Events

```rust
/// Handles snapshot events
pub fn handle_snapshot_events(
    mut commands: Commands,
    mut snapshot_events: EventReader<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
    mut snapshot_registry: ResMut<SnapshotRegistry>,
    mut processed_events: EventWriter<SnapshotProcessedEvent>,
    game_state: Res<GameState>,
    time: Res<Time>,
    world: &World,
) {
    // Implementation details...
}
```

### Process Pending Snapshots

```rust
/// Process any pending snapshots in the queue
pub fn process_pending_snapshots(
    mut commands: Commands,
    mut pending: ResMut<PendingSnapshots>,
    mut processed_events: EventWriter<SnapshotProcessedEvent>,
    config: Res<SnapshotConfig>,
) {
    // Implementation details...
}
```

### Automatic Snapshot Triggers

```rust
/// Triggers a snapshot when the turn changes
pub fn trigger_snapshot_on_turn_change(
    mut turn_events: EventReader<TurnChangeEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    config: Res<SnapshotConfig>,
) {
    // Implementation details...
}

/// Triggers a snapshot when the phase changes
pub fn trigger_snapshot_on_phase_change(
    mut phase_events: EventReader<PhaseChangeEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    config: Res<SnapshotConfig>,
) {
    // Implementation details...
}
```

### Cleanup System

```rust
/// Cleans up old snapshots to maintain the history size limit
pub fn cleanup_old_snapshots(
    mut snapshot_registry: ResMut<SnapshotRegistry>,
    config: Res<SnapshotConfig>,
) {
    // Implementation details...
}
```

## Functions

### Create Snapshot

```rust
/// Creates a complete snapshot of the current game state
pub fn create_game_snapshot(
    world: &World,
    game_state: &GameState,
) -> GameSnapshot {
    // Implementation details...
}
```

### Serialize Entity

```rust
/// Serializes an entity to a snapshot
pub fn serialize_entity_to_snapshot(
    world: &World,
    entity: Entity,
    snapshot: &mut GameSnapshot,
) {
    // Implementation details...
}
```

### Apply Snapshot

```rust
/// Applies a snapshot to restore game state
pub fn apply_snapshot(
    commands: &mut Commands,
    snapshot: &GameSnapshot,
) {
    // Implementation details...
}
```

### Save and Load

```rust
/// Saves a snapshot to disk
pub fn save_snapshot_to_disk(
    snapshot: &GameSnapshot,
    path: &str,
) -> Result<(), std::io::Error> {
    // Implementation details...
}

/// Loads a snapshot from disk
pub fn load_snapshot_from_disk(
    path: &str,
) -> Result<GameSnapshot, std::io::Error> {
    // Implementation details...
}
```

## Networking Integration

### NetworkSnapshotPlugin

```rust
/// Plugin that integrates the snapshot system with networking
pub struct NetworkSnapshotPlugin;

impl Plugin for NetworkSnapshotPlugin {
    fn build(&self, app: &mut App) {
        // Implementation details...
    }
}
```

### NetworkSnapshotConfig

```rust
/// Resource that tracks network-specific snapshot configuration
#[derive(Resource)]
pub struct NetworkSnapshotConfig {
    /// Frequency of network snapshot updates (in seconds)
    pub sync_frequency: f32,
    /// Maximum size of a snapshot packet (in bytes)
    pub max_packet_size: usize,
    /// Whether to compress network snapshots
    pub compress_network_snapshots: bool,
    /// Number of snapshots to keep for rollback purposes
    pub rollback_history_size: usize,
}
```

### NetworkSnapshotable

```rust
/// Component marking entities with network-specific snapshot requirements
#[derive(Component)]
pub struct NetworkSnapshotable {
    /// Which player IDs can see this entity
    pub visible_to: Vec<u64>,
    /// Priority for synchronization (higher values sync first)
    pub sync_priority: u8,
}
```

## Usage Examples

### Basic Usage

```rust
// Add the snapshot plugin to your app
app.add_plugins(SnapshotPlugin);

// Configure the snapshot system
let mut config = SnapshotConfig::default();
config.auto_snapshot_on_turn = true;
config.max_history_size = 50;
app.insert_resource(config);

// Mark entities for snapshot inclusion
commands.spawn((
    Snapshotable,
    MyComponent { value: 42 },
));

// Manually create a snapshot
app.world.send_event(SnapshotEvent::Take);

// Apply a snapshot
let snapshot_id = app.world.resource::<SnapshotRegistry>()
                         .most_recent().unwrap().id;
app.world.send_event(SnapshotEvent::Apply(snapshot_id));

// Save a snapshot to disk
app.world.send_event(SnapshotEvent::Save("save_game_1.snapshot".to_string()));

// Load a snapshot from disk
app.world.send_event(SnapshotEvent::Load("save_game_1.snapshot".to_string()));
```

### Network Integration

```rust
// Add the network snapshot plugin
app.add_plugins((
    SnapshotPlugin,
    NetworkSnapshotPlugin,
));

// Configure network snapshot settings
let mut network_config = NetworkSnapshotConfig::default();
network_config.sync_frequency = 0.1; // 10 updates per second
network_config.compress_network_snapshots = true;
app.insert_resource(network_config);

// Mark entities for network visibility
commands.spawn((
    Snapshotable,
    NetworkSnapshotable {
        visible_to: vec![1, 2], // Only visible to players 1 and 2
        sync_priority: 10,      // High priority
    },
    MyComponent { value: 42 },
));
```

### Rollback Usage

```rust
// Add rollback plugin
app.add_plugins((
    SnapshotPlugin,
    NetworkSnapshotPlugin,
    RollbackPlugin,
));

// Trigger a rollback to a specific turn
app.world.send_event(RollbackEvent {
    turn: 5,
    phase: Some(Phase::Combat),
});
```

## Common Patterns

### Snapshot Listener

```rust
fn my_snapshot_listener(
    mut snapshot_events: EventReader<SnapshotProcessedEvent>,
) {
    for event in snapshot_events.iter() {
        println!("Snapshot processed: {}", event.id);
        if !event.success {
            if let Some(error) = &event.error {
                println!("Error: {}", error);
            }
        }
    }
}
```

### Custom Snapshot Trigger

```rust
fn trigger_snapshot_on_critical_event(
    mut critical_events: EventReader<CriticalGameEvent>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
) {
    for event in critical_events.iter() {
        // Create a snapshot when critical events occur
        snapshot_events.send(SnapshotEvent::Take);
    }
}
```

### Snapshot Analysis

```rust
fn analyze_snapshots(
    snapshot_registry: Res<SnapshotRegistry>,
) {
    let snapshots = snapshot_registry.all_sorted_by_time();
    for snapshot in snapshots {
        println!("Snapshot {} - Turn {}, Phase {:?}", 
                 snapshot.id, snapshot.turn, snapshot.phase);
    }
}
``` 