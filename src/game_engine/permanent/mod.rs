//! Module for permanent entities on the battlefield

mod components;
mod owner;
mod systems;

use bevy::prelude::*;

pub use components::*;
pub use owner::*;
pub use systems::*;

/// Plugin for permanent-related functionality
pub struct PermanentPlugin;

impl Plugin for PermanentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Permanent>()
            .register_type::<PermanentController>()
            .register_type::<PermanentOwner>()
            .register_type::<PermanentState>()
            .add_systems(FixedUpdate, update_permanent_state);
    }
}
