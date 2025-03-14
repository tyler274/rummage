# Card Effects

Card effects are the mechanisms through which cards interact with the game state. This section documents how card effects are implemented, resolved, and tested in Rummage.

## Overview

Magic: The Gathering cards have a wide variety of effects that modify the game state in different ways:

- Adding or removing counters
- Dealing damage
- Moving cards between zones
- Modifying attributes of other cards
- Creating tokens
- Altering the rules of the game

The effect system in Rummage is designed to handle all these interactions in a consistent and extensible way.

## Implementation Approach

Effects are implemented using a combination of:

- **Components**: Data that defines the effect's properties
- **Systems**: Logic that processes and applies the effects
- **Events**: Notifications that trigger and respond to effects
- **Queries**: Selections of target entities to affect

## Core Components

- [Effect Resolution](effect_resolution.md): How effects are processed and applied
- [Targeting](targeting.md): How targets are selected and validated
- [Complex Interactions](complex_interactions.md): Handling interactions between multiple effects

## Integration

The effects system integrates with:

- [State Management](../../game_engine/state/index.md): Effects modify game state
- [Event System](../../game_engine/events/index.md): Effects are triggered by and generate events
- [MTG Rules](../../mtg_rules/index.md): Effects follow the comprehensive rules

## Testing

For information on how to test card effects, see [Effect Verification](../testing/effect_verification.md). 