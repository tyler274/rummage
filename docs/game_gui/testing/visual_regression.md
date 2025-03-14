# Visual Regression Testing

This guide covers visual regression testing for the Rummage game's UI components, ensuring visual consistency across updates.

## Introduction to Visual Regression Testing

Visual regression testing is a technique that ensures UI components maintain their visual appearance across code changes. It works by capturing screenshots of UI components and comparing them against baseline images to detect unexpected visual changes.

In Rummage, visual regression testing is essential for maintaining a consistent, polished user interface across the diverse board states that can occur in a Magic: The Gathering game.

## Visual Testing Framework

Rummage uses a custom visual testing framework built on top of Bevy's testing capabilities, with these key components:

1. **Snapshot Capture**: Renders UI components to off-screen buffers and captures their visual state
2. **Image Comparison**: Compares captured images against baseline images pixel-by-pixel
3. **Difference Visualization**: Generates difference images highlighting visual changes
4. **Test Reporting**: Provides detailed reports on visual changes

## Setting Up Visual Regression Tests

Visual regression tests are located in the `tests/visual` directory and follow this structure:

```
tests/
  visual/
    components/
      card_test.rs
      menu_test.rs
    screens/
      battlefield_test.rs
      hand_test.rs
    reference_images/
      card_normal.png
      card_tapped.png
```

## Writing Visual Regression Tests

Here's how to write a visual regression test:

```rust
#[test]
fn test_card_visual_appearance() {
    // Set up a test app with required plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(RenderPlugin)
       .add_plugin(UiPlugin)
       .add_plugin(VisualTestPlugin);
    
    // Create a test card entity
    let card_entity = app.world.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(63.0, 88.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Card {
            name: "Test Card".to_string(),
            // Other card properties
        },
        CardVisual {
            state: CardVisualState::Normal,
        },
    )).id();
    
    // Set up camera
    app.world.spawn(Camera2dBundle::default());
    
    // Capture and compare visual state
    let visual_test = VisualTest::new(&mut app)
        .capture_entity(card_entity)
        .compare_to_reference("card_normal.png")
        .with_tolerance(0.01);  // 1% pixel difference tolerance
    
    // Assert visual consistency
    assert!(visual_test.is_visually_consistent());
}
```

## Testing Different Visual States

It's important to test different visual states of UI components:

```rust
#[test]
fn test_card_visual_states() {
    // Set up test app and card entity
    // ...
    
    // Test normal state
    let visual_test = VisualTest::new(&mut app)
        .capture_entity(card_entity)
        .compare_to_reference("card_normal.png");
    assert!(visual_test.is_visually_consistent());
    
    // Change to tapped state
    app.world.entity_mut(card_entity).get_mut::<CardVisual>().unwrap().state = CardVisualState::Tapped;
    app.world.entity_mut(card_entity).get_mut::<Transform>().unwrap().rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
    
    // Update systems to apply visual changes
    app.update();
    
    // Test tapped state
    let visual_test = VisualTest::new(&mut app)
        .capture_entity(card_entity)
        .compare_to_reference("card_tapped.png");
    assert!(visual_test.is_visually_consistent());
}
```

## Testing Responsive Layouts

Visual tests should also verify appearance across different screen sizes:

```rust
#[test]
fn test_responsive_menu_layout() {
    // Set up test app
    // ...
    
    // Test different screen sizes
    for (width, height, suffix) in [
        (1920, 1080, "full_hd"),
        (1280, 720, "hd"),
        (800, 600, "low_res"),
    ] {
        // Set window size
        app.world.resource_mut::<Windows>().primary_mut().set_resolution(
            width as f32, 
            height as f32
        );
        
        // Update systems to apply layout changes
        app.update();
        
        // Capture and compare
        let reference_name = format!("menu_{}.png", suffix);
        let visual_test = VisualTest::new(&mut app)
            .capture_full_screen()
            .compare_to_reference(&reference_name);
            
        assert!(visual_test.is_visually_consistent());
    }
}
```

## Generating Baseline Images

When developing new UI components, you'll need to generate baseline images:

```rust
#[test]
fn generate_baseline_for_new_component() {
    // Set up test app and component
    // ...
    
    // Generate baseline image instead of comparing
    let visual_test = VisualTest::new(&mut app)
        .capture_entity(component_entity)
        .generate_baseline("new_component.png");
        
    // Verify the baseline was created
    assert!(visual_test.baseline_generated());
}
```

## Handling Animations

For components with animations, capture key frames:

```rust
#[test]
fn test_card_draw_animation() {
    // Set up test app
    // ...
    
    // Start animation
    app.world.spawn(DrawCardEvent { ... });
    
    // Capture key frames of the animation
    let frames = [0, 5, 10, 15, 20];
    
    for frame in frames {
        // Advance animation to specific frame
        for _ in 0..frame {
            app.update();
        }
        
        // Capture and compare
        let reference_name = format!("card_draw_frame_{}.png", frame);
        let visual_test = VisualTest::new(&mut app)
            .capture_entity(card_entity)
            .compare_to_reference(&reference_name);
            
        assert!(visual_test.is_visually_consistent());
    }
}
```

## Configuring Test Thresholds

Adjust tolerance thresholds for different components:

```rust
// Text rendering may vary slightly across platforms
let text_test = VisualTest::new(&mut app)
    .capture_entity(text_entity)
    .compare_to_reference("card_text.png")
    .with_tolerance(0.03);  // 3% tolerance

// Card art should be pixel-perfect
let art_test = VisualTest::new(&mut app)
    .capture_entity(art_entity)
    .compare_to_reference("card_art.png")
    .with_tolerance(0.0);  // 0% tolerance
```

## Reporting and Debugging

When visual tests fail, detailed reports help identify the issue:

```rust
let result = visual_test.run();
if !result.passed {
    // Generate detailed report
    result.generate_report("visual_test_report");
    
    // Output difference statistics
    println!("Pixel difference: {}%", result.difference_percentage);
    println!("Most different area: {:?}", result.most_different_region);
    
    // Save difference image
    result.save_difference_image("difference.png");
}
```

## Integration with CI/CD

Visual regression tests can be integrated into CI/CD pipelines:

```yaml
# .github/workflows/visual-tests.yml
visual-tests:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v2
    - name: Set up Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y libxcb-shape0-dev libxcb-xfixes0-dev
    - name: Run visual regression tests
      run: cargo test --package rummage --test visual_tests
    - name: Upload difference images on failure
      if: failure()
      uses: actions/upload-artifact@v2
      with:
        name: visual-diff-images
        path: target/visual-test-output/
```

## Best Practices

1. **Keep baseline images in version control** to track intentional visual changes
2. **Test across different themes** (light/dark mode)
3. **Use appropriate tolerances** for different components
4. **Set up CI/CD integration** to catch visual regressions early
5. **Test across different screen resolutions** to ensure responsive design works
6. **Include visual tests in your development workflow**

## Troubleshooting

### Common Issues

1. **Platform differences**: Different operating systems may render text slightly differently
2. **Resolution variations**: High-DPI displays may produce different pixel counts
3. **Color profile differences**: Different monitors may display colors differently

### Solutions

1. Use tolerance thresholds appropriate to the component
2. Normalize rendering environment in CI/CD
3. Implement platform-specific baseline images when necessary

## Related Documentation

- [UI Component Testing](component_testing.md)
- [Card Visualization](../cards/card_visualization.md)
- [Visual Themes](../playmat/visual_themes.md) 