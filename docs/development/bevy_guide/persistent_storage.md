# Persistent Storage with bevy_persistent

This guide provides an overview of using `bevy_persistent` for data persistence in Rummage.

## Introduction

`bevy_persistent` is a Bevy crate that makes it easy to save and load data across application sessions. It enables robust persistence for:

- User settings
- Game saves
- Deck collections
- Game state snapshots
- Player profiles
- Achievement data

## Getting Started

### Adding the Dependency

First, add `bevy_persistent` to your `Cargo.toml`:

```toml
[dependencies]
bevy_persistent = "0.4.0"  # Use the latest compatible version
```

### Basic Usage

The basic pattern for using `bevy_persistent` is:

```rust
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

// Define a persistent resource
#[derive(Resource, Serialize, Deserialize, Default)]
struct GameSettings {
    volume: f32,
    fullscreen: bool,
    resolution: (u32, u32),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Initialize persistent resource
        .insert_resource(
            Persistent::<GameSettings>::builder()
                .name("settings")
                .format(StorageFormat::Ron)
                .path("user://settings.ron")
                .default(GameSettings {
                    volume: 0.5,
                    fullscreen: false,
                    resolution: (1920, 1080),
                })
                .build()
        )
        .add_systems(Startup, load_settings)
        .add_systems(Update, save_settings_on_change)
        .run();
}

// Load settings at startup
fn load_settings(mut settings: ResMut<Persistent<GameSettings>>) {
    if let Err(err) = settings.load() {
        error!("Failed to load settings: {}", err);
    } else {
        info!("Settings loaded successfully");
    }
}

// Save settings when they change
fn save_settings_on_change(settings: Res<Persistent<GameSettings>>) {
    if settings.is_changed() {
        if let Err(err) = settings.save() {
            error!("Failed to save settings: {}", err);
        } else {
            info!("Settings saved successfully");
        }
    }
}
```

## Key Features

### Builder Pattern

The library uses a builder pattern for constructing persistent resources:

```rust
Persistent::<T>::builder()
    .name("resource_name")         // Human-readable name
    .format(StorageFormat::Ron)    // Serialization format
    .path("user://file.ron")       // Storage path
    .default(T::default())         // Default value
    .build()
```

### Storage Formats

`bevy_persistent` supports multiple storage formats:

- **Ron**: Human-readable Rusty Object Notation (good for configs)
- **Json**: Standard JSON format (good for interoperability)
- **Bincode**: Efficient binary format (good for large data)
- **Toml**: Config-friendly format (good for settings)
- **Yaml**: Human-readable structured format

### Storage Paths

Paths can use special prefixes:

- `user://`: User-specific data directory
- `config://`: Configuration directory
- `cache://`: Cache directory
- `assets://`: Assets directory

For example:
```rust
.path("user://saves/profile1.save")
```

### Hot Reloading

During development, you can enable hot reloading of persistent resources:

```rust
fn hot_reload_system(mut settings: ResMut<Persistent<GameSettings>>) {
    if settings.was_modified_on_disk() {
        if let Err(err) = settings.load() {
            error!("Failed to hot reload settings: {}", err);
        } else {
            info!("Hot reloaded settings from disk");
        }
    }
}
```

### Error Handling

The library provides comprehensive error handling:

```rust
match settings.load() {
    Ok(()) => info!("Loaded successfully"),
    Err(PersistentError::Io(err)) => error!("I/O error: {}", err),
    Err(PersistentError::Deserialize(err)) => error!("Deserialization error: {}", err),
    Err(err) => error!("Other error: {}", err),
}
```

## Best Practices

### Atomicity

To ensure atomic updates (all-or-nothing):

```rust
// Make multiple changes in a transaction-like manner
fn update_settings(mut settings: ResMut<Persistent<GameSettings>>) {
    // Make changes
    settings.volume = 0.8;
    settings.fullscreen = true;
    settings.resolution = (3840, 2160);
    
    // Save all changes at once
    if let Err(err) = settings.save() {
        error!("Failed to save settings: {}", err);
        // Optionally revert changes on error
    }
}
```

### Versioning

For schema changes, use serde's versioning support:

```rust
#[derive(Resource, Serialize, Deserialize)]
struct GameSettings {
    // Add version field for schema migration
    #[serde(default = "default_version")]
    version: u32,
    
    // Original fields
    volume: f32,
    fullscreen: bool,
    
    // New fields with defaults
    #[serde(default)]
    resolution: (u32, u32),
}

fn default_version() -> u32 { 1 }
```

### Resource Granularity

Choose the right granularity for persistent resources:

- **Too coarse**: One resource for all settings makes recovery harder
- **Too fine**: Too many small resources increases I/O overhead

Good balance:
```rust
// Audio settings in one resource
#[derive(Resource, Serialize, Deserialize, Default)]
struct AudioSettings { /* ... */ }

// Video settings in another
#[derive(Resource, Serialize, Deserialize, Default)]
struct VideoSettings { /* ... */ }

// Controls in another
#[derive(Resource, Serialize, Deserialize, Default)]
struct ControlSettings { /* ... */ }
```

### Change Detection

For efficient saving, only save when something has actually changed:

```rust
fn save_if_changed(
    settings: Res<Persistent<GameSettings>>,
    time: Res<Time>,
    mut last_save: Local<f64>,
) {
    // Only check if resource changed
    if settings.is_changed() {
        let now = time.elapsed_seconds_f64();
        // Don't save too frequently (debounce)
        if now - *last_save > 5.0 {
            if let Err(err) = settings.save() {
                error!("Failed to save: {}", err);
            } else {
                *last_save = now;
            }
        }
    }
}
```

## Use Cases in Rummage

### Deck Database

For the deck database, use `bevy_persistent` to store user decks:

```rust
// See: docs/card_systems/deck_database/persistent_storage.md
```

### Game State Snapshots

For game state snapshots and rollback functionality:

```rust
// See: docs/core_systems/snapshot/bevy_persistent_integration.md
```

### User Settings

For user preferences and settings:

```rust
#[derive(Resource, Serialize, Deserialize, Default)]
struct UserPreferences {
    username: String,
    card_style: CardStyle,
    audio_volume: f32,
    enable_animations: bool,
    enable_auto_tap: bool,
    enable_hints: bool,
}

// Initialize in plugin
fn build_user_settings(app: &mut App) {
    let preferences = Persistent::<UserPreferences>::builder()
        .name("user_preferences")
        .format(StorageFormat::Ron)
        .path("user://preferences.ron")
        .default(UserPreferences::default())
        .build();
        
    app.insert_resource(preferences)
       .add_systems(Startup, load_user_preferences)
       .add_systems(Update, save_preferences_on_change);
}
```

## Troubleshooting

### Common Issues

1. **File Not Found**: Check if the directory exists and has write permissions
2. **Serialization Errors**: Make sure all fields are serializable
3. **Path Resolution**: Use the correct path prefix for different platforms

### Debugging Tips

Enable logging to debug storage issues:

```rust
// Enable debug logs for bevy_persistent
fn setup_logging() {
    env_logger::Builder::from_default_env()
        .filter_module("bevy_persistent", log::LevelFilter::Debug)
        .init();
}
```

## Related Documentation

- [Deck Database Persistence](../../card_systems/deck_database/persistent_storage.md)
- [State Persistence](../../core_systems/snapshot/bevy_persistent_integration.md) 