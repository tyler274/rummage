# Rummage - MTG Commander Game Engine Documentation

Welcome to the official documentation for Rummage, a robust end-to-end tested Magic: The Gathering Commander format game engine built with Bevy 0.15.x.

## Documentation Structure

The documentation is organized into the following major sections:

1. **[MTG Core Rules](mtg_core/index.md)** - Implementation of fundamental Magic: The Gathering rules
2. **[Game Formats](formats/commander/index.md)** - Format-specific rules implementation (currently Commander)
3. **[Game UI](game_gui/index.md)** - User interface systems and components
4. **[Networking](networking/index.md)** - Multiplayer functionality using bevy_replicon
5. **[Card Systems](card_systems/index.md)** - Card representation, effects, and interactions
6. **[Testing](testing/index.md)** - Testing framework and methodologies
7. **[Development](development/index.md)** - Development guidelines and tools
8. **[API Reference](api/index.md)** - Technical API documentation

## Getting Started

If you're new to the project, we recommend starting with the following documents:

- [MTG Core Rules Overview](mtg_core/index.md) - Introduction to the core MTG rules implementation
- [Commander Format Overview](formats/commander/overview/index.md) - Introduction to the Commander format
- [Game UI Overview](game_gui/overview.md) - Introduction to the game's user interface
- [Development Guide](development/getting_started.md) - How to get started with development

## Implementation Status

This documentation represents both implemented features and design specifications for planned features. Components are marked as follows:

- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Technical Architecture

Rummage is built on the following key technologies:

1. **Bevy 0.15.x** - Entity Component System (ECS) game engine
2. **Bevy Replicon** - Networking and state synchronization
3. **Rust** - Memory-safe, high-performance language

The architecture follows these principles:

1. **Entity Component System** - Game elements are composed of entities with components
2. **Event-driven Architecture** - Systems communicate through events
3. **Data-oriented Design** - Optimized for cache coherence and performance
4. **Deterministic Game Logic** - Ensures consistency across network play

## Development Standards

The Rummage codebase adheres to the following standards:

1. **Bevy 0.15.x Compatibility**: Using non-deprecated Bevy APIs
2. **End-to-End Testing**: Comprehensive test coverage for all features
3. **Documentation-First Development**: New features are documented before implementation
4. **Performance Focus**: Optimization for smooth gameplay

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