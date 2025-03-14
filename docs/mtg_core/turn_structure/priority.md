# Priority System

This document describes the implementation of Magic: The Gathering's priority system in Rummage.

## Overview

The priority system determines when players can take actions during a game of Magic: The Gathering. It's a fundamental component that controls the game's flow and ensures players have appropriate opportunities to act.

## Core Priority Rules

In Magic: The Gathering, priority follows these key rules:

1. The active player receives priority first in each step and phase.
2. When a player has priority, they may cast spells, activate abilities, or pass.
3. When a player passes priority, the next player in turn order receives priority.
4. When all players pass priority in succession with an empty stack, the current step or phase ends.
5. When all players pass priority in succession with objects on the stack, the top object on the stack resolves, then the active player receives priority.

## Implementation Details

The priority system in Rummage is implemented through a combination of resources and systems:

```rust
#[derive(Resource, Debug, Clone)]
pub struct PriorityManager {
    pub current_player: Entity,
    pub all_passed: bool,
    pub stack_empty_when_passed: bool,
    pub last_to_act: Option<Entity>,
    pub player_order: Vec<Entity>,
}

impl PriorityManager {
    pub fn new(starting_player: Entity, player_order: Vec<Entity>) -> Self {
        PriorityManager {
            current_player: starting_player,
            all_passed: false,
            stack_empty_when_passed: true,
            last_to_act: None,
            player_order,
        }
    }
    
    pub fn pass_priority(&mut self, stack: &Stack) -> PriorityResult {
        let current_index = self.player_order
            .iter()
            .position(|&p| p == self.current_player)
            .expect("Current player not found in player order");
        
        let next_index = (current_index + 1) % self.player_order.len();
        let next_player = self.player_order[next_index];
        
        // Record if this is the last player to act
        if let Some(last_player) = self.last_to_act {
            if last_player == self.current_player {
                // All players have passed priority
                self.all_passed = true;
                self.stack_empty_when_passed = stack.is_empty();
                
                if stack.is_empty() {
                    return PriorityResult::EndPhase;
                } else {
                    return PriorityResult::ResolveStack;
                }
            }
        } else {
            // First player to pass
            self.last_to_act = Some(self.current_player);
        }
        
        // Pass to next player
        self.current_player = next_player;
        PriorityResult::Continue
    }
    
    pub fn reset_for_new_phase(&mut self, active_player: Entity) {
        self.current_player = active_player;
        self.all_passed = false;
        self.stack_empty_when_passed = true;
        self.last_to_act = None;
    }
    
    pub fn reset_after_stack_resolution(&mut self, active_player: Entity) {
        self.current_player = active_player;
        self.all_passed = false;
        self.last_to_act = None;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriorityResult {
    Continue,      // Continue with priority passing
    EndPhase,      // End the current phase
    ResolveStack,  // Resolve the top of the stack
}
```

## Priority Systems

The following systems manage priority in the game:

1. **Initialize Priority System**: Sets up priority at the beginning of a phase
2. **Handle Priority Actions System**: Processes actions from the player with priority
3. **Pass Priority System**: Handles the passing of priority between players
4. **Stack Resolution System**: Resolves stack objects when all players pass

```rust
pub fn handle_priority_actions(
    mut commands: Commands,
    priority: Res<PriorityManager>,
    mut action_events: EventReader<PlayerAction>,
    mut pass_events: EventWriter<PassPriorityEvent>,
    mut stack: ResMut<Stack>,
    game_state: Res<GameState>,
) {
    for action in action_events.read() {
        if action.player != priority.current_player {
            // Only the player with priority can act
            continue;
        }
        
        match &action.action_type {
            ActionType::CastSpell { card, targets } => {
                // Handle casting a spell
                // ...
            },
            ActionType::ActivateAbility { source, ability_id, targets } => {
                // Handle activating an ability
                // ...
            },
            ActionType::PlayLand { card } => {
                // Handle playing a land
                // ...
            },
            ActionType::Pass => {
                // Player passes priority
                pass_events.send(PassPriorityEvent { 
                    player: action.player,
                    phase: game_state.current_phase.clone(),
                });
            },
            // Other action types...
        }
    }
}
```

## Special Phase Rules

Priority is handled differently in certain phases and steps:

1. **Untap Step**: No player receives priority
2. **Cleanup Step**: No player receives priority unless a triggered ability triggers
3. **Combat Damage Step**: Players receive priority after combat damage is dealt

```rust
pub fn should_receive_priority(phase: &Phase) -> bool {
    match phase {
        Phase::Beginning(BeginningStep::Untap) => false,
        Phase::Ending(EndingStep::Cleanup) => false,
        _ => true,
    }
}
```

## Integration with Other Systems

The priority system integrates closely with several other game systems:

- **Turn Structure**: Controls when phases begin and end based on priority passing
- **Stack**: Determines when objects on the stack resolve
- **Triggered Abilities**: Manages when triggered abilities are put on the stack
- **State-Based Actions**: Checks whenever a player would receive priority

## Format-Specific Extensions

For Commander-specific priority implementation details, see [Commander Priority System](../../formats/commander/turns_and_phases/priority_system.md).

## Related Documentation

- [Turn Structure](index.md): How turns are structured
- [Stack](../stack/index.md): How the stack handles spell and ability resolution
- [Commander Priority System](../../formats/commander/turns_and_phases/priority_system.md): Commander-specific priority rules 