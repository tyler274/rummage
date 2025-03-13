# Bevy 0.15.x API Deprecations Guide

This guide documents the deprecated APIs in Bevy 0.15.x and their recommended replacements for use in the Rummage project.

## Bundle Deprecations

Bevy 0.15.x has deprecated many bundle types in favor of a more streamlined component approach.

### Rendering Bundles

| Deprecated | Replacement |
|------------|-------------|
| `Text2dBundle` | `Text2d` component |
| `SpriteBundle` | `Sprite` component |
| `ImageBundle` | `Image` component |

**Example:**

```rust
// ❌ Deprecated approach
commands.spawn(SpriteBundle {
    texture: asset_server.load("path/to/sprite.png"),
    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ..default()
});

// ✅ New approach
commands.spawn((
    Sprite::default(),
    asset_server.load::<Image>("path/to/sprite.png"),
    Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
));
```

### UI Bundles

| Deprecated | Replacement |
|------------|-------------|
| `NodeBundle` | `Node` component |
| `ButtonBundle` | Combine `Button` with other components |
| `TextBundle` | Combine `Text` and other components |

**Example:**

```rust
// ❌ Deprecated approach
commands.spawn(NodeBundle {
    style: Style {
        width: Val::Px(200.0),
        height: Val::Px(100.0),
        ..default()
    },
    background_color: Color::WHITE.into(),
    ..default()
});

// ✅ New approach
commands.spawn((
    Node {
        // Node properties
    },
    Style {
        width: Val::Px(200.0),
        height: Val::Px(100.0),
        ..default()
    },
    BackgroundColor(Color::WHITE),
));
```

### Camera Bundles

| Deprecated | Replacement |
|------------|-------------|
| `Camera2dBundle` | `Camera2d` component |
| `Camera3dBundle` | `Camera3d` component |

**Example:**

```rust
// ❌ Deprecated approach
commands.spawn(Camera2dBundle::default());

// ✅ New approach
commands.spawn(Camera2d::default());
```

### Transform Bundles

| Deprecated | Replacement |
|------------|-------------|
| `SpatialBundle` | Combine `Transform` and `Visibility` |
| `TransformBundle` | `Transform` component |
| `GlobalTransform` (manual insertion) | Just insert `Transform` |

**Example:**

```rust
// ❌ Deprecated approach
commands.spawn(SpatialBundle::default());

// ✅ New approach
commands.spawn((
    Transform::default(),
    Visibility::default(),
));
```

## Required Component Pattern

Bevy 0.15.x uses a "required component" pattern where inserting certain components will automatically insert their prerequisites. This means you no longer need to explicitly add all dependencies.

### Examples of Required Component Pattern

```rust
// Camera2d will automatically add Camera, Transform, and other required components
commands.spawn(Camera2d::default());

// Transform will automatically add GlobalTransform
commands.spawn(Transform::default());

// Sprite will automatically add TextureAtlas related components
commands.spawn(Sprite::default());
```

## Event API Changes

| Deprecated | Replacement |
|------------|-------------|
| `Events::get_reader()` | `Events::get_cursor()` |
| `ManualEventReader` | `EventCursor` |

**Example:**

```rust
// ❌ Deprecated approach
let mut reader = events.get_reader();
for event in reader.read(&events) {
    // Handle event
}

// ✅ New approach
let mut cursor = events.get_cursor();
for event in cursor.read(&events) {
    // Handle event
}
```

## Entity Access Changes

| Deprecated | Replacement |
|------------|-------------|
| `Commands.get_or_spawn()` | `Commands.entity()` or `Commands.spawn()` |
| `World.get_or_spawn()` | `World.entity()` or `World.spawn()` |

**Example:**

```rust
// ❌ Deprecated approach
commands.get_or_spawn(entity).insert(MyComponent);

// ✅ New approach
if world.contains_entity(entity) {
    commands.entity(entity).insert(MyComponent);
} else {
    commands.spawn((entity, MyComponent));
}
```

## UI Node Access Changes

| Deprecated | Replacement |
|------------|-------------|
| `Node::logical_rect()` | `Rect::from_center_size` with translation and node size |
| `Node::physical_rect()` | `Rect::from_center_size` with translation and node size |

**Example:**

```rust
// ❌ Deprecated approach
let rect = node.logical_rect(transform);

// ✅ New approach
let rect = Rect::from_center_size(
    transform.translation().truncate(),
    node.size(),
);
```

## Window and Input Changes

| Deprecated | Replacement |
|------------|-------------|
| `CursorIcon` field in `Window` | `CursorIcon` component on window entity |
| `Window.add_*_listener` methods | Use event systems |

**Example:**

```rust
// ❌ Deprecated approach
window.cursor_icon = CursorIcon::Hand;

// ✅ New approach
commands.entity(window_entity).insert(CursorIcon::Hand);
```

## Best Practices

1. **Check Compiler Warnings**: Always check for compiler warnings after making changes, as they will indicate usage of deprecated APIs.
2. **Use Component Approach**: Prefer the component-based approach over bundles.
3. **Required Components**: Leverage Bevy's automatic insertion of required components.
4. **Run Tests**: Run tests frequently to ensure compatibility.

## Troubleshooting

If you encounter issues after replacing deprecated APIs:

1. **Check Component Dependencies**: Some components may have implicit dependencies that need to be explicitly added.
2. **Verify Insertion Order**: In some cases, the order of component insertion matters.
3. **Update Queries**: Update your queries to match the new component structure.
4. **Check Bevy Changelog**: Refer to the Bevy changelog for detailed API changes. 