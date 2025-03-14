# Card Selection

The card selection system provides visual feedback and interaction mechanisms for selecting cards in the game.

## Selection Modes

The system supports different selection modes:

- **Single selection**: Only one card can be selected at a time
- **Multi-selection**: Multiple cards can be selected simultaneously (e.g., for mass actions)
- **Range selection**: Cards within a range can be selected together

## Visual Feedback

When a card is selected, visual feedback is provided to the player through:

- Highlight effects around the card
- Change in elevation or z-index
- Animation effects

## Integration

Card selection integrates with other systems:

- [Drag and Drop](drag_and_drop.md): Selected cards can be dragged as a group
- [Targeting System](targeting.md): Selected cards can be used as sources for abilities
- [Game State](../../game_engine/state_management.md): Selection states are tracked in the game state

## Implementation

The selection system uses Bevy's component and event system:

1. Cards have a `Selectable` component
2. Selection state is managed through events
3. Visual feedback is applied through transform and material changes

## Usage Example

```rust
// Spawn a selectable card
commands.spawn((
    card_bundle,
    Selectable {
        selected: false,
        hover: false,
        selection_group: None,
    },
));
```

For more details on implementing custom selection behavior, see the [card integration documentation](../card_integration.md). 