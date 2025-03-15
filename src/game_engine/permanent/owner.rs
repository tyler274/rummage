use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Component to track who owns a permanent
/// The owner is the player who originally had the card in their deck
/// This doesn't change even if control changes
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PermanentOwner {
    /// The player who owns this permanent
    pub player: Entity,
}

impl PermanentOwner {
    /// Creates a new owner component
    pub fn new(player: Entity) -> Self {
        Self { player }
    }
}

/// Component to track who controls a permanent on the battlefield
/// This may be different from the owner if control has changed
#[derive(Component, Debug, Clone, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
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
