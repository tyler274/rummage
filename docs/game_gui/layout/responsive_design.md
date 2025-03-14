# Responsive Design

This document describes how the Rummage game UI adapts to different screen sizes, aspect ratios, and device types to provide an optimal user experience across platforms.

## Responsive Architecture

The responsive design system is built on several key components:

- **Breakpoint System**: Defines screen size thresholds for layout changes
- **Flexible Layout**: Uses relative sizing and positioning
- **Dynamic Scaling**: Adjusts UI element sizes based on screen dimensions
- **Priority-based Visibility**: Shows/hides elements based on available space
- **Orientation Handling**: Adapts to landscape and portrait orientations

## Breakpoint System

The UI defines several breakpoints that trigger layout changes:

```rust
pub enum ScreenBreakpoint {
    Mobile,      // < 768px width
    Tablet,      // 768px - 1279px width
    Desktop,     // 1280px - 1919px width
    LargeDesktop // >= 1920px width
}
```

Each breakpoint has associated layout configurations that adjust:

- Element sizes and positions
- Information density
- Touch target sizes
- Visibility of secondary elements

## Layout Adaptation

### Desktop Layout

On larger screens, the UI maximizes information visibility:

- Full battlefield view with minimal scrolling
- Expanded card hand display
- Detailed player information panels
- Side-by-side arrangement of game zones

```rust
fn configure_desktop_layout(
    mut layout_query: Query<&mut LayoutContainer, With<RootContainer>>,
) {
    for mut container in layout_query.iter_mut() {
        container.flex_direction = FlexDirection::Row;
        container.size = Vec2::new(1920.0, 1080.0);
        // Additional desktop-specific configuration
    }
}
```

### Tablet Layout

On medium-sized screens, the UI balances information and usability:

- Slightly compressed game zones
- Collapsible information panels
- Touch-friendly spacing and controls
- Optional side panels that can be toggled

### Mobile Layout

On small screens, the UI prioritizes core gameplay elements:

- Vertically stacked layout
- Swipeable/scrollable zones
- Collapsible hand that expands on tap
- Minimized information displays with expandable details
- Larger touch targets for better usability

```rust
fn configure_mobile_layout(
    mut layout_query: Query<&mut LayoutContainer, With<RootContainer>>,
) {
    for mut container in layout_query.iter_mut() {
        container.flex_direction = FlexDirection::Column;
        container.size = Vec2::new(390.0, 844.0); // iPhone 12 Pro dimensions as example
        // Additional mobile-specific configuration
    }
}
```

## Dynamic Element Scaling

UI elements scale proportionally based on screen size:

- Cards maintain aspect ratio while adjusting overall size
- Text scales to remain readable on all devices
- Touch targets maintain minimum size for usability
- Spacing adjusts to prevent crowding on small screens

```rust
fn scale_ui_elements(
    mut card_query: Query<&mut Transform, With<CardVisual>>,
    screen_size: Res<ScreenDimensions>,
) {
    let scale_factor = calculate_scale_factor(screen_size.width, screen_size.height);
    
    for mut transform in card_query.iter_mut() {
        transform.scale = Vec3::splat(scale_factor);
    }
}
```

## Orientation Handling

The UI adapts to device orientation changes:

### Landscape Orientation

- Horizontal layout with wide battlefield
- Hand displayed along bottom edge
- Player information in corners

### Portrait Orientation

- Vertical layout with stacked elements
- Hand displayed along bottom edge
- Battlefield takes central position
- Player information at top and bottom

```rust
fn handle_orientation_change(
    mut orientation_events: EventReader<OrientationChangedEvent>,
    mut layout_query: Query<&mut LayoutContainer, With<RootContainer>>,
) {
    for event in orientation_events.read() {
        match event.orientation {
            Orientation::Landscape => configure_landscape_layout(&mut layout_query),
            Orientation::Portrait => configure_portrait_layout(&mut layout_query),
        }
    }
}
```

## Priority-based Element Visibility

When screen space is limited, elements are shown or hidden based on priority:

1. **Essential**: Always visible (battlefield, hand, stack)
2. **Important**: Visible when space allows (player info, graveyards)
3. **Secondary**: Collapsed by default, expandable on demand (exile, detailed card info)
4. **Optional**: Hidden on small screens (decorative elements, full game log)

## Implementation

The responsive design system is implemented through several components:

```rust
// Tracks current screen dimensions and orientation
#[derive(Resource)]
pub struct ScreenDimensions {
    pub width: f32,
    pub height: f32,
    pub orientation: Orientation,
    pub breakpoint: ScreenBreakpoint,
}

// System to detect screen changes
fn detect_screen_changes(
    mut screen_dimensions: ResMut<ScreenDimensions>,
    windows: Res<Windows>,
    mut orientation_events: EventWriter<OrientationChangedEvent>,
) {
    // Implementation details
}
```

## Testing

The responsive design is tested across multiple device profiles:

- Desktop monitors (various resolutions)
- Tablets (iPad, Android tablets)
- Mobile phones (iOS, Android)
- Ultrawide monitors
- Small laptops

## Integration

The responsive design system integrates with:

- [Layout System](layout_system.md): Provides the foundation for responsive layouts
- [Zone Visualization](zone_visualization.md): Zones adapt based on screen size
- [Card Visualization](../cards/index.md): Cards scale appropriately

For more details on specific UI components and how they adapt, see the relevant component documentation. 