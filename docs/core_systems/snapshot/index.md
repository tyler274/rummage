# Snapshot System

The snapshot system is a core component of the Rummage game engine that provides serialization and deserialization of game state. This system is essential for networking, replay functionality, and save/load capabilities.

## Overview

The snapshot system enables:

- **Networked Multiplayer**: Synchronizing game state between players
- **Replay System**: Recording and replaying games
- **Undo Functionality**: Allowing players to revert to previous game states
- **Save/Load**: Persisting game state between sessions
- **Crash Recovery**: Automatically recovering from application crashes

## Documentation Sections

This documentation covers the following aspects of the snapshot system:

- **[Overview](overview.md)**: Detailed introduction to the snapshot concept and architecture
- **[Implementation](implementation.md)**: Technical details of the snapshot implementation
- **[Integration with Networking](networking_integration.md)**: How snapshots are used in the multiplayer system
- **[Persistent Storage](bevy_persistent_integration.md)**: Using bevy_persistent for robust state persistence
- **[Testing](testing.md)**: Approaches to testing snapshot functionality
- **[API Reference](api_reference.md)**: Complete reference of snapshot-related types and functions

## Technical Architecture

The snapshot system uses a component-based approach to serialize and deserialize entities in the game world. It captures the state of all relevant entities and components at a specific point in time, allowing the game state to be reconstructed later.

## Key Features

- **Selective Serialization**: Only relevant components are included in snapshots
- **Efficient Storage**: Compact binary representation of game state
- **Event-Based Triggering**: Snapshots can be triggered by game events
- **Queue Management**: Processing of snapshots is managed to avoid performance impact
- **Integration Points**: Well-defined interfaces for networking and replay systems
- **Persistent Storage**: Robust save/load functionality with automatic recovery

## Usage in Rummage

The snapshot system is used throughout Rummage:

1. **Networking**: Synchronizing game state between clients
2. **Game History**: Recording turn-by-turn snapshots for replay and analysis
3. **Testing**: Verifying game state correctness in unit and integration tests
4. **Save/Load**: Allowing games to be saved and resumed later
5. **Crash Recovery**: Automatically restoring state after unexpected crashes
6. **Rollback**: Supporting undo functionality for players

## bevy_persistent Integration

The snapshot system leverages the `bevy_persistent` crate to provide robust save/load functionality:

- **Automatic Persistence**: Game state is automatically saved at key moments
- **Error Recovery**: Comprehensive error handling for corrupted saves
- **Efficient Format**: Optimized binary serialization for large game states
- **Incremental Saves**: Only changed data is saved to improve performance
- **Cross-Platform**: Works consistently across all supported platforms

See the detailed documentation sections for more information on each aspect of the snapshot system. 