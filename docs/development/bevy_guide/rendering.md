# Rendering

This guide explains how rendering is implemented in Rummage using Bevy, focusing on card rendering, UI components, and visual effects.

## Table of Contents

1. [Introduction to Bevy Rendering](#introduction-to-bevy-rendering)
2. [Rendering Architecture in Rummage](#rendering-architecture-in-rummage)
3. [Card Rendering](#card-rendering)
4. [UI Components](#ui-components)
5. [Visual Effects](#visual-effects)
6. [Performance Optimization](#performance-optimization)
7. [Best Practices](#best-practices)
8. [Common Issues](#common-issues)

## Introduction to Bevy Rendering

Bevy provides a powerful, flexible rendering system that uses a render graph to define how rendering occurs. Key components of Bevy's rendering system include:

- **Render pipeline**: Configures how meshes, textures, and other visual elements are processed by the GPU
- **Materials**: Define surface properties of rendered objects
- **Meshes**: 3D geometry data
- **Textures**: Image data used for rendering
- **Cameras**: Define viewports into the rendered scene
- **Sprites**: 2D images rendered in the world
- **UI nodes**: User interface elements

In Bevy 0.15.x, some important changes were made to the rendering API:

- `Text2dBundle`, `SpriteBundle`, and `NodeBundle` are deprecated in favor of `Text2d`, `Sprite`, and `Node` components
- Enhanced material system
- Improved shader support
- Better handling of textures and assets

## Rendering Architecture in Rummage

Rummage employs a layered rendering architecture:

1. **Game World Layer**: Renders the battlefield, zones, and cards
2. **UI Overlay Layer**: Renders UI elements like menus, tooltips, and dialogs
3. **Effect Layer**: Renders visual effects and animations

The rendering is managed through several dedicated plugins:

- `RenderPlugin`: Core rendering setup
- `CardRenderPlugin`: Card-specific rendering
- `UIPlugin`: User interface rendering
- `EffectPlugin`: Visual effects and animations

## Card Rendering

Cards are the central visual element in a Magic: The Gathering game. Rummage's card rendering system handles both full cards and minimized versions.

### Card Components

Card rendering uses these key components:

```rust
// Card visual representation
#[derive(Component)]
pub struct CardVisual {
    pub style: CardStyle,
    pub state: CardVisualState,
}

// Card visual state (tapped, highlighted, etc.)
#[derive(Component)]
pub enum CardVisualState {
    Normal,
    Tapped,
    Highlighted,
    Selected,
    Targeted,
}

// Card render options
#[derive(Component)]
pub struct CardRenderOptions {
    pub show_full_art: bool,
    pub zoom_on_hover: bool,
    pub animation_speed: f32,
}
```

### Card Rendering System

The main card rendering system:

```rust
// Example system for updating card visuals based on game state
fn update_card_visuals(
    mut commands: Commands,
    mut card_query: Query<(Entity, &Card, &mut Transform, Option<&CardVisual>)>,
    card_state_query: Query<&CardGameState>,
    asset_server: Res<AssetServer>,
) {
    for (entity, card, mut transform, visual) in &mut card_query {
        // Get card state (tapped, etc.)
        let card_state = card_state_query.get(entity).unwrap_or(&CardGameState::Normal);
        
        // If no visual component or state changed, update visual
        if visual.is_none() || visual.unwrap().state != card_state.into() {
            // Load card texture
            let texture = asset_server.load(&format!("cards/{}.png", card.id));
            
            // Update or add visual components
            commands.entity(entity).insert((
                // In Bevy 0.15, we use the Sprite component directly, not SpriteBundle
                Sprite {
                    custom_size: Some(Vec2::new(CARD_WIDTH, CARD_HEIGHT)),
                    ..default()
                },
                // Add texture
                texture,
                // Add visual state
                CardVisual {
                    style: CardStyle::Standard,
                    state: card_state.into(),
                },
            ));
            
            // Update transform based on state (e.g., rotate if tapped)
            if matches!(card_state, CardGameState::Tapped) {
                transform.rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
            } else {
                transform.rotation = Quat::IDENTITY;
            }
        }
    }
}
```

### Card Layout

Cards are laid out using a custom layout system that handles positioning, stacking, and organization:

```rust
fn position_battlefield_cards(
    mut card_query: Query<(Entity, &BattlefieldCard, &mut Transform)>,
    battlefield_query: Query<&Battlefield>,
) {
    if let Ok(battlefield) = battlefield_query.get_single() {
        let mut positions = calculate_card_positions(battlefield);
        
        for (entity, battlefield_card, mut transform) in &mut card_query {
            if let Some(position) = positions.get(&battlefield_card.position) {
                transform.translation = Vec3::new(position.x, position.y, battlefield_card.layer as f32);
            }
        }
    }
}
```

## UI Components

Rummage uses Bevy's UI system for menus, dialogs, and game interface elements.

### UI Structure

The UI is organized hierarchically:

- Main UI root
  - Game UI (playmat, zones, etc.)
    - Player areas
    - Stack visualization
    - Phase indicator
  - Menu UI (game menu, settings, etc.)
  - Dialog UI (modal dialogs)
  - Tooltip UI (card info, ability info)

### UI Components in Bevy 0.15

In Bevy 0.15, UI components use the new approach:

```rust
// Create a UI node
commands.spawn((
    // Node component instead of NodeBundle
    Node {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: Color::rgba(0.1, 0.1, 0.1, 0.8),
        ..default()
    },
    // Other components
    UIRoot,
));

// Create text
commands.spawn((
    // Text component instead of TextBundle
    Text {
        sections: vec![TextSection {
            value: "Player 1".to_string(),
            style: TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        }],
        alignment: TextAlignment::Center,
        ..default()
    },
    // Style info
    Style {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    },
    // Additional components
    PlayerNameLabel(1),
));
```

### Dynamic UI Updates

UI elements are updated based on game state:

```rust
fn update_phase_indicator(
    mut text_query: Query<&mut Text, With<PhaseIndicator>>,
    game_state: Res<GameState>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("Phase: {}", game_state.current_phase.to_string());
    }
}
```

## Visual Effects

Rummage includes a variety of visual effects to enhance the gaming experience.

### Effect Components

Effects use dedicated components:

```rust
// Visual effect component
#[derive(Component)]
pub struct VisualEffect {
    pub effect_type: EffectType,
    pub duration: Timer,
    pub intensity: f32,
}

// Effect types
pub enum EffectType {
    CardGlow,
    Explosion,
    Sparkle,
    DamageFlash,
    HealingGlow,
}
```

### Effect Systems

Effects are processed by dedicated systems:

```rust
fn process_visual_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut effect_query: Query<(Entity, &mut VisualEffect, &mut Sprite)>,
) {
    for (entity, mut effect, mut sprite) in &mut effect_query {
        // Update effect timer
        effect.duration.tick(time.delta());
        
        // Calculate effect progress (0.0 to 1.0)
        let progress = effect.duration.percent();
        
        // Apply effect based on type
        match effect.effect_type {
            EffectType::CardGlow => {
                // Modify sprite color based on progress
                let intensity = (progress * std::f32::consts::PI).sin() * effect.intensity;
                sprite.color = sprite.color.with_a(0.5 + intensity * 0.5);
            },
            // Handle other effect types
            // ...
        }
        
        // Remove completed effects
        if effect.duration.finished() {
            commands.entity(entity).remove::<VisualEffect>();
            // Reset sprite to normal
            sprite.color = sprite.color.with_a(1.0);
        }
    }
}
```

## Performance Optimization

Rendering can be resource-intensive, especially with many cards and effects. Rummage includes several optimizations:

### Culling

Objects outside the view are culled to reduce rendering load:

```rust
fn cull_distant_cards(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    card_query: Query<(Entity, &GlobalTransform), With<Card>>,
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        let camera_pos = camera_transform.translation().truncate();
        
        for (entity, transform) in &card_query {
            let distance = camera_pos.distance(transform.translation().truncate());
            
            // If card is too far away, disable its rendering
            if distance > MAX_CARD_RENDER_DISTANCE {
                commands.entity(entity).insert(Visibility::Hidden);
            } else {
                commands.entity(entity).insert(Visibility::Visible);
            }
        }
    }
}
```

### Level of Detail

Cards far from the camera use simplified rendering:

```rust
fn adjust_card_detail(
    mut commands: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    card_query: Query<(Entity, &GlobalTransform, &CardVisual)>,
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        let camera_pos = camera_transform.translation().truncate();
        
        for (entity, transform, visual) in &card_query {
            let distance = camera_pos.distance(transform.translation().truncate());
            
            // Adjust detail level based on distance
            let detail_level = if distance < CLOSE_DETAIL_THRESHOLD {
                CardDetailLevel::High
            } else if distance < MEDIUM_DETAIL_THRESHOLD {
                CardDetailLevel::Medium
            } else {
                CardDetailLevel::Low
            };
            
            // Update detail level if changed
            if visual.detail_level != detail_level {
                commands.entity(entity).insert(CardDetailLevel(detail_level));
            }
        }
    }
}
```

### Batching

Similar rendering operations are batched to reduce draw calls:

```rust
fn setup_material_batching(
    mut render_app: ResMut<App>,
    render_device: Res<RenderDevice>,
) {
    // Set up batched materials for cards with similar properties
    render_app.insert_resource(CardBatchingOptions {
        max_batch_size: 64,
        use_instancing: true,
    });
}
```

## Best Practices

When working with rendering in Rummage, follow these best practices:

### Asset Management

- **Preload assets**: Use asset preprocessing to load common textures early
- **Texture atlases**: Group related textures in atlases to reduce binding changes
- **Asset handles**: Reuse asset handles instead of loading the same texture multiple times

### Rendering Organization

- **Separation of concerns**: Keep rendering logic separate from game logic
- **Component-based approach**: Use components to define visual properties
- **System organization**: Group related rendering systems together

### UI Design

- **Responsive layouts**: Design UI that adapts to different screen sizes
- **Consistent styling**: Use consistent colors, fonts, and spacing
- **Performance awareness**: Minimize UI elements and updates for better performance

## Common Issues

### Multiple Camera Issue

When using multiple cameras, queries might return multiple entities:

**Problem**:
```rust
// This will panic if there are multiple cameras
let (camera, transform) = camera_query.single();
```

**Solution**:
```rust
// Use a marker component to identify the main camera
#[derive(Component)]
struct MainCamera;

// Then query with the marker
let (camera, transform) = camera_query.get_single().unwrap_or_else(|_| {
    panic!("Expected exactly one main camera")
});
```

### Z-Fighting

When cards or UI elements overlap, they might flicker due to z-fighting:

**Problem**:
```rust
// Cards at the same z position
transform.translation = Vec3::new(x, y, 0.0);
```

**Solution**:
```rust
// Assign incrementing z values
transform.translation = Vec3::new(x, y, layer_index as f32 * 0.01);
```

### Texture Loading Errors

Missing textures can cause rendering issues:

**Problem**:
```rust
// No error handling for missing textures
let texture = asset_server.load(&format!("cards/{}.png", card.id));
```

**Solution**:
```rust
// Use a fallback texture
let texture_path = format!("cards/{}.png", card.id);
let texture_handle = asset_server.load(&texture_path);

// Set up a system to check for load errors
fn check_texture_loading(
    mut events: EventReader<AssetEvent<Image>>,
    mut card_query: Query<(&CardIdentifier, &Handle<Image>, &mut Visibility)>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        if let AssetEvent::LoadFailed(handle) = event {
            // Find cards with the failed texture and use fallback
            for (card_id, image_handle, mut visibility) in &mut card_query {
                if image_handle == handle {
                    // Load default texture instead
                    commands.entity(entity).insert(asset_server.load("cards/default.png"));
                }
            }
        }
    }
}
```

---

For questions or assistance with rendering in Rummage, please contact the development team. 