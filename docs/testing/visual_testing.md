# Visual Testing

Visual testing in Rummage ensures that the game's user interface renders correctly and consistently across different environments. This approach helps identify visual regressions and ensures a high-quality user experience.

## Overview

Visual testing in Rummage:
- Verifies UI component rendering
- Catches visual regressions
- Ensures consistent appearance
- Validates UI interactions visually

## Testing Approach

Visual testing uses image comparison and automated validation:

1. **Reference Images**: Maintain a set of approved reference images
2. **Render Comparison**: Generate new renders and compare against references
3. **Pixel Tolerance**: Allow small differences to accommodate rendering variations
4. **Visual Regression Detection**: Identify unintended visual changes

## Example: Card Rendering Test

```rust
#[test]
fn test_card_rendering() {
    // Setup test environment with rendering support
    let mut app = App::new();
    app.add_plugins(RenderTestPlugins);
    
    // Create test card
    let card = setup_test_card(&mut app, "Lightning Bolt");
    
    // Render card to texture
    let texture = render_entity_to_texture(&mut app, card);
    
    // Compare with reference image
    let reference = load_reference_image("cards/lightning_bolt.png");
    
    // Calculate similarity (allowing for minor variations)
    let comparison = compare_images(texture, reference);
    
    // Verify similarity exceeds threshold
    assert!(comparison.similarity > 0.99, 
           "Card should render correctly with 99% similarity to reference");
    
    // If failed, save diff image for review
    if comparison.similarity <= 0.99 {
        save_diff_image("failed_tests/lightning_bolt_diff.png", comparison.diff_image);
    }
}
```

## UI Component Testing

Test individual UI components for correct rendering:

```rust
#[test]
fn test_mana_symbol_rendering() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(RenderTestPlugins);
    
    // Test all mana symbol types
    let symbol_types = vec!["W", "U", "B", "R", "G", "C", "1", "X"];
    
    for symbol in symbol_types {
        // Create mana symbol entity
        let entity = app.world.spawn((
            ManaSymbol { symbol: symbol.to_string() },
            Transform::default(),
            Visibility::default(),
        )).id();
        
        // Render symbol
        let texture = render_entity_to_texture(&mut app, entity);
        
        // Compare with reference
        let reference = load_reference_image(&format!("mana_symbols/{}.png", symbol));
        let comparison = compare_images(texture, reference);
        
        // Verify rendering
        assert!(comparison.similarity > 0.99, 
               "Mana symbol {} should render correctly", symbol);
    }
}
```

## Layout Testing

Test that UI layouts render correctly at different resolutions:

```rust
#[test]
fn test_battlefield_layout() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(RenderTestPlugins);
    
    // Test different screen resolutions
    let resolutions = vec![
        (1280, 720),   // HD
        (1920, 1080),  // Full HD
        (2560, 1440),  // QHD
        (3840, 2160),  // 4K
    ];
    
    for (width, height) in resolutions {
        // Set resolution
        app.world.resource_mut::<RenderSettings>().resolution = (width, height);
        
        // Setup a basic battlefield with some cards
        setup_test_battlefield(&mut app, 5);  // 5 cards on battlefield
        
        // Render full battlefield
        let texture = render_screen_to_texture(&mut app);
        
        // Compare with reference for this resolution
        let reference = load_reference_image(&format!("layouts/battlefield_{}x{}.png", width, height));
        let comparison = compare_images(texture, reference);
        
        // Verify layout is correct
        assert!(comparison.similarity > 0.98, 
               "Battlefield layout should render correctly at {}x{}", width, height);
    }
}
```

## Animation Testing

Test that animations render correctly:

```rust
#[test]
fn test_card_draw_animation() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(RenderTestPlugins);
    
    // Setup player with library and hand
    let player = setup_test_player(&mut app);
    
    // Set up animation capture
    let frames = capture_animation_frames(&mut app, 30, || {
        // Trigger card draw animation
        app.world.send_event(DrawCardEvent { player });
        app.update();
    });
    
    // Check key frames against references
    let key_frame_indices = vec![0, 10, 20, 29];  // Start, middle, end frames
    
    for idx in key_frame_indices {
        let frame = &frames[idx];
        let reference = load_reference_image(&format!("animations/draw_card_frame_{}.png", idx));
        let comparison = compare_images(frame, reference);
        
        assert!(comparison.similarity > 0.97, 
               "Animation frame {} should match reference", idx);
    }
}
```

## Accessibility Visual Testing

Test accessibility features visually:

```rust
#[test]
fn test_high_contrast_mode() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(RenderTestPlugins);
    
    // Enable high contrast mode
    app.world.resource_mut::<AccessibilitySettings>().high_contrast_mode = true;
    
    // Render battlefield with cards
    setup_test_battlefield(&mut app, 3);
    let texture = render_screen_to_texture(&mut app);
    
    // Compare with high contrast reference
    let reference = load_reference_image("accessibility/high_contrast_battlefield.png");
    let comparison = compare_images(texture, reference);
    
    assert!(comparison.similarity > 0.98, 
           "High contrast mode should render correctly");
    
    // Verify contrast ratios meet WCAG guidelines
    let contrast_analysis = analyze_contrast_ratios(texture);
    assert!(contrast_analysis.min_ratio >= 4.5, 
           "Minimum contrast ratio should meet WCAG AA standard");
}
```

## Testing CI Pipeline Integration

Visual tests can be integrated into CI/CD pipelines:

```rust
// This code would be in your CI setup, not an actual test
fn setup_visual_testing_ci() {
    // Run all visual tests
    let test_results = run_visual_tests();
    
    // Process results
    if !test_results.all_passed {
        // Generate report with diffs
        let report = generate_visual_diff_report(test_results);
        
        // Upload diffs as artifacts
        upload_artifacts(report.diff_images);
        
        // Fail the build
        std::process::exit(1);
    }
}
```

## Best Practices

For effective visual testing in Rummage:

1. **Maintain Reference Images**: Keep a versioned set of approved reference images
2. **Use Appropriate Tolerance**: Allow for minor rendering differences across platforms
3. **Test Multiple Resolutions**: Verify UI works across different screen sizes
4. **Automate Visual Testing**: Integrate visual tests into CI/CD pipelines
5. **Test Accessibility Modes**: Verify high-contrast and other accessibility features
6. **Generate Visual Reports**: Create visual reports for failed tests
7. **Test With Different Themes**: Verify rendering in all visual themes

## Related Documentation

For more information on testing in Rummage, see:

- [Unit Testing](unit_testing.md)
- [Integration Testing](integration_testing.md)
- [End-to-End Testing](end_to_end_testing.md)
- [Game UI Testing](../game_gui/testing/index.md) 