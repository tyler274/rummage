# Turn Structure

This document provides an overview of the turn structure rules in Magic: The Gathering. For the detailed implementation in Rummage, please see [Turn Structure Implementation](../mtg_core/turn_structure/index.md).

## Overview of Turn Structure

A turn in Magic: The Gathering consists of five phases, some of which are divided into steps. The phases proceed in the following order:

1. **Beginning Phase**
   - Untap Step
   - Upkeep Step
   - Draw Step

2. **First Main Phase**

3. **Combat Phase**
   - Beginning of Combat Step
   - Declare Attackers Step
   - Declare Blockers Step
   - Combat Damage Step
   - End of Combat Step

4. **Second Main Phase**

5. **Ending Phase**
   - End Step
   - Cleanup Step

## Phase and Step Rules

Phase transitions follow these rules:

1. Each phase or step begins with game events that happen automatically
2. Then, the active player receives priority
3. When all players pass priority in succession with an empty stack, the phase or step ends
4. The game proceeds to the next phase or step

## Special Turn Structure Rules

### Priority

- Players receive priority in APNAP (Active Player, Non-Active Player) order
- No player receives priority during the untap step or cleanup step (unless a trigger occurs)
- The active player receives priority first in each phase/step

### Skipping Steps

- If no player has a triggered ability triggering at the beginning of a step and no player takes an action during that step, that step is skipped
- The upkeep step, draw step, and end step are never skipped this way

### Combat Phase

The combat phase has special rules:
- Only creatures can attack
- Creatures with summoning sickness cannot attack
- The active player chooses which creatures attack and which player or planeswalker they attack
- The defending player(s) choose which creatures block and how blocking is assigned

## Format-Specific Variations

Different formats may have specific variations in how turns proceed:

- **Commander**: See [Commander Turn Structure](../formats/commander/turns_and_phases/turn_structure.md) for multiplayer turn order rules
- **Two-Headed Giant**: Teams share turns and certain steps
- **Planechase**: Additional actions may be taken during main phases

## Related Documentation

For detailed information about turn structure in Rummage, please see:

- [Turn Structure Implementation](../mtg_core/turn_structure/index.md): Core implementation details
- [Combat](../mtg_core/combat/index.md): How combat fits into the turn structure
- [Stack](../mtg_core/stack/index.md): How spells and abilities interact with turn structure
- [Priority System](../mtg_core/turn_structure/priority.md): Detailed rules on priority

For format-specific implementations:
- [Commander Turn Structure](../formats/commander/turns_and_phases/turn_structure.md): Commander-specific turn rules
- [Multiplayer Turns](../formats/commander/turns_and_phases/multiplayer_turns.md): How turns work in multiplayer games 