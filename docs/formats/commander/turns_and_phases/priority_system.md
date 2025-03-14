# Priority System

## Overview

The priority system determines which player may take game actions at any given time during a Commander game. In a multiplayer format like Commander, properly managing priority is essential for maintaining game flow and ensuring all players have appropriate opportunities to respond.

## Priority Rules

1. The active player receives priority at the beginning of each step or phase, except the untap and cleanup steps
2. When a player has priority, they may:
   - Cast a spell
   - Activate an ability
   - Take a special action
   - Pass priority
3. After a player casts a spell or activates an ability, they receive priority again
4. Priority passes in turn order (clockwise) from the active player
5. When all players pass priority in succession:
   - If the stack is not empty, the top item on the stack resolves, then the active player gets priority
   - If the stack is empty, the current step or phase ends and the game moves to the next step or phase

## Implementation

The priority system is tracked within the `TurnManager` resource:

```rust
#[derive(Resource)]
pub struct TurnManager {
    // Priority system fields
    pub priority_player_index: usize,
    pub all_players_passed: bool,
    pub stack_is_empty: bool,
    
    // Step tracking for UI and timers
    pub step_started_at: std::time::Instant,
    pub auto_pass_enabled: bool,
    pub auto_pass_delay: std::time::Duration,
    
    // Other fields
    // ...
}
```

## Special Priority Cases

### Simultaneous Actions

When multiple players would take actions simultaneously:

1. First, the active player performs all their actions in any order they choose
2. Then, each non-active player in turn order performs their actions

### Auto-Pass System

For better digital gameplay experience, the system implements an auto-pass feature:

```rust
pub fn check_auto_pass_system(
    time: Res<Time>,
    mut turn_manager: ResMut<TurnManager>,
    player_states: Query<&PlayerState>,
) {
    if turn_manager.auto_pass_enabled && 
       time.elapsed_seconds() - turn_manager.step_started_at > turn_manager.auto_pass_delay {
        // Auto-pass for the current priority player if they have no available actions
        // ...
    }
}
```

### Stop Button

Players can place "stops" on specific phases and steps to ensure they receive priority, even when auto-pass is enabled:

```rust
#[derive(Component)]
pub struct PlayerStops {
    pub phases: HashMap<Phase, bool>,
    // Additional stop settings
}
```

## UI Representation

The priority system is visually represented to players through:

1. A highlighted border around the active player's avatar
2. A timer indicator showing how long the current player has had priority
3. Special effects when priority passes
4. Visual cues for auto-pass situations

## Multiplayer Considerations

In Commander, the priority system manages additional complexity:

- Tracking priority across 3-6 players
- Handling "table talk" periods during complex game states
- Supporting take-backs by mutual agreement (optional house rule)
- Managing disconnected players in digital play 