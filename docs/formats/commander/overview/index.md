# Commander Overview

This section provides a high-level overview of the Commander format implementation in our game engine.

## Contents

- [Format Rules](format_rules.md) - Core rules specific to the Commander format
- [Architecture](architecture.md) - Overview of the implementation architecture
- [Implementation Approach](implementation.md) - General approach to implementing the Commander format

## Format Summary

The Commander format is a multiplayer variant of Magic: The Gathering with special rules including:
- Each player begins the game with a designated legendary creature as their "commander"
- Players start with 40 life
- A player can cast their commander from the command zone for its mana cost, plus an additional 2 mana for each previous time it's been cast
- If a commander would be put into a library, hand, graveyard or exile, its owner may choose to move it to the command zone
- A player who has been dealt 21 or more combat damage by the same commander loses the game

## Implementation Principles

Our implementation of the Commander format follows these key principles:

1. **Rule Accuracy** - Faithful implementation of the official Commander rules
2. **Performance** - Optimized for multiplayer games with complex board states
3. **Extensibility** - Designed to easily incorporate new Commander-specific cards and mechanics
4. **Testability** - Comprehensive test suite for validating format-specific rules

See the [Implementation Approach](implementation.md) document for more detailed information on how these principles are applied in our codebase.

## Related Sections

- [Game Mechanics](../game_mechanics/index.md) - For detailed mechanics implementation
- [Combat](../combat/index.md) - For Commander damage tracking and combat interactions
- [Game Zones](../zones/index.md) - For Command Zone implementation details 