# Player Avatars

This document describes the player avatar system in Rummage, which provides visual representation of players in the game interface.

## Table of Contents

1. [Overview](#overview)
2. [Avatar Components](#avatar-components)
3. [Visual Representation](#visual-representation)
4. [Player State Indicators](#player-state-indicators)
5. [Customization](#customization)
6. [Implementation Details](#implementation-details)
7. [Testing](#testing)

## Overview

The avatar system provides a visual representation of each player in the Commander game. Avatars help players:

- Quickly identify who controls which game elements
- See player status information at a glance
- Express identity through customization
- Visualize social interactions and game effects

## Avatar Components

Each player avatar consists of multiple visual components:

### Core Components

- **Profile Picture**: The primary visual identity of the player
- **Name Plate**: Displays the player's username
- **Life Counter**: Shows current life total with emphasis on changes
- **Priority Indicator**: Shows when a player has priority
- **Turn Indicator**: Highlights the active player
- **Commander Damage Tracker**: Displays commander damage received from each opponent

### Example Avatar Layout

```
┌────────────────────────────────────┐
│  ┌──────┐   Username               │
│  │      │   Life: 40 ▲2            │
│  │ IMG  │                          │
│  │      │   ⚡ Priority             │
│  └──────┘                          │
├────────────────────────────────────┤
│ CMD DMG: P1(0) P2(6) P3(0) P4(0)   │
└────────────────────────────────────┘
```

## Visual Representation

Avatars appear in different locations depending on the context:

### In-Game Placement

- **Local Player**: Usually positioned at the bottom of the screen
- **Opponents**: Positioned around the virtual table based on turn order
- **Active Player**: Receives visual emphasis through highlighting or scaling

### Avatar Sizing

Avatars adapt to different contexts:

- **Full Size**: In player information panels
- **Medium Size**: At the head of playmat areas
- **Small Size**: Near cards to indicate control
- **Minimal**: In chat and notification areas

## Player State Indicators

Avatars reflect player state through visual cues:

### Game State Indicators

- **Turn Status**: Highlight for active player
- **Priority Status**: Indicator when player has priority
- **Thinking Status**: Animation when player is taking an action
- **Passed Status**: Indicator when player has passed priority

### Player Status Indicators

- **Life Total**: Shows current life with animations for changes
- **Hand Size**: Indicates number of cards in hand
- **Disconnected Status**: Indicator for disconnected players
- **Away Status**: Indicator for temporarily inactive players

### Hand Status Indicators

- **Card Count**: Shows number of cards in hand
- **Mulligan Status**: Indicates if player is mulliganing
- **Drawing Status**: Animation when drawing cards

## Customization

Players can customize their avatars to express identity:

### Visual Customization

- **Profile Pictures**: Select from built-in options or upload custom images
- **Borders**: Decorative frames around profile pictures
- **Effects**: Special visual effects for achievements or rankings
- **Color Schemes**: Customize colors of avatar UI elements

### Indicator Customization

- **Life Counter Style**: Different visualization styles
- **Animation Types**: Preference for animation effects
- **Sound Effects**: Custom sounds for avatar-related events

## Implementation Details

Avatars are implemented using Bevy's ECS system:

```rust
/// Component for player avatars
#[derive(Component)]
struct PlayerAvatar {
    player_id: PlayerId,
    display_name: String,
    profile_image: Handle<Image>,
    border_style: AvatarBorderStyle,
    customization_settings: AvatarCustomization,
}

/// Component for avatar life counter display
#[derive(Component)]
struct AvatarLifeCounter {
    player_id: PlayerId,
    current_life: i32,
    previous_life: i32,
    animation_timer: Timer,
    is_animating: bool,
}

/// Setup function for player avatars
fn setup_player_avatar(
    mut commands: Commands,
    player: &Player,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) -> Entity {
    // Default profile image
    let profile_image = player.profile_image.clone()
        .unwrap_or_else(|| asset_server.load("textures/avatars/default.png"));
    
    commands
        .spawn((
            PlayerAvatar {
                player_id: player.id,
                display_name: player.name.clone(),
                profile_image: profile_image.clone(),
                border_style: AvatarBorderStyle::default(),
                customization_settings: player.avatar_customization.clone(),
            },
            Node {
                width: Val::Px(280.0),
                height: Val::Px(80.0),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
            BorderColor(Color::rgb(0.6, 0.6, 0.6)),
            Outline::new(Val::Px(1.0)),
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            GlobalTransform::default(),
            AppLayer::GameUI.layer(),
        ))
        .with_children(|parent| {
            // Profile image container
            parent
                .spawn(Node {
                    width: Val::Px(70.0),
                    height: Val::Px(70.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|image_container| {
                    // Profile image
                    image_container.spawn((
                        Sprite {
                            custom_size: Some(Vec2::new(70.0, 70.0)),
                            ..default()
                        },
                        profile_image,
                    ));
                });
            
            // Information container
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|info| {
                    // Username
                    info.spawn((
                        Text2d {
                            text: player.name.clone(),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ));
                    
                    // Life counter
                    info.spawn((
                        Text2d {
                            text: format!("Life: {}", player.life),
                            font_size: 16.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                        AvatarLifeCounter {
                            player_id: player.id,
                            current_life: player.life,
                            previous_life: player.life,
                            animation_timer: Timer::from_seconds(0.5, TimerMode::Once),
                            is_animating: false,
                        },
                    ));
                    
                    // Priority indicator (hidden by default)
                    info.spawn((
                        Text2d {
                            text: "⚡ Priority".to_string(),
                            font_size: 16.0,
                            color: Color::YELLOW,
                        },
                        PriorityIndicator { 
                            player_id: player.id, 
                            visible: false 
                        },
                        Visibility::Hidden,
                    ));
                });
        })
        .id()
}
```

### Life Total Update System

```rust
fn update_avatar_life_counters(
    time: Res<Time>,
    mut life_counter_query: Query<(&mut AvatarLifeCounter, &mut Text2d)>,
    player_query: Query<(&Player, &PlayerId)>,
) {
    for (player, player_id) in player_query.iter() {
        for (mut life_counter, mut text) in life_counter_query.iter_mut() {
            if life_counter.player_id == *player_id {
                // Check if life has changed
                if player.life != life_counter.current_life {
                    // Update previous life for animation
                    life_counter.previous_life = life_counter.current_life;
                    life_counter.current_life = player.life;
                    life_counter.is_animating = true;
                    life_counter.animation_timer.reset();
                }
                
                // Update text display
                if life_counter.is_animating {
                    life_counter.animation_timer.tick(time.delta());
                    
                    // Determine color based on life change
                    let life_change = life_counter.current_life - life_counter.previous_life;
                    let color = if life_change > 0 {
                        Color::GREEN
                    } else if life_change < 0 {
                        Color::RED
                    } else {
                        Color::WHITE
                    };
                    
                    // Format text with change indicator
                    let change_text = if life_change > 0 {
                        format!("▲{}", life_change)
                    } else if life_change < 0 {
                        format!("▼{}", life_change.abs())
                    } else {
                        String::new()
                    };
                    
                    text.text = format!("Life: {} {}", life_counter.current_life, change_text);
                    text.color = color;
                    
                    // End animation after timer completes
                    if life_counter.animation_timer.finished() {
                        life_counter.is_animating = false;
                        text.color = Color::WHITE;
                        text.text = format!("Life: {}", life_counter.current_life);
                    }
                }
            }
        }
    }
}
```

### Priority Indicator System

```rust
fn update_priority_indicators(
    mut priority_query: Query<(&mut Visibility, &PriorityIndicator)>,
    game_state: Res<GameState>,
) {
    // Find player with priority
    let priority_player_id = game_state.priority_player;
    
    // Update all priority indicators
    for (mut visibility, indicator) in priority_query.iter_mut() {
        if indicator.player_id == priority_player_id {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
```

## Testing

The avatar system should be thoroughly tested to ensure proper functionality and visual appearance.

### Unit Tests

- Test initialization with different player data
- Verify life counter updates correctly
- Ensure priority indicators toggle correctly
- Test avatar positioning logic

### Visual Tests

- Verify appearance across different screen sizes
- Test with varied life totals
- Verify animations for life changes
- Ensure visibility of all avatar elements

### Integration Tests

- Test avatar updates with game state changes
- Verify commander damage display updates correctly
- Test avatar interactions with turn system 