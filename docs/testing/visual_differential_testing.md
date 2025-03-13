# Visual Differential Testing

## Overview

Visual differential testing is a critical component of our testing strategy that ensures rendering consistency across different platforms, hardware configurations, and code changes. This approach compares visual outputs from different rendering paths or configurations to detect unintended visual regressions.

## Why Visual Differential Testing

For a card game like MTG Commander, visual presentation is essential to gameplay. Players need to:

1. Clearly see card text and artwork
2. Recognize game state at a glance
3. Identify UI elements consistently
4. Experience smooth animations and transitions

Traditional unit and integration tests verify logic but cannot fully validate the visual correctness of rendered output. Visual differential testing fills this gap.

## Implementation Approach

Our visual differential testing framework uses the following approach:

### 1. Reference Image Generation

```rust
#[test]
fn generate_reference_images() {
    let mut app = App::new();
    app.add_plugins(RenderingTestPlugins)
       .add_systems(Startup, setup_reference_scene);
    
    // Capture specific game states
    capture_game_states(&mut app, "reference_images");
    
    // Generate reference images for each state
    for state in TEST_STATES {
        app.world.send_event(SetGameStateEvent(state));
        app.update();
        
        let screenshot = take_screenshot(&app);
        save_reference_image(screenshot, &format!("{}_reference.png", state));
    }
}
```

### 2. Comparison Testing

```rust
#[test]
fn compare_with_reference_images() {
    let mut app = App::new();
    app.add_plugins(RenderingTestPlugins)
       .add_systems(Startup, setup_test_scene);
    
    // Test each state against reference
    for state in TEST_STATES {
        app.world.send_event(SetGameStateEvent(state));
        app.update();
        
        let current_screenshot = take_screenshot(&app);
        let reference = load_reference_image(&format!("{}_reference.png", state));
        
        let difference = compare_images(&current_screenshot, &reference);
        assert!(
            difference.similarity_score > 0.99,
            "Visual difference detected for state {}: score {}",
            state,
            difference.similarity_score
        );
    }
}
```

## Key Components

### Screenshot Capture System

The screenshot capture system uses Bevy's render extraction to create high-fidelity captures:

```rust
/// Captures the current frame buffer as an image
fn take_screenshot(app: &App) -> Image {
    // Access render resources
    let render_app = app.sub_app(RenderApp);
    let render_device = render_app.world.resource::<RenderDevice>();
    
    // Extract current frame buffer
    // ... extraction code ...
    
    // Return as image
    image
}
```

### Image Comparison Tools

We use multiple comparison strategies to detect visual differences:

1. **Pixel-Perfect Comparison**
   - Exact per-pixel comparison for critical UI elements
   - Zero tolerance for differences in text rendering

2. **Perceptual Hashing**
   - Hash-based comparison resistant to minor compression artifacts
   - Useful for detecting major layout changes

3. **Structural Similarity (SSIM)**
   - Detects changes in luminance, contrast, and structure
   - Allows tolerance for minor visual variations

```rust
/// Compare two images using multiple strategies
fn compare_images(image1: &Image, image2: &Image) -> ComparisonResult {
    let pixel_diff = pixel_perfect_compare(image1, image2);
    let perceptual_hash_diff = perceptual_hash_compare(image1, image2);
    let ssim_score = structural_similarity(image1, image2);
    
    ComparisonResult {
        pixel_difference_count: pixel_diff,
        phash_difference: perceptual_hash_diff,
        similarity_score: ssim_score,
        // Additional metrics
    }
}
```

### Failure Analysis Tools

When visual tests fail, developers need to understand why. Our framework provides:

1. **Difference Visualization**
   - Generates heatmap of pixel differences
   - Highlights areas of major change

2. **Automatic Baseline Updates**
   - Command to update reference images when changes are intentional
   - Version controlled to track visual evolution

3. **Test Metadata**
   - Records rendering context (GPU, driver, resolution)
   - Enables filtering for platform-specific issues

## Testing Specific Visual Components

### Card Rendering Tests

Card rendering must be consistent across all representations:

1. **Card in Hand**
   - Test legibility of card text at various zoom levels
   - Verify artwork rendering quality

2. **Battlefield Representation**
   - Test card visibility in tapped/untapped states
   - Verify counter and attachment visibility

3. **Stack Representation**
   - Test visual clarity of cards on the stack
   - Verify targeting indicators

### UI Element Tests

UI elements must maintain functional visibility:

1. **Button States**
   - Test normal, hover, pressed, and disabled states
   - Verify text rendering within buttons

2. **Dialog Components**
   - Test modal overlays and popups
   - Verify scroll area rendering

3. **Game Status Indicators**
   - Test phase indicators, turn markers
   - Verify player status displays

### Animation Tests

Key animations must remain smooth and consistent:

1. **Card Movement**
   - Test zone transition animations
   - Verify consistency of motion paths

2. **Effect Animations**
   - Test spell and ability visual effects
   - Verify particle system rendering

## Platform-Specific Testing

Our differential testing accounts for platform variations:

1. **Cross-Platform Comparison**
   - Compare renders between Windows, Linux, macOS
   - Document expected minor variations

2. **Hardware Configuration Matrix**
   - Test on reference low, medium, and high-end GPUs
   - Verify scaling behavior at different resolutions

## Integration with CI/CD

Visual differential tests are integrated into our CI/CD pipeline:

1. **Scheduled Visual Testing**
   - Daily visual regression tests on reference hardware
   - Comparison against golden master images

2. **PR Testing**
   - Visual impact assessment for UI-related changes
   - Optional manual review for intentional visual changes

3. **Artifact Generation**
   - Storage of test images for manual review
   - Historical tracking of visual evolution

## Performance Considerations

Visual testing can be resource-intensive, so we employ several optimizations:

1. **Selective Testing**
   - Test only visually affected components
   - Skip tests for non-visual code changes

2. **Parallelization**
   - Run visual tests across multiple workers
   - Cache intermediate results when possible

3. **Headless Testing**
   - Use GPU acceleration in headless environments
   - Generate minimum viable renderings for comparison

## Tools and Libraries

Our visual testing leverages:

1. **Bevy Render Extraction**
   - Direct access to frame buffer data
   - High-fidelity image capture

2. **Image Comparison Libraries**
   - ImageMagick for pixel comparisons
   - DHash for perceptual hashing
   - OpenCV for SSIM calculations

3. **Custom Diffing Tools**
   - Specialized card rendering comparisons
   - UI-aware difference detection

## Best Practices

When developing visual differential tests:

1. **Isolate Visual Components**
   - Test specific visual elements in isolation
   - Control all inputs that affect rendering

2. **Establish Clear Baselines**
   - Document when and how reference images were generated
   - Include visual specifications with tests

3. **Handle Animation and Dynamic Content**
   - Use consistent time steps for animated content
   - Freeze animations during testing

4. **Plan for Evolution**
   - Establish process for intentional visual changes
   - Document expected visual behavior

## Case Study: Card Rendering Test

```rust
#[test]
fn test_card_rendering_consistency() {
    let mut app = App::new();
    app.add_plugins(RenderingTestPlugins)
       .add_systems(Startup, setup_card_render_test);
    
    // Test standard card rendering
    let card_entity = spawn_test_card(&mut app, "Test Card", "Test Card Description");
    app.update();
    
    // Capture card rendering
    let card_image = capture_entity_rendering(&app, card_entity);
    
    // Compare with reference
    let reference = load_reference_image("standard_card_reference.png");
    let difference = compare_images(&card_image, &reference);
    
    // Assert within acceptable tolerance
    assert!(
        difference.similarity_score > 0.995,
        "Card rendering differs from reference: score {}",
        difference.similarity_score
    );
    
    // If test fails, save difference visualization
    if difference.similarity_score <= 0.995 {
        save_difference_visualization(&card_image, &reference, "card_rendering_diff.png");
    }
}
```

This comprehensive approach to visual differential testing ensures that the Rummage game engine maintains consistent and high-quality rendering across all supported platforms and configurations. 