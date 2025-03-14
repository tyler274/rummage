# Card States

This document describes how different card states are visually represented in the game UI.

## Common Card States

Cards can be in various states during gameplay:

- **Normal**: The default state of a card
- **Tapped**: Rotated 90 degrees to indicate it has been used
- **Flipped**: Turned 180 degrees per specific card abilities
- **Face-down**: Showing the card back instead of the face
- **Revealed**: Temporarily shown to specific players
- **Highlighted**: Visually emphasized for selection or targeting

## Visual Representation

Each state has specific visual cues:

- **Tapped**: 90-degree rotation with possible shader effects
- **Flipped**: 180-degree rotation along the Y-axis
- **Face-down**: Shows the card back texture
- **Revealed**: Temporarily elevated z-position with highlight effects
- **Highlighted**: Glow effect or outline based on the context

## State Transitions

When a card changes state, it undergoes an animation to provide visual feedback:

- Tap/untap animations are smooth rotations
- Reveal animations include scaling and elevation changes
- Highlight animations use pulsing effects

For details on these animations, see [Card Animations](card_animations.md).

## Integration

Card states integrate with:

- [Card Rendering](card_rendering.md): The base rendering is modified by state
- [Drag and Drop](../interaction/drag_and_drop.md): Some states affect drag behavior
- [Game Rules](../../mtg_rules/card_states.md): States reflect game rule concepts

## Implementation

Each state is managed through components and systems that modify the card's visual representation. 