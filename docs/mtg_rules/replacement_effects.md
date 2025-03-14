# Replacement Effects

This document provides an overview of replacement effects in Magic: The Gathering.

## What Are Replacement Effects?

Replacement effects are continuous effects that watch for a particular event to occur and completely or partially replace that event with a different event. They use the words "instead of," "rather than," or "skip."

Examples:
- "If you would draw a card, instead draw two cards."
- "If a creature would die, exile it instead."
- "If damage would be dealt to you, prevent that damage. You gain that much life instead."

## How Replacement Effects Work

Replacement effects modify events before they occur:

1. They don't use the stack and can't be responded to
2. They apply before the event happens (not after)
3. Only one replacement effect can apply to a particular event
4. If multiple replacement effects could apply, the affected player or controller of the affected object chooses which to apply first
5. After applying one replacement effect, the resulting event is checked again for other applicable replacement effects

## Types of Replacement Effects

### Self-Replacement Effects

These modify how a spell or ability resolves:
- "Draw three cards. You lose 3 life unless you discard a card for each card drawn this way."

### Prevention Effects

A special type of replacement effect that prevents damage:
- "Prevent all damage that would be dealt to you and creatures you control this turn."
- "If a source would deal damage to target creature, prevent 3 of that damage."

### Redirection Effects

These change where an effect applies:
- "If a source would deal damage to you, it deals that damage to target creature instead."

## Multiple Replacement Effects

When multiple replacement effects could apply to the same event:

1. The affected player (or controller of the affected object) chooses which to apply first
2. After applying one effect, check if any remaining replacement effects still apply
3. If so, the player chooses among those, and so on
4. This continues until no more applicable replacement effects remain

## Implementation Note

For information on the implementation of replacement effects in Rummage, see:

- [Effects](../mtg_core/effects/index.md): General effects documentation, if available
- [Static Abilities](../mtg_core/abilities.md): Replacement effects are often implemented as static abilities

These documents provide technical details on how replacement effects are handled in the game engine. 