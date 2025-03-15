use bevy::prelude::*;
use std::env;

use crate::tests::visual_testing::config::{VisualTestArgs, VisualTestConfig};
use crate::tests::visual_testing::utils::ensure_test_directories;

/// Detect if we're running in a CI environment
pub fn is_ci_environment() -> bool {
    env::var("CI").is_ok()
}

/// Configure environment-specific settings for the CI environment
pub fn configure_for_ci(config: Option<ResMut<VisualTestConfig>>) {
    if is_ci_environment() {
        // Ensure test directories exist
        let _ = ensure_test_directories();

        // Configure CI-specific settings
        if let Some(mut config) = config {
            // Use more lenient thresholds in CI environments due to potential rendering differences
            config.similarity_threshold = 0.99;

            // Store artifacts in CI-specific directory for easier artifact collection
            config.artifact_dir = "test_artifacts/visual_diff".to_string();

            // Apply environment variables for CI configurations
            if env::var("GENERATE_REFERENCES").is_ok() {
                config.update_references = true;
                info!("CI: Running in reference generation mode");
            }
        }
    }
}

/// Apply CI-specific command line arguments
pub fn apply_ci_args(mut args: ResMut<VisualTestArgs>) {
    if is_ci_environment() {
        // Override with CI-specific settings from environment variables
        if env::var("GENERATE_REFERENCES").is_ok() {
            args.update_references = true;
        }

        if let Ok(threshold) = env::var("SIMILARITY_THRESHOLD") {
            if let Ok(value) = threshold.parse::<f32>() {
                args.similarity_threshold = Some(value);
            }
        }
    }
}

/// Setup for running a visual test in a CI environment
pub fn setup_ci_visual_test(app: &mut App) {
    if is_ci_environment() {
        info!("Setting up visual test for CI environment");
        // Add a startup system that configures CI-specific settings
        app.add_systems(Startup, configure_for_ci);
    }
}
