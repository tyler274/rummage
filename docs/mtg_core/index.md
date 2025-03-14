# MTG Core Rules

This section documents the implementation of fundamental Magic: The Gathering rules in the Rummage engine.

## Overview

The MTG Core Rules module implements the foundational mechanics defined in the [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules), forming the basis for all supported game formats. These include:

- Turn structure and phase sequence
- Card types, characteristics, and properties
- Game zones and zone transitions
- Stack implementation and resolution mechanics
- State-based actions
- Combat system
- Mana and casting costs

## Implementation Architecture

Rummage implements MTG rules using Bevy's Entity Component System (ECS) architecture:

| MTG Concept | ECS Representation |
|-------------|-------------------|
| Cards, permanents, players | Entities |
| Card characteristics, states | Components |
| Rules procedures, actions | Systems |
| Game transitions, triggers | Events |

This architecture creates a clean separation between game data and logic, enabling:
- Higher testability of individual game mechanics
- Parallel processing of independent game systems
- Easier extension for format-specific rules
- Greater code modularity and maintainability

For implementation details, see [ECS Implementation](ecs_implementation.md).

## Core Game Elements

### Turn Structure

MTG turns follow a fixed sequence of phases and steps:

1. **Beginning Phase**
   - Untap: Active player untaps their permanents
   - Upkeep: Triggers "at beginning of upkeep" abilities
   - Draw: Active player draws a card

2. **Pre-Combat Main Phase**
   - Player may play lands and cast spells

3. **Combat Phase**
   - Beginning of Combat: Last chance for effects before attacks
   - Declare Attackers: Active player declares attacking creatures
   - Declare Blockers: Defending players assign blockers
   - Combat Damage: Creatures deal damage
   - End of Combat: Triggers "at end of combat" abilities

4. **Post-Combat Main Phase**
   - Player may play lands (if not done in first main phase) and cast spells

5. **Ending Phase**
   - End Step: Triggers "at beginning of end step" abilities
   - Cleanup: Damage clears, "until end of turn" effects end

Each phase transition is implemented as a state change, with systems that execute appropriate actions for each phase.

### Game Zones

MTG defines distinct zones where cards can exist:

| Zone | Description | Implementation |
|------|-------------|----------------|
| Library | Player's deck | Ordered collection, face-down |
| Hand | Cards held by a player | Private collection |
| Battlefield | Cards in play | Public collection with positioning |
| Graveyard | Discarded/destroyed cards | Ordered collection |
| Stack | Spells being cast, abilities being activated | LIFO data structure |
| Exile | Cards removed from game | Public collection |
| Command | Format-specific zone (e.g., Commanders) | Special collection |

Each zone is implemented as an entity with components that track contained cards. Zone transitions trigger specific events and state updates.

### Card Types and Characteristics

The engine supports all standard MTG card types:

- **Land**: Produces mana
- **Creature**: Can attack and block
- **Artifact**: Represents magical items
- **Enchantment**: Ongoing magical effects
- **Planeswalker**: Powerful ally with loyalty abilities
- **Instant**: One-time effect at any time
- **Sorcery**: One-time effect during main phase

Cards are represented as entities with components describing their characteristics (name, types, mana cost, etc.) and current state.

### Stack and Priority

The stack is MTG's core mechanic for resolving spells and abilities:

1. Active player receives priority first in each step/phase
2. Players may cast spells/activate abilities when they have priority
3. Spells/abilities go on the stack when cast/activated
4. When all players pass priority consecutively, the top item on the stack resolves
5. After resolution, active player receives priority again

Our implementation uses a dedicated stack system integrated with a priority manager that tracks which player can act.

### State-Based Actions

State-based actions are automatic game rules that check and apply whenever a player would receive priority:

- Creatures with toughness ≤ 0 are put into their owners' graveyards
- Players with life ≤ 0 lose the game
- Auras without legal targets are put into owners' graveyards
- Legendary permanents with the same name are put into graveyards
- And many more...

These are implemented as systems that run at specific points in the game loop to enforce rules consistency.

## Format Extensibility

The core rules are implemented in a format-agnostic way to enable:

1. **Consistent Base Behavior**: All formats share the same fundamental mechanics
2. **Extension Points**: Format-specific plugins can override or extend core behavior
3. **Configuration**: Format-specific parameters (starting life, deck requirements, etc.)

For Commander-specific implementations that build upon these core rules, see the [Commander Format](../commander/index.md) section.

## Technical Components

The core rules implementation includes these key technical components:

- **Turn Manager**: Controls phase/step progression
- **Zone Manager**: Handles card movement between zones
- **Stack Resolution Engine**: Manages spell/ability resolution
- **State-Based Action Checker**: Enforces automatic game rules
- **Combat Resolver**: Handles attack/block/damage processes
- **Mana System**: Tracks and processes mana production/consumption

Each component is implemented as a Bevy plugin that adds relevant systems, components, and resources.

## Next Steps

- [Turn Structure](turn_structure/index.md): Detailed implementation of turn phases and steps
- [Zones](zones/index.md): Implementation of game zones and zone transitions
- [Stack](stack/index.md): Stack implementation and priority system
- [State-Based Actions](state_actions/index.md): Implementation of automatic game rules
- [Combat](combat/index.md): Combat phase implementation
- [ECS Implementation](ecs_implementation.md): Technical details of the ECS approach 