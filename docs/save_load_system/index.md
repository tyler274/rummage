# Save/Load System

The save/load system provides robust functionality to persist and restore game states in Rummage's MTG Commander format game engine. This allows games to be saved, loaded, and replayed at any point.

## Key Features

- **Binary Serialization**: Uses `bincode` for efficient and compact save files
- **Persistent Game State**: Complete serialization of game state, players, zones, and commander data
- **Game Replays**: Support for replaying games from any saved point
- **Automatic Saving**: Configurable auto-save during state-based action checks
- **Save Metadata**: Tracking of available save files with timestamps and information
- **Entity Mapping**: Handles Bevy's entity references through save/load cycles

## Use Cases

- Save games for later continuation
- Create game checkpoints before critical decisions
- Analyze past games through replay functionality
- Debug and testing of complex game states
- Share interesting game states with other players

## Components

The save/load system consists of several interrelated components:

1. **Plugin** - The `SaveLoadPlugin` managing initialization and registration
2. **Events** - Events for triggering save, load, and replay functionality
3. **Resources** - Configuration and state tracking resources
4. **Data Structures** - Serializable representations of game data
5. **Systems** - Bevy systems for handling save/load operations and replay

## Getting Started

See the [Overview](overview.md) for a quick introduction to using the save/load system, or dive directly into the [Implementation Details](implementation.md) for more technical information.

For existing projects, the [Integration Guide](bevy_persistent_integration.md) explains how to incorporate save/load functionality into your game systems. 