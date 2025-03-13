use crate::game_engine::visual_testing::config::VisualTestConfig;
use bevy::prelude::*;
use image::DynamicImage;
use std::fs;
use std::path::Path;

/// Saves a reference image
pub fn save_reference_image(image: DynamicImage, name: &str) -> Result<(), String> {
    match image.as_rgba8() {
        Some(rgba) => {
            let config = VisualTestConfig::default();
            let dir = Path::new(&config.reference_dir);

            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| format!("Failed to create reference directory: {}", e))?;
            }

            let path = dir.join(name);
            rgba.save(path)
                .map_err(|e| format!("Failed to save reference image: {}", e))?;

            info!("Saved reference image: {}", name);
            Ok(())
        }
        None => Err("Failed to convert image to RGBA format".to_string()),
    }
}

/// Loads a reference image
pub fn load_reference_image(name: &str) -> Result<DynamicImage, String> {
    let config = VisualTestConfig::default();
    let path = Path::new(&config.reference_dir).join(name);

    image::open(path).map_err(|e| format!("Failed to load reference image: {}", e))
}

/// Ensures test directories exist
pub fn ensure_test_directories() -> Result<(), String> {
    let config = VisualTestConfig::default();

    // Create reference directory if it doesn't exist
    let ref_dir = Path::new(&config.reference_dir);
    if !ref_dir.exists() {
        fs::create_dir_all(ref_dir)
            .map_err(|e| format!("Failed to create reference directory: {}", e))?;
        info!("Created reference directory: {}", config.reference_dir);
    }

    // Create artifact directory if it doesn't exist
    let artifact_dir = Path::new(&config.artifact_dir);
    if !artifact_dir.exists() {
        fs::create_dir_all(artifact_dir)
            .map_err(|e| format!("Failed to create artifact directory: {}", e))?;
        info!("Created artifact directory: {}", config.artifact_dir);
    }

    Ok(())
}

/// Clean up test artifacts
pub fn clean_test_artifacts() -> Result<(), String> {
    let config = VisualTestConfig::default();
    let artifact_dir = Path::new(&config.artifact_dir);

    if artifact_dir.exists() {
        fs::remove_dir_all(artifact_dir)
            .map_err(|e| format!("Failed to clean artifact directory: {}", e))?;

        // Recreate the empty directory
        fs::create_dir_all(artifact_dir)
            .map_err(|e| format!("Failed to recreate artifact directory: {}", e))?;

        info!("Cleaned artifact directory: {}", config.artifact_dir);
    }

    Ok(())
}

/// Clear all reference images
pub fn clear_reference_images() -> Result<(), String> {
    let config = VisualTestConfig::default();
    let ref_dir = Path::new(&config.reference_dir);

    if ref_dir.exists() {
        fs::remove_dir_all(ref_dir)
            .map_err(|e| format!("Failed to remove reference directory: {}", e))?;

        // Recreate the empty directory
        fs::create_dir_all(ref_dir)
            .map_err(|e| format!("Failed to recreate reference directory: {}", e))?;

        info!("Cleared all reference images");
    }

    Ok(())
}

/// Get list of all reference images
pub fn list_reference_images() -> Result<Vec<String>, String> {
    let config = VisualTestConfig::default();
    let ref_dir = Path::new(&config.reference_dir);
    let mut images = Vec::new();

    if !ref_dir.exists() {
        return Ok(images);
    }

    let entries =
        fs::read_dir(ref_dir).map_err(|e| format!("Failed to read reference directory: {}", e))?;

    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".png") || name.ends_with(".jpg") {
                    images.push(name.to_string());
                }
            }
        }
    }

    Ok(images)
}

/// Get count of reference images
pub fn count_reference_images() -> Result<usize, String> {
    list_reference_images().map(|images| images.len())
}

/// Create a timestamp string for unique filenames
pub fn timestamp_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", now.as_secs())
}
