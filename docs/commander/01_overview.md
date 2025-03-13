# Commander Rules Engine Overview

## Introduction

This document outlines the architecture for implementing a comprehensive Magic: The Gathering rules engine for the Commander format. The engine is designed to support games with 2-13 players while maintaining all the unique aspects of Commander gameplay according to the official Magic: The Gathering Comprehensive Rules (section 903).

## Architecture Overview

The Commander rules engine is organized into several interconnected modules:

1. **Game State Management** - Core state tracking and game flow management
2. **Player Management** - Player data, life totals (starting at 40), and Commander-specific player mechanics
3. **Turn Structure & Phases** - Implementation of the complete turn sequence with priority passing
4. **Command Zone** - Special zone management for Commander cards, commander tax, and zone transitions
5. **Combat System** - Multiplayer combat including attack declaration, blocking, and commander damage tracking
6. **Priority & Stack** - Priority passing and stack resolution in multiplayer context
7. **State-Based Actions** - Game state checks including commander damage threshold (21) and other format-specific checks
8. **Special Commander Rules** - Color identity, commander zone movement, replacement effects
9. **Multiplayer Politics** - Voting, deal-making, and other multiplayer social mechanics

## Integration with Existing Codebase

The Commander rules engine builds upon the existing card implementation, leveraging:
- Card data structures and types
- Mana system and color identity validation
- Basic game loop framework
- Bevy ECS systems and events

## Implementation Approach

The implementation follows these principles:
- Modular design with clear separation of concerns
- Extensive use of Bevy ECS for game state management
- Event-driven architecture for game actions
- Comprehensive testing for complex rule interactions
- Rules enforcement using state-based actions

Each module is described in detail in its respective document, with specific implementations of the official comprehensive rules as defined in section 903.

## Key Rules Implementation

1. **Commander Placement**: Commanders start in the command zone (rule 903.6)
2. **Life Totals**: Players start with 40 life (rule 903.7)
3. **Commander Tax**: Additional cost of {2} for each previous cast from command zone (rule 903.8)
4. **Zone Transitions**: Special handling for commander movement between zones (rule 903.9)
5. **Commander Damage**: 21+ combat damage from same commander causes loss (rule 903.10a)
6. **Color Identity**: Deck construction restrictions based on commander's color identity (rule 903.5)

## Key Challenges

1. **Scale & Performance**: Supporting up to 13 players requires optimized systems
2. **Rule Complexity**: Commander has many unique rules and card interactions
3. **UI Considerations**: Managing the visual representation of a complex multiplayer state
4. **Networking**: Future multiplayer implementation over network connections

## Next Steps

1. Implement core game state management with proper turn structure
2. Build Commander-specific zone handling with state-based actions
3. Develop multiplayer turn structure with priority passing
4. Create comprehensive testing framework for rule validation 