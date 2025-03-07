# Magic: The Gathering Game Loop Implementation

## Overview
This document outlines the implementation plan for the core game loop and phase system in our MTG digital implementation using Bevy 0.15.3.

## Core Components

### Game Phase System
```rust
pub enum Phase {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep)
}

pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw
}

pub enum PrecombatStep {
    Main
}

pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    End
}

pub enum PostcombatStep {
    Main
}

pub enum EndingStep {
    End,
    Cleanup
}
```

### Priority System
```rust
pub struct PrioritySystem {
    active_player: Entity,
    has_priority: bool,
    priority_queue: Vec<Entity>,
    stack_is_empty: bool,
    all_players_passed: bool
}
```

### Stack System
```rust
pub struct GameStack {
    items: Vec<StackItem>,
    resolving: bool
}

pub struct StackItem {
    effect: Box<dyn Effect>,
    controller: Entity,
    targets: Vec<Entity>
}
```

## Implementation Plan

### Phase 1: Basic Turn Structure
1. Implement phase transitions
   - Each phase should be a Bevy state
   - Automatic transitions for phases that don't require player input
   - Priority checks for phases that do require input

2. Turn order management
   - Track active player
   - Handle priority passing
   - Manage phase/step transitions

### Phase 2: Priority and Stack
1. Priority system implementation
   - Track which player has priority
   - Handle priority passing
   - Manage special priority rules (e.g., combat)

2. Stack implementation
   - Push/pop mechanics
   - Resolution timing
   - State-based actions

### Phase 3: Game Actions
1. Player actions
   - Playing lands
   - Casting spells
   - Activating abilities
   - Combat actions

2. Timing restrictions
   - Sorcery-speed vs Instant-speed
   - Special timing rules (e.g., land drops)

## Bevy Systems Implementation

### Core Systems
```rust
// Phase management system
fn phase_transition_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut phase: ResMut<Phase>,
    time: Res<Time>,
) {
    // Handle phase transitions
}

// Priority system
fn priority_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    stack: Res<GameStack>,
) {
    // Handle priority passing and checks
}

// Stack system
fn stack_resolution_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut game_state: ResMut<GameState>,
) {
    // Handle stack resolution
}
```

### State-Based Actions
```rust
fn state_based_actions_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    players: Query<&Player>,
    creatures: Query<&Creature>,
) {
    // Check and apply state-based actions
}
```

## Integration with Existing Codebase

### Required Changes
1. Update `main.rs` to include new systems
2. Modify `card.rs` to support stack interactions
3. Enhance `player.rs` for priority handling
4. Add phase-specific restrictions to card playing logic

### New Resource Types
```rust
#[derive(Resource)]
pub struct GameState {
    turn_number: u32,
    active_player: Entity,
    current_phase: Phase,
    priority_holder: Entity,
}
```

## Testing Strategy

1. Unit Tests
   - Phase transitions
   - Priority passing
   - Stack resolution
   - State-based actions

2. Integration Tests
   - Full turn cycle
   - Multiple player interactions
   - Complex stack interactions

## Next Steps

1. Implement basic phase system
2. Add priority management
3. Develop stack mechanics
4. Integrate with existing card and player systems
5. Add state-based action checks
6. Implement timing restrictions
7. Add comprehensive testing

## Notes

- All systems should be implemented as Bevy ECS systems
- Use events for phase transitions and stack interactions
- Maintain clear separation between game rules and UI logic
- Consider networking implications in system design 