# Deck Structure

This document details the core data structures used to represent decks in Rummage.

## Core Types

The deck system is built around several key types:

### Deck

The `Deck` struct is the fundamental representation of a deck in Rummage:

```rust
/// Represents a deck of Magic cards
#[derive(Debug, Clone)]
pub struct Deck {
    /// Name of the deck
    pub name: String,
    /// Type of the deck (Commander, Standard, etc.)
    pub deck_type: DeckType,
    /// Cards in the deck
    pub cards: Vec<Card>,
    /// Commander card ID if this is a Commander deck
    pub commander: Option<Entity>,
    /// Partner commander card ID if applicable
    pub partner: Option<Entity>,
    /// Owner of the deck
    pub owner: Option<Entity>,
}
```

This structure contains all essential information about a deck, including its contents, format type, and ownership details.

### PlayerDeck

The `PlayerDeck` component attaches a deck to a player entity in the ECS:

```rust
/// Component to track a player's deck
#[derive(Component, Debug, Clone)]
pub struct PlayerDeck {
    /// The actual deck data
    pub deck: Deck,
}
```

This wrapper allows decks to be proper ECS components that can be independently queried and modified.

### DeckType

The `DeckType` enum defines the supported formats:

```rust
/// Represents different types of Magic decks
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeckType {
    /// Standard format deck (60 card minimum)
    Standard,
    /// Commander/EDH format deck (100 card singleton with Commander)
    Commander,
    /// Modern format deck
    Modern,
    /// Legacy format deck
    Legacy,
    /// Vintage format deck
    Vintage,
    /// Pauper format deck
    Pauper,
    /// Pioneer format deck
    Pioneer,
    /// Limited format deck (40 card minimum)
    Limited,
    /// Brawl format deck
    Brawl,
    /// Custom format with special rules
    Custom(String),
}
```

Each format has specific rules for deck construction and validation.

## Deck Operations

The `Deck` struct provides methods for common operations:

### Creation and Setup

```rust
// Create a new deck
pub fn new(name: String, deck_type: DeckType, cards: Vec<Card>) -> Self

// Set the owner of this deck
pub fn set_owner(&mut self, owner: Entity)

// Set the commander for this deck
pub fn set_commander(&mut self, commander: Entity)

// Set the partner commander for this deck
pub fn set_partner(&mut self, partner: Entity)
```

### Deck Manipulation

```rust
// Shuffle the deck
pub fn shuffle(&mut self)

// Draw a card from the top of the deck
pub fn draw(&mut self) -> Option<Card>

// Draw multiple cards from the top of the deck
pub fn draw_multiple(&mut self, count: usize) -> Vec<Card>

// Add a card to the top of the deck
pub fn add_top(&mut self, card: Card)

// Add a card to the bottom of the deck
pub fn add_bottom(&mut self, card: Card)
```

### Deck Analysis

```rust
// Get the number of cards in the deck
pub fn card_count(&self) -> usize

// Search for cards by name
pub fn search(&self, name: &str) -> Vec<&Card>

// Validate the deck against format rules
pub fn validate(&self) -> Result<(), DeckValidationError>
```

## Player Deck Operations

The `PlayerDeck` component provides its own convenience methods:

```rust
// Create a new player deck component
pub fn new(deck: Deck) -> Self

// Draw a card from the top of the deck
pub fn draw(&mut self) -> Option<Card>

// Draw multiple cards from the top of the deck
pub fn draw_multiple(&mut self, count: usize) -> Vec<Card>
```

## Validation

Deck validation is format-specific and can return various error types:

```rust
/// Errors that can occur during deck validation
#[derive(Debug)]
pub enum DeckValidationError {
    /// Deck doesn't have enough cards
    TooFewCards { required: usize, actual: usize },
    /// Deck has illegal cards (e.g., banned cards)
    IllegalCards(Vec<String>),
    /// Deck has too many copies of a card
    TooManyCopies {
        card_name: String,
        max_allowed: usize,
        actual: usize,
    },
    /// Deck has cards outside the Commander's color identity
    ColorIdentityViolation(Vec<String>),
    /// Commander is missing
    MissingCommander,
    /// Other validation errors
    OtherError(String),
}
```

## Related Documentation

- [Deck Builder](deck_builder.md): Creating and modifying decks
- [Deck Registry](deck_registry.md): Managing multiple decks
- [Format Validation](format_validation.md): Format-specific deck constraints 