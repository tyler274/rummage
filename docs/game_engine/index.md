# Game Engine

This section covers the core game engine that powers Rummage, the Magic: The Gathering Commander format implementation built with Bevy 0.15.x.

## Architecture Overview

The Rummage game engine is built on Bevy's Entity Component System (ECS) architecture, providing a robust foundation for implementing the complex rules and interactions of Magic: The Gathering. The engine is designed with the following principles:

- **Separation of Concerns**: Game logic is separated from rendering and input handling
- **Data-Oriented Design**: Game state is represented as components on entities
- **Event-Driven Architecture**: Systems communicate through events
- **Deterministic Execution**: Game logic runs deterministically for network play
- **Extensible Systems**: New cards and mechanics can be added without modifying core systems

## Core Components

The game engine consists of several interconnected systems:

- **[State Management](state/index.md)**: How game state is tracked and updated
- **[Event System](events/index.md)**: How game events are processed and handled
- **[Snapshot System](../core_systems/snapshot/index.md)**: How game state is serialized for networking and replays

## Integration Points

The game engine integrates with several other systems:

- **[Card Systems](../card_systems/index.md)**: Card representation and effects
- **[MTG Rules](../mtg_rules/index.md)**: Implementation of game rules
- **[UI Systems](../game_gui/index.md)**: Visual representation and interaction
- **[Networking](../networking/index.md)**: Multiplayer functionality

## Plugin Structure

The game engine is organized as a set of Bevy plugins that can be added to your Bevy application:

```rust
// Initialize the game engine
App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RummageGameEnginePlugin)
    .run();
```

## Implementation Status

The game engine currently implements:

- ✅ Core turn structure
- ✅ Basic card mechanics
- ✅ Zone management
- 🔄 Stack implementation
- 🔄 Combat system
- ⚠️ Comprehensive rules coverage
- ⚠️ Advanced card interactions

## Extending the Engine

For information on extending the game engine with new cards or mechanics, see the [Development Guide](../development/index.md). 