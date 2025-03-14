# Persistent Storage with bevy_persistent

This document details the implementation of robust save/load functionality for decks using `bevy_persistent`, a crate that provides efficient and reliable persistence for Bevy resources.

## Introduction to bevy_persistent

`bevy_persistent` is a crate that enables easy persistence of Bevy resources to disk with automatic serialization and deserialization. It provides:

- **Automatic saving**: Resources are automatically saved when modified
- **Error handling**: Robust error handling and recovery
- **Hot reloading**: Changes to saved files can be detected and loaded at runtime
- **Format flexibility**: Support for various serialization formats (JSON, RON, TOML, etc.)
- **Path configuration**: Flexible configuration of save paths

## Integrating bevy_persistent with DeckRegistry

The `DeckRegistry` resource can be enhanced with `bevy_persistent` to provide automatic, robust persistence:

```rust
use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

/// Persistent registry for storing decks
#[derive(Resource, Serialize, Deserialize, Default)]
pub struct PersistentDeckRegistry {
    /// All registered decks
    pub decks: std::collections::HashMap<String, Deck>,
    /// Last saved timestamp
    #[serde(skip)]
    pub last_saved: Option<std::time::SystemTime>,
}

// Extension for the DeckPlugin
impl DeckPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the persistent deck registry
        let persistent_registry = Persistent::<PersistentDeckRegistry>::builder()
            .name("decks")
            .format(StorageFormat::Ron)
            .path("user://deck_registry.ron")
            .default(PersistentDeckRegistry::default())
            .build();

        app.insert_resource(persistent_registry)
            .add_systems(Update, autosave_registry)
            .add_systems(Startup, load_decks_on_startup);
    }
}
```

## Autosave System

The autosave system ensures decks are saved whenever they are modified:

```rust
/// System to automatically save the deck registry when modified
fn autosave_registry(
    mut registry: ResMut<Persistent<PersistentDeckRegistry>>,
    time: Res<Time>,
) {
    // Check if registry was modified since last save
    if registry.is_changed() {
        // Only save every few seconds to avoid excessive disk I/O
        let now = std::time::SystemTime::now();
        let should_save = match registry.last_saved {
            Some(last_saved) => {
                now.duration_since(last_saved)
                    .unwrap_or_default()
                    .as_secs() >= 5
            }
            None => true,
        };

        if should_save {
            info!("Auto-saving deck registry...");
            if let Err(err) = registry.save() {
                error!("Failed to save deck registry: {}", err);
            } else {
                registry.last_saved = Some(now);
                info!("Deck registry saved successfully");
            }
        }
    }
}
```

## Loading Decks on Startup

Decks are automatically loaded when the application starts:

```rust
/// System to load decks on startup
fn load_decks_on_startup(
    mut registry: ResMut<Persistent<PersistentDeckRegistry>>,
    mut commands: Commands,
) {
    info!("Loading deck registry from persistent storage...");
    
    // Try to load the registry from disk
    match registry.load() {
        Ok(_) => {
            info!("Successfully loaded {} decks from registry", registry.decks.len());
            
            // Additional setup for loaded decks if needed
            for (name, deck) in registry.decks.iter() {
                debug!("Loaded deck: {}", name);
            }
        }
        Err(err) => {
            error!("Failed to load deck registry: {}", err);
            info!("Using default empty registry instead");
        }
    }
}
```

## API for Deck Management

The persistent registry provides a clean API for deck management:

```rust
/// Add a deck to the registry and save it
pub fn add_deck(
    mut registry: ResMut<Persistent<PersistentDeckRegistry>>,
    name: &str,
    deck: Deck,
) -> Result<(), String> {
    info!("Adding deck '{}' to registry", name);
    registry.decks.insert(name.to_string(), deck);
    
    match registry.save() {
        Ok(_) => {
            info!("Deck '{}' added and saved successfully", name);
            Ok(())
        }
        Err(err) => {
            error!("Failed to save deck registry after adding '{}': {}", name, err);
            Err(format!("Failed to save: {}", err))
        }
    }
}

/// Remove a deck from the registry
pub fn remove_deck(
    mut registry: ResMut<Persistent<PersistentDeckRegistry>>,
    name: &str,
) -> Result<(), String> {
    if registry.decks.remove(name).is_none() {
        return Err(format!("Deck '{}' not found in registry", name));
    }
    
    match registry.save() {
        Ok(_) => {
            info!("Deck '{}' removed and registry saved", name);
            Ok(())
        }
        Err(err) => {
            error!("Failed to save registry after removing '{}': {}", name, err);
            Err(format!("Failed to save: {}", err))
        }
    }
}
```

## Error Recovery

The system includes mechanisms for error recovery in case of corruption:

```rust
/// System to handle corrupted deck files
fn handle_corrupted_registry(
    mut registry: ResMut<Persistent<PersistentDeckRegistry>>,
) {
    // If loading failed due to deserialization errors
    if let Err(PersistentError::Deserialize(_)) = registry.try_load() {
        warn!("Deck registry file appears to be corrupted");
        
        // Create a backup of the corrupted file
        if let Some(path) = registry.path() {
            let backup_path = format!("{}.backup", path.display());
            if let Err(e) = std::fs::copy(path, backup_path.clone()) {
                error!("Failed to create backup of corrupted file: {}", e);
            } else {
                info!("Created backup of corrupted file at {}", backup_path);
            }
        }
        
        // Reset to default and save
        *registry = Persistent::builder()
            .name("decks")
            .format(StorageFormat::Ron)
            .path("user://deck_registry.ron")
            .default(PersistentDeckRegistry::default())
            .build();
            
        if let Err(e) = registry.save() {
            error!("Failed to save new default registry: {}", e);
        } else {
            info!("Reset deck registry to default state");
        }
    }
}
```

## Integration with Player Systems

The persistent deck registry can be integrated with player systems:

```rust
/// System to assign persistent decks to players
fn assign_persistent_decks(
    registry: Res<Persistent<PersistentDeckRegistry>>,
    mut commands: Commands,
    players: Query<(Entity, &PlayerPreferences)>,
) {
    for (player_entity, preferences) in players.iter() {
        if let Some(preferred_deck) = preferences.preferred_deck.as_ref() {
            if let Some(deck) = registry.decks.get(preferred_deck) {
                // Clone the deck from the registry
                let player_deck = PlayerDeck::new(deck.clone());
                
                // Assign the deck to the player
                commands.entity(player_entity).insert(player_deck);
                
                info!("Assigned deck '{}' to player", preferred_deck);
            }
        }
    }
}
```

## Benefits Over Manual Persistence

Using `bevy_persistent` offers several advantages over manual file I/O:

1. **Automatic Change Detection**: Resources are only saved when actually modified
2. **Error Handling**: Built-in error recovery mechanisms
3. **Hot Reloading**: Changes to deck files can be detected at runtime
4. **Format Flexibility**: Easy switching between serialization formats
5. **Path Management**: Cross-platform handling of save paths

## Related Documentation

- [Deck Structure](deck_structure.md): Core data structures for decks
- [Deck Registry](deck_registry.md): Managing multiple decks
- [State Persistence](../../core_systems/snapshot/bevy_persistent_integration.md): Using bevy_persistent for state rollbacks 