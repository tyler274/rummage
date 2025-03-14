# Turns and Phases

This section covers the implementation of turn structure and phases in the Commander format.

## Contents

- [Turn Structure](turn_structure.md) - Complete turn sequence implementation
- [Phase Management](phase_management.md) - Handling of individual phases and steps
- [Priority System](priority_system.md) - Priority passing within phases
- [Extra Turns & Modifications](extra_turns.md) - Extra turns and turn modification effects
- [Multiplayer Turn Handling](multiplayer_turn_handling.md) - Multiplayer-specific turn considerations

The turns and phases section defines how the turn structure of Commander games is implemented, with special consideration for multiplayer dynamics:

- Standard Magic turn structure (untap, upkeep, draw, main phases, combat, etc.)
- Multiplayer turn order determination and rotation
- Special handling for simultaneous player actions
- Turn-based effects in multiplayer contexts
- Turn modification effects (extra turns, skipped phases, etc.)

While the basic turn structure in Commander follows standard Magic rules, the multiplayer nature of the format introduces complexity in turn management, particularly around priority passing and simultaneous effects. 