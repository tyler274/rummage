use crate::tests::visual_testing::capture::{ScreenshotEvent, capture_screenshot_system};
use bevy::prelude::*;

/// Configuration for visual testing
#[derive(Resource)]
pub struct VisualTestConfig {
    /// Directory to store reference images
    pub reference_dir: String,
    /// Directory to store failure artifacts
    pub artifact_dir: String,
    /// Similarity threshold (0.0 to 1.0)
    pub similarity_threshold: f32,
    /// Whether to update reference images
    pub update_references: bool,
    /// Image comparison method to use
    pub comparison_method: ComparisonMethod,
}

/// Available comparison methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonMethod {
    /// Exact pixel-by-pixel comparison
    PixelPerfect,
    /// Perceptual hash comparison
    PerceptualHash,
    /// Structural similarity index
    SSIM,
    /// Combined approach using multiple methods
    Combined,
}

/// Plugin for visual differential testing
pub struct VisualTestingPlugin;

impl Plugin for VisualTestingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisualTestConfig>()
            .add_event::<ScreenshotEvent>()
            .add_systems(Update, capture_screenshot_system);

        // Don't add render systems for now - we'll implement this properly later
        // This is causing a panic in the main app
    }
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        Self {
            reference_dir: "test_assets/visual_references".to_string(),
            artifact_dir: "test_artifacts/visual_diff".to_string(),
            similarity_threshold: 0.995,
            update_references: false,
            comparison_method: ComparisonMethod::Combined,
        }
    }
}

/// Command line arguments for controlling visual testing
#[derive(Debug, Clone, Resource)]
pub struct VisualTestArgs {
    /// Run visual tests
    pub run_visual_tests: bool,
    /// Update reference images
    pub update_references: bool,
    /// Directory for reference images
    pub reference_dir: Option<String>,
    /// Directory for failure artifacts
    pub artifact_dir: Option<String>,
    /// Similarity threshold override
    pub similarity_threshold: Option<f32>,
    /// Comparison method override
    pub comparison_method: Option<ComparisonMethod>,
}

/// Apply command line arguments to the config
pub fn apply_visual_test_args(mut config: ResMut<VisualTestConfig>, args: Res<VisualTestArgs>) {
    if args.update_references {
        config.update_references = true;
    }

    if let Some(ref dir) = args.reference_dir {
        config.reference_dir = dir.clone();
    }

    if let Some(ref dir) = args.artifact_dir {
        config.artifact_dir = dir.clone();
    }

    if let Some(threshold) = args.similarity_threshold {
        config.similarity_threshold = threshold;
    }

    if let Some(method) = args.comparison_method {
        config.comparison_method = method;
    }
}

/// Setup for a headless visual testing environment
pub fn setup_headless_visual_test_environment(app: &mut App) {
    // Configure for headless testing
    app.add_plugins(MinimalPlugins)
        .add_plugins(bevy::render::RenderPlugin {
            render_creation: bevy::render::settings::RenderCreation::Automatic(
                bevy::render::settings::WgpuSettings {
                    // Use Vulkan backend for better compatibility in headless environments
                    backends: Some(bevy::render::settings::Backends::VULKAN),
                    // Use low power mode for CI environments
                    power_preference: bevy::render::settings::PowerPreference::LowPower,
                    // Disable unnecessary features
                    features: bevy::render::settings::WgpuFeatures::empty(),
                    // Don't wait for pipeline compilation (handled differently in 0.15+)
                    ..bevy::render::settings::WgpuSettings::default()
                },
            ),
            ..default()
        })
        .add_plugins(bevy::window::WindowPlugin {
            primary_window: Some(Window {
                // Create a fixed-size window for deterministic testing
                resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                present_mode: bevy::window::PresentMode::Immediate,
                visible: false, // Hidden window for headless operation
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        })
        .add_plugins(bevy::asset::AssetPlugin::default())
        .add_plugins(bevy::render::texture::ImagePlugin::default())
        .add_plugins(VisualTestingPlugin);
}
