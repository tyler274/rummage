# Game UI System Overview

This document provides a high-level overview of the user interface system for Rummage's Commander format game. For more detailed information on specific components, please see the related documentation files.

## UI Architecture Overview

The Rummage game UI is built using Bevy's Entity Component System (ECS) architecture, with a clear separation of concerns between game logic and visual presentation. The UI follows a layered approach to ensure clean organization, efficient rendering, and maintainable code.

### Key Components

The UI system uses non-deprecated Bevy 0.15.x components:

- `Node` for layout containers (replacing deprecated `NodeBundle`)
- `Text2d` for text display (replacing deprecated `Text2dBundle`)
- `Sprite` for images (replacing deprecated `SpriteBundle`)
- `Button` for interactive elements
- `RenderLayers` for visibility control

### Layers

The UI architecture is organized into distinct render layers as defined in `src/camera/components.rs`:

```rust
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AppLayer {
    #[default]
    Shared,         // Layer 0: Shared elements visible to all cameras
    Background,     // Layer 1: Game background elements
    GameWorld,      // Layer 2: Game world elements
    Cards,          // Layer 3: Card entities
    GameUI,         // Layer 4: In-game UI elements
    Effects,        // Layer 5: Visual effects
    Overlay,        // Layer 6: Game overlays
    Menu,           // Layer 7: Menu elements
    Popup,          // Layer 8: Popup dialogs
    Debug,          // Layer 9: Debug visuals
    DebugText,      // Layer 10: Debug text
    DebugGizmo,     // Layer 11: Debug gizmos
    Wireframe,      // Layer 12: Wireframe visualization
    Game = 31,      // Layer 31: Legacy game layer (backward compatibility)
}
```

These layers enable precise control over which UI elements are visible in different game states.

## Design Philosophy

The Rummage UI design follows these key principles:

1. **Clarity First**: Game state information must be clear and unambiguous
2. **Accessibility**: UI should be usable by players with diverse accessibility needs
3. **Intuitive Interaction**: Similar to physical Magic, but enhanced by digital capabilities
4. **Visual Hierarchy**: Important elements stand out through size, color, and animation
5. **Responsive Design**: Adapts to different screen sizes and orientations
6. **Performance**: Optimized rendering for smooth gameplay

## Core UI Systems

### 1. Layout Management

The game employs a flexible layout system that organizes UI elements into zones:

- **Player Zones**: Hand, battlefield, graveyard, exile, library positions
- **Shared Zones**: Stack, command zone, and turn structure indicators
- **Information Zones**: Game log, phase indicators, and supplementary information

Layouts adjust dynamically based on player count, screen size, and game state.

### 2. Interaction System

The interaction system handles:

- **Drag and Drop**: Moving cards between zones, with preview of valid targets
- **Targeting**: Selecting targets for spells and abilities
- **Context Menus**: Right-click (or long press) for additional card actions
- **Hotkeys**: Keyboard shortcuts for common actions

### 3. State Visualization

The UI visualizes game state through:

- **Card Transformations**: Visual representation of card states (tapped, attacking, etc.)
- **Animations**: Visual feedback for game events and transitions
- **Effects**: Particles and visual flourishes for special events

### 4. Information Display

Information is conveyed through:

- **Text**: Game text, rules text, and reminders
- **Icons**: Visual representation of card types, abilities, and states
- **Tooltips**: Contextual help and explanations
- **Game Log**: Record of game events and actions

## Integration with Game Logic

The UI system integrates with the game logic through:

- **Systems**: Reacting to game state changes and updating visuals
- **Events**: Processing user interactions and converting them to game actions
- **Resources**: Accessing shared game state to reflect in the UI

## Technical Implementation

The UI system is implemented using Bevy's ECS pattern:

- **Components**: Define UI element properties and behaviors
- **Systems**: Update and render UI elements based on game state
- **Resources**: Store shared UI state information
- **Events**: Handle user input and trigger UI updates

## Accessibility Features

The game includes several accessibility features:

- **Color Blind Modes**: Alternative color schemes for color-blind players
- **Text Scaling**: Adjustable text size for readability
- **Screen Reader Support**: Critical game information is accessible via screen readers
- **Keyboard Controls**: Full game functionality available through keyboard
- **Animation Reduction**: Option to reduce or disable animations

## Performance Considerations

The UI system is optimized for performance through:

- **Element Pooling**: Reusing UI elements to reduce allocation
- **Batched Rendering**: Minimizing draw calls
- **Culling**: Only rendering visible elements
- **Async Loading**: Loading assets in the background

## Future Enhancements

Planned enhancements to the UI system include:

- **Custom Themes**: Player-selectable UI themes
- **UI Animations**: Enhanced visual feedback
- **Mobile Optimization**: Touch-specific controls and layouts
- **VR Mode**: Virtual reality support for immersive gameplay 