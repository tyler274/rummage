use bevy::prelude::*;

/// Configuration for camera movement and zoom behavior.
///
/// This resource controls how the camera responds to user input,
/// including movement speed and zoom limits.
#[derive(Resource)]
pub struct CameraConfig {
    /// Movement speed in units per second (for keyboard movement)
    pub move_speed: f32,
    /// Zoom speed multiplier
    pub zoom_speed: f32,
    /// Minimum zoom level (most zoomed out)
    pub min_zoom: f32,
    /// Maximum zoom level (most zoomed in)
    pub max_zoom: f32,
    /// Mouse pan sensitivity multiplier
    pub pan_sensitivity: f32,
    /// Zoom interpolation speed (higher = smoother but slower)
    pub zoom_interpolation_speed: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            move_speed: 500.0,
            zoom_speed: 1.0, // Adjusted for smoother zooming
            // In OrthographicProjection, min_zoom limits how far you can zoom out (higher value)
            // and max_zoom limits how far you can zoom in (lower value)
            min_zoom: 0.1,                 // Most zoomed in
            max_zoom: 5.0,                 // Most zoomed out
            pan_sensitivity: 1.0,          // Base sensitivity, adjust if needed
            zoom_interpolation_speed: 5.0, // Controls how smoothly zoom changes are applied
        }
    }
}
