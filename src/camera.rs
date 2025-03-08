use bevy::prelude::*;
use bevy::window::WindowResized;

/// Sets up the 2D orthographic camera with a fixed scale and view area.
///
/// # Coordinate System Debugging Notes
/// During development, we encountered several issues with text positioning relative to cards:
/// 1. Initial approach: Using fixed pixel offsets
///    - Text positions were inconsistent across different screen positions
///    - Text appeared correctly aligned only in screen center
///
/// 2. Second approach: Screen-space to world-space conversion
///    - Attempted to convert screen coordinates to world space for text positioning
///    - Still had perspective issues due to camera projection
///
/// 3. Final solution: Parent-child relationships
///    - Made text entities children of card entities
///    - Used relative transforms for text positioning
///    - Positions now maintain consistency regardless of screen position
///
/// The key insight was that Bevy's transform system handles parent-child relationships
/// automatically, eliminating the need for manual coordinate conversion.
pub fn setup_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 1.0;
    projection.near = -1000.0;
    projection.far = 1000.0;

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        projection,
    ));
}

/// Handles window resize events by maintaining a fixed vertical size and adjusting
/// the horizontal size based on aspect ratio.
pub fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut windows: Query<&mut Window>,
) {
    for resize_event in resize_events.read() {
        if let Ok(mut projection) = projection_query.get_single_mut() {
            // Maintain fixed height and adjust width based on aspect ratio
            let aspect = resize_event.width / resize_event.height;
            projection.scale = 1.0 / resize_event.height * 1080.0 * aspect; // Scale relative to 1080p height

            // Update window surface
            if let Ok(mut window) = windows.get_single_mut() {
                window
                    .resolution
                    .set(resize_event.width, resize_event.height);
            }
        }
    }
}
