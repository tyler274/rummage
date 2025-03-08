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
    projection.scale = 2.0; // Scale to make cards a reasonable size in the viewport
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
            // Scale based on card height (936px) relative to window height
            // We want the card to take up roughly 1/3 of the screen height
            let target_card_height = 936.0;
            let aspect = resize_event.width / resize_event.height;
            // Scale by aspect ratio to maintain card proportions across different window sizes
            projection.scale = (target_card_height / resize_event.height) * 2.0 * aspect;

            // Update window surface
            if let Ok(mut window) = windows.get_single_mut() {
                window
                    .resolution
                    .set(resize_event.width, resize_event.height);
            }
        }
    }
}
