# Architecture Overview

## System Architecture

The Commander rules engine is organized into several interconnected modules that work together to provide a complete implementation of the Commander format:

1. **Game State Management**
   - Core state tracking and game flow coordination
   - Central resource for game metadata and state
   - Integration point for all other systems

2. **Player Management** 
   - Player data structures and tracking
   - Life total management (starting at 40)
   - Commander damage tracking

3. **Command Zone Management**
   - Special zone for Commander cards
   - Commander casting and tax implementation
   - Zone transition rules

4. **Turn Structure & Phases**
   - Complete turn sequence implementation
   - Priority passing in multiplayer context
   - Phase-based effects and triggers

5. **Combat System**
   - Multiplayer combat implementation
   - Commander damage tracking
   - Attack declaration and blocking in multiplayer

6. **Priority & Stack**
   - Priority passing algorithms for multiplayer
   - Stack implementation for spells and abilities
   - Resolution mechanics in complex scenarios

7. **State-Based Actions**
   - Game state checks including commander damage threshold (21)
   - Format-specific state checks
   - Automatic game actions

8. **Special Commander Rules**
   - Color identity validation
   - Commander zone movement replacement effects
   - Partner and Background mechanics

9. **Multiplayer Politics**
   - Voting mechanics
   - Deal-making systems
   - Multiplayer-specific card effects

## Integration with Bevy ECS

The Commander implementation leverages Bevy ECS (Entity Component System) for game state management:

```rust
// Game state as a Bevy resource
#[derive(Resource)]
pub struct CommanderGameState {
    // Game state fields
}

// Player as an entity with components
#[derive(Component)]
pub struct CommanderPlayer {
    // Player data
}

// Systems for game logic
fn process_commander_damage(
    mut game_state: ResMut<CommanderGameState>,
    query: Query<(&CommanderPlayer, &CommanderDamage)>,
) {
    // Implementation
}
```

The architecture allows for clean separation of concerns while maintaining the complex relationships between different aspects of the Commander format. 