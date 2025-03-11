use bevy::{
    app::AppExit,
    prelude::*,
    window::{
        PresentMode, Window, WindowPlugin, WindowPosition, WindowResizeConstraints,
        WindowResolution,
    },
    winit::WinitSettings,
};

use super::{systems::*, utils::detect_wsl2};

/// Handle exit events from the application
fn handle_exit_events(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.read() {
        info!("WSL2 plugin: Processing exit event");
    }
}

/// Plugin that adds WSL2-specific compatibility features to the application
pub struct WSL2CompatibilityPlugin;

impl Plugin for WSL2CompatibilityPlugin {
    fn build(&self, app: &mut App) {
        if !detect_wsl2() {
            return; // Only apply these settings in WSL2
        }

        info!("Applying WSL2 compatibility plugin for drag resizing");

        // Add only the drag resizing handler for WSL2
        app.add_systems(First, safe_wsl2_resize_handler);
    }
}

/// Get window settings for the application
///
/// This function returns standard window settings regardless of WSL2 detection.
/// The WSL2-specific handling is now done only in the drag resizing system.
pub fn get_wsl2_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Rummage".into(),
            resolution: WindowResolution::new(1024.0, 768.0),
            present_mode: PresentMode::AutoVsync,
            // Allow resizing
            resizable: true,
            // Set reasonable constraints to prevent degenerate window sizes
            resize_constraints: WindowResizeConstraints {
                min_width: 640.0,
                min_height: 480.0,
                ..default()
            },
            // Use reasonable defaults for everything else
            ..default()
        }),
        // Use default plugin settings
        ..default()
    }
}
