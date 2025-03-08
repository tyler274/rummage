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
    mut resize_reader: EventReader<WindowResized>,
    mut projection_query: Query<&mut OrthographicProjection>,
    mut windows: Query<&mut Window>,
) {
    let Ok(mut window) = windows.get_single_mut() else { return };
    
    for event in resize_reader.read() {
        // Force a window redraw to clear any artifacts
        window.present_mode = bevy::window::PresentMode::AutoVsync;
        
        if let Ok(mut projection) = projection_query.get_single_mut() {
            // Keep a fixed vertical size and scale the horizontal size with aspect ratio
            let vertical_size = 600.0; // Match the spawn_hand height
            let aspect_ratio = event.width / event.height;
            let horizontal_size = vertical_size * aspect_ratio;
            
            projection.area = Rect::new(
                -horizontal_size / 2.0,
                -vertical_size / 2.0,
                horizontal_size / 2.0,
                vertical_size / 2.0,
            );
            projection.scale = 1.0;
        }
    }
} 