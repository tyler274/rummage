# Game UI Layout Components

This document describes the layout components used in the Rummage game interface, focusing on the spatial organization and structure of the UI elements.

## Table of Contents

1. [Playmat](playmat.md)
2. [Command Zone](command_zone.md)
3. [Battlefield](battlefield.md)
4. [Player Zones](player_zones.md)
5. [Stack Visualization](stack.md)
6. [Turn Structure UI](turn_structure.md)

## Layout Philosophy

The Rummage game layout follows these principles:

1. **Spatial Clarity**: Each game zone has a distinct location
2. **Player Symmetry**: Player areas are arranged consistently
3. **Focus on Active Elements**: Important game elements receive visual prominence
4. **Efficient Screen Usage**: Maximize the viewing area for the battlefield
5. **Logical Grouping**: Related UI elements are positioned near each other

## Layout Overview

The game UI is structured around a central battlefield with player zones positioned around it:

```
┌─────────────────────────────────────────────────────────────────┐
│                     OPPONENT INFORMATION                         │
├───────────┬─────────────────────────────────────┬───────────────┤
│           │                                     │               │
│ OPPONENT  │                                     │   OPPONENT    │
│ EXILE     │                                     │   GRAVEYARD   │
│           │                                     │               │
├───────────┤                                     ├───────────────┤
│           │                                     │               │
│           │                                     │               │
│ OPPONENT  │                                     │   STACK       │
│ LIBRARY   │          BATTLEFIELD                │   AREA        │
│           │                                     │               │
│           │                                     │               │
├───────────┤                                     ├───────────────┤
│           │                                     │               │
│ PLAYER    │                                     │   PLAYER      │
│ LIBRARY   │                                     │   GRAVEYARD   │
│           │                                     │               │
├───────────┼─────────────────────────────────────┼───────────────┤
│           │           PLAYER HAND               │   COMMAND     │
│  PLAYER   ├─────────────────────────────────────┤   ZONE        │
│  EXILE    │         PLAYER INFORMATION          │               │
└───────────┴─────────────────────────────────────┴───────────────┘
```

This layout adjusts for multiplayer games, positioning all opponents around the battlefield.

## Responsive Adaptations

The layout adapts to different screen sizes and orientations:

### Desktop Layout
- Horizontal layout with wide battlefield
- Hand cards displayed horizontally
- Full information displays

### Tablet Layout
- Similar to desktop with slightly compressed zones
- Touch-friendly spacing
- Collapsible information panels

### Mobile Layout
- Vertical orientation with stacked zones
- Scrollable battlefield
- Collapsible hand and information displays
- Gesture-based zone navigation

## Dynamic Player Positioning

For multiplayer games (3-4 players), the layout adjusts to position player zones around the battlefield:

### Four Player Layout:
```
┌───────────┬─────────────────────────────┬───────────┐
│           │      OPPONENT 2             │           │
│ OPPONENT2 │  ┌───────────────────────┐  │ OPPONENT2 │
│ ZONES     │  │                       │  │ ZONES     │
├───────────┤  │                       │  ├───────────┤
│           │  │                       │  │           │
│ OPPONENT1 │  │                       │  │ OPPONENT3 │
│ ZONES     │  │      BATTLEFIELD      │  │ ZONES     │
│           │  │                       │  │           │
│           │  │                       │  │           │
│           │  │                       │  │           │
├───────────┤  │                       │  ├───────────┤
│ COMMAND   │  └───────────────────────┘  │   STACK   │
│ ZONE      │       PLAYER ZONES          │   AREA    │
└───────────┴─────────────────────────────┴───────────┘
```

## Zone Transitions

Zones provide visual cues during transitions:

- **Highlight**: Active zones highlight during player turns
- **Animation**: Cards animate when moving between zones
- **Focusing**: Relevant zones enlarge when cards within them are targeted or selected

## Implementation Details

Layout components are implemented using Bevy's UI system with nested Node components:

```rust
// Example layout container for a player's battlefield area
commands
    .spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(40.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BattlefieldContainer,
        AppLayer::GameUI.layer(),
    ))
    .with_children(|parent| {
        // Individual battlefield row containers
        // ...
    });
```

## Layout Managers

Layout position is managed by dedicated systems that:

1. Calculate zone positions based on player count and screen size
2. Adjust UI element scale to maintain usability
3. Reposition elements in response to game state changes
4. Handle element focus and highlighting

The primary layout management systems include:

- `update_layout_for_player_count`: Adjusts layout based on number of players
- `update_responsive_layout`: Adapts layout to screen size changes
- `position_cards_in_zones`: Manages card positioning within zones

Each layout component has dedicated documentation explaining its specific implementation. 