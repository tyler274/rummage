use bevy::{
    prelude::*,
    window::{PresentMode, WindowFocused, WindowResized},
};

/// A simple heartbeat system that keeps the app responsive in WSL2
#[allow(dead_code)]
pub fn wsl2_heartbeat() {
    // This empty system runs every frame and helps prevent the app from freezing
    // by ensuring the event loop continues to process events
}

/// Handle window focus events to prevent freezing
#[allow(dead_code)]
pub fn handle_window_focus(
    mut window_focused_events: EventReader<WindowFocused>,
    mut windows: Query<&mut Window>,
) {
    for event in window_focused_events.read() {
        info!("Window focus changed: focused={}", event.focused);
        // Force a redraw when focus changes
        if let Ok(mut window) = windows.get_mut(event.window) {
            // Request a redraw to ensure the window updates
            window.present_mode = if event.focused {
                PresentMode::Fifo // VSync when focused
            } else {
                PresentMode::AutoNoVsync // More responsive when unfocused
            };
        }
    }
}

/// Safely handle window resize events in WSL2
///
/// This prevents the window from getting into a bad state during resize operations
pub fn safe_wsl2_resize_handler(
    mut resize_events: EventReader<WindowResized>,
    mut window_query: Query<(Entity, &mut Window)>,
) {
    // Process all resize events
    for event in resize_events.read() {
        info!(
            "Window resized: width={}, height={}",
            event.width, event.height
        );

        // Ensure the window size is reasonable
        if event.width < 50.0 || event.height < 50.0 {
            if let Ok((_, mut window)) = window_query.get_mut(event.window) {
                // Force a minimum size to prevent degenerate window states
                window
                    .resolution
                    .set(f32::max(event.width, 640.0), f32::max(event.height, 480.0));
            }
        }
    }
}
