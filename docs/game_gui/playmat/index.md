# Player Playmat

This document details the player playmat system in Rummage, representing each player's personal play area with all the zones required for Magic: The Gathering Commander gameplay.

## Table of Contents

1. [Overview](#overview)
2. [Playmat Zones](#playmat-zones)
3. [Zone Interactions](#zone-interactions)
4. [Adaptive Sizing](#adaptive-sizing)
5. [Visual Customization](#visual-customization)
6. [Implementation Details](#implementation-details)
7. [Testing](#testing)

## Overview

Each player in a Rummage game has their own playmat that:

- Contains all required Magic: The Gathering game zones
- Provides clear visual separation between zones
- Ensures cards and game elements are easily accessible
- Adapts based on available screen space
- Integrates with the overall virtual table layout

## Playmat Zones

The playmat includes the following zones, as dictated by Magic: The Gathering rules:

### Battlefield

The primary play area where permanents (creatures, artifacts, enchantments, planeswalkers, lands) are placed when cast.

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                                                     │
│                                                     │
│                  BATTLEFIELD                        │
│                                                     │
│                                                     │
│                                                     │
└─────────────────────────────────────────────────────┘
```

- Supports tapped/untapped card states
- Organizes cards by type (creatures, lands, etc.)
- Allows custom grouping of permanents
- Supports tokens and copy effects

### Hand

The player's hand of cards, visible only to the player (and spectators with appropriate permissions).

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                      HAND                           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

- Displays cards in a fan-out layout
- Provides detail view on hover/select
- Shows card count for opponents
- Supports sorting and filtering

### Library (Deck)

The player's library, from which they draw cards.

```
┌───────────┐
│           │
│  LIBRARY  │
│           │
└───────────┘
```

- Shows deck position and card count
- Animates card draw and shuffle
- Displays deck state (e.g., being searched)
- Supports placing cards on top/bottom of library

### Graveyard

Discard pile for cards that have been destroyed, discarded, or sacrificed.

```
┌───────────┐
│           │
│ GRAVEYARD │
│           │
└───────────┘
```

- Shows most recent cards on top
- Supports browsing full contents
- Displays count of cards in graveyard
- Animates cards entering the graveyard

### Exile

Area for cards removed from the game.

```
┌───────────┐
│           │
│   EXILE   │
│           │
└───────────┘
```

- Distinguishes between face-up and face-down exiled cards
- Groups cards by the effect that exiled them
- Shows duration for temporary exile effects

### Command Zone

Special zone for commander cards and emblems.

```
┌───────────┐
│           │
│  COMMAND  │
│   ZONE    │
│           │
└───────────┘
```

- Prominently displays the player's commander(s)
- Shows commander tax amount
- Tracks commander damage given/received
- Displays emblems and other command zone objects

### Complete Playmat Layout

The full playmat integrates these zones into a cohesive layout:

```
┌─────────────────────────────────────────────────────┐
│                      HAND                           │
├───────────┬───────────────────────────┬─────────────┤
│           │                           │             │
│  LIBRARY  │                           │  GRAVEYARD  │
│           │                           │             │
├───────────┤                           ├─────────────┤
│           │                           │             │
│           │       BATTLEFIELD         │             │
│   EXILE   │                           │  COMMAND    │
│           │                           │   ZONE      │
│           │                           │             │
└───────────┴───────────────────────────┴─────────────┘
```

## Zone Interactions

The playmat facilitates several interactions between zones:

### Card Movement

Cards can move between zones through:

- Drag and drop gestures
- Context menu actions
- Keyboard shortcuts
- Game action triggers

Each movement includes appropriate animations to indicate the source and destination zones.

### Zone Focus

A zone can be focused to display more detailed information:

- Expanded graveyard view
- Detailed hand card inspection
- Library search views

Focus interactions maintain context by showing the relationship to other zones.

## Adaptive Sizing

The playmat adapts to different screen sizes and player counts:

### Size Adaptations

- **Full View**: When it's the player's turn or they have priority
- **Compact View**: During opponent turns
- **Minimal View**: When the player is not directly involved in the current game action

### Active Zone Emphasis

The current phase of the game influences zone emphasis:

- During main phases, the battlefield and hand are emphasized
- During combat, the battlefield receives more space
- When searching library, the library zone expands

## Visual Customization

Players can customize their playmat's appearance:

- Custom playmat backgrounds
- Zone color themes
- Card arrangement preferences
- Animation settings

## Implementation Details

The playmat is implemented using Bevy's UI system:

```rust
fn setup_player_playmat(
    mut commands: Commands,
    player: &Player,
    asset_server: &Res<AssetServer>,
) {
    // Main playmat container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            PlayerPlaymat { player_id: player.id },
            AppLayer::GameUI.layer(),
        ))
        .with_children(|playmat| {
            // Hand zone
            setup_hand_zone(playmat, player, asset_server);
            
            // Middle section containing battlefield and side zones
            playmat
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(80.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|middle| {
                    // Left side zones
                    setup_left_zones(middle, player, asset_server);
                    
                    // Battlefield
                    setup_battlefield(middle, player, asset_server);
                    
                    // Right side zones
                    setup_right_zones(middle, player, asset_server);
                });
        });
}
```

### Zone Components

Each zone is implemented as a component that can be queried and manipulated:

```rust
/// Component for the player's hand zone
#[derive(Component)]
struct HandZone {
    player_id: PlayerId,
    expanded: bool,
}

/// Component for the battlefield zone
#[derive(Component)]
struct BattlefieldZone {
    player_id: PlayerId,
    organization: BattlefieldOrganization,
}

// Other zone components follow a similar pattern
```

### Responsive Layout Systems

Systems manage the responsive behavior of the playmat:

```rust
fn update_playmat_layout(
    mut playmat_query: Query<(&mut Node, &PlayerPlaymat)>,
    player_turn_query: Query<&ActiveTurn>,
    window: Query<&Window>,
) {
    let window = window.single();
    let active_turn = player_turn_query.single();
    let is_landscape = window.width() > window.height();
    
    for (mut node, playmat) in playmat_query.iter_mut() {
        let is_active_player = active_turn.player_id == playmat.player_id;
        
        // Adjust layout based on active player and orientation
        if is_active_player {
            node.height = Val::Percent(if is_landscape { 40.0 } else { 50.0 });
        } else {
            node.height = Val::Percent(if is_landscape { 20.0 } else { 25.0 });
        }
    }
}
```

## Testing

The playmat should be thoroughly tested to ensure proper functionality and visual appearance:

### Unit Tests

- Test correct initialization of all zones
- Verify component hierarchy
- Ensure proper event handling for zone interactions

### Integration Tests

- Test card movement between zones
- Verify zone focus behavior
- Test responsive layout changes

### Visual Tests

- Verify appearance across different screen sizes
- Test with different card counts in each zone
- Ensure readability of card information

### Gameplay Tests

- Test common gameplay patterns involving multiple zones
- Verify commander damage tracking
- Test special zone interactions like flashback from graveyard 