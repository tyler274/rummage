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

While it's possible to access the entire ECS `World`