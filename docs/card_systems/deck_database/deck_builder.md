# Deck Builder

The deck builder system provides a flexible API for creating and configuring Magic decks in Rummage. It uses the builder pattern to provide a fluent interface for deck construction.

## Builder Pattern

The `DeckBuilder` struct follows the builder pattern, allowing incremental construction of a deck with clear, chainable methods:

```rust
/// Builder for creating decks
#[derive(Default)]
pub struct DeckBuilder {
    name: Option<String>,
    deck_type: Option<DeckType>,
    cards: Vec<Card>,
    commander: Option<Entity>,
    partner: Option<Entity>,
    owner: Option<Entity>,
}
```

This pattern makes deck creation more readable and easier to maintain.

## Basic Usage

Creating a standard 60-card deck:

```rust
let deck = DeckBuilder::new()
    .with_name("My Standard Deck")
    .with_type(DeckType::Standard)
    .with_cards(my_cards)
    .with_owner(player_entity)
    .build()?;
```

Creating a Commander deck:

```rust
let deck = DeckBuilder::new()
    .with_name("My Commander Deck")
    .with_type(DeckType::Commander)
    .with_cards(main_deck_cards)
    .with_commander(commander_entity)
    .with_owner(player_entity)
    .build()?;
```

## Available Methods

The `DeckBuilder` provides these core methods:

### Initialization

```rust
// Create a new empty deck builder
pub fn new() -> Self
```

### Configuration

```rust
// Set the name of the deck
pub fn with_name(mut self, name: &str) -> Self

// Set the type of the deck
pub fn with_type(mut self, deck_type: DeckType) -> Self

// Set the commander (for Commander format)
pub fn with_commander(mut self, commander: Entity) -> Self

// Set the partner commander (for Commander format)
pub fn with_partner(mut self, partner: Entity) -> Self

// Set the owner of the deck
pub fn with_owner(mut self, owner: Entity) -> Self
```

### Card Management

```rust
// Add multiple cards at once
pub fn with_cards(mut self, cards: Vec<Card>) -> Self

// Add a single card
pub fn add_card(mut self, card: Card) -> Self

// Add multiple copies of a card
pub fn add_copies(mut self, card: Card, count: usize) -> Self
```

### Building

```rust
// Build the final deck
pub fn build(self) -> Result<Deck, String>

// Build a shuffled deck
pub fn build_shuffled(self) -> Result<Deck, String>
```

## Implementation Details

The builder handles default values for optional fields:

```rust
pub fn build(self) -> Result<Deck, String> {
    let name = self.name.unwrap_or_else(|| "Untitled Deck".to_string());
    let deck_type = self.deck_type.unwrap_or(DeckType::Standard);

    let mut deck = Deck::new(name, deck_type, self.cards);

    if let Some(commander) = self.commander {
        deck.set_commander(commander);
    }

    if let Some(partner) = self.partner {
        deck.set_partner(partner);
    }

    if let Some(owner) = self.owner {
        deck.set_owner(owner);
    }

    Ok(deck)
}
```

This ensures that decks always have reasonable defaults even when not all fields are specified.

## Format-Specific Usage

### Commander Decks

When building Commander decks, additional fields are required:

```rust
let deck = DeckBuilder::new()
    .with_name("Atraxa Superfriends")
    .with_type(DeckType::Commander)
    .with_cards(deck_cards)
    .with_commander(atraxa_entity)
    .build()?;
```

### Partner Commanders

For decks with partner commanders:

```rust
let deck = DeckBuilder::new()
    .with_name("Partners Deck")
    .with_type(DeckType::Commander)
    .with_cards(deck_cards)
    .with_commander(thrasios_entity)
    .with_partner(tymna_entity)
    .build()?;
```

## Validation

The builder doesn't perform validation during construction. Validation is handled separately when needed:

```rust
let deck = deck_builder.build()?;
if let Err(validation_error) = deck.validate() {
    // Handle validation error
}
```

This separation allows for creating incomplete or invalid decks when necessary (e.g., during deck construction in the UI).

## Related Documentation

- [Deck Structure](deck_structure.md): Core data structures for decks
- [Format Validation](format_validation.md): Format-specific deck constraints
- [Deck Registry](deck_registry.md): Managing multiple decks 