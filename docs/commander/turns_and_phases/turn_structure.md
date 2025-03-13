# Turn Structure

## Overview

The Turn Structure module manages the flow of a Commander game, handling the sequence of phases and steps within each player's turn. It coordinates player transitions, priority passing, and phase-specific actions while accounting for the multiplayer nature of Commander.

## Core Turn Sequence

A turn in Commander follows the standard Magic: The Gathering sequence:

1. **Beginning Phase**
   - Untap Step
   - Upkeep Step
   - Draw Step

2. **Precombat Main Phase**

3. **Combat Phase**
   - Beginning of Combat Step
   - Declare Attackers Step
   - Declare Blockers Step
   - Combat Damage Step
   - End of Combat Step

4. **Postcombat Main Phase**

5. **Ending Phase**
   - End Step
   - Cleanup Step

## Multiplayer Considerations

In Commander, turn order proceeds clockwise from the starting player. The format introduces special considerations:

- **Turn Order Determination**: Typically random at the start of the game
- **Player Elimination**: When a player loses, turns continue with the remaining players
- **Extra Turns**: Cards that grant extra turns work the same as in standard Magic
- **"Skip your next turn" effects**: These follow standard Magic rules but can have significant political impact

## Implementation

The turn structure is managed through a combination of phase enums and the `TurnManager` resource:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep),
}

// Other step enums defined similarly

#[derive(Resource)]
pub struct TurnManager {
    // Current turn information
    pub current_phase: Phase,
    pub active_player_index: usize,
    pub turn_number: u32,
    
    // Priority system
    pub priority_player_index: usize,
    pub all_players_passed: bool,
    pub stack_is_empty: bool,
    
    // Multiplayer tracking
    pub player_order: Vec<Entity>,
    pub extra_turns: VecDeque<(Entity, ExtraTurnSource)>,
    pub skipped_turns: HashSet<Entity>,
}
```

## Special Turn Rules

Commander implements all standard special turn rules from Magic, including:

- Extra turns
- Skipped turns
- Modified turns (e.g., additional combat phases)
- Turn control effects

These are tracked and managed by the `TurnManager` resource. 