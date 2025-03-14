# The Stack

This document provides an overview of the stack rules in Magic: The Gathering. For the detailed implementation in Rummage, please see [Stack and Priority System](../mtg_core/stack/index.md).

## What is the Stack?

The stack is a zone in Magic: The Gathering where spells and abilities exist while waiting to resolve. It uses the "last in, first out" (LIFO) principle - the most recently added object on the stack is the first to resolve.

## Stack Rules

### Adding to the Stack

The following objects use the stack:

1. **Spells**: When a player casts a spell, it's put on the stack.
2. **Activated Abilities**: When a player activates an ability, it's put on the stack.
3. **Triggered Abilities**: When a triggered ability triggers, it's put on the stack the next time a player would receive priority.

### Resolution Process

1. Each player, in turn order starting with the active player, receives priority.
2. A player with priority may:
   - Cast a spell
   - Activate an ability
   - Pass priority
3. When all players pass priority in succession:
   - If the stack is empty, the current phase or step ends.
   - If the stack has objects, the topmost object resolves.

### Resolving Stack Objects

When a spell or ability resolves:

1. Its instructions are followed in order.
2. If it's a permanent spell, it enters the battlefield.
3. It leaves the stack and ceases to exist (unless it's a permanent spell).

### Special Stack Rules

- **Split second**: Spells with split second prevent players from casting spells or activating abilities while they're on the stack (except for mana abilities and certain special actions).
- **Counterspells**: These spells target other spells on the stack and prevent them from resolving.
- **Mana abilities**: These abilities don't use the stack and resolve immediately.
- **Special actions**: Certain game actions don't use the stack (e.g., playing a land).

## Related Documentation

For the detailed implementation of the stack in Rummage, including code examples and integration with other systems, see:

- [Stack and Priority System](../mtg_core/stack/index.md): Core implementation details
- [Priority System](../formats/commander/turns_and_phases/priority_system.md): Detailed rules on priority
- [Spell Casting](../mtg_core/casting_spells.md): Rules for casting spells
- [Abilities](../mtg_core/abilities.md): Types of abilities and how they work 