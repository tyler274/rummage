# Card Systems

This section documents the card systems of the Rummage MTG Commander game engine, covering the card database, effects implementation, rendering, and testing strategies.

## Table of Contents

1. [Overview](#overview)
2. [Key Components](#key-components)
3. [Implementation Status](#implementation-status)

## Overview

The Card Systems module is the heart of Rummage's MTG implementation, responsible for representing cards, their attributes, and behaviors. This module handles everything from card data storage to effect resolution and visual representation.

## Key Components

The Card Systems consist of the following major components:

1. [Card Database](database/index.md)
   - Storage and retrieval of card data
   - Card attributes and properties
   - Oracle text processing

2. [Card Effects](effects/index.md)
   - Effect resolution system
   - Targeting mechanism
   - Complex card interactions

3. [Card Rendering](rendering/index.md)
   - Visual representation of cards
   - Card layout and templating
   - Art asset management

4. [Testing Cards](testing/index.md)
   - Effect verification methodology
   - Interaction testing
   - Edge case coverage

## Implementation Status

| Component | Status | Description |
|-----------|--------|-------------|
| Core Card Model | ✅ | Basic card data structure and properties |
| Card Database | ✅ | Storage and retrieval of card information |
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

---

For more detailed information, please refer to the specific subsections. 