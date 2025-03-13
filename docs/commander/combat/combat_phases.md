# Combat Phases

## Overview

The Combat Phase in Commander follows the standard Magic: The Gathering combat sequence, with special considerations for the multiplayer nature of the format. This document provides an overview of the combat phase structure and links to detailed implementation documentation for each step.

## Combat Phase Sequence

The Combat Phase consists of five distinct steps:

1. **Beginning of Combat Step** - The phase begins and "at beginning of combat" triggered abilities go on the stack
2. **Declare Attackers Step** - The active player declares attackers and "when attacks" triggered abilities go on the stack
3. **Declare Blockers Step** - Each defending player declares blockers and "when blocks/blocked" triggered abilities go on the stack
4. **Combat Damage Step** - Combat damage is assigned and dealt, and "when deals damage" triggered abilities go on the stack
5. **End of Combat Step** - The phase ends and "at end of combat" triggered abilities go on the stack

## Implementation Architecture

Combat phases are implemented using a combination of phase-specific systems and a shared combat state:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    FirstStrike,
    CombatDamage,
    End,
}

// Systems for handling different combat steps
pub fn beginning_of_combat_system(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut commands: Commands,
    // Other system parameters
) {
    // Implementation
}

pub fn declare_attackers_system(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    // Other system parameters
) {
    // Implementation
}

// And so on for other combat steps
```

## Detailed Phase Documentation

Each combat step has its own specialized implementation with unique rules and edge cases:

- [Beginning of Combat](beginning_of_combat.md) - Initialization of combat and "beginning of combat" triggers
- [Declare Attackers](declare_attackers.md) - Attack declaration, restrictions, and requirements 
- [Declare Blockers](declare_blockers.md) - Block declaration, restrictions, and requirements
- [Combat Damage](combat_damage.md) - Damage assignment, ordering, and resolution
- [End of Combat](end_of_combat.md) - Combat cleanup and "end of combat" triggers

## Combat Phase Transitions

The transition between combat steps is managed by the `TurnManager`, which ensures that:

1. Each step is processed in the correct order
2. Priority is passed to all players in turn order during each step
3. The stack is emptied before proceeding to the next step
4. Any special effects that modify the combat phase flow are properly handled

## Integration with Turn Structure

The combat phase is integrated into the overall turn structure via the `TurnManager`:

```rust
impl TurnManager {
    pub fn advance_phase(&mut self) -> Result<Phase, TurnError> {
        self.current_phase = match self.current_phase {
            // Other phases...
            Phase::Combat(CombatStep::Beginning) => Phase::Combat(CombatStep::DeclareAttackers),
            Phase::Combat(CombatStep::DeclareAttackers) => Phase::Combat(CombatStep::DeclareBlockers),
            Phase::Combat(CombatStep::DeclareBlockers) => {
                if self.has_first_strike_creatures() {
                    Phase::Combat(CombatStep::FirstStrike)
                } else {
                    Phase::Combat(CombatStep::CombatDamage)
                }
            },
            Phase::Combat(CombatStep::FirstStrike) => Phase::Combat(CombatStep::CombatDamage),
            Phase::Combat(CombatStep::CombatDamage) => Phase::Combat(CombatStep::End),
            Phase::Combat(CombatStep::End) => Phase::Postcombat(PostcombatStep::Main),
            // Other phases...
        };
        
        // Further implementation
    }
}
``` 