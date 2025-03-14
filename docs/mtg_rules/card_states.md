# Card States in MTG

This document describes the various states that cards can have according to the Magic: The Gathering rules, and how these states are implemented in Rummage.

## Core Card States

According to the Magic: The Gathering rules, cards can have the following states:

### Tapped vs. Untapped

- **Tapped**: A card that has been turned sideways to indicate it has been used
- **Untapped**: A card in its normal vertical orientation

### Face-up vs. Face-down

- **Face-up**: A card's face is visible to all players
- **Face-down**: A card's face is hidden from all players (with certain exceptions)

### Flipped vs. Unflipped

- **Flipped**: A card that has been turned 180 degrees
- **Unflipped**: A card in its normal orientation

## Special States

In addition to the core states, cards can have several special states:

### Phased In/Out

- **Phased In**: Normal state, the card exists in the game
- **Phased Out**: Card is treated as though it doesn't exist

### Transformed

- For double-faced cards, the state of which face is currently showing

### Meld

- When certain cards are combined into a single larger card

## State Tracking

In Rummage, card states are tracked using components:

```rust
pub struct CardState {
    pub tapped: bool,
    pub face_down: bool,
    pub flipped: bool,
    pub phased_out: bool,
    pub transformed: bool,
    // Other states
}
```

## Rules for State Changes

State changes follow specific rules:

1. **Tapping**: Usually happens as a cost or an effect
2. **Untapping**: Normally happens during the untap step
3. **Face-down**: Usually through effects like Morph
4. **Flipping**: Only happens through specific card effects

## State Interaction with Game Rules

Card states interact with game rules in various ways:

- Tapped creatures can't attack or use tap abilities
- Face-down creatures are 2/2 creatures with no text, name, or types
- Phased-out cards are treated as though they don't exist

## Visual Representation

For details on how these states are visually represented in the game UI, see [Card States Visualization](../game_gui/cards/card_states.md). 