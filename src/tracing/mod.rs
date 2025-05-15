use bevy::prelude::*;
use std::panic;

/// Plugin that configures enhanced logging and diagnostics for the application
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing Diagnostics Plugin");

        // Register panic hook to capture system panics with better diagnostics
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // Format and log the panic information
            let panic_message = format!("{}", panic_info);
            error!("🚨 PANIC DETECTED: {}", panic_message);

            // Call the previous hook
            previous_hook(panic_info);
        }));

        // Add Bevy's built-in diagnostics
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);

        // Add startup diagnostic system
        app.add_systems(Startup, log_startup_info)
            .add_systems(Last, log_frame_completion);

        info!("Diagnostics Plugin initialized");
    }
}

// Log useful information during startup
fn log_startup_info(schedules: Option<Res<Schedules>>) {
    info!("=== APPLICATION STARTUP ===");

    // Log registered schedules
    if let Some(schedules) = schedules {
        let schedule_names: Vec<_> = schedules
            .iter()
            .map(|(id, _)| format!("{:?}", id))
            .collect();
        info!("Registered schedules: {}", schedule_names.join(", "));
    }

    // Log system information
    info!("Running on: {}", std::env::consts::OS);

    debug!("Startup diagnostics complete");
}

// Log frame completion
fn log_frame_completion() {
    trace!("Frame completed");
}
