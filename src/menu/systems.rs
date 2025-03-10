use crate::camera::components::{AppLayer, MenuCamera};
use bevy::prelude::*;
use bevy::render::view::{ColorGrading, RenderLayers};

/// Sets up the menu camera with WSL2-friendly configuration
pub fn setup_menu_camera(mut commands: Commands) {
    info!("Setting up menu camera");

    // Use a separate camera for the menu UI only
    let camera_entity = commands
        .spawn((
            Camera2d::default(),
            Camera {
                order: 10, // Higher order to ensure it renders on top of game camera
                clear_color: ClearColorConfig::None, // Don't clear the screen, just overlay
                ..default()
            },
            // Set up standard camera components
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Visible, // Explicitly set to visible
            InheritedVisibility::default(),
            ViewVisibility::default(),
            // Add menu-specific components
            MenuCamera,
            ColorGrading::default(),
            RenderLayers::layer(1), // Use a specific layer for menu rendering
        ))
        .id();

    info!("Spawned menu camera: {:?}", camera_entity);
}
