# MTG Core Rules

This section contains documentation about the core Magic: The Gathering rules implementation in Rummage. These are the fundamental rules that apply to all MTG formats, not just Commander.

## Overview

The MTG Core Rules module implements the foundational game mechanics defined in the [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules). These include:

- Basic game structure and turn progression
- Card types and properties
- Zones and zone transitions
- Stack implementation and spell/ability resolution
- State-based actions
- Combat mechanics
- Mana and casting costs

## Core Game Elements

The following core elements are implemented according to the MTG Comprehensive Rules:

### Turn Structure and Phases

The standard MTG turn structure consists of the following phases, each with specific steps:

1. Beginning Phase
   - Untap Step
   - Upkeep Step
   - Draw Step
2. Main Phase (Pre-Combat)
3. Combat Phase
   - Beginning of Combat Step
   - Declare Attackers Step
   - Declare Blockers Step
   - Combat Damage Step
   - End of Combat Step
4. Main Phase (Post-Combat)
5. Ending Phase
   - End Step
   - Cleanup Step

### Game Zones

MTG has the following game zones, each with specific rules for how cards interact with them:

- Library
- Hand
- Battlefield
- Graveyard
- Stack
- Exile
- Command (primarily used in Commander format)

### Card Types

The system implements all standard MTG card types:

- Land
- Creature
- Artifact
- Enchantment
- Planeswalker
- Instant
- Sorcery

### Stack and Priority

The stack is a fundamental MTG mechanic that determines the order in which spells and abilities resolve:

- Players receive priority in turn order
- The active player gets priority first
- Spells and abilities use the stack (with exceptions like mana abilities)
- The stack resolves in LIFO (Last In, First Out) order

### State-Based Actions

State-based actions are checks that the game performs whenever a player would receive priority, handling conditions such as:

- Creatures with 0 or less toughness being put into graveyards
- Players with 0 or less life losing the game
- Auras without legal enchantment targets being put into graveyards
- And many more

## Implementation Details

The core rules are implemented in a format-agnostic way to allow for:

1. Consistent behavior across different formats
2. Extension points for format-specific rules
3. Reuse of common game logic

## Format Customization

The format-specific rules (like those for Commander) build upon these core rules by:

1. Overriding certain behaviors where the format differs
2. Adding new rules and mechanics specific to the format
3. Adjusting starting parameters (life totals, deck construction rules, etc.)

For Commander-specific rules implementation, see the [Commander Format](../formats/commander/index.md) section.

---

Next: [Turn Structure](turn_structure/index.md) 