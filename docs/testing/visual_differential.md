# Visual Differential Testing in Rummage

Visual differential testing is a crucial part of Rummage's testing strategy, ensuring visual consistency across different platforms, hardware configurations, and code changes. This guide explains how to use and extend our visual testing system.

## Overview

The visual differential testing system captures screenshots during tests, compares them against known reference images, and identifies visual regressions. This approach allows us to:

1. Detect unintended visual changes
2. Ensure consistent rendering across platforms
3. Verify UI components maintain their appearance
4. Test animations and transitions

## System Architecture

Our system is organized into several modular components:

- **Capture**: Handles screenshot capture during tests
- **Comparison**: Implements various image comparison methods
- **Config**: Manages test configuration and settings
- **Fixtures**: Provides test scene setup and standard states
- **Utils**: Common utilities for file management and test setup

## Setting Up Visual Tests

### Basic Test Setup

To set up a basic visual test:

```rust
use bevy::prelude::*;
use crate::game_engine::tests::setup_visual_test_environment;
use crate::game_engine::visual_testing::{
    take_screenshot, compare_images, load_reference_image, save_difference_visualization,
};

#[test]
fn test_visual_component() {
    // Set up test environment
    let mut app = App::new();
    setup_visual_test_environment(&mut app);
    
    // Set up test scene
    // ...
    
    // Take screenshot
    let screenshot = take_screenshot(&app).unwrap();
    
    // Compare with reference
    let reference = load_reference_image("component.png").unwrap();
    let result = compare_images(&screenshot, &reference);
    
    // Assert similar appearance
    assert!(result.similarity_score >= 0.99, "Visual regression detected");
}
```

### Running Tests With Different Comparison Methods

The system supports multiple comparison methods:

- **Pixel-Perfect**: Exact pixel-by-pixel comparison
- **Perceptual Hash**: Hash-based comparison resistant to minor compression artifacts
- **SSIM**: Structural similarity that focuses on luminance, contrast, and structure
- **Combined**: A weighted combination of multiple methods

To specify a comparison method:

```rust
app.insert_resource(VisualTestConfig {
    comparison_method: ComparisonMethod::SSIM,
    ..Default::default()
});
```

### Generating Reference Images

To generate or update reference images:

```rust
app.insert_resource(VisualTestConfig {
    update_references: true,
    ..Default::default()
});
```

You can also use the `GENERATE_REFERENCE_IMAGES` environment variable with the reference image generation test:

```bash
GENERATE_REFERENCE_IMAGES=1 cargo test test_generate_reference_images -- --ignored
```

## Standard Test States

The system includes standard test states for common scenarios:

### Card Test States

- `card_normal`: Standard card appearance
- `card_tapped`: Card in tapped state
- `card_highlighted`: Card with highlight effect
- `card_attacking`: Card in attacking state
- `card_blocking`: Card in blocking state
- `card_with_counters`: Card with various counters
- `card_with_attachments`: Card with attached cards/auras
- `card_foil`: Card with foil treatment

### UI Test States

- `menu_main`: Main menu
- `menu_options`: Options menu
- `game_empty_board`: Empty game board
- `game_complex_board`: Complex game board state
- `dialog_confirm`: Confirmation dialog
- `dialog_choose_cards`: Card selection dialog
- `dialog_stack`: Stack interaction dialog
- `dialog_targeting`: Targeting dialog

### Animation Tests

- `card_draw`: Card draw animation (frames 0, 5, 10)
- `card_play`: Card play animation (frames 0, 5, 10)
- `attack`: Attack animation (frames 0, 5, 10)

## Extending the System

### Adding New Test States

To add new test states:

1. Update the appropriate constants in `src/game_engine/visual_testing/mod.rs`
2. Add handling in the matching setup function (e.g., `setup_card_state`)
3. Update documentation

### Adding New Comparison Methods

To add a new comparison method:

1. Add it to the `ComparisonMethod` enum in `config.rs`
2. Implement the comparison function in `comparison.rs`
3. Add the method to the match statement in `compare_images`

### Creating Custom Test Fixtures

To create custom test fixtures for specific features:

1. Create a new test function in your test module
2. Set up the environment and entities
3. Define specific test states
4. Capture screenshots and compare

Example:

```rust
#[test]
fn test_custom_feature() {
    let mut app = App::new();
    setup_visual_test_environment(&mut app);
    
    // Set up your custom feature
    app.add_systems(Startup, setup_custom_feature);
    
    // Define test states
    let custom_states = &["state1", "state2", "state3"];
    
    for state in custom_states {
        // Set up state
        // Capture screenshot
        // Compare with reference
    }
}
```

## Best Practices

### Maintaining Stable Test Environments

For consistent results:

- Use fixed window sizes in tests
- Use deterministic rendering settings
- Document GPU driver requirements
- Use headless testing when possible

### Version Control for Reference Images

Best practices for reference images:

- Store reference images with the corresponding code
- Document visual changes in commit messages
- Use separate branches for intentional visual changes
- Review visual differences as part of code review

### Testing Across Platforms

For cross-platform testing:

- Generate reference images on a standard platform
- Test with different GPU configurations
- Document expected minor differences
- Use appropriate tolerance levels for each platform

## Troubleshooting

### Dealing with Test Failures

When a test fails:

1. Check the difference visualization in the artifacts directory
2. Look for specific areas of difference
3. Compare with baseline images to understand changes
4. Determine if changes are intentional or bugs

### Common Issues

- **Random test failures**: Ensure deterministic rendering
- **High sensitivity**: Adjust similarity thresholds
- **Platform differences**: Use platform-specific reference images
- **Performance issues**: Use selective testing for faster iteration

## Further Reading

- [Bevy Rendering Documentation](https://docs.rs/bevy/latest/bevy/render/index.html)
- [Image Processing and Comparison](https://docs.rs/image/latest/image/)
- [Structural Similarity Index](https://en.wikipedia.org/wiki/Structural_similarity)
- [Perceptual Hashing](https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html) 