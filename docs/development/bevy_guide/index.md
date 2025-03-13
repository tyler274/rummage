# Working with Bevy

This section provides detailed guidance on working with the Bevy game engine in the Rummage project. Bevy is a data-driven game engine built in Rust that uses an Entity Component System (ECS) architecture, which is particularly well-suited for building complex game systems like those required for a Magic: The Gathering implementation.

## Table of Contents

1. [Introduction](#introduction)
2. [Key Bevy Concepts](#key-bevy-concepts)
3. [Rummage Bevy Patterns](#rummage-bevy-patterns)
4. [Bevy Version Considerations](#bevy-version-considerations)
5. [Detailed Guides](#detailed-guides)

## Introduction

Rummage uses Bevy 0.15.x as its foundation, leveraging Bevy's modular design and performant ECS to create a robust MTG Commander game engine. This section explains how we use Bevy, our architecture patterns, and best practices for working with Bevy components, systems, and resources.

## Key Bevy Concepts

Before diving into Rummage-specific patterns, it's important to understand these core Bevy concepts:

- **Entity**: A unique ID that can have components attached to it
- **Component**: Data attached to entities (e.g., `Card`, `Player`, `Battlefield`)
- **System**: Logic that operates on components (e.g., drawing cards, resolving abilities)
- **Resource**: Global data not tied to any specific entity (e.g., `GameState`, `RngResource`)
- **Plugin**: A collection of systems, components, and resources that can be added to the app
- **Query**: A way to access entities and their components in systems
- **Events**: Message-passing mechanism between systems

## Rummage Bevy Patterns

Rummage follows these patterns for Bevy implementation:

1. **Domain-Specific Plugins**: Each game domain (cards, player, zones, etc.) has its own plugin
2. **Component-Heavy Design**: Game state is represented primarily through components
3. **Event-Driven Interactions**: Game actions are often communicated via events
4. **State-Based Architecture**: Game flow is controlled through Bevy's state system
5. **Clean Resource Management**: Resources are used judiciously for truly global state

## Bevy Version Considerations

Rummage uses Bevy 0.15.x, which introduces some important changes from earlier versions:

- **Deprecated UI Components**: `Text2dBundle`, `SpriteBundle`, and `NodeBundle` are deprecated in favor of `Text2d`, `Sprite`, and `Node` respectively
- **App Initialization**: Modified approach to app configuration and plugin registration
- **System Sets**: Updated system ordering mechanism
- **Asset Loading**: Enhanced asset loading system
- **Time Management**: Improved time and timer APIs

Always check for and fix any compiler warnings that might indicate usage of deprecated APIs.

## Detailed Guides

For more detailed information on working with Bevy in the Rummage project, explore these specific topics:

1. [Entity Component System](ecs.md) - Detailed guide on how Rummage uses ECS architecture
2. [Plugin Architecture](plugins.md) - How plugins are organized and composed in Rummage
3. [Rendering](rendering.md) - Card rendering, UI, and visual effects implementation
4. [Camera Management](camera.md) - Camera setup, management, and common issues
5. [Random Number Generation](rng.md) - Using bevy_rand for entity-attached RNGs and networked state sharing

---

Next: [Entity Component System](ecs.md) 