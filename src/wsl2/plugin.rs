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

        info!("Applying WSL2 compatibility plugin");

        // Add WSL2-specific systems in the correct order
        app
            // Handle window resizing safely first
            .add_systems(First, safe_wsl2_resize_handler)
            // Ensure the app doesn't hang when the window loses focus
            .add_systems(Update, handle_exit_events)
            // Add a heartbeat system to prevent freezing
            .add_systems(Update, wsl2_heartbeat)
            // Handle window focus events
            .add_systems(Update, handle_window_focus)
            // Prevent frame freezing by using appropriate update modes
            .insert_resource(WinitSettings {
                focused_mode: bevy::winit::UpdateMode::Continuous,
                unfocused_mode: bevy::winit::UpdateMode::Continuous,
            });
    }
}

/// Get optimized window settings for WSL2
pub fn get_wsl2_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Rummage".into(),
            // Use a standard resolution with scale factor override to prevent WSL2 scaling issues
            resolution: WindowResolution::new(800.0, 600.0).with_scale_factor_override(1.0),
            // For WSL2, force Fifo (VSync) to prevent timing issues
            present_mode: PresentMode::Fifo,
            // CRITICAL: Disable decorations completely in WSL2 to avoid window border issues
            decorations: false,
            // Avoid alpha compositing which can cause issues in WSL2
            transparent: false,
            // Allow resizing but with constraints
            resizable: true,
            // Set reasonable constraints to prevent degenerate window sizes
            resize_constraints: WindowResizeConstraints {
                min_width: 640.0,
                min_height: 480.0,
                ..default()
            },
            // Force a position to avoid window placement issues
            position: WindowPosition::At(IVec2::new(50, 50)),
            // Use reasonable defaults for everything else
            ..default()
        }),
        // Use default plugin settings
        ..default()
    }
}
