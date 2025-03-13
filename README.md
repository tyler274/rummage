# Rummage - A Magic: The Gathering Engine

![Magic: The Gathering](https://img.shields.io/badge/MTG-Commander-green)
![Bevy](https://img.shields.io/badge/Bevy-0.15.3-blue)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
[![Patreon](https://img.shields.io/badge/Patreon-Support-FF424D?logo=patreon)](https://www.patreon.com/c/DabneyEngineeringIncorporated)
[![Documentation](https://img.shields.io/badge/Docs-GitHub%20Pages-brightgreen)](https://tyler274.github.io/rummage/)

Rummage is a Magic: The Gathering game engine built with Bevy, focusing on the Commander format.

## Features

- Core MTG game loop implementation
- Phase and turn management
- Priority and stack system
- Commander-specific rules and mechanics
- Zone management (battlefield, graveyard, exile, command zone, etc.)
- Commander damage tracking
- Card representation and rendering

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

- **src/game_engine/** - Core game engine
  - **phase.rs** - Game phase system
  - **priority.rs** - Player priority management
  - **stack.rs** - Spell stack implementation
  - **state.rs** - Game state tracking
  - **zones.rs** - Game zones (hand, library, battlefield, etc.)
  - **commander.rs** - Commander-specific rules
  - **actions.rs** - Game actions
  - **turns.rs** - Turn sequence

- **src/card.rs** - Card data structures and functionality
- **src/mana.rs** - Mana system
- **src/player.rs** - Player management

## License

This project is licensed under the MIT License - see the LICENSE.txt file for details.