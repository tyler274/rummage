# System Reference

This document provides a detailed reference of the various systems in Rummage that implement game logic, handle events, and manage game state.

## Overview

Systems in Bevy ECS are the workhorses that process game logic. In Rummage, systems are organized into several major categories that map to Magic: The Gathering game concepts.

## State Management Systems

State management systems handle the tracking and updating of game state.

### Game State Systems

```rust
// Initialize the primary game state
fn init_game_state(mut commands: Commands) {
    commands.insert_resource(GameState {
        turn: 1,
        phase: Phase::Beginning,
        step: Step::Untap,
        active_player: 0,
        priority_player: 0,
        // ...
    });
}

// Update the game state based on input events
fn update_game_state(
    mut game_state: ResMut<GameState>,
    mut phase_events: EventReader<PhaseChangeEvent>,
    // ...
) {
    // Implementation...
}
```

### Player State Systems

```rust
// Initialize player state
fn init_player_state(mut commands: Commands, game_config: Res<GameConfig>) {
    for player_idx in 0..game_config.player_count {
        commands.spawn((
            Player {
                id: player_idx,
                life_total: 40, // Commander starting life
                // ...
            },
            // Other components...
        ));
    }
}

// Update player state based on game events
fn update_player_state(
    mut player_query: Query<(&mut Player, &CommanderDamage)>,
    mut damage_events: EventReader<PlayerDamageEvent>,
    // ...
) {
    // Implementation...
}
```

### Card State Systems

```rust
// Track state of cards on the battlefield
fn update_battlefield_cards(
    mut commands: Commands,
    mut card_query: Query<(Entity, &mut Card, &Zone)>,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    // ...
) {
    // Implementation...
}
```

## Stack Systems

Systems that implement the MTG stack and priority mechanisms.

### Stack Management

```rust
// Add objects to the stack
fn add_to_stack(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    mut stack_events: EventReader<AddToStackEvent>,
    // ...
) {
    // Implementation...
}

// Resolve objects from the stack
fn resolve_stack(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    mut game_state: ResMut<GameState>,
    // ...
) {
    // Implementation...
}
```

### Priority Systems

```rust
// Handle passing of priority between players
fn handle_priority(
    mut game_state: ResMut<GameState>,
    mut priority_events: EventReader<PriorityPassedEvent>,
    // ...
) {
    // Implementation...
}
```

## Combat Systems

Systems that implement the combat phase and damage resolution.

### Combat Flow

```rust
// Start the combat phase
fn start_combat(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut phase_events: EventWriter<PhaseChangeEvent>,
    // ...
) {
    // Implementation...
}

// Handle the declare attackers step
fn declare_attackers(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut attacker_query: Query<(Entity, &Card), With<Attacker>>,
    // ...
) {
    // Implementation...
}

// Handle the declare blockers step
fn declare_blockers(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut blocker_query: Query<(Entity, &Card), With<Blocker>>,
    // ...
) {
    // Implementation...
}

// Combat damage calculation and assignment
fn combat_damage(
    mut commands: Commands,
    mut attacker_query: Query<(Entity, &Attacker, &Card)>,
    mut blocker_query: Query<(Entity, &Blocker, &Card)>,
    mut damage_events: EventWriter<DamageEvent>,
    // ...
) {
    // Implementation...
}
```

### Combat Damage

```rust
// Apply combat damage to creatures and players
fn apply_combat_damage(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Card)>,
    mut player_query: Query<(Entity, &mut Player)>,
    mut damage_events: EventReader<DamageEvent>,
    // ...
) {
    // Implementation...
}
```

## Turn Systems

Systems that handle turn structure and phase progression.

### Turn Progression

```rust
// Start a new turn
fn start_turn(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut turn_events: EventWriter<TurnStartEvent>,
    // ...
) {
    // Implementation...
}

// Transition between phases
fn change_phase(
    mut game_state: ResMut<GameState>,
    mut phase_events: EventReader<PhaseChangeEvent>,
    // ...
) {
    // Implementation...
}

// End the current turn
fn end_turn(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut turn_events: EventWriter<TurnEndEvent>,
    // ...
) {
    // Implementation...
}
```

## Commander Systems

Systems specific to the Commander format.

### Commander Tax

```rust
// Track and apply commander tax
fn apply_commander_tax(
    mut commands: Commands,
    mut commander_query: Query<(Entity, &mut CommanderCard)>,
    mut cast_events: EventReader<CommanderCastEvent>,
    // ...
) {
    // Implementation...
}
```

### Commander Damage

```rust
// Track commander damage between players
fn track_commander_damage(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut CommanderDamage)>,
    mut damage_events: EventReader<CommanderDamageEvent>,
    // ...
) {
    // Implementation...
}
```

## Event Systems

Systems that process various game events.

### Event Dispatch

```rust
// Main event dispatcher system
fn dispatch_events(
    mut commands: Commands,
    mut card_played_events: EventReader<CardPlayedEvent>,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    // Other event readers...
    mut card_query: Query<(Entity, &mut Card)>,
    // ...
) {
    // Implementation...
}
```

### Event Processing

```rust
// Process various types of events
fn process_card_played(
    mut commands: Commands,
    mut card_played_events: EventReader<CardPlayedEvent>,
    // ...
) {
    // Implementation...
}

fn process_zone_changes(
    mut commands: Commands,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    // ...
) {
    // Implementation...
}
```

## Snapshot Systems

Systems for capturing and restoring game state.

### Snapshot Creation

```rust
// Create a game state snapshot
fn create_snapshot(
    world: &World, 
    game_state: Res<GameState>,
    mut snapshot_events: EventWriter<SnapshotEvent>,
    // ...
) {
    // Implementation...
}
```

### Snapshot Restoration

```rust
// Restore from a snapshot
fn apply_snapshot(
    mut commands: Commands,
    snapshot: Res<GameSnapshot>,
    // ...
) {
    // Implementation...
}
```

## UI Integration Systems

Systems that connect game logic to the user interface.

### UI Update Systems

```rust
// Update UI elements based on game state
fn update_battlefield_ui(
    mut commands: Commands,
    card_query: Query<(Entity, &Card, &Zone)>,
    mut ui_query: Query<(Entity, &mut UiTransform, &CardUi)>,
    // ...
) {
    // Implementation...
}

// Handle user input events
fn handle_card_interaction(
    mut commands: Commands,
    mut interaction_events: EventReader<CardInteractionEvent>,
    card_query: Query<(Entity, &Card)>,
    // ...
) {
    // Implementation...
}
```

## Network Integration Systems

Systems that handle network synchronization.

### State Synchronization

```rust
// Synchronize game state across the network
fn sync_game_state(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut replicon: ResMut<Replicon>,
    // ...
) {
    // Implementation...
}
```

### Action Broadcasting

```rust
// Broadcast player actions to all clients
fn broadcast_actions(
    mut commands: Commands,
    mut action_events: EventReader<PlayerActionEvent>,
    mut replicon: ResMut<Replicon>,
    // ...
) {
    // Implementation...
}
```

## System Registration

Systems are registered with the Bevy App in the following manner:

```rust
// Register all game systems
fn build_game_systems(app: &mut App) {
    app
        // Core game state systems
        .add_systems(Startup, init_game_state)
        .add_systems(Update, update_game_state)
        
        // Turn and phase systems
        .add_systems(Update, (
            start_turn,
            change_phase,
            end_turn,
        ).chain())
        
        // Combat systems
        .add_systems(Update, (
            start_combat,
            declare_attackers,
            declare_blockers,
            combat_damage,
            apply_combat_damage,
        ).chain().run_if(in_combat_phase))
        
        // Stack systems
        .add_systems(Update, (
            add_to_stack,
            resolve_stack,
            handle_priority,
        ).chain())
        
        // Event systems
        .add_systems(Update, dispatch_events)
        .add_systems(Update, (
            process_card_played,
            process_zone_changes,
        ))
        
        // Commander-specific systems
        .add_systems(Update, (
            apply_commander_tax,
            track_commander_damage,
        ))
        
        // Snapshot systems
        .add_systems(Update, (
            create_snapshot.run_if(resource_exists::<SnapshotConfig>()),
            apply_snapshot.run_if(resource_exists::<PendingSnapshots>()),
        ))
        
        // UI integration systems
        .add_systems(Update, (
            update_battlefield_ui,
            handle_card_interaction,
        ))
        
        // Network integration systems
        .add_systems(Update, (
            sync_game_state,
            broadcast_actions,
        ).run_if(resource_exists::<Replicon>()));
}
``` 