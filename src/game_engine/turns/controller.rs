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
    pub fn new(player: Entity) -> Self {
        Self { player }
    }
} 