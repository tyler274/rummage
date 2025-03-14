# Card Selection

The card selection system provides visual feedback and interaction mechanisms for selecting cards in the game.

## Selection Modes

The system supports different selection modes:

- **Single selection**: Only one card can be selected at a time
- **Multi-selection**: Multiple cards can be selected simultaneously (e.g., for mass actions)
- **Range selection**: Cards within a range can be selected together
- **Context-aware selection**: Selection behavior changes based on game phase or action

## Visual Feedback

When a card is selected, visual feedback is provided to the player through:

- Highlight effects around the card
- Change in elevation or z-index
- Animation effects
- Pulsing glow or outline with phase-appropriate colors
- Optional sound effects for accessibility

## User Input Methods

Cards can be selected through various input methods:

- **Mouse click**: Standard selection method
- **Shift+click**: Add to current selection (multi-select)
- **Ctrl+click**: Toggle selection of individual card
- **Click and drag**: Select multiple cards by dragging a selection box
- **Tab navigation**: For keyboard-only control
- **Touch**: Single tap selects, double tap activates

## Selection Context

Selection behavior adapts based on the current game context:

- **Main phase**: Selection allows for playing cards or activating abilities
- **Combat phase**: Selection highlights potential attackers or blockers
- **Targeting phase**: Selection restricted to valid targets
- **Stack interaction**: Selection focuses on spells or abilities on the stack

## Integration

Card selection integrates with other systems:

- [Drag and Drop](drag_and_drop.md): Selected cards can be dragged as a group
- [Targeting System](targeting.md): Selected cards can be used as sources for abilities
- [Game State](../../game_engine/state_management.md): Selection states are tracked in the game state
- [Card Effects](../../card_systems/effects/index.md): Selection can trigger card-specific effects
- [Accessibility](../accessibility/index.md): Selection provides screen reader feedback

## Implementation

The selection system uses Bevy's component and event system:

1. Cards have a `Selectable` component
2. Selection state is managed through events
3. Visual feedback is applied through transform and material changes

### Component Structure

```rust
#[derive(Component)]
pub struct Selectable {
    /// Whether the card is currently selected
    pub selected: bool,
    /// Whether the cursor is hovering over the card
    pub hover: bool,
    /// Optional grouping for multi-select operations
    pub selection_group: Option<u32>,
    /// History of selection for undo operations
    pub selection_history: Vec<SelectionState>,
    /// Custom highlight color for this selectable
    pub highlight_color: Option<Color>,
}

#[derive(Event)]
pub struct CardSelectedEvent {
    pub entity: Entity,
    pub selected: bool,
    pub selection_mode: SelectionMode,
}
```

### Selection System Implementation

The core selection system consists of several systems:

```rust
/// Process mouse selection inputs
fn process_selection_input(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut card_query: Query<(Entity, &GlobalTransform, &mut Selectable)>,
    mut selection_events: EventWriter<CardSelectedEvent>,
) {
    // Implementation details
}

/// Update visual appearance based on selection state
fn update_selection_visuals(
    mut commands: Commands,
    mut query: Query<(Entity, &Selectable, Option<&mut Transform>)>,
    time: Res<Time>,
) {
    // Implementation details
}
```

## Usage Example

```rust
// Spawn a selectable card
commands.spawn((
    card_bundle,
    Selectable {
        selected: false,
        hover: false,
        selection_group: None,
        selection_history: Vec::new(),
        highlight_color: None,
    },
));

// System to detect selection and perform an action
fn handle_card_selection(
    mut selection_events: EventReader<CardSelectedEvent>,
    card_query: Query<&Card>,
) {
    for event in selection_events.read() {
        if event.selected && card_query.get(event.entity).is_ok() {
            // Card was selected, perform action
            println!("Card selected: {:?}", event.entity);
        }
    }
}
```

## Performance Considerations

The selection system is optimized to handle large numbers of selectable entities:

- Spatial partitioning for efficient hover detection
- Event-based notifications to avoid polling
- Selective visual updates for better rendering performance

## Future Enhancements

Planned improvements to the selection system include:

- Customizable selection visuals through themes
- Advanced multi-select patterns (double-click to select all similar cards)
- Persistent selection groups for complex game actions
- AI assistance for suggested selections

For more details on implementing custom selection behavior, see the [card integration documentation](../card_integration.md). 