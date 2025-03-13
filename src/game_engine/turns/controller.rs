use bevy::prelude::*;

/// Component to track who controls a permanent on the battlefield
/// This may be different from the owner if control has changed
#[derive(Component, Debug, Clone)]
pub struct PermanentController {
    /// The player who controls this permanent
    pub player: Entity,
}

impl PermanentController {
    /// Creates a new controller component
    /// TODO: Implement when adding permanents to the battlefield with controller tracking
    #[allow(dead_code)]
    pub fn new(player: Entity) -> Self {
        Self { player }
    }
}
