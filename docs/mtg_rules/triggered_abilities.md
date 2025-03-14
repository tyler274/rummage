# Triggered Abilities

This document provides an overview of triggered abilities in Magic: The Gathering.

## What Are Triggered Abilities?

Triggered abilities are abilities that automatically trigger when certain game events occur. They are written as "When/Whenever/At [event], [effect]."

Examples:
- "When this creature enters the battlefield, draw a card."
- "Whenever an opponent gains life, you may draw a card."
- "At the beginning of your upkeep, you gain 1 life."

## Triggers

Triggered abilities can trigger based on:

1. **State Changes**: "When [state change] occurs..."
   - A creature entering the battlefield
   - A creature dying
   - A creature attacking or blocking

2. **Phase/Step Changes**: "At the beginning of [phase]..."
   - At the beginning of upkeep
   - At the beginning of your draw step
   - At the beginning of combat

3. **Player Actions**: "Whenever a player [action]..."
   - Whenever a player casts a spell
   - Whenever a player plays a land
   - Whenever a player activates an ability

## Resolution Process

When a triggered ability triggers:

1. The ability is put on the stack the next time a player would receive priority
2. If multiple abilities trigger at the same time, they go on the stack in APNAP order (Active Player, Non-Active Player)
3. If multiple abilities trigger for a single player, that player chooses the order
4. Triggered abilities resolve like any other ability on the stack

## Special Types of Triggered Abilities

### Delayed Triggered Abilities

These set up an effect that will happen later:
- "At the beginning of the next end step, return the exiled card to its owner's hand."
- "At the beginning of your next upkeep, sacrifice this permanent."

### State Triggers

These continually check if a condition is true:
- "When you have no cards in hand, sacrifice this permanent."
- "When you control no creatures, sacrifice this enchantment."

## Implementation Note

For the detailed implementation of triggered abilities in Rummage, including code examples and integration with other systems, see:

- [Abilities](../mtg_core/abilities.md): General abilities documentation
- [Triggered Ability Implementation](../formats/commander/game_mechanics/triggered_abilities.md): Format-specific details

These documents provide technical details on how triggered abilities are handled in the game engine. 