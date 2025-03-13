# Game UI System Documentation

This section provides a comprehensive overview of the in-game user interface systems for Rummage's Commander format Magic: The Gathering implementation.

## Table of Contents

1. [Overview](#overview)
2. [Key Components](#key-components)
3. [Implementation Status](#implementation-status)
4. [Integration with Card Systems](#integration-with-card-systems)
5. [Integration with Networking](#integration-with-networking)

## Overview

The Game UI system is responsible for rendering the game state, facilitating player interactions, and providing visual feedback. Built using Bevy's Entity Component System (ECS) architecture, the UI serves as the visual representation layer that connects the player to the underlying game mechanics.

Our UI design philosophy focuses on three key principles:

1. **Clarity**: Making game state clearly visible and understandable
2. **Usability**: Providing intuitive interactions for complex game mechanics
3. **Immersion**: Creating a visually appealing and thematically consistent experience

The UI system functions as a bridge between the player and the game engine, translating ECS component data into visual elements and user inputs into game actions. It maintains its own set of UI-specific entities and components that represent the visual state of the game, which are updated in response to changes in the core game state.

For a more detailed overview, see the [overview document](overview.md).

## Key Components

The Game UI system consists of the following major components:

1. [Layout Components](layout/index.md)
   - Playmat
   - Command Zone
   - Battlefield
   - Player Zones
   - Stack Visualization

2. [Card Visualization](cards/index.md)
   - Card Rendering
   - Card States (Tapped, Exiled, etc.)
   - Card Animations
   - State Transitions

3. [Interaction Systems](interaction/index.md)
   - Card Selection
   - Drag and Drop
   - Action Menus
   - Targeting System
   - Input Validation

4. [Information Display](information/index.md)
   - Game Log
   - Phase Indicators
   - Priority Visualization
   - Tooltips and Helpers
   - Rules References

5. [Game Flow](flow/index.md)
   - Turn Visualization
   - Phase Transitions
   - Priority Passing
   - Timer Indicators

6. [Special UI Elements](special/index.md)
   - Modal Dialogs
   - Choice Interfaces
   - Decision Points
   - Triggered Ability Selections

7. [Multiplayer Considerations](multiplayer/index.md)
   - Player Positioning
   - Visibility Controls
   - Opponent Actions
   - Synchronization Indicators

8. [Table View](table/index.md)
   - Battlefield Layout
   - Card Stacking
   - Zone Visualization
   - Spatial Management

9. [Playmat Design](playmat/index.md)
   - Background Design
   - Zone Demarcation
   - Visual Themes
   - Customization Options

10. [Chat System](chat/index.md)
    - Message Display
    - Input Interface
    - Emotes
    - Communication Filters

11. [Avatar System](avatar/index.md)
    - Player Avatars
    - Avatar Selection
    - Custom Avatar Support
    - Visual Feedback

12. [Testing](testing/index.md)
    - Unit Testing UI Components
    - Integration Testing
    - UI Automation Testing
    - Visual Regression Testing

## Implementation Status

This documentation represents the design and implementation of the Game UI system. Components are marked as follows:

| Component | Status | Description |
|-----------|--------|-------------|
| Core UI Framework | ✅ | Basic UI rendering and interaction system |
| Card Visualization | ✅ | Rendering cards and their states |
| Battlefield Layout | ✅ | Arrangement of permanents on the battlefield |
| Hand Interface | ✅ | Player's hand visualization and interaction |
| Stack Visualization | 🔄 | Visual representation of the spell stack |
| Command Zone | 🔄 | Interface for commanders and command zone abilities |
| Phase/Turn Indicators | 🔄 | Visual indicators for game phases and turns |
| Player Information | ✅ | Display of player life, mana, and other stats |
| Targeting System | 🔄 | System for selecting targets for spells and abilities |
| Decision Interfaces | ⚠️ | Interfaces for player decisions and choices |
| Chat System | ⚠️ | In-game communication system |
| Settings Menu | ⚠️ | Interface for adjusting game settings |

Legend:
- ✅ Implemented and tested
- 🔄 In progress
- ⚠️ Planned but not yet implemented

## Integration with Card Systems

The Game UI system works in close collaboration with the [Card Systems](../card_systems/index.md) to transform the core game data into interactive visual elements:

### Visualization Pipeline

The UI receives card data from the Card Systems and renders it according to the current game state:

- Card entities from the game engine are mapped to visual representations
- The UI listens for changes to card components (like tapped status or counters)
- Visual effects are applied based on card state changes

### Interaction Translation

User interactions with card visuals are translated back into game engine commands:

- Dragging a card triggers zone transfer requests in the game engine
- Clicking on cards selects them for targeting or activation
- Right-clicking shows context-sensitive actions relevant to the card

### Special Card Rendering

Some cards require special UI handling:

- Modal cards present choice interfaces
- Split cards have multiple faces to visualize
- Transformed cards need to show both states
- Cards with counters display them visually

For detailed information on how cards are represented in the data model, see the [Card Database](../card_systems/database/index.md) documentation.

## Integration with Networking

In multiplayer games, the UI system coordinates with the [Networking](../networking/index.md) module to ensure consistent visual representation across all clients:

### State Synchronization

The UI responds to network synchronization events:

- Updates card positions and states based on received snapshots
- Provides visual indicators for network operations (opponent thinking, sync status)
- Handles delayed or out-of-order updates gracefully

### Action Broadcasting

When a player performs an action, the UI:

1. Shows a local preview of the expected result
2. Sends the action to the server via the networking system
3. Updates the display when confirmation is received
4. Handles conflicts if server state differs from prediction

### Latency Compensation

To provide a responsive feel despite network latency:

- The UI implements client-side prediction for common actions
- Visual feedback indicates when actions are pending confirmation
- Animated transitions smooth out state updates from the server

For more detailed information on how the UI integrates with networked gameplay, see the [Gameplay Networking](../networking/gameplay/index.md) documentation.

---

For detailed information on specific UI components, please refer to the respective sections listed above. 