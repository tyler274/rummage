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

### 1. Maintain Stable Test Environments

Visual tests are sensitive to environmental changes, so maintain stable test environments:

- Use fixed window sizes for tests
- Standardize font rendering settings
- Control hardware acceleration options

### 2. Version Control Reference Images

Reference images should be version controlled:

- Check in reference images with corresponding code
- Document intentional visual changes
- Label reference image sets by version

### 3. Tolerance Calibration

Set appropriate tolerance levels for different UI components:

- Text elements: High precision (low tolerance)
- Animations: Lower precision (higher tolerance)
- Background elements: Medium precision

### 4. Test Organization

Organize visual tests logically:

- Group by UI component
- Separate critical vs. non-critical visuals
- Create dedicated suites for performance-sensitive renders

## Implementation in Rummage

In Rummage, we've implemented visual differential testing with these components:

### Screenshot Capture System

```rust
// src/game_engine/tests/visual_diff.rs

pub fn take_screenshot(app: &App) -> Option<DynamicImage> {
    // Access render resources
    if let Ok(render_app) = app.get_sub_app(RenderApp) {
        let render_device = render_app.world.resource::<RenderDevice>();
        
        // Get the window
        if let Some(window) = app.world().get_resource::<bevy::window::PrimaryWindow>() {
            // Extract pixels from GPU
            // ... implementation details ...
            
            return Some(image);
        }
    }
    None
}
```

### Comparison Methods

```rust
// src/game_engine/tests/visual_diff.rs

// Our comparison methods include:

// 1. Pixel-perfect comparison
fn pixel_perfect_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // ... implementation details ...
}

// 2. Perceptual hash comparison
fn perceptual_hash_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // ... implementation details ...
}

// 3. Structural similarity comparison
fn structural_similarity_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // ... implementation details ...
}

// 4. Combined approach
fn combined_compare(image1: &DynamicImage, image2: &DynamicImage) -> ComparisonResult {
    // ... implementation details ...
}
```

### Test Examples

```rust
// src/game_engine/tests/visual_diff.rs

#[test]
fn test_card_rendering_consistency() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_plugin(VisualTestingPlugin)
       .add_systems(Startup, setup_test_scene);
    
    // List of card states to test
    let test_states = [
        "card_in_hand",
        "card_on_battlefield",
        "card_tapped",
        "card_with_counters",
    ];
    
    // Test each card state
    for state in &test_states {
        setup_card_state(&mut app, state);
        app.update();
        
        if let Some(screenshot) = take_screenshot(&app) {
            // Compare with reference or generate if needed
            // ... implementation details ...
        }
    }
}
```

## Common Challenges and Solutions

### 1. Flaky Tests Due to Animation

**Challenge**: Animation frames can cause inconsistent screenshots.

**Solution**: Implement deterministic animation timing for tests:

```rust
// Freeze animations at specific frames for testing
fn setup_deterministic_animation_state(app: &mut App, animation: &str, frame: usize) {
    app.insert_resource(AnimationTestState {
        freeze_animations: true,
        current_frame: frame,
    });
    
    app.update();
}
```

### 2. Text Rendering Differences

**Challenge**: Font rendering can vary across platforms.

**Solution**: Custom text comparison that focuses on content rather than exact pixels:

```rust
fn compare_text_elements(image1: &DynamicImage, image2: &DynamicImage, regions: &[TextRegion]) -> bool {
    // Extract text using OCR from defined regions
    // Compare text content rather than exact pixels
    // ... implementation details ...
}
```

### 3. Resolution Differences

**Challenge**: Different screen resolutions affect rendering.

**Solution**: Normalize images before comparison:

```rust
fn normalize_for_comparison(image: &DynamicImage) -> DynamicImage {
    // Resize to standard test resolution
    // Apply consistent post-processing
    // ... implementation details ...
}
```

## Future Improvements

1. **AI-Assisted Comparison**
   - Use machine learning to identify semantically important differences
   - Reduce false positives from minor visual changes

2. **Automated Tolerance Adjustment**
   - Dynamically adjust comparison thresholds based on component type
   - Learn from historical test results

3. **Visual Component Isolation**
   - Test individual visual components in isolation
   - Compose tested components into complete UI

## Conclusion

Visual differential testing is essential for maintaining a consistent and polished user experience in Rummage. By systematically capturing, comparing, and analyzing visual components, we can detect unintended changes early and ensure that the game looks and feels consistent across all platforms and configurations.

When implementing visual tests, remember:
1. Start with critical UI components
2. Establish baseline images early
3. Set appropriate tolerance levels
4. Document intentional visual changes
5. Integrate with your CI/CD pipeline

These practices will help maintain visual quality throughout development and ensure players have a consistent, high-quality experience. 