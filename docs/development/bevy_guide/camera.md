# Camera Management

This guide explains camera management in Rummage using Bevy, with a focus on best practices for handling multiple cameras and common camera-related issues.

## Table of Contents

1. [Introduction to Cameras in Bevy](#introduction-to-cameras-in-bevy)
2. [Camera Architecture in Rummage](#camera-architecture-in-rummage)
3. [Setting Up Cameras](#setting-up-cameras)
4. [Accessing Camera Data](#accessing-camera-data)
5. [Camera Controls](#camera-controls)
6. [Multiple Camera Management](#multiple-camera-management)
7. [Camera Projection](#camera-projection)
8. [Common Camera Issues](#common-camera-issues)

## Introduction to Cameras in Bevy

Cameras in Bevy define how the game world is viewed. They determine what is rendered and from what perspective. In Bevy, cameras are entities with camera-related components:

- **Camera**: The core camera component that defines rendering properties
- **GlobalTransform**: Position and orientation of the camera in world space
- **Projection**: Orthographic or perspective projection settings
- **CameraRenderGraph**: Defines what render graph the camera uses

In Bevy 0.15.x, cameras have been improved with better control over rendering order, layers, and viewport settings.

## Camera Architecture in Rummage

Rummage uses a multi-camera setup to handle different views of the game:

1. **Main Game Camera**: An orthographic camera that views the game board
2. **UI Camera**: A specialized camera for UI elements
3. **Hand Camera**: A dedicated camera for viewing the player's hand
4. **Detail Camera**: A camera for viewing card details up close

Each camera is assigned specific render layers to control what they render:

```rust
// Render layers in Rummage
#[derive(Copy, Clone, Debug, Default, Component, Reflect)]
pub enum RenderLayer {
    #[default]
    Game = 0,       // Game elements (battlefield, etc.)
    UI = 1,         // UI elements
    Hand = 2,       // Hand cards
    CardDetail = 3, // Card detail view
}
```

## Setting Up Cameras

Here's how cameras are set up in Rummage:

```rust
fn setup_cameras(mut commands: Commands) {
    // Main game camera (orthographic, top-down view)
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            projection: OrthographicProjection {
                scale: 3.0,
                ..default()
            }
            .into(),
            ..default()
        },
        // Important! Mark this as the main camera
        MainCamera,
        // Specify what this camera renders
        RenderLayers::from_layers(&[RenderLayer::Game as u8]),
    ));
    
    // UI camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // UI camera renders after main camera
                order: 1,
                ..default()
            },
            ..default()
        },
        UiCamera,
        RenderLayers::from_layers(&[RenderLayer::UI as u8]),
    ));
    
    // Additional cameras as needed...
}
```

## Accessing Camera Data

The most important practice when working with cameras in a multi-camera system is to use markers and filtered queries. This prevents the dreaded "MultipleEntities" panic that occurs when using `single()` or `single_mut()` with multiple cameras:

```rust
// Camera marker components
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct HandCamera;

// Correctly accessing a specific camera
fn process_main_camera(
    // Filter the query to only get the main camera
    main_camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // Use get_single() instead of single() for better error handling
    if let Ok((camera, transform)) = main_camera_query.get_single() {
        // Now we can safely work with the camera data
        let camera_position = transform.translation();
        // ...
    }
}
```

## Camera Controls

Rummage implements several camera control systems:

### Pan and Zoom

```rust
fn camera_pan_system(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    input: Res<Input<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
) {
    if input.pressed(MouseButton::Middle) {
        let mut camera_transform = match camera_query.get_single_mut() {
            Ok(transform) => transform,
            Err(_) => return, // Safely handle the error
        };
        
        for event in motion_events.iter() {
            let delta = event.delta;
            // Pan the camera based on mouse movement
            camera_transform.translation.x -= delta.x * PAN_SPEED;
            camera_transform.translation.y += delta.y * PAN_SPEED;
        }
    }
}

fn camera_zoom_system(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    if let Ok((mut projection, mut transform)) = camera_query.get_single_mut() {
        for event in scroll_events.iter() {
            // Zoom based on scroll direction
            projection.scale -= event.y * ZOOM_SPEED;
            // Clamp to reasonable values
            projection.scale = projection.scale.clamp(MIN_ZOOM, MAX_ZOOM);
        }
    }
}
```

### Camera Transitions

For smooth transitions between camera views:

```rust
#[derive(Component)]
pub struct CameraTransition {
    pub target_position: Vec3,
    pub target_rotation: Quat,
    pub duration: f32,
    pub timer: Timer,
}

fn camera_transition_system(
    time: Res<Time>,
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, &mut CameraTransition)>,
) {
    for (entity, mut transform, mut transition) in &mut camera_query {
        transition.timer.tick(time.delta());
        let progress = transition.timer.percent();
        
        // Interpolate position and rotation
        transform.translation = transform.translation
            .lerp(transition.target_position, progress);
        transform.rotation = transform.rotation
            .slerp(transition.target_rotation, progress);
        
        // Remove the transition component when complete
        if transition.timer.finished() {
            commands.entity(entity).remove::<CameraTransition>();
        }
    }
}
```

## Multiple Camera Management

When working with multiple cameras, follow these guidelines:

1. **Use marker components**: Always attach marker components to differentiate cameras
2. **Filtered queries**: Use query filters to target specific cameras
3. **Render layers**: Assign render layers to control what each camera sees
4. **Render order**: Set camera order to control rendering sequence
5. **Error handling**: Use `get_single()` with error handling instead of `single()`

This system coordinates multiple cameras:

```rust
fn coordinate_cameras(
    card_detail_state: Res<State<CardDetailState>>,
    mut main_camera_query: Query<&mut Camera, (With<MainCamera>, Without<UiCamera>)>,
    mut ui_camera_query: Query<&mut Camera, With<UiCamera>>,
) {
    // Get cameras with proper error handling
    let mut main_camera = match main_camera_query.get_single_mut() {
        Ok(camera) => camera,
        Err(_) => return,
    };
    
    let mut ui_camera = match ui_camera_query.get_single_mut() {
        Ok(camera) => camera,
        Err(_) => return,
    };
    
    // Adjust cameras based on game state
    match card_detail_state.get() {
        CardDetailState::Viewing => {
            // While viewing a card detail, disable the main camera
            main_camera.is_active = false;
        }
        CardDetailState::None => {
            // When not viewing details, enable the main camera
            main_camera.is_active = true;
        }
    }
    
    // UI camera is always active
    ui_camera.is_active = true;
}
```

## Camera Projection

Rummage uses orthographic projection for the main game camera, as it provides a clearer view of the card game board:

```rust
fn setup_orthographic_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                near: -1000.0,
                far: 1000.0,
                ..default()
            }
            .into(),
            transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        MainCamera,
    ));
}
```

For specialized views like card details, a perspective camera might be used:

```rust
fn setup_perspective_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            projection: PerspectiveProjection {
                fov: std::f32::consts::PI / 4.0,
                near: 0.1,
                far: 100.0,
                aspect_ratio: 1.0,
            }
            .into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DetailCamera,
    ));
}
```

## Common Camera Issues

### MultipleEntities Error

The most common camera-related error is the "MultipleEntities" panic, which occurs when multiple entities match a camera query that expects a single result:

**Problem**:
```rust
fn problematic_camera_system(
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // This will panic if there are multiple cameras
    let (camera, transform) = camera_query.single();
    // ...
}
```

**Solution**:
```rust
// Add a marker component to your cameras
#[derive(Component)]
struct MainCamera;

// Then query with the marker
fn fixed_camera_system(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Ok((camera, transform)) = camera_query.get_single() {
        // Now we only get the camera with the MainCamera marker
    } else {
        // Handle the error case gracefully
        warn!("Expected exactly one main camera");
    }
}
```

### Incorrect View Frustum

If objects aren't visible when they should be, check the camera's near and far planes:

**Problem**:
```rust
// Objects might be outside the camera's view frustum
OrthographicProjection {
    near: 0.1,
    far: 100.0,
    // ...
}
```

**Solution**:
```rust
// Use more generous near/far values for card games
OrthographicProjection {
    near: -1000.0,  // Allow objects "behind" the camera in orthographic view
    far: 1000.0,    // See objects far away
    // ...
}
```

### Camera Depth Issues

Objects appearing in unexpected order:

**Problem**:
```rust
// Z-fighting or depth order issues
transform.translation = Vec3::new(x, y, 0.0);
```

**Solution**:
```rust
// Use the z-coordinate for explicit depth ordering
transform.translation = Vec3::new(x, y, layer * 0.1);

// Or adjust the camera's transform
camera_transform.translation = Vec3::new(0.0, 0.0, z_distance);
camera_transform.look_at(Vec3::ZERO, Vec3::Y);
```

---

Next: [Handling Game State](game_state.md) 