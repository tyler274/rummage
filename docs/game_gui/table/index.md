# Virtual Table Layout

This document details the virtual table system used in Rummage for multiplayer Commander games. The table design accommodates anywhere from 2 to 6+ players while maintaining an intuitive and functional interface.

## Table of Contents

1. [Overview](#overview)
2. [Adaptive Player Positioning](#adaptive-player-positioning)
3. [Shared Zones](#shared-zones)
4. [Implementation Details](#implementation-details)
5. [Testing](#testing)

## Overview

The virtual table serves as the central organizing element of the game UI. It:

- Positions players around a central battlefield
- Scales dynamically based on player count
- Provides appropriate space for individual and shared game zones
- Maintains visual clarity regardless of player count

The table's design simulates sitting around a physical table but takes advantage of the digital medium to maximize usability and information presentation.

## Adaptive Player Positioning

### Player Count Configurations

The table dynamically adjusts to accommodate different player counts:

#### Two Player Configuration

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│               OPPONENT PLAYMAT                      │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│                 SHARED AREA                         │
│          (Command Zone, Stack, etc.)                │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│                 PLAYER PLAYMAT                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

#### Three Player Configuration

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│    OPPONENT 1 PLAYMAT      OPPONENT 2 PLAYMAT       │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│                 SHARED AREA                         │
│          (Command Zone, Stack, etc.)                │
│                                                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│                 PLAYER PLAYMAT                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

#### Four Player Configuration (Default)

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│               OPPONENT 2 PLAYMAT                    │
│                                                     │
├────────────────┬────────────────┬──────────────────┤
│                │                │                  │
│  OPPONENT 1    │  SHARED AREA   │   OPPONENT 3     │
│   PLAYMAT      │ (Command Zone, │     PLAYMAT      │
│                │  Stack, etc.)  │                  │
│                │                │                  │
├────────────────┴────────────────┴──────────────────┤
│                                                     │
│                 PLAYER PLAYMAT                      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

#### Five+ Player Configuration

For five or more players, the table uses a scrollable/zoomable view that arranges players in a circular pattern, with the active player and their adjacent opponents given visual priority.

### Adaptive Positioning Algorithm

The table employs an adaptive algorithm that:

1. Places the local player at the bottom
2. Positions opponents based on turn order
3. Allocates screen space proportionally based on player count
4. Adjusts element scaling to maintain readability
5. Prioritizes visibility for the active player and important game elements

## Shared Zones

The virtual table includes shared game zones accessible to all players:

### Command Zone

Central area for commander cards, emblems, and other command-zone specific elements.

### The Stack

Visual representation of spells and abilities currently on the stack, showing order and targeting information.

### Exile Zone

Shared exile area for face-up exiled cards that should be visible to all players.

### Game Information Display

Shows game state information such as turn number, active phase, priority holder, etc.

## Implementation Details

The table is implemented using Bevy's UI system with hierarchical nodes:

```rust
fn setup_virtual_table(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) {
    // Main table container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            VirtualTable,
            AppLayer::GameUI.layer(),
        ))
        .with_children(|table| {
            // Layout depends on player count
            match game_state.player_count {
                2 => setup_two_player_layout(table, &asset_server),
                3 => setup_three_player_layout(table, &asset_server),
                4 => setup_four_player_layout(table, &asset_server),
                _ => setup_many_player_layout(table, &asset_server, game_state.player_count),
            }
        });
}
```

### Player Position Calculation

Each player's position is calculated based on the total player count and their index:

```rust
/// Calculate angle and position for player zones based on player count
fn calculate_player_position(player_index: usize, player_count: usize) -> Vec2 {
    let angle = (player_index as f32 / player_count as f32) * std::f32::consts::TAU;
    let distance = 400.0; // Distance from center
    Vec2::new(
        distance * angle.cos(),
        distance * angle.sin(),
    )
}
```

### Dynamic Resizing

The table responds to window size changes, maintaining appropriate proportions:

```rust
fn update_table_responsive_layout(
    mut query: Query<&mut Node, With<VirtualTable>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let aspect_ratio = window.width() / window.height();
    
    for mut node in query.iter_mut() {
        // Adjust layout based on aspect ratio
        if aspect_ratio > 1.5 {
            // Landscape orientation
            node.flex_direction = FlexDirection::Column;
        } else {
            // Portrait orientation
            node.flex_direction = FlexDirection::Row;
        }
    }
}
```

## Testing

The virtual table's adaptive layout should be thoroughly tested across different scenarios:

### Unit Tests

- Test player position calculation for different player counts
- Verify layout component hierarchy for each player configuration
- Test responsive resizing behavior

### Integration Tests

- Verify all player zones are visible and accessible
- Test navigation between player areas
- Ensure shared zones maintain visibility for all players

### Visual Tests

- Confirm layout appearance across different screen sizes
- Verify scaling behavior for UI elements
- Test readability of cards and game information

### Performance Tests

- Measure rendering performance with maximum player count
- Test scrolling/zooming smoothness
- Verify UI responsiveness during layout transitions 