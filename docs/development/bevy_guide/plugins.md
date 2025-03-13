# Plugin Architecture

This guide explains how the plugin architecture is implemented in Rummage using Bevy, and provides guidelines for working with and creating plugins.

## Table of Contents

1. [Introduction to Bevy Plugins](#introduction-to-bevy-plugins)
2. [Rummage Plugin Structure](#rummage-plugin-structure)
3. [Core Plugins](#core-plugins)
4. [Creating Plugins](#creating-plugins)
5. [Plugin Dependencies](#plugin-dependencies)
6. [Testing Plugins](#testing-plugins)
7. [Best Practices](#best-practices)

## Introduction to Bevy Plugins

Bevy's plugin system is a powerful way to organize and modularize game functionality. Plugins can add systems, resources, events, and other game elements to the Bevy App in a self-contained way. This modular approach allows for:

- **Code organization**: Grouping related functionality
- **Reusability**: Using plugins across different projects
- **Composability**: Building complex behavior from simpler plugins
- **Testability**: Testing plugins in isolation

## Rummage Plugin Structure

The Rummage codebase is organized around domain-specific plugins that encapsulate different aspects of the game engine. Here's the high-level plugin architecture:

```
src/
├── plugins/
│   ├── mod.rs                 # Exports all plugins
│   └── core.rs                # Core plugin configuration
├── game_engine/               # Game engine plugins
│   ├── mod.rs
│   ├── plugin.rs              # Main game engine plugin
│   └── ...
├── card/                      # Card-related plugins
│   ├── mod.rs
│   ├── plugin.rs              # Card system plugin
│   └── ...
├── player/                    # Player-related plugins
│   ├── mod.rs
│   ├── plugin.rs              # Player system plugin
│   └── ...
└── ...
```

In Rummage, each major subsystem is implemented as a plugin, which may compose multiple smaller plugins.

## Core Plugins

Rummage has several core plugins that provide essential functionality:

### GameEnginePlugin

The `GameEnginePlugin` is responsible for the core game mechanics:

```rust
pub struct GameEnginePlugin;

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut App) {
        app
            // Add game engine resources
            .init_resource::<GameState>()
            .init_resource::<TurnState>()
            
            // Register game engine events
            .add_event::<PhaseChangeEvent>()
            .add_event::<TurnChangeEvent>()
            
            // Add game engine systems
            .add_systems(Update, (
                process_game_phase,
                handle_turn_changes,
                check_state_based_actions,
            ));
    }
}
```

### CardPlugin

The `CardPlugin` handles card-related functionality:

```rust
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add card-related resources
            .init_resource::<CardDatabase>()
            
            // Register card-related events
            .add_event::<CardDrawnEvent>()
            .add_event::<CardPlayedEvent>()
            
            // Add card-related systems
            .add_systems(Update, (
                load_card_database,
                process_card_effects,
            ));
    }
}
```

### PlayerPlugin

The `PlayerPlugin` manages player-related functionality:

```rust
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add player-related resources
            .init_resource::<PlayerRegistry>()
            
            // Register player-related events
            .add_event::<PlayerDamageEvent>()
            .add_event::<PlayerLifeChangeEvent>()
            
            // Add player-related systems
            .add_systems(Update, (
                update_player_life,
                check_player_elimination,
            ));
    }
}
```

## Creating Plugins

When creating a new plugin for Rummage, follow these steps:

1. **Identify responsibility**: Define a clear domain of responsibility for your plugin
2. **Create plugin structure**: Create a new module with the plugin definition
3. **Implement resources and components**: Define data structures needed
4. **Implement systems**: Create systems that operate on your data
5. **Register with app**: Implement the Plugin trait to register everything

Here's a template for creating a new plugin:

```rust
use bevy::prelude::*;

// Define your plugin
pub struct MyFeaturePlugin;

// Define plugin-specific resources
#[derive(Resource, Default)]
pub struct MyFeatureResource {
    // Resource data
}

// Define plugin-specific components
#[derive(Component)]
pub struct MyFeatureComponent {
    // Component data
}

// Define plugin-specific events
#[derive(Event)]
pub struct MyFeatureEvent {
    // Event data
}

// Define plugin-specific systems
fn my_feature_system(
    mut commands: Commands,
    query: Query<&MyFeatureComponent>,
    mut resource: ResMut<MyFeatureResource>,
) {
    // System logic
}

// Implement the Plugin trait
impl Plugin for MyFeaturePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<MyFeatureResource>()
            
            // Register events
            .add_event::<MyFeatureEvent>()
            
            // Add systems
            .add_systems(Update, (
                my_feature_system,
                // Other systems
            ));
    }
}
```

## Plugin Dependencies

Plugins often depend on functionality provided by other plugins. In Bevy, plugin dependencies are managed through the order in which plugins are added to the app.

### Explicit Ordering

In Rummage, we handle plugin dependencies explicitly in the main app setup:

```rust
// In main.rs or lib.rs
app
    // Core plugins first
    .add_plugins(RummageGameCorePlugins)
    
    // Dependent plugins next
    .add_plugin(CardPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(ZonePlugin)
    
    // Higher-level plugins that depend on the above
    .add_plugin(CombatPlugin)
    .add_plugin(EffectsPlugin);
```

### Plugin Groups

For related plugins, we use Bevy's plugin groups to organize them:

```rust
pub struct RummageGameCorePlugins;

impl PluginGroup for RummageGameCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GameStatePlugin)
            .add(EventLoggingPlugin)
            .add(AssetLoadingPlugin)
    }
}
```

## Testing Plugins

Plugins should be tested in isolation as much as possible. Bevy provides utilities for testing plugins.

### Unit Testing Plugins

Here's how to test a single system from a plugin:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    
    #[test]
    fn test_my_feature_system() {
        // Set up a minimal App with just what we need
        let mut app = App::new();
        
        // Add resources and register components
        app.init_resource::<MyFeatureResource>();
        
        // Add the system under test
        app.add_systems(Update, my_feature_system);
        
        // Set up test entities
        app.world.spawn(MyFeatureComponent { /* ... */ });
        
        // Run the system
        app.update();
        
        // Assert expected outcomes
        let resource = app.world.resource::<MyFeatureResource>();
        assert_eq!(resource.some_value, expected_value);
    }
}
```

### Integration Testing Plugins

For testing a complete plugin:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    
    #[test]
    fn test_my_feature_plugin() {
        // Set up a minimal App
        let mut app = App::new();
        
        // Add our plugin
        app.add_plugin(MyFeaturePlugin);
        
        // Set up test entities and initial state
        // ...
        
        // Run systems for a few frames
        app.update();
        app.update();
        
        // Assert expected outcomes
        // ...
    }
}
```

## Best Practices

When working with plugins in Rummage, follow these best practices:

### Organization

- **One domain per plugin**: Each plugin should have a single, well-defined responsibility
- **Hierarchical structure**: Compose complex plugins from simpler ones
- **Clear naming**: Name plugins descriptively based on functionality

### Plugin Design

- **Minimal dependencies**: Minimize dependencies between plugins
- **Configuration options**: Make plugins configurable through parameters or resources
- **Clean interfaces**: Define clear interfaces for inter-plugin communication

### Implementation

- **Documentation**: Document each plugin's purpose and functionality
- **Resource naming**: Use descriptive, domain-specific names for resources
- **Event-based communication**: Use events for loose coupling between plugins
- **Testability**: Design plugins to be testable in isolation

### Example: Well-Designed Plugin

```rust
/// Plugin responsible for handling Magic card drawing mechanics
///
/// This plugin manages:
/// - Drawing cards from library to hand
/// - "Draw X cards" effects
/// - Replacement effects for card drawing
/// - Events related to card drawing
pub struct CardDrawPlugin {
    /// Maximum hand size (default: 7)
    pub max_hand_size: usize,
}

impl Default for CardDrawPlugin {
    fn default() -> Self {
        Self {
            max_hand_size: 7,
        }
    }
}

impl Plugin for CardDrawPlugin {
    fn build(&self, app: &mut App) {
        app
            // Store configuration
            .insert_resource(CardDrawConfig {
                max_hand_size: self.max_hand_size,
            })
            
            // Register events
            .add_event::<CardDrawEvent>()
            .add_event::<DrawReplacementEvent>()
            
            // Add systems with appropriate ordering
            .add_systems(Update, (
                process_draw_effects,
                move_cards_to_hand,
                check_maximum_hand_size,
            ).chain());
    }
}
```

---

Next: [Rendering](rendering.md) 