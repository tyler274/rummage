# Commander Format Documentation

This section covers the implementation of the Magic: The Gathering Commander format in the Rummage game engine.

## Table of Contents

1. [Overview](#overview)
2. [Structure](#structure)
3. [Key Commander Rules](#key-commander-rules)
4. [Testing Approach](#testing-approach)

## Overview

Commander (formerly known as Elder Dragon Highlander or EDH) is a multiplayer format for Magic: The Gathering with the following key characteristics:

- 100-card singleton format (only one copy of each card except for basic lands)
- Each player has a legendary creature designated as their "commander"
- Deck can only include cards that match the color identity of the commander
- Players start with 40 life
- Commanders begin in the Command Zone and can be cast for an increasing cost
- 21 combat damage from a single commander causes a player to lose

This documentation outlines our implementation of these rules and mechanics in the Rummage game engine.

## Structure

The documentation is organized into the following sections:

- [Overview](overview/index.md) - High-level overview of the Commander format and implementation approach
- [Game Mechanics](game_mechanics/index.md) - Core game state and mechanics implementation
  - Game State Management
  - State-Based Actions
  - Random Mechanics (coin flips, dice rolls)
- [Player Mechanics](player_mechanics/index.md) - Player-specific rules and interactions
  - Life Total Management
  - Commander Tax
  - Color Identity
- [Game Zones](zones/index.md) - Implementation of game zones, especially the Command Zone
  - Command Zone
  - Zone Transfers
  - Zone-specific Rules
- [Turns and Phases](turns_and_phases/index.md) - Turn structure and phase management
  - Turn Order
  - Phase Management
  - Multiplayer Considerations
- [Stack and Priority](stack_and_priority/index.md) - Stack implementation and priority system
  - Priority Passing
  - Stack Resolution
  - Special Timing Rules
- [Combat](combat/index.md) - Combat mechanics including commander damage
  - Combat Phases
  - Commander Damage Tracking
  - Multiplayer Combat
- [Special Rules](special_rules/index.md) - Format-specific rules and unique mechanics
  - Partner Commanders
  - Commander Death Triggers
  - Commander-specific Abilities

## Key Commander Rules

The following key Commander rules are implemented in our engine:

| Rule | Description | Implementation Status |
|------|-------------|----------------------|
| Singleton | Only one copy of each card allowed (except basic lands) | ✅ |
| Commander | Legendary creature in command zone | ✅ |
| Color Identity | Cards must match commander's color identity | ✅ |
| Command Zone | Special zone for commanders | ✅ |
| Commander Tax | Additional {2} cost each time cast from command zone | ✅ |
| Commander Damage | 21 combat damage from a single commander | ✅ |
| Starting Life | 40 life points | ✅ |
| Commander Replacement | Optional replacement to command zone | ✅ |
| Partner Commanders | Special commanders that can be paired | 🔄 |
| Commander Ninjutsu | Special ability for certain commanders | ⚠️ |
| Commander-specific Cards | Cards that reference the command zone or commanders | 🔄 |

## Testing Approach

Each section includes detailed test cases to validate the correct implementation of Commander rules. Our testing approach ensures:

1. Full coverage of Commander-specific rules
2. Edge case handling for unique interactions
3. Performance validation for multiplayer scenarios
4. Verification of correct rule application in complex board states

For implementation details, see the respective sections.

---

For more information on the official Commander rules, refer to the [Commander Format Rules](https://mtgcommander.net/index.php/rules/). 