use bevy::prelude::*;
use bevy_persistent::prelude::*;
use std::path::PathBuf;

use crate::game_engine::save::resources::*;

/// System to set up the save system on startup
pub fn setup_save_system(mut commands: Commands) {
    // Create save directory if it doesn't exist
    let config = SaveConfig::default();

    // Only try to create directory on native platforms
    #[cfg(not(target_arch = "wasm32"))]
    {
        match std::fs::create_dir_all(&config.save_directory) {
            Ok(_) => info!(
                "Ensured save directory exists at: {:?}",
                config.save_directory
            ),
            Err(e) => {
                error!("Failed to create save directory: {}", e);
                // Check if directory exists despite the error (might be a permission issue)
                if !config.save_directory.exists() {
                    warn!("Save directory does not exist, saves may fail");
                }
            }
        }
    }

    // Determine the appropriate base path for persistence based on platform
    let metadata_path = get_storage_path(&config, "metadata.toml");

    // Initialize persistent save metadata
    let save_metadata = match Persistent::builder()
        .name("save_metadata")
        .format(StorageFormat::Toml)
        .path(metadata_path)
        .default(SaveMetadata::default())
        .build()
    {
        Ok(metadata) => metadata,
        Err(e) => {
            error!("Failed to create persistent save metadata: {}", e);
            // Create a new in-memory metadata resource instead
            Persistent::builder()
                .name("save_metadata")
                .format(StorageFormat::Toml)
                .path(PathBuf::from("metadata.toml")) // Fallback path
                .default(SaveMetadata::default())
                .build()
                .unwrap_or_else(|_| {
                    // If even that fails, create a completely in-memory resource
                    let metadata = SaveMetadata::default();
                    Persistent::builder()
                        .name("save_metadata")
                        .format(StorageFormat::Toml)
                        .path(PathBuf::from("metadata.toml"))
                        .default(metadata)
                        .build()
                        .expect("Failed to create even basic metadata")
                })
        }
    };

    commands.insert_resource(config.clone());
    commands.insert_resource(AutoSaveTracker::default());
    commands.insert_resource(ReplayState::default());
    commands.insert_resource(save_metadata);
}

/// Helper function to get the appropriate storage path based on platform
pub fn get_storage_path(config: &SaveConfig, filename: &str) -> PathBuf {
    #[cfg(target_arch = "wasm32")]
    {
        // For WebAssembly, use local storage with a prefix
        Path::new("/local/saves").join(filename)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // For native platforms, use the filesystem from config
        config.save_directory.join(filename)
    }
}
