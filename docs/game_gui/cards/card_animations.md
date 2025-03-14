# Card Animations

This document describes how cards animate between different states and positions in the game UI.

## Types of Animations

The animation system supports several types of card animations:

- **Movement**: Cards moving between zones or positions
- **State Change**: Visual changes when a card's state changes
- **Highlight**: Temporary visual effects to draw attention
- **Special Effects**: Specific animations for card abilities or events

## Animation System

Animations are implemented using Bevy's animation system:

1. **Tweening**: Smooth transitions between states using easing functions
2. **Animation Tracks**: Sequences of animations that can be chained
3. **Event-driven**: Animations trigger in response to game events

## Common Animations

### Movement Animations

When cards move between zones:

- **Draw**: Cards animate from deck to hand
- **Play**: Cards animate from hand to battlefield
- **Discard**: Cards animate from hand to graveyard
- **Shuffle**: Cards animate into the library

### State Change Animations

- **Tap/Untap**: Smooth rotation animation
- **Flip**: 3D rotation along the Y-axis
- **Reveal**: Card scales up briefly
- **Transform**: Special effect for double-faced cards

## Integration

Card animations integrate with:

- [Card Rendering](card_rendering.md): Animations modify the rendered card
- [Card States](card_states.md): State changes trigger animations
- [Drag and Drop](../interaction/drag_and_drop.md): Dragging has specific movement animations

## Implementation Example

```rust
// Example of creating a card draw animation
fn animate_card_draw(
    commands: &mut Commands,
    card_entity: Entity,
    start_pos: Vec3,
    end_pos: Vec3,
) {
    commands.entity(card_entity).insert(AnimationSequence {
        animations: vec![
            Animation::new(
                start_pos,
                end_pos,
                Duration::from_millis(300),
                EaseFunction::CubicOut,
            ),
            Animation::new_scale(
                Vec3::splat(0.9),
                Vec3::ONE,
                Duration::from_millis(150),
                EaseFunction::BackOut,
            ),
        ],
        on_complete: Some(AnimationComplete::CardDrawn),
    });
}
```

For more information on how animations integrate with the game flow, see [Game Flow Visualization](../overview.md#game-flow-visualization). 