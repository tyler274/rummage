# Entity Component System

This guide explains how the Entity Component System (ECS) architecture is implemented in Rummage using Bevy, and provides practical advice for working with ECS patterns.

## Table of Contents

1. [Introduction to ECS](#introduction-to-ecs)
2. [ECS in Bevy](#ecs-in-bevy)
3. [Game Entities in Rummage](#game-entities-in-rummage)
4. [Component Design](#component-design)
5. [System Design](#system-design)
6. [Queries and Filters](#queries-and-filters)
7. [ECS Best Practices](#ecs-best-practices)
8. [Common Pitfalls](#common-pitfalls)
9. [Safely Using Parameter Sets](#safely-using-parameter-sets)
   - [Understanding Parameter Sets](#understanding-parameter-sets)
   - [Disjoint Queries with Param Sets](#disjoint-queries-with-param-sets)
   - [Using Component Access for Safety](#using-component-access-for-safety)
   - [Avoiding World References](#avoiding-world-references)
   - [Query Lifetimes and Temporary Storage](#query-lifetimes-and-temporary-storage)
   - [Using System Sets for Dependency Management](#using-system-sets-for-dependency-management)
   - [Testing for Query Conflicts](#testing-for-query-conflicts)
   - [Working with Snapshot Systems](#working-with-snapshot-systems)
   - [Debugging Snapshot Systems with Trace Logging](#debugging-snapshot-systems-with-trace-logging)
   - [MTG-Specific Example: Card Manipulation Safety](#mtg-specific-example-card-manipulation-safety)

## Introduction to ECS

Entity Component System (ECS) is an architectural pattern that separates identity (entities), data (components), and logic (systems). This separation offers several advantages:

- **Performance**: Enables cache-friendly memory layouts and parallel execution
- **Flexibility**: Allows for dynamic composition of game objects
- **Modularity**: Decouples data from behavior for better code organization
- **Extensibility**: Makes it easier to add new features without modifying existing code

## ECS in Bevy

Bevy's ECS implementation includes these core elements:

### Entities

Entities in Bevy are simply unique identifiers that components can be attached to. They're created using the `Commands` API:

```rust
// Creating a new entity
commands.spawn_empty();

// Creating an entity with components
commands.spawn((
    Card { id: "fireball", cost: "{X}{R}" },
    CardName("Fireball".to_string()),
    SpellType::Instant,
));
```

### Components

Components are simple data structures that can be attached to entities:

```rust
// Component definition
#[derive(Component)]
struct Health {
    current: i32,
    maximum: i32,
}

// Component with derive macros for common functionality
#[derive(Component, Debug, Clone, PartialEq)]
struct ManaCost {
    blue: u8,
    black: u8,
    red: u8,
    green: u8,
    white: u8,
    colorless: u8,
}
```

### Systems

Systems are functions that operate on components:

```rust
// Simple system that operates on Health components
fn heal_system(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.current = health.current.min(health.maximum);
    }
}

// System that uses multiple component types
fn damage_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Health, &DamageReceiver)>,
    time: Res<Time>,
) {
    for (entity, mut health, damage) in &mut query {
        health.current -= damage.amount;
        
        if health.current <= 0 {
            commands.entity(entity).insert(DeathMarker);
        }
    }
}
```

### Resources

Resources are global singleton data structures:

```rust
// Resource definition
#[derive(Resource)]
struct GameState {
    turn: usize,
    phase: Phase,
    active_player: usize,
}

// Accessing resources in systems
fn turn_system(mut game_state: ResMut<GameState>) {
    game_state.turn += 1;
    // ...
}
```

## Game Entities in Rummage

Rummage represents game concepts as entities with appropriate components:

### Cards

Cards are entities with components like:
- `Card` - Core card data
- `CardName` - The card's name
- `ManaCost` - Mana cost information
- `CardType` - Card type information
- Position components for visual placement

### Players

Players are entities with components like:
- `Player` - Player information
- `Life` - Current life total
- `Hand` - Reference to hand entity
- `Commander` - Reference to commander entity
- `Library` - Reference to library entity

### Zones

Game zones (like battlefield, graveyard) are entities with components like:
- `Zone` - Zone type and metadata
- `ZoneContents` - References to contained entities
- Visual placement components

## Component Design

When designing components for Rummage, follow these guidelines:

### Keep Components Focused

Components should represent a single aspect of an entity. For example, separate `Health`, `AttackPower`, and `BlockStatus` rather than a single `CombatStats` component.

### Efficient Component Storage

Consider the memory layout of components:
- Use primitive types where possible
- For small fixed-size collections, use arrays instead of Vecs
- For larger collections, consider using entity references instead of direct data storage

### Component Relationships

Use entity references to establish relationships between components:

```rust
#[derive(Component)]
struct Attachments {
    attached_to: Entity,
    attached_cards: Vec<Entity>,
}
```

## System Design

Systems should follow these design principles:

### Single Responsibility

Each system should have a clear, well-defined responsibility. For example:
- `draw_card_system` - Handles drawing cards from library to hand
- `apply_damage_system` - Applies damage to creatures and players
- `check_state_based_actions` - Checks and applies state-based actions

### System Organization

Systems are organized in the codebase by domain:
- `card/systems.rs` - Card-related systems
- `combat/systems.rs` - Combat-related systems
- `player/systems.rs` - Player-related systems

### System Scheduling

Bevy 0.15 uses system sets for scheduling. Rummage organizes systems into sets like:

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CardSystemSet {
    Draw,
    Play,
    Resolve,
}

app
    .configure_sets(
        Update, 
        (CardSystemSet::Draw, CardSystemSet::Play, CardSystemSet::Resolve).chain()
    )
    .add_systems(
        Update,
        (draw_card, mill_cards).in_set(CardSystemSet::Draw)
    );
```

Now that we understand how systems are organized and scheduled, let's explore how systems access and manipulate entity data through queries.

## Queries and Filters

Queries are the primary way to access entity data in systems. Here are some common query patterns used in Rummage:

### Basic Queries

```rust
// Query for a single component type
fn system(query: Query<&Card>) {
    for card in &query {
        // Use card data
    }
}

// Query for multiple component types
fn system(query: Query<(&Card, &CardName, &ManaCost)>) {
    for (card, name, cost) in &query {
        // Use all components
    }
}

// Query with mutable access
fn system(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.current += 1;
    }
}
```

### Filtering Queries

```rust
// Filter to only entities with specific components
fn system(query: Query<&Card, With<Creature>>) {
    // Only processes cards that are creatures
}

// Filter to exclude entities with specific components
fn system(query: Query<&Card, Without<Tapped>>) {
    // Only processes cards that aren't tapped
}

// Combining filters
fn system(query: Query<&Card, (With<Creature>, Without<Tapped>)>) {
    // Only processes creature cards that aren't tapped
}
```

### Entity Access

```rust
// Getting the entity ID along with components
fn system(query: Query<(Entity, &Card)>) {
    for (entity, card) in &query {
        // Use entity ID and card
    }
}

// Looking up a specific entity
fn system(
    commands: Commands, 
    query: Query<&Card>,
    player_query: Query<&Player>
) {
    if let Ok(player) = player_query.get(player_entity) {
        // Use player data
    }
}
```

## ECS Best Practices

### Performance Considerations

1. **Batch operations**: Use commands.spawn_batch() for creating multiple similar entities
2. **Query optimization**: Be specific about which components you query
3. **Change detection**: Use Changed<T> to only run logic when components change
4. **Parallelism awareness**: Design systems to avoid conflicts that would prevent parallelism

### Maintainable Code

1. **Document component purposes**: Each component should have clear documentation
2. **System naming**: Use clear, descriptive names for systems
3. **Consistent patterns**: Follow established patterns for similar features
4. **Tests**: Write unit tests for systems and component interactions

## Common Pitfalls

### Multiple Mutable Borrows

Bevy will panic if you try to mutably access the same component multiple times in a system.

**Problem**:
```rust
fn problematic_system(mut query: Query<(&mut Health, &mut Damage)>) {
    // This could panic if an entity has both Health and Damage components
}
```

**Solution**:
```rust
fn fixed_system(
    mut health_query: Query<&mut Health>,
    damage_query: Query<(Entity, &Damage)>,
) {
    for (entity, damage) in &damage_query {
        if let Ok(mut health) = health_query.get_mut(entity) {
            health.current -= damage.amount;
        }
    }
}
```

### Query For Single Entity

When you expect a single entity to match a query but get multiple, Bevy will panic with a "MultipleEntities" error.

**Problem**:
```rust
fn get_camera_system(camera_query: Query<(&Camera, &GlobalTransform)>) {
    // This will panic if there are multiple camera entities
    let (camera, transform) = camera_query.single();
}
```

**Solution**:
```rust
fn get_camera_system(camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>) {
    // Add a marker component to your main camera
    if let Ok((camera, transform)) = camera_query.get_single() {
        // Now we only get the one with the MainCamera marker
    }
}
```

### Event Overflow

Event readers that don't consume all events can cause memory growth.

**Problem**:
```rust
fn card_draw_system(mut event_reader: EventReader<DrawCardEvent>) {
    // Only process the first event each frame
    if let Some(event) = event_reader.iter().next() {
        // Process one event, leaving others unconsumed
    }
}
```

**Solution**:
```rust
fn card_draw_system(mut event_reader: EventReader<DrawCardEvent>) {
    // Process all events
    for event in event_reader.iter() {
        // Process each event
    }
}
```

The pitfalls discussed above highlight some of the common issues you might encounter when working with Bevy's ECS. In the next section, we'll explore more advanced techniques to prevent these issues from occurring in the first place.

## Safely Using Parameter Sets

Bevy's ECS enforces strict borrowing rules to maintain memory safety and enable parallelism. A common cause of runtime panics is query parameter conflicts, especially when working with complex systems. This section covers techniques to write robust systems that avoid these issues.

### Understanding Parameter Sets

Parameter sets provide a way to group related parameters and control how they interact with each other. By explicitly defining parameter sets, you can prevent Bevy from attempting to run systems with conflicting queries in parallel, which would cause runtime panics.

### Disjoint Queries with Param Sets

The `ParamSet` type allows you to create multiple queries that would otherwise conflict with each other:

```rust
use bevy::ecs::system::ParamSet;

fn safe_system(
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<Enemy>>
    )>
) {
    // Access the first query (player transforms)
    for mut transform in param_set.p0().iter_mut() {
        // Modify player transforms
    }
    
    // Access the second query (enemy transforms)
    for mut transform in param_set.p1().iter_mut() {
        // Modify enemy transforms
    }
}
```

This approach is safer than trying to use separate queries because `ParamSet` guarantees that access to each query is sequential rather than simultaneous.

### Using Component Access for Safety

For more complex systems, you can use the `ComponentAccess` trait to explicitly control which components your system accesses:

```rust
#[derive(Default, Resource)]
struct SafeComponentAccess {
    processing_cards: bool,
}

fn card_system(
    mut access: ResMut<SafeComponentAccess>,
    mut query: Query<&mut Card>,
) {
    // Set flag to indicate we're processing cards
    access.processing_cards = true;
    
    for mut card in &mut query {
        // Process cards safely
    }
    
    // Release the lock
    access.processing_cards = false;
}

fn other_card_system(
    access: Res<SafeComponentAccess>,
    mut commands: Commands,
) {
    // Check if another system is processing cards
    if !access.processing_cards {
        // Safe to spawn or modify cards
        commands.spawn(Card::default());
    }
}
```

### Avoiding World References

While it's possible to access the entire ECS `World` in a system, this approach bypasses Bevy's safety mechanisms and should be avoided whenever possible:

**Problematic Approach**:
```rust
fn unsafe_world_system(world: &mut World) {
    // Direct world access bypasses Bevy's safety checks
    let mut cards = world.query::<&mut Card>();
    
    for mut card in cards.iter_mut(world) {
        // This might conflict with other systems
    }
}
```

**Safer Alternative**:
```rust
fn safe_system(mut query: Query<&mut Card>) {
    for mut card in &mut query {
        // Bevy will handle safety and scheduling
    }
}
```

### Query Lifetimes and Temporary Storage

This example shows how to safely implement card manipulation systems that would otherwise conflict with each other. By either using `ParamSet` or breaking the operation into separate systems with clear dependencies, we avoid the common causes of ECS panics.

### Testing for Query Conflicts

```rust
#[test]
fn verify_system_sets_compatibility() {
    let mut app = App::new();
    
    // Add systems that should be compatible
    app.add_systems(Update, (system_a, system_b));
    
    // Verify no conflicts using Bevy's built-in detection
    app.world_mut().get_archetypes();
}
```

While the techniques above apply broadly to all ECS systems, some specific system types present unique challenges. One particularly complex area in game development is state snapshotting and replay, which we'll explore next.

### Working with Snapshot Systems

Game state snapshot systems can be particularly prone to query conflicts since they often need to access a wide range of components. Here are patterns to make snapshot systems more robust:

#### Isolating Snapshot Systems

Place snapshot-related systems in dedicated sets that run at specific points in the frame:

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SnapshotSystemSet {
    PrepareSnapshot,
    ProcessEvents,
    ApplySnapshot,
}

app
    .configure_sets(
        Update,
        (
            // Run snapshot systems after regular game systems
            GameSystemSet::All,
            SnapshotSystemSet::PrepareSnapshot,
            SnapshotSystemSet::ProcessEvents,
            SnapshotSystemSet::ApplySnapshot,
        ).chain()
    );
```

#### Deferred Snapshot Processing

Instead of trying to query and modify components immediately, gather snapshot data and defer processing:

```rust
#[derive(Resource, Default)]
struct PendingSnapshots {
    snapshots: Vec<GameSnapshot>,
}

// First system: collect snapshot data
fn handle_snapshot_events(
    mut event_reader: EventReader<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
    query: Query<&GameState>,
) {
    for event in event_reader.iter() {
        // Collect necessary data without modifying anything
        let snapshot = create_snapshot(&query, event);
        pending.snapshots.push(snapshot);
    }
}

// Second system: process collected snapshots
fn process_pending_snapshots(
    mut commands: Commands,
    mut pending: ResMut<PendingSnapshots>,
) {
    for snapshot in pending.snapshots.drain(..) {
        // Now apply changes using commands
        apply_snapshot(&mut commands, snapshot);
    }
}
```

#### Read-Only Snapshots

When possible, make snapshots read-only operations that don't modify components directly:

```rust
fn create_snapshot(
    query: Query<(Entity, &Transform, &Health), With<Snapshotable>>,
) -> GameSnapshot {
    let mut snapshot = GameSnapshot::default();
    
    for (entity, transform, health) in &query {
        snapshot.entities.push(SnapshotEntry {
            entity,
            position: transform.translation,
            health: health.current,
        });
    }
    
    snapshot
}
```

#### Command-Based Modifications

When applying snapshots, use Commands to defer actual entity modifications:

```rust
fn apply_snapshot_system(
    mut commands: Commands,
    snapshots: Res<SnapshotRepository>,
    entities: Query<Entity, With<Snapshotable>>,
) {
    if let Some(snapshot) = snapshots.get_latest() {
        // First remove outdated entities
        for entity in &entities {
            if !snapshot.contains(entity) {
                commands.entity(entity).despawn_recursive();
            }
        }
        
        // Then apply snapshot data using commands
        for entry in &snapshot.entities {
            commands.spawn((
                Snapshotable,
                Transform::from_translation(entry.position),
                Health { current: entry.health, maximum: entry.max_health },
            ));
        }
    }
}
```

This approach ensures that entity modifications happen at safe times controlled by Bevy's command buffer system.

#### Debugging Snapshot Systems with Trace Logging

Snapshot systems can be particularly difficult to debug due to their complex interactions with the ECS. Structured logging can help identify where issues occur:

```rust
fn handle_snapshot_events(
    mut event_reader: EventReader<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
) {
    // Log system entry with count of events
    trace!(system = "handle_snapshot_events", event_count = event_reader.len(), "Entering system");
    
    // Process events
    for event in event_reader.iter() {
        trace!(system = "handle_snapshot_events", event_id = ?event.id, "Processing event");
        
        match process_event(event, &mut pending) {
            Ok(_) => trace!(system = "handle_snapshot_events", event_id = ?event.id, "Successfully processed"),
            Err(e) => error!(system = "handle_snapshot_events", event_id = ?event.id, error = ?e, "Failed to process"),
        }
    }
    
    // Log system exit
    trace!(system = "handle_snapshot_events", "Exiting system");
}
```

When debugging snapshot systems, look for these common patterns in logs:

1. Systems that enter but never exit (indicating a panic or infinite loop)
2. Mismatched counts between processed and expected items
3. Systems that execute in unexpected orders
4. Repeated errors processing the same entities

For complex debugging, consider a custom snapshot debug viewer:

```rust
#[derive(Resource)]
struct SnapshotDebugger {
    history: Vec<SnapshotDebugEntry>,
    active_systems: HashSet<&'static str>,
}

impl SnapshotDebugger {
    fn system_enter(&mut self, name: &'static str) {
        self.active_systems.insert(name);
        self.history.push(SnapshotDebugEntry {
            timestamp: std::time::Instant::now(),
            event: format!("System entered: {}", name),
        });
    }
    
    fn system_exit(&mut self, name: &'static str) {
        self.active_systems.remove(name);
        self.history.push(SnapshotDebugEntry {
            timestamp: std::time::Instant::now(),
            event: format!("System exited: {}", name),
        });
    }
}

// Add to app startup
app.init_resource::<SnapshotDebugger>();

// Modified system with detailed tracing
fn handle_snapshot_events(
    mut debugger: ResMut<SnapshotDebugger>,
    mut event_reader: EventReader<SnapshotEvent>,
    mut pending: ResMut<PendingSnapshots>,
) {
    let system_name = "handle_snapshot_events";
    debugger.system_enter(system_name);
    
    // System logic here
    
    debugger.system_exit(system_name);
}
```

This approach creates a permanent record of system execution that persists even if the system panics, making it easier to reconstruct what happened.

### MTG-Specific Example: Card Manipulation Safety

Now that we've covered the general techniques for safe ECS usage, let's apply these concepts to a concrete example in our Magic: The Gathering implementation. Card manipulation systems are a perfect illustration of where these safety techniques are crucial.

```rust
// Define components for MTG cards
#[derive(Component)]
struct Card {
    id: String,
    power: Option<i32>,
    toughness: Option<i32>,
}

#[derive(Component)]
enum CardZone {
    Battlefield,
    Graveyard,
    Hand,
    Library,
    Exile,
}

// ParamSet approach for a card movement system
fn move_card_system(
    mut param_set: ParamSet<(
        // Queries for different card zones
        Query<(Entity, &Card), With<CardZone>>,
        Query<&mut CardZone>,
    )>,
    commands: Commands,
) {
    // First gather all relevant card entities
    let mut cards_to_move = Vec::new();
    
    // Using the first query to find cards
    for (entity, card) in param_set.p0().iter() {
        if should_move_card(card) {
            cards_to_move.push(entity);
        }
    }
    
    // Then use the second query to update zone components
    for entity in cards_to_move {
        if let Ok(mut zone) = param_set.p1().get_mut(entity) {
            // Update the zone safely
            *zone = CardZone::Graveyard;
        }
    }
}

// Alternative approach using system sets for battlefield organization
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CardSystemSet {
    Preparation,
    ZoneChanges,
    StatUpdates,
    Cleanup,
}

// First system: identify cards to organize
fn identify_battlefield_cards(
    query: Query<Entity, With<CardZone>>,
    mut card_commands: ResMut<CardCommands>,
) {
    card_commands.to_organize.clear();
    
    for entity in &query {
        if should_organize(entity) {
            card_commands.to_organize.push(entity);
        }
    }
}

// Second system: update card positions
fn organize_battlefield_cards(
    mut query: Query<&mut Transform>,
    card_commands: Res<CardCommands>,
) {
    for (index, &entity) in card_commands.to_organize.iter().enumerate() {
        if let Ok(mut transform) = query.get_mut(entity) {
            // Calculate new position
            let position = calculate_card_position(index);
            transform.translation = position;
        }
    }
}

// Register systems in the correct order
app
    .configure_sets(
        Update,
        (
            CardSystemSet::Preparation,
            CardSystemSet::ZoneChanges,
            CardSystemSet::StatUpdates,
            CardSystemSet::Cleanup,
        ).chain()
    )
    .add_systems(
        Update,
        identify_battlefield_cards.in_set(CardSystemSet::Preparation)
    )
    .add_systems(
        Update,
        organize_battlefield_cards.in_set(CardSystemSet::ZoneChanges)
    );
```

This example shows how to safely implement card manipulation systems that would otherwise conflict with each other. By either using `ParamSet` or breaking the operation into separate systems with clear dependencies, we avoid the common causes of ECS panics.

## Conclusion

Bevy's ECS provides a powerful foundation for building complex game systems, but it requires careful attention to system design and query patterns to avoid runtime panics. By following the best practices and safety techniques outlined in this guide, you can build robust, maintainable systems for your Magic: The Gathering implementation.

Remember these key principles:
- Keep components focused and well-documented
- Use appropriate query filters to target exactly the entities you need
- Handle potential conflicts with ParamSet and system ordering
- Use Commands for deferred modifications when appropriate
- Add detailed logging for complex system interactions
- Test your systems thoroughly, including compatibility verification

---

Next: [Plugin Architecture](plugins.md)