# Interaction Systems

This section covers the various interaction systems that allow players to interface with the game. These systems are designed to provide intuitive and responsive control over game elements.

## Overview

The interaction systems in Rummage are built to be:

- **Intuitive**: Players should be able to understand how to interact with game elements without extensive tutorials
- **Responsive**: Interactions should feel snappy and provide appropriate feedback
- **Accessible**: Interactions should work across different input methods and with accessibility considerations

## Key Interaction Systems

- [Card Selection](card_selection.md): How players select cards and receive visual feedback
- [Drag and Drop](drag_and_drop.md): How players move cards and other game elements between zones
- [Targeting System](targeting.md): How players select targets for spells and abilities

## Implementation Details

Interaction systems are implemented using Bevy's event and component systems. Each interaction type is maintained as a separate module with its own components and systems, but they are designed to work together seamlessly. 