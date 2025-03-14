# Drag and Drop System

The drag and drop system allows players to move game objects like cards between different zones using intuitive mouse-based interactions.

## Components

### Draggable

The `Draggable` component marks entities that can be dragged and contains:

```rust
pub struct Draggable {
    /// Whether the entity is currently being dragged
    pub dragging: bool,
    /// Offset from the mouse cursor to the entity's origin
    pub drag_offset: Vec2,
    /// Z-index for rendering order
    pub z_index: f32,
}
```

## Core System

The drag system is implemented in `drag_system` which handles:

1. Detecting when a draggable entity is clicked
2. Starting the drag operation with the appropriate offset
3. Moving the entity with the mouse cursor
4. Handling release events to end the drag operation

## Z-index Management

The system automatically manages z-indices to ensure that dragged objects appear on top of other game elements. When multiple draggable entities overlap, the one with the highest z-index is selected for dragging.

## Integration Points

### Card Integration

The drag and drop system integrates with the [card visualization system](../cards/card_rendering.md) to allow players to:

- Move cards between hand, battlefield, and other zones
- Organize cards within a zone
- Reveal cards or interact with them in specific ways

### Targeting Integration

Drag and drop operations are used in conjunction with the [targeting system](targeting.md) to select targets for spells and abilities. This creates a fluid interaction where players can drag a card to play it and then naturally continue to select targets.

## Plugin Setup

To enable drag and drop functionality, the `DragPlugin` must be added to your Bevy app:

```rust
app.add_plugins(DragPlugin);
```

## Implementation Example

Below is a simplified example of how to make an entity draggable:

```rust
// Spawn a card entity with a draggable component
commands.spawn((
    card_bundle,
    Draggable {
        dragging: false,
        drag_offset: Vec2::ZERO,
        z_index: 1.0,
    },
));
```

## Customization

The drag and drop system can be customized by:

1. Adding different visual feedback during dragging
2. Implementing custom drop target detection
3. Creating drag constraints for certain game zones

For more information on implementing custom drag behaviors, see the [game state integration](../../game_engine/state_management.md) documentation. 