# Format Validation

The format validation system ensures that decks comply with the rules of their respective formats. Each Magic: The Gathering format has specific deck construction requirements that must be validated.

## Validation Process

Deck validation is performed through the `validate` method on the `Deck` struct:

```rust
impl Deck {
    pub fn validate(&self) -> Result<(), DeckValidationError> {
        match self.deck_type {
            DeckType::Commander => self.validate_commander(),
            DeckType::Standard => self.validate_standard(),
            DeckType::Modern => self.validate_modern(),
            DeckType::Legacy => self.validate_legacy(),
            DeckType::Vintage => self.validate_vintage(),
            DeckType::Pauper => self.validate_pauper(),
            DeckType::Pioneer => self.validate_pioneer(),
            DeckType::Limited => self.validate_limited(),
            DeckType::Brawl => self.validate_brawl(),
            DeckType::Custom(ref _custom) => Ok(()), // Custom formats don't have fixed validation
        }
    }
}
```

## Validation Errors

Validation failures return a `DeckValidationError` that describes the specific issue:

```rust
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

## Format-Specific Validation

Each format has its own validation rules implemented as a private method:

### Commander Validation

```rust
fn validate_commander(&self) -> Result<(), DeckValidationError> {
    // Check deck size (100 cards including commander)
    if self.cards.len() < 99 {
        return Err(DeckValidationError::TooFewCards {
            required: 99,
            actual: self.cards.len(),
        });
    }

    // Check if commander exists
    if self.commander.is_none() {
        return Err(DeckValidationError::MissingCommander);
    }

    // Check singleton rule (except basic lands)
    let mut card_counts = std::collections::HashMap::new();
    for card in &self.cards {
        if !card.is_basic_land() {
            *card_counts.entry(&card.name).or_insert(0) += 1;
        }
    }

    for (card_name, count) in card_counts {
        if count > 1 {
            return Err(DeckValidationError::TooManyCopies {
                card_name: card_name.to_string(),
                max_allowed: 1,
                actual: count,
            });
        }
    }

    // Check color identity
    if let Some(commander_entity) = self.commander {
        // Logic to check that all cards match commander's color identity
        // ...
    }

    Ok(())
}
```

### Standard Validation

```rust
fn validate_standard(&self) -> Result<(), DeckValidationError> {
    // Check minimum deck size (60 cards)
    if self.cards.len() < 60 {
        return Err(DeckValidationError::TooFewCards {
            required: 60,
            actual: self.cards.len(),
        });
    }

    // Check card copy limit (max 4 of any card except basic lands)
    let mut card_counts = std::collections::HashMap::new();
    for card in &self.cards {
        if !card.is_basic_land() {
            *card_counts.entry(&card.name).or_insert(0) += 1;
        }
    }

    for (card_name, count) in card_counts {
        if count > 4 {
            return Err(DeckValidationError::TooManyCopies {
                card_name: card_name.to_string(),
                max_allowed: 4,
                actual: count,
            });
        }
    }

    // Check for banned cards
    let banned_cards: Vec<_> = self.cards
        .iter()
        .filter(|card| is_banned_in_standard(card))
        .map(|card| card.name.clone())
        .collect();

    if !banned_cards.is_empty() {
        return Err(DeckValidationError::IllegalCards(banned_cards));
    }

    // Check set legality
    let illegal_sets: Vec<_> = self.cards
        .iter()
        .filter(|card| !is_legal_in_standard_sets(card))
        .map(|card| card.name.clone())
        .collect();

    if !illegal_sets.is_empty() {
        return Err(DeckValidationError::IllegalCards(illegal_sets));
    }

    Ok(())
}
```

## Format Rules Implementation

The validation system relies on several helper functions:

```rust
// Check if a card is a basic land
fn is_basic_land(card: &Card) -> bool {
    // Implementation to check if a card is a basic land
}

// Check if a card is banned in a specific format
fn is_banned_in_format(card: &Card, format: &DeckType) -> bool {
    // Implementation to check format-specific ban lists
}

// Check if a card's set is legal in the standard rotation
fn is_legal_in_standard_sets(card: &Card) -> bool {
    // Implementation to check standard set legality
}

// Get a card's color identity
fn get_color_identity(card: &Card) -> Vec<Color> {
    // Implementation to determine a card's color identity
}
```

## UI Integration

The validation system integrates with the deck builder UI to provide immediate feedback:

```rust
// System to validate decks in the deck builder UI
fn validate_deck_in_builder(
    deck: &Deck,
    mut validation_state: &mut ValidationState,
) {
    match deck.validate() {
        Ok(()) => {
            validation_state.is_valid = true;
            validation_state.errors.clear();
        }
        Err(error) => {
            validation_state.is_valid = false;
            validation_state.errors.push(error);
        }
    }
}
```

## Validation Timing

Validation is performed at several key points:

1. **During deck building**: Validate as cards are added/removed
2. **Before saving**: Ensure decks are valid before saving to the registry
3. **Before game start**: Verify all player decks are valid for the format
4. **After format changes**: Re-validate when card legality changes

## Related Documentation

- [Deck Structure](deck_structure.md): Core data structures for decks
- [Deck Builder](deck_builder.md): Creating and modifying decks
- [Card Database](../database/index.md): Integration with the card database 