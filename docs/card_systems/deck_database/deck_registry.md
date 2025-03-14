# Deck Registry

The Deck Registry is a global resource that manages collections of predefined and user-created decks. It provides a central repository for deck storage, retrieval, and management.

## Registry Resource

The `DeckRegistry` is implemented as a Bevy resource:

```rust
#[derive(Resource, Default)]
pub struct DeckRegistry {
    decks: std::collections::HashMap<String, Deck>,
}
```

This resource is initialized during application startup:

```rust
// In the DeckPlugin implementation
fn build(&self, app: &mut App) {
    app.init_resource::<DeckRegistry>()
       .add_systems(Startup, register_default_decks)
       .add_systems(Startup, shuffle_all_player_decks);
}
```

## Core Registry Operations

The `DeckRegistry` provides several key methods:

### Registration

```rust
// Register a deck with the registry
pub fn register_deck(&mut self, name: &str, deck: Deck) {
    self.decks.insert(name.to_string(), deck);
}
```

### Retrieval

```rust
// Get a specific deck by name
pub fn get_deck(&self, name: &str) -> Option<&Deck> {
    self.decks.get(name)
}

// Get all registered decks
pub fn get_all_decks(&self) -> Vec<(&String, &Deck)> {
    self.decks.iter().collect()
}
```

## Default Decks

The registry includes a startup system that registers default decks:

```rust
// Register default decks for testing/examples
fn register_default_decks(mut registry: ResMut<DeckRegistry>) {
    // Register predefined decks
    // These could be loaded from files, created programmatically, etc.
    
    // Example: Register a basic test deck
    let test_deck = create_test_deck();
    registry.register_deck("Test Deck", test_deck);
    
    // Example: Register Commander precons
    let precon_decks = create_precon_decks();
    for (name, deck) in precon_decks {
        registry.register_deck(&name, deck);
    }
}
```

## Integration with Player Systems

The registry is designed to work with player-specific deck instances:

```rust
// System to assign decks to players from the registry
fn assign_decks_to_players(
    registry: Res<DeckRegistry>,
    mut commands: Commands,
    players: Query<(Entity, &PlayerPreferences)>,
) {
    for (player_entity, preferences) in players.iter() {
        if let Some(preferred_deck) = preferences.preferred_deck.as_ref() {
            if let Some(deck) = registry.get_deck(preferred_deck) {
                // Clone the deck from the registry
                let player_deck = PlayerDeck::new(deck.clone());
                
                // Assign the deck to the player
                commands.entity(player_entity).insert(player_deck);
            }
        }
    }
}
```

## Custom Deck Registration

Players can register their own custom decks:

```rust
// Register a player's custom deck
fn register_custom_deck(
    mut registry: ResMut<DeckRegistry>,
    deck_builder: DeckBuilder,
    player_name: &str,
) -> Result<(), String> {
    let deck_name = format!("{}'s Custom Deck", player_name);
    let deck = deck_builder.build()?;
    
    registry.register_deck(&deck_name, deck);
    Ok(())
}
```

## Deck Shuffling

The registry works with a system that ensures all player decks are properly shuffled:

```rust
// System to ensure all player decks are properly shuffled independently
fn shuffle_all_player_decks(mut player_decks: Query<&mut PlayerDeck>) {
    for mut player_deck in player_decks.iter_mut() {
        player_deck.deck.shuffle();
    }
}
```

## Persistence

In a complete implementation, the registry also handles saving and loading decks to/from disk:

```rust
// Save all registered decks to disk
pub fn save_all_decks(&self, path: &Path) -> Result<(), io::Error> {
    // Implementation for serializing and saving decks
}

// Load decks from disk
pub fn load_decks(&mut self, path: &Path) -> Result<(), io::Error> {
    // Implementation for loading and deserializing decks
}
```

## Related Documentation

- [Deck Structure](deck_structure.md): Core data structures for decks
- [Deck Builder](deck_builder.md): Creating and modifying decks
- [Format Validation](format_validation.md): Format-specific deck constraints 