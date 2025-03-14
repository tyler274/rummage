# Priority System

This document details the implementation of the priority system in Rummage, which determines which player can take actions at any given time during a game of Magic: The Gathering.

## Overview

The priority system is a fundamental part of Magic: The Gathering's turn structure. It determines when players can cast spells, activate abilities, and take other game actions. Understanding and correctly implementing the priority system is essential for proper game flow.

## Core Priority Rules

The basic rules of priority in MTG are:

1. The active player receives priority first in each step and phase
2. When a player has priority, they may:
   - Cast a spell
   - Activate an ability
   - Take a special action
   - Pass priority
3. When a player passes priority, the next player in turn order receives priority
4. When all players pass priority in succession:
   - If the stack is empty, the current step or phase ends
   - If the stack has objects, the top object on the stack resolves, then the active player gets priority again

## Implementation

In Rummage, the priority system is implemented as follows:

```rust
#[derive(Resource)]
pub struct PrioritySystem {
    // The player who currently has priority
    pub current_player: Entity,
    
    // Set of players who have passed priority in succession
    pub passed_players: HashSet<Entity>,
    
    // Whether the priority system is currently active
    pub active: bool,
}

#[derive(Event)]
pub struct PriorityEvent {
    pub player: Entity,
    pub action: PriorityAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityAction {
    Receive,  // Player receives priority
    Pass,     // Player passes priority
    TakeAction, // Player takes an action (cast spell, activate ability, etc.)
}
```

## Priority Flow

The flow of priority follows this pattern:

1. **Phase/Step Start**: At the beginning of each phase or step, the active player receives priority
2. **Action Taken**: If a player takes an action, all players who have passed are reset, and priority returns to the active player
3. **Passing**: When a player passes, the next player in turn order receives priority
4. **Resolution**: When all players pass in succession, either the top of the stack resolves or the phase/step ends

### Priority System Implementation

```rust
pub fn handle_priority_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut priority_events: EventReader<PriorityEvent>,
    game_state: Res<GameState>,
    stack: Res<Stack>,
) {
    for event in priority_events.iter() {
        match event.action {
            PriorityAction::Pass => {
                // Record that this player passed
                priority.passed_players.insert(event.player);
                
                // Check if all players have passed
                if priority.passed_players.len() == game_state.players.len() {
                    // All players have passed
                    if !stack.items.is_empty() {
                        // Resolve top of stack
                        commands.add(resolve_stack_command());
                        
                        // Reset passed players
                        priority.passed_players.clear();
                        
                        // Active player gets priority again
                        priority.current_player = game_state.active_player;
                    } else {
                        // Stack is empty, end current phase/step
                        commands.add(advance_phase_command());
                        
                        // Priority will be set by the phase transition system
                        priority.active = false;
                    }
                } else {
                    // Not all players have passed, give priority to next player
                    let next_player = get_next_player(event.player, &game_state);
                    priority.current_player = next_player;
                }
            },
            PriorityAction::TakeAction => {
                // Player took an action, reset passed players
                priority.passed_players.clear();
                
                // Active player gets priority again
                priority.current_player = game_state.active_player;
            },
            PriorityAction::Receive => {
                // Player receives priority (usually at beginning of phase/step)
                priority.current_player = event.player;
                priority.active = true;
            }
        }
    }
}
```

## Special Priority Rules

### No Priority Phases

Some steps do not normally grant players priority:

- **Untap Step**: No player receives priority during this step
- **Cleanup Step**: No player receives priority unless a triggered ability triggers

```rust
pub fn should_grant_priority(phase: Phase, step: Step) -> bool {
    match (phase, step) {
        (Phase::Beginning, Step::Untap) => false,
        (Phase::Ending, Step::Cleanup) => false,
        _ => true,
    }
}
```

### Triggered Abilities During No-Priority Steps

If a triggered ability triggers during a step where players don't normally receive priority, players will receive priority:

```rust
pub fn handle_cleanup_triggers(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    game_state: Res<GameState>,
    triggers: Res<TriggeredAbilities>,
) {
    // If we're in cleanup step and there are triggers
    if game_state.current_phase == Phase::Ending && 
       game_state.current_step == Step::Cleanup &&
       !triggers.pending.is_empty() {
        
        // Grant priority to active player
        priority.active = true;
        priority.current_player = game_state.active_player;
        priority.passed_players.clear();
    }
}
```

## APNAP Order

When multiple players would receive priority simultaneously (such as for triggered abilities), they are processed in APNAP (Active Player, Non-Active Player) order:

```rust
pub fn get_players_in_apnap_order(game_state: &GameState) -> Vec<Entity> {
    let mut players = Vec::new();
    
    // Start with active player
    let mut current = game_state.active_player;
    players.push(current);
    
    // Add remaining players in turn order
    for _ in 1..game_state.players.len() {
        current = get_next_player(current, game_state);
        players.push(current);
    }
    
    players
}
```

## Integration with Other Systems

The priority system integrates with:

1. **Turn Structure**: Phase and step transitions affect priority
2. **Stack System**: Stack resolution and priority are tightly coupled
3. **Action System**: Player actions affect priority flow
4. **UI System**: The UI must indicate which player has priority

## Implementation Status

The priority system implementation currently:

- ✅ Handles basic priority passing
- ✅ Integrates with stack resolution
- ✅ Implements APNAP order
- ✅ Handles special steps without priority
- ✅ Supports triggered abilities during cleanup
- 🔄 Implementing special actions that don't use the stack
- 🔄 Handling priority with split second spells

---

Next: [Turn Phases](phases.md) 