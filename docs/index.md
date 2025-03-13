# Rummage - MTG Commander Game Engine Documentation

Welcome to the official documentation for Rummage, a robust end-to-end tested Magic: The Gathering Commander format game engine built with Bevy 0.15.x.

## Documentation Structure

The documentation is organized into the following major sections:

1. **[Commander Rules](commander/index.md)** - Implementation of MTG Commander format rules and mechanics
2. **[Game UI](game_gui/index.md)** - User interface systems and components
3. **[Networking](networking/index.md)** - Multiplayer functionality using bevy_replicon

## Getting Started

If you're new to the project, we recommend starting with the following documents:

- [Commander Format Overview](commander/overview/index.md) - Introduction to the Commander format
- [Game UI Overview](game_gui/overview.md) - Introduction to the game's user interface
- [Networking Overview](networking/core/architecture_overview.md) - Introduction to the networking architecture

## Implementation Status

This documentation represents both implemented features and design specifications for planned features. Components are marked as follows:

- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Development Standards

The Rummage codebase adheres to the following standards:

1. **Bevy 0.15.x Compatibility**: Using non-deprecated Bevy APIs
2. **End-to-End Testing**: Comprehensive test coverage for all features
3. **Documentation-First Development**: New features are documented before implementation
4. **Performance Focus**: Optimization for smooth gameplay

## Contributing

If you're interested in contributing to the Rummage project, please review:

1. The [GitHub repository](https://github.com/your-org/rummage)
2. Our [contribution guidelines](CONTRIBUTING.md)
3. The [code of conduct](CODE_OF_CONDUCT.md)

## Reference Materials

The implementation is based on official MTG rules:

- [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules)
- [Commander Format Rules](https://mtgcommander.net/index.php/rules/)

A local copy of the comprehensive rules is available in this repository: [MagicCompRules 20250207.txt](MagicCompRules%2020250207.txt).

---

This documentation will evolve as the project progresses. Last updated: March 2025. 