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

## ECS Implementation Approach

Rummage uses Bevy's Entity Component System (ECS) architecture to implement these rules in a way that is both performant and flexible:

- **Entities** represent game objects like cards, players, and game zones
- **Components** store data about these entities (e.g., a card's power/toughness, a player's life total)
- **Systems** implement the game logic that processes these components
- **Events** communicate game state changes between systems

This approach allows for a clean separation between game data and logic, making the implementation more maintainable and testable. For details on how we use ECS, see the [Entity Component System](../development/bevy_guide/ecs.md) guide.

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

Each phase and step is represented in the ECS as a distinct game state, with systems that handle the transitions between them and the actions that occur during each.

### Game Zones

MTG has the following game zones, each with specific rules for how cards interact with them:

- Library
- Hand
- Battlefield
- Graveyard
- Stack
- Exile
- Command (primarily used in Commander format)

In our implementation, zones are entities with components that track their contained cards. Systems handle the movement of cards between zones according to the rules.

### Card Types

The system implements all standard MTG card types:

- Land
- Creature
- Artifact
- Enchantment
- Planeswalker
- Instant
- Sorcery

Cards are represented as entities with components that define their properties, abilities, and current state.

### Stack and Priority

The stack is a fundamental MTG mechanic that determines the order in which spells and abilities resolve:

- Players receive priority in turn order
- The active player gets priority first
- Spells and abilities use the stack (with exceptions like mana abilities)
- The stack resolves in LIFO (Last In, First Out) order

Our implementation uses a dedicated stack system that manages the ordering and resolution of spells and abilities, integrated with the priority system that determines which player can act.

### State-Based Actions

State-based actions are checks that the game performs whenever a player would receive priority, handling conditions such as:

- Creatures with 0 or less toughness being put into graveyards
- Players with 0 or less life losing the game
- Auras without legal enchantment targets being put into graveyards
- And many more

State-based actions are implemented as systems that run at specific points in the game loop, checking for and applying these conditions.

## Implementation Details

The core rules are implemented in a format-agnostic way to allow for:

1. Consistent behavior across different formats
2. Extension points for format-specific rules
3. Reuse of common game logic

This modularity is achieved through Bevy's plugin system, which allows format-specific code to modify or extend the core behavior.

## Format Customization

The format-specific rules (like those for Commander) build upon these core rules by:

1. Overriding certain behaviors where the format differs
2. Adding new rules and mechanics specific to the format
3. Adjusting starting parameters (life totals, deck construction rules, etc.)

## Connecting to Commander Format

The Commander format extends these core rules with specific additions and modifications:

- The Command Zone becomes a central element of gameplay
- Commander Tax is applied when casting your commander from the Command Zone
- Starting life totals are increased to 40
- Decks must adhere to Color Identity restrictions
- Commander Damage tracking becomes relevant

These extensions are implemented through additional components, systems, and events that interact with the core rule systems. The interface between core rules and format-specific rules is carefully designed to maintain consistency while allowing for format uniqueness.

For the complete implementation of Commander-specific rules and mechanics, proceed to the [Commander Format](../formats/commander/index.md) section.

---

Next: [Turn Structure](turn_structure/index.md) 