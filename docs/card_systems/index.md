# Card Systems

This section documents the card systems of the Rummage MTG Commander game engine, covering the card database, deck database, effects implementation, rendering, and testing strategies.

## Table of Contents

1. [Overview](#overview)
2. [Key Components](#key-components)
3. [Implementation Status](#implementation-status)
4. [Integration with Game UI](#integration-with-game-ui)

## Overview

The Card Systems module is the heart of Rummage's MTG implementation, responsible for representing cards, their attributes, and behaviors. This module handles everything from card data storage to deck management, effect resolution and visual representation.

In Rummage's ECS architecture, cards are entities with various components that define their properties, current state, and behaviors. These components include card type, mana cost, power/toughness for creatures, and other attributes defined in the MTG Comprehensive Rules. Systems then process these components to implement game mechanics such as casting spells, resolving abilities, and applying state-based actions.

## Key Components

The Card Systems consist of the following major components:

1. [Card Database](database/index.md)
   - Storage and retrieval of card data
   - Card attributes and properties
   - Oracle text processing
   - Card metadata and identification

2. [Deck Database](deck_database/index.md)
   - Deck creation and management
   - Format-specific validation
   - Deck persistence and sharing
   - Runtime deck operations

3. [Card Effects](effects/index.md)
   - Effect resolution system
   - Targeting mechanism
   - Complex card interactions
   - Ability parsing and implementation

4. [Card Rendering](rendering/index.md)
   - Visual representation of cards
   - Card layout and templating
   - Art asset management
   - Dynamic card state visualization

5. [Testing Cards](testing/index.md)
   - Effect verification methodology
   - Interaction testing
   - Edge case coverage
   - Rules compliance verification

## Implementation Status

| Component | Status | Description |
|-----------|--------|-------------|
| Core Card Model | ✅ | Basic card data structure and properties |
| Card Database | ✅ | Storage and retrieval of card information |
| Deck Database | ✅ | Deck creation, storage, and manipulation |
| Format Validation | 🔄 | Deck validation for various formats |
| Basic Effects | ✅ | Simple card effects (damage, draw, etc.) |
| Complex Effects | 🔄 | Advanced card effects and interactions |
| Targeting System | 🔄 | System for selecting targets for effects |
| Card Rendering | ✅ | Visual representation of cards |
| Effect Testing | 🔄 | Comprehensive testing of card effects |
| Card Symbols | ✅ | Rendering of mana symbols and other icons |
| Keywords | 🔄 | Implementation of MTG keywords |
| Ability Resolution | 🔄 | Resolving triggered and activated abilities |

Legend:
- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Integration with Game UI

The Card Systems module works closely with the [Game UI](../game_gui/index.md) module to create a seamless player experience. This integration occurs through several key interfaces:

### Visual Representation

The Card Rendering system provides the necessary data for the Game UI to visualize cards on screen:

- The rendering pipeline transforms card data into visual assets
- Dynamic updates reflect card state changes (tapped, counters, attachments)
- Special visual effects for activated abilities and spells being cast

### Deck Management

The Deck Database integrates with the UI to provide deck building and management interfaces:

- Deck builder UI for creating and editing decks
- Deck validation feedback
- Importing and exporting deck lists
- Deck statistics and analysis

### User Interaction

The Card Systems module exposes interaction points that the Game UI uses to enable player actions:

- Dragging cards between zones
- Targeting for spells and abilities
- Selecting options for modal abilities
- Viewing card details and related information

### State Feedback

As card states change due to game actions, this information is communicated to the UI:

- Legal play highlighting (e.g., showing which cards can be cast)
- Targeting validity feedback
- Stack visualization as spells and abilities resolve

For a complete understanding of how cards are visualized and interacted with in the game, continue to the [Game UI System](../game_gui/index.md) documentation, which builds upon the foundational card systems described here.

---

For more detailed information about card systems, please refer to the specific subsections. 