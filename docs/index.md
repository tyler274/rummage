# Rummage - MTG Commander Game Engine Documentation

Welcome to the official documentation for Rummage, a robust end-to-end tested Magic: The Gathering Commander format game engine built with Bevy 0.15.x.

## About Rummage

Rummage is a modern, open-source implementation of the Magic: The Gathering Commander format, focusing on correctness, performance, and extensibility. Built on Bevy's Entity Component System (ECS) architecture, Rummage provides a solid foundation for complex card interactions while maintaining deterministic gameplay essential for networked multiplayer.

Our goal is to create a comprehensive digital implementation that faithfully reproduces the Commander experience while leveraging modern game engine technologies.

## Documentation Structure

The documentation is organized into interconnected sections that guide you from understanding MTG rules to technical implementation details:

1. **[MTG Core Rules](mtg_core/index.md)** - Implementation of fundamental Magic: The Gathering rules that form the foundation of all gameplay
2. **[Game Formats](formats/commander/index.md)** - Format-specific rules implementation, currently focusing on the Commander format
3. **[Game UI](game_gui/index.md)** - User interface systems for visualizing and interacting with the game state
4. **[Networking](networking/index.md)** - Multiplayer functionality using bevy_replicon for synchronized gameplay
5. **[Card Systems](card_systems/index.md)** - Card representation, effects, and interactions that drive gameplay
6. **[Testing](testing/index.md)** - Comprehensive testing framework to ensure rule correctness and system reliability
7. **[Development](development/index.md)** - Guidelines and tools for contributors to the Rummage project
8. **[API Reference](api/index.md)** - Technical documentation of Rummage's code structure and interfaces

These sections build upon each other, with core MTG rules providing the foundation for format-specific implementations, which in turn inform the design of card systems, user interfaces, and networking components. Our testing framework ensures that all these elements work together correctly.

## Getting Started

If you're new to the project, we recommend exploring the documentation in this order:

1. **[MTG Core Rules Overview](mtg_core/index.md)** - Understand how Rummage implements the fundamental MTG rules
2. **[Commander Format Overview](formats/commander/overview/index.md)** - Learn about the Commander-specific rules and mechanics
3. **[Development Guide](development/getting_started.md)** - Set up your development environment
4. **[Bevy ECS Guide](development/bevy_guide/ecs.md)** - Learn how we use Bevy's Entity Component System
5. **[Testing Overview](testing/index.md)** - Understand our testing approach and methodology

## Technical Architecture

Rummage integrates several key technologies:

1. **Bevy 0.15.x** - Entity Component System (ECS) game engine that provides the architectural foundation
2. **Bevy Replicon** - Networking and state synchronization for multiplayer gameplay
3. **Rust** - Memory-safe, high-performance language for reliable game logic

Our architecture follows these principles:

1. **Entity Component System** - Game elements are composed of entities with components, providing a flexible and performant structure for representing cards, players, and game state
2. **Event-driven Architecture** - Systems communicate through events, enabling loose coupling and flexible interactions
3. **Data-oriented Design** - Optimized for cache coherence and performance, critical for handling complex board states
4. **Deterministic Game Logic** - Ensures consistency across network play by maintaining predictable state transitions
5. **Snapshot System** - Enables game state serialization for networking, replays, and save/load functionality

## Implementation Status

This documentation represents both implemented features and design specifications for planned features. Components are marked as follows:

- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Development Standards

The Rummage codebase adheres to the following standards:

1. **Bevy 0.15.x Compatibility**: Using non-deprecated Bevy APIs (e.g., Text2d instead of Text2dBundle)
2. **End-to-End Testing**: Comprehensive test coverage for all features
3. **Documentation-First Development**: New features are documented before implementation
4. **Performance Focus**: Optimization for smooth gameplay even with complex board states

## Contributing

If you're interested in contributing to the Rummage project, please review:

1. [Contribution Guidelines](CONTRIBUTING.md)
2. [Documentation Guide](contributing/documentation.md)
3. [Code Style Guide](development/code_style.md)

## Reference Materials

The implementation is based on official MTG rules:

- [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules)
- [Commander Format Rules](https://mtgcommander.net/index.php/rules/)

A local copy of the comprehensive rules is available in this repository: [MagicCompRules 20250207.txt](mtg_rules/MagicCompRules%2020250207.txt).

---

This documentation will evolve as the project progresses. Last updated: March 2025. 