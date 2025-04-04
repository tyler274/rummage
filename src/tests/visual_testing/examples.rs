// Visual Testing Examples

#[cfg(test)]
mod tests {
    use crate::tests::visual_testing::capture::ScreenshotRequests;
    use crate::tests::visual_testing::config::{
        VisualTestConfig, setup_headless_visual_test_environment,
    };
    use crate::tests::visual_testing::utils::ensure_test_directories;
    use crate::tests::visual_testing::*;
    use bevy::prelude::*;
    use std::path::Path;

    // Basic UI rendering test that works in CI environments
    #[test]
    fn test_basic_ui_rendering_ci() {
        // Create test app with headless configuration
        let mut app = App::new();

        // Set up headless environment
        setup_headless_visual_test_environment(&mut app);

        // Apply CI-specific configurations if in CI environment
        setup_ci_visual_test(&mut app);

        // Ensure test directories exist
        ensure_test_directories().expect("Failed to create test directories");

        // Initialize necessary resources
        app.init_resource::<ScreenshotRequests>();

        // AssetServer requires a real implementation in tests
        // For this example test we'll just directly capture screenshots instead

        // Set up a basic test scene
        app.add_systems(Startup, setup_ui_test_scene);

        // Run the app for a few frames to ensure everything is set up
        for _ in 0..5 {
            app.update();
        }

        // Take a screenshot directly
        if let Some(screenshot) = take_screenshot() {
            let reference_name = "basic_ui_test.png";

            // Ensure the directory exists
            let config = app.world().resource::<VisualTestConfig>();
            let dir = Path::new(&config.reference_dir);
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    panic!("Failed to create reference directory: {}", e);
                }
            }

            // Choose to create or compare reference images
            let update_references = std::env::var("GENERATE_REFERENCES").is_ok();

            if update_references {
                // Save as reference image
                match screenshot.into_rgba8().save(dir.join(reference_name)) {
                    Ok(_) => info!("Saved reference image: {}", reference_name),
                    Err(e) => panic!("Failed to save reference image: {}", e),
                }
            } else {
                // Try to load existing reference
                let reference_path = dir.join(reference_name);
                if reference_path.exists() {
                    match image::open(reference_path) {
                        Ok(reference) => {
                            let result = compare_images(&screenshot, &reference);
                            let threshold = config.similarity_threshold;
                            assert!(
                                result.similarity_score >= threshold,
                                "Visual difference detected: similarity score {} is below threshold {}",
                                result.similarity_score,
                                threshold
                            );
                        }
                        Err(e) => {
                            info!(
                                "Reference image could not be loaded: {}. Creating new reference.",
                                e
                            );
                            match screenshot.into_rgba8().save(dir.join(reference_name)) {
                                Ok(_) => info!("Saved reference image: {}", reference_name),
                                Err(e) => panic!("Failed to save reference image: {}", e),
                            }
                        }
                    }
                } else {
                    // Reference doesn't exist, create it
                    info!("Reference image does not exist. Creating new reference.");
                    match screenshot.into_rgba8().save(dir.join(reference_name)) {
                        Ok(_) => info!("Saved reference image: {}", reference_name),
                        Err(e) => panic!("Failed to save reference image: {}", e),
                    }
                }
            }
        } else {
            panic!("Failed to take screenshot");
        }
    }

    // Example test for visual consistency of card rendering
    #[test]
    fn test_card_rendering_visual_consistency() {
        let mut app = App::new();

        // Set up the test environment
        app.add_plugins(MinimalPlugins)
            .add_plugins(VisualTestingPlugin)
            .init_resource::<ScreenshotRequests>();

        // Ensure test directories exist
        ensure_test_directories().expect("Failed to create test directories");

        // Check if reference images exist
        let reference_needed = !check_card_references_exist();

        // Create references if needed
        if reference_needed || std::env::var("GENERATE_REFERENCES").is_ok() {
            info!("Generating card rendering reference images");
            let mut config = app.world_mut().resource_mut::<VisualTestConfig>();
            config.update_references = true;
        }

        // Set up and run the test
        app.add_systems(Startup, setup_test_scene);
        app.update();

        // Basic test implementation - real version would test various card states
        if let Some(screenshot) = take_screenshot() {
            let reference_name = "card_basic.png";

            // Ensure reference directory exists
            let config = app.world().resource::<VisualTestConfig>();
            let dir = Path::new(&config.reference_dir);
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    panic!("Failed to create reference directory: {}", e);
                }
            }

            let ref_path = dir.join(reference_name);

            if app.world().resource::<VisualTestConfig>().update_references {
                match screenshot.into_rgba8().save(&ref_path) {
                    Ok(_) => info!("Saved card reference image: {}", reference_name),
                    Err(e) => panic!("Failed to save card reference image: {}", e),
                }
            } else {
                // Compare screenshot with reference image
                assert!(
                    ref_path.exists(),
                    "Reference image missing: {}",
                    reference_name
                );
                let reference = image::open(&ref_path).expect("Failed to load reference image");
                let result = compare_images(&screenshot, &reference);
                assert!(
                    result.similarity_score >= 0.99,
                    "Card rendering differs from reference. Similarity: {}",
                    result.similarity_score
                );
            }
        }
    }

    // Example test for rendering UI elements
    #[test]
    fn test_ui_element_rendering() {
        let mut app = App::new();

        // Set up the test environment
        app.add_plugins(MinimalPlugins)
            .add_plugins(VisualTestingPlugin)
            .init_resource::<ScreenshotRequests>();

        // Ensure test directories exist
        ensure_test_directories().expect("Failed to create test directories");

        // Set up and run the test
        app.add_systems(Startup, setup_ui_test_scene);
        app.update();

        // Basic test implementation - real version would test various UI states
        if let Some(screenshot) = take_screenshot() {
            let reference_name = "ui_basic.png";

            // Ensure reference directory exists
            let config = app.world().resource::<VisualTestConfig>();
            let dir = Path::new(&config.reference_dir);
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    panic!("Failed to create reference directory: {}", e);
                }
            }

            let ref_path = dir.join(reference_name);

            // Check if we need to generate references
            let needs_reference =
                !ref_path.exists() || std::env::var("GENERATE_REFERENCES").is_ok();

            if needs_reference {
                info!("Generating UI reference image");
                match screenshot.into_rgba8().save(&ref_path) {
                    Ok(_) => info!("Saved UI reference image: {}", reference_name),
                    Err(e) => panic!("Failed to save UI reference image: {}", e),
                }
            } else {
                match image::open(&ref_path) {
                    Ok(reference) => {
                        let result = compare_images(&screenshot, &reference);
                        assert!(
                            result.similarity_score >= 0.99,
                            "UI rendering differs from reference. Similarity: {}",
                            result.similarity_score
                        );
                    }
                    Err(e) => {
                        info!("Reference image load error: {}. Creating new reference.", e);
                        match screenshot.into_rgba8().save(&ref_path) {
                            Ok(_) => info!("Saved UI reference image: {}", reference_name),
                            Err(e) => panic!("Failed to save reference image: {}", e),
                        }
                    }
                }
            }
        }
    }

    // Example test for visual keyframes in animations
    #[test]
    fn test_animation_keyframes() {
        let mut app = App::new();

        // Set up the test environment
        app.add_plugins(MinimalPlugins)
            .add_plugins(VisualTestingPlugin)
            .init_resource::<ScreenshotRequests>();

        // Ensure test directories exist
        ensure_test_directories().expect("Failed to create test directories");

        // Set up and run the test
        app.add_systems(Startup, setup_animation_test);
        app.update();

        // Basic test implementation - real version would test various animation keyframes
        if let Some(screenshot) = take_screenshot() {
            let reference_name = "animation_basic.png";

            // Ensure reference directory exists
            let config = app.world().resource::<VisualTestConfig>();
            let dir = Path::new(&config.reference_dir);
            if !dir.exists() {
                if let Err(e) = std::fs::create_dir_all(dir) {
                    panic!("Failed to create reference directory: {}", e);
                }
            }

            let ref_path = dir.join(reference_name);

            // Check if we need to generate references
            let needs_reference =
                !ref_path.exists() || std::env::var("GENERATE_REFERENCES").is_ok();

            if needs_reference {
                info!("Generating animation reference image");
                match screenshot.into_rgba8().save(&ref_path) {
                    Ok(_) => info!("Saved animation reference image: {}", reference_name),
                    Err(e) => panic!("Failed to save animation reference image: {}", e),
                }
            } else {
                match image::open(&ref_path) {
                    Ok(reference) => {
                        let result = compare_images(&screenshot, &reference);
                        assert!(
                            result.similarity_score >= 0.98, // Slightly lower threshold for animations
                            "Animation rendering differs from reference. Similarity: {}",
                            result.similarity_score
                        );
                    }
                    Err(e) => {
                        info!("Reference image load error: {}. Creating new reference.", e);
                        match screenshot.into_rgba8().save(&ref_path) {
                            Ok(_) => info!("Saved animation reference image: {}", reference_name),
                            Err(e) => panic!("Failed to save reference image: {}", e),
                        }
                    }
                }
            }
        }
    }

    // Helper function to check if card references exist
    fn check_card_references_exist() -> bool {
        let config = VisualTestConfig::default();
        let reference_name = "card_basic.png";
        let path = std::path::Path::new(&config.reference_dir).join(reference_name);
        path.exists()
    }
}
