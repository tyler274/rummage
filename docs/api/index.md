# API Reference

This section provides detailed documentation for the Rummage game engine API, including core types, components, systems, and their relationships.

## Overview

The Rummage API is built around Bevy's Entity Component System (ECS) architecture, organizing game elements through components, resources, and systems. This documentation helps developers understand how these pieces fit together to implement MTG Commander gameplay.

## Core Modules

The API is organized into the following core modules:

- **Game Engine** - Core game logic, rules implementation, and state management
- **Card Systems** - Card representation, effects, and interactions
- **Player** - Player state, resources, and interactions
- **UI** - Game interface and visual elements
- **Networking** - Multiplayer functionality and state synchronization

## Game Engine APIs

The game engine implements the core MTG rules through several interconnected modules:

- [Actions](core_types.md#game-actions) - Game actions and their resolution
- [State](system_reference.md#state-management) - Game state tracking and transitions
- [Zones](component_reference.md#zone-components) - Game zones (library, hand, battlefield, etc.)
- [Stack](system_reference.md#stack-systems) - The MTG stack and priority system
- [Turns & Phases](component_reference.md#turn-components) - Turn structure and phase progression
- [Combat](system_reference.md#combat-systems) - Combat mechanics and damage resolution
- [Commander](system_reference.md#commander-systems) - Commander-specific rules implementation

## How to Use This Documentation

- **Core Types** - Fundamental types like `GameState`, `Phase`, etc.
- **Component Reference** - Documentation for all ECS components
- **System Reference** - Documentation for all ECS systems and their functions
- **Resource Reference** - Documentation for game resources and global state

For implementation details, see the corresponding sections in the [MTG Core Rules](../mtg_core/index.md) and [Commander Format](../formats/commander/index.md) documentation.
