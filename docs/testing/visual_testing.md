# Visual Testing

Visual testing in Rummage allows us to detect unintended visual regressions in the game's UI and rendering. By capturing screenshots during automated tests and comparing them to reference images, we can identify changes that might break the user experience.

## How Visual Testing Works

1. **Test Fixture Setup**: Each test creates a controlled environment with known entities and camera settings
2. **Reference Images**: The system captures screenshots and compares them against reference images
3. **Difference Detection**: Using image comparison algorithms, the system identifies visual differences
4. **Artifact Generation**: When differences exceed a threshold, visual diffs are generated for inspection

## Running Visual Tests Locally

To run the visual tests locally, use:

```bash
cargo test --package rummage --lib "tests::visual_testing::"
```

### Updating Reference Images

If you've made intentional visual changes, update the reference images:

```bash
GENERATE_REFERENCES=1 cargo test --package rummage --lib "tests::visual_testing::"
```

## CI Integration

Visual tests run automatically on GitHub Actions:

1. **Pull Requests**: Tests run to catch visual regressions
2. **Reference Updates**: When visual changes are intentional, update references with `GENERATE_REFERENCES=1`
3. **Artifact Inspection**: Test failures produce visual diffs that can be downloaded as artifacts

## Creating New Visual Tests

To create a new visual test:

1. Create a test fixture that sets up the specific visual scenario
2. Use `request_screenshot()` to capture the scene
3. Run your test with `GENERATE_REFERENCES=1` to create the initial reference images
4. Verify the reference images match your expectations
5. Run without the flag to ensure tests pass

## Headless Rendering

For CI environments, we use Xvfb (X Virtual Framebuffer) to provide a virtual display for rendering. This allows tests to run in headless environments like GitHub Actions.

The visual testing system uses a special headless configuration with:

- Fixed window size for deterministic rendering
- Vulkan backend for better compatibility
- Low power mode for CI environments
- Invisible windows to avoid flickering on CI servers

## Troubleshooting

If tests are failing unexpectedly:

1. **Download Artifacts**: Check the visual diff artifacts from the GitHub Actions workflow
2. **Check for Non-Determinism**: Ensure your test setup is deterministic
3. **Verify References**: Make sure reference images are up to date with the current visual design
4. **Check Environment**: The test environment should match the CI environment as closely as possible

## Best Practices

- Keep visual tests focused on specific components or screens
- Use deterministic values for positions and sizes
- Avoid animation-dependent tests that might be flaky
- Update reference images when intentional design changes are made

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

# Visual Differential Testing

Visual differential testing is a technique to automatically detect visual changes between versions of the codebase. Rummage implements a robust visual testing system that integrates with the save/load/replay system to provide powerful testing capabilities.

## Core Features

- **Save Game Snapshots**: Capture the visual state of any saved game
- **Replay Point Captures**: Take snapshots at specific points in game replays
- **Visual Comparison**: Compare images against reference images
- **Difference Visualization**: Generate visual difference maps
- **CI Integration**: Run visual tests in continuous integration pipelines

## Basic Usage

### Manually Capturing a Screenshot

```rust
use rummage::tests::visual_testing::capture::{request_screenshot, take_screenshot};

// Take a screenshot of the whole screen
let image = take_screenshot();

// Request a screenshot of a specific entity
world.send_event(request_screenshot(entity, "my_screenshot.png"));
```

### Working with Saved Games

```rust
use rummage::tests::visual_testing::capture::capture_saved_game_snapshot;

// Capture a screenshot of the current state of a saved game
let image = capture_saved_game_snapshot(world, "my_save", None, None);

// Capture a screenshot of a specific turn
let image = capture_saved_game_snapshot(world, "my_save", Some(3), None);

// Capture a screenshot at a specific replay step
let image = capture_saved_game_snapshot(world, "my_save", None, Some(5));
```

### Comparing with Reference Images

```rust
use rummage::tests::visual_testing::comparison::compare_images;
use rummage::tests::visual_testing::utils::load_reference_image;

// Load a reference image
let reference = load_reference_image("my_reference.png").unwrap();

// Compare with a captured image
let result = compare_images(&reference, &captured, 0.1);

// Check if there are significant differences
if result.has_significant_differences() {
    // Save the difference visualization
    result.save_difference("difference.png");
    
    // Take action based on the difference
    panic!("Visual difference detected!");
}
```

## Automatic Testing

The visual testing system can automatically test saved games:

```rust
use rummage::tests::visual_testing::fixtures::test_all_saved_games;

// Test all saved games against reference images
let results = test_all_saved_games(world);

// Check results
for result in results {
    if !result.success {
        println!("Test failed: {}", result.name);
    }
}
```

## Integration with Save/Load System

The visual testing system integrates with the save/load system to automatically capture snapshots when:

1. A game is saved
2. A replay is stepped through
3. A game is loaded from a save file

This happens automatically through the following systems:

- `take_save_game_snapshot`: Captures snapshots when games are saved
- `take_replay_snapshot`: Captures snapshots during replay steps

## CI Testing Pipeline

The visual testing system can be integrated into a CI pipeline:

```rust
use rummage::tests::visual_testing::ci::{setup_ci_visual_test, is_ci_environment};

// If running in CI environment, configure for CI
if is_ci_environment() {
    setup_ci_visual_test(app);
}

// Run tests
let results = run_visual_tests();

// Report results
for result in results {
    if !result.success {
        // Upload difference images to CI artifacts
        upload_artifact(&result.difference_image);
    }
}
```

## Implementation Details

### Screenshot Capture

The system can capture screenshots using different methods:

1. **Whole Screen**: Capture the entire game window
2. **Entity Focus**: Focus on a specific entity
3. **Camera View**: Capture what a specific camera sees

### Image Comparison

Images are compared using one of several methods:

1. **Pixel-by-Pixel**: Exact comparison of each pixel
2. **Histogram**: Compare color distributions
3. **Feature-Based**: Compare structural features
4. **Neural**: Use neural networks for perceptual comparison

### Testing Configuration

Configure the visual testing system using the `VisualTestConfig` resource:

```rust
use rummage::tests::visual_testing::config::{VisualTestConfig, ComparisonMethod};

// Configure visual testing
app.insert_resource(VisualTestConfig {
    reference_directory: PathBuf::from("reference_images"),
    difference_directory: PathBuf::from("difference_images"),
    comparison_method: ComparisonMethod::PixelByPixel,
    similarity_threshold: 0.95,
    generate_references: false,
});
```

## Save/Load Integration for Visual Differential Testing

The Rummage visual testing system integrates with the save/load/replay systems to enable visual differential testing of specific game states. This allows developers to:

1. **Capture Game State Snapshots**: Automatically take screenshots when games are saved
2. **Replay Visual Validation**: Capture visuals during replay for regression testing
3. **Time-Travel Debugging**: Compare visuals at different points in game history

### Using Save Game Snapshots

Snapshots are automatically captured when a game is saved:

```rust
// Save a game, which triggers a snapshot
world.send_event(SaveGameEvent {
    slot_name: "test_save".to_string(),
});
```

### Using Replay Snapshots

Snapshots are taken at each step during replay:

```rust
// Step through a replay, capturing visuals at each step
world.send_event(StepReplayEvent);
```

### Testing Game State Evolution

You can use this system to test how the game state evolves visually:

```rust
#[test]
fn test_visual_game_progression() {
    // Setup game and save initial state
    setup_game();
    world.send_event(SaveGameEvent { slot_name: "initial".to_string() });
    
    // Make game progress
    play_turn();
    
    // Save and capture the state after progression
    world.send_event(SaveGameEvent { slot_name: "after_turn".to_string() });
    
    // Compare the visuals between states
    assert_visual_difference("initial", "after_turn", 0.2);
}
```

### Implementation

The integration between save/load and visual systems is handled by:

1. The `SaveGameSnapshot` component attached to cameras during save/load operations
2. `take_save_game_snapshot` and `take_replay_snapshot` systems that respond to game events
3. The visual testing framework that can compare snapshot images

This integration enables automated visual regression testing against saved game states, providing a powerful tool for validating game behavior. 