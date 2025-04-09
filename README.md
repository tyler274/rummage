# Rummage - A Magic: The Gathering Engine

![Magic: The Gathering](https://img.shields.io/badge/MTG-Commander-green)
![Bevy](https://img.shields.io/badge/Bevy-0.15.3-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
[![Patreon](https://img.shields.io/badge/Patreon-Support-FF424D?logo=patreon)](https://www.patreon.com/c/DabneyEngineeringIncorporated)
[![Documentation](https://img.shields.io/badge/Docs-GitHub%20Pages-brightgreen)](https://tyler274.github.io/rummage/)

Rummage is aiming to be a robust end-to-end tested Magic: The Gathering Commander format game engine built with Bevy 0.15.x.

## Features

- Core MTG game loop implementation
- Phase and turn management
- Priority and stack system
- Commander-specific rules and mechanics
- Zone management (battlefield, graveyard, exile, command zone, etc.)
- Commander damage tracking
- Card representation and rendering
- Multiplayer networking with bevy_replicon
- Modern UI with Bevy's UI components

## Commander Format Support

Rummage implements the specific rules for the Commander format:

- Starting life total of 40
- Commander damage tracking (21 damage from a single commander eliminates a player)
- Command zone mechanics (commanders can be cast from the command zone)
- Commander tax (2 additional mana each time cast from command zone)
- Color identity rules for deck construction

## Building & Running

```bash
# Build the project
cargo build

# Run the game
cargo run

# Run tests
cargo test
```

## Project Structure

### Game Engine Core
- **src/game_engine/** - Core game engine
  - **phase.rs** - Game phase system
  - **priority.rs** - Player priority management
  - **stack.rs** - Spell stack implementation
  - **state.rs** - Game state tracking
  - **zones.rs** - Game zones (hand, library, battlefield, etc.)
  - **commander.rs** - Commander-specific rules
  - **actions.rs** - Game actions
  - **turns.rs** - Turn sequence

### Cards and Resources
- **src/card/** - Card data structures and functionality
- **src/mana.rs** - Mana system implementation
- **src/deck/** - Deck management

### Player and UI Components
- **src/player/** - Player management 
- **src/camera/** - Camera control systems
- **src/text/** - Text rendering utilities
- **src/menu/** - Menu systems and UI

### Networking
- **src/networking/** - Multiplayer functionality using bevy_replicon
  - State synchronization
  - Rollback system
  - RNG synchronization

## Development Standards

The Rummage codebase adheres to the following standards:

1. **Bevy 0.15.x Compatibility**: Using non-deprecated Bevy APIs
2. **End-to-End Testing**: Comprehensive test coverage for all features
3. **Documentation-First Development**: New features are documented before implementation
4. **Performance Focus**: Optimization for smooth gameplay

## Documentation

For full documentation, including implementation details and architecture decisions, visit our [GitHub Pages documentation](https://tyler274.github.io/rummage/).

The documentation contains:
- Commander rules implementation details
- Game UI system documentation
- Networking architecture information
- API references

## Reference Materials

The implementation is based on official MTG rules:

- [Magic: The Gathering Comprehensive Rules](https://magic.wizards.com/en/rules)
- [Commander Format Rules](https://mtgcommander.net/index.php/rules/)

## License

This project is licensed under the MIT License - see the LICENSE.txt file for details.
