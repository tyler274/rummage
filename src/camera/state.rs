use bevy::prelude::*;

/// Resource to track camera panning state
///
/// This struct stores the current state of camera panning,
/// keeping track of when the middle mouse button is pressed
/// and the last known mouse position to calculate movement deltas.
/// The fields are public to allow proper access in testing and
/// other modules that might need to interact with panning state.
#[derive(Resource, Default)]
pub struct CameraPanState {
    /// Whether the camera is currently being panned
    pub is_panning: bool,
    /// Last mouse position during pan
    pub last_mouse_pos: Option<Vec2>,
}
