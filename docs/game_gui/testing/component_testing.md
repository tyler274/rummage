# UI Component Testing

This guide covers how to test UI components in the Rummage game engine.

## Overview

Testing UI components is essential to ensure that the game's interface remains consistent, responsive, and intuitive. In Rummage, we use a combination of unit tests, integration tests, and visual regression tests to validate our UI components.

## Testing Approach

Our UI component testing strategy follows these principles:

1. **Component Isolation**: Test individual UI components in isolation
2. **Behavior Verification**: Verify that components respond correctly to user interactions
3. **Layout Validation**: Ensure components maintain proper layout across different screen sizes
4. **Integration Testing**: Test how components interact with each other
5. **Visual Consistency**: Maintain visual appearance through visual regression testing

## Test Structure

UI component tests are organized as follows:

```
src/
  tests/
    ui/
      components/
        buttons_test.rs
        cards_test.rs
        menus_test.rs
      interactions/
        drag_drop_test.rs
        selection_test.rs
```

## Testing UI Components with Bevy

Bevy's entity-component-system architecture requires a specific approach to UI testing.

### Component Unit Tests

For component unit tests, we focus on testing the component's properties and behaviors:

```rust
#[test]
fn test_button_component() {
    // Create a test app with required plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(UiPlugin);
    
    // Spawn a button entity
    let button_entity = app.world.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        },
        Button {},
    )).id();
    
    // Test button properties
    let button_query = app.world.query::<(&Button, &Style)>();
    let (_, style) = button_query.get(&app.world, button_entity).unwrap();
    
    assert_eq!(style.width, Val::Px(150.0));
    assert_eq!(style.height, Val::Px(50.0));
}
```

### Interaction Tests

Interaction tests validate how components respond to user input:

```rust
#[test]
fn test_button_click() {
    // Create a test app with required plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(UiPlugin)
       .add_event::<ButtonClickEvent>();
    
    // Add a system to handle button interaction
    app.add_systems(Update, button_click_system);
    
    // Spawn a button entity
    let button_entity = app.world.spawn((
        ButtonBundle { ... },
        Button {},
    )).id();
    
    // Simulate interaction with the button
    app.world.entity_mut(button_entity)
             .insert(Interaction::Clicked);
    
    // Run systems
    app.update();
    
    // Check if appropriate events were generated
    let button_events = app.world.resource::<Events<ButtonClickEvent>>();
    let mut reader = button_events.get_reader();
    let events: Vec<_> = reader.read(button_events).collect();
    
    assert_eq!(events.len(), 1);
}
```

### Layout Tests

Layout tests verify that UI components maintain proper layout:

```rust
#[test]
fn test_responsive_layout() {
    // Create a test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(UiPlugin);
    
    // Create a responsive UI component
    let panel_entity = app.world.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                // Other style properties
                ..default()
            },
            ..default()
        },
    )).id();
    
    // Test different window sizes
    for window_width in [800.0, 1200.0, 1600.0] {
        // Update window size
        app.world.resource_mut::<Windows>().primary_mut().set_resolution(window_width, 900.0);
        
        // Run layout systems
        app.update();
        
        // Verify layout adaptation
        // ...
    }
}
```

## Visual Regression Testing

For visual regression testing, see the [Visual Regression](visual_regression.md) guide.

## Integration with Game Logic

UI component tests should also verify correct integration with game logic when applicable:

```rust
#[test]
fn test_card_drag_affects_game_state() {
    // Setup test app with UI and game state
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(UiPlugin)
       .add_plugin(GameStatePlugin);
    
    // Setup game state
    // ...
    
    // Test drag interaction
    // ...
    
    // Verify game state was updated correctly
    // ...
}
```

## Testing Accessibility

UI component tests should also verify accessibility requirements:

```rust
#[test]
fn test_button_accessibility() {
    // Create a test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(UiPlugin)
       .add_plugin(AccessibilityPlugin);
    
    // Create a button with accessibility attributes
    let button_entity = app.world.spawn((
        ButtonBundle { ... },
        Button {},
        AccessibilityNode {
            label: "Submit".into(),
            role: AccessibilityRole::Button,
            ..default()
        },
    )).id();
    
    // Verify accessibility properties
    let accessibility_query = app.world.query::<&AccessibilityNode>();
    let accessibility = accessibility_query.get(&app.world, button_entity).unwrap();
    
    assert_eq!(accessibility.label, "Submit");
    assert_eq!(accessibility.role, AccessibilityRole::Button);
}
```

## Best Practices

1. **Test Real Scenarios**: Focus on testing real user scenarios rather than implementation details
2. **Isolate UI Logic**: Keep UI logic separate from game logic to make testing easier
3. **Test Different Screen Sizes**: Verify that UI works across different screen resolutions
4. **Test Accessibility**: Ensure UI components meet accessibility standards
5. **Use Visual Regression Tests**: Complement code tests with visual regression tests

## Related Documentation

- [Visual Regression](visual_regression.md)
- [Drag and Drop](../interaction/drag_and_drop.md)
- [Card Rendering](../cards/card_rendering.md)
- [Accessibility](../accessibility/screen_reader.md) 