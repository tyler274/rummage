// Re-exports from the zones system module
pub mod events;
pub mod resources;
pub mod systems;
pub mod types;

// Public exports
pub use events::*;
pub use resources::*;
pub use systems::*;
pub use types::*;

use bevy::prelude::*;

/// Plugin for zone-related functionality
pub struct ZonesPlugin;

impl Plugin for ZonesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ZoneMarker>()
            .add_event::<events::ZoneChangeEvent>()
            .add_event::<events::EntersBattlefieldEvent>();

        // Add systems for managing zones - moved to FixedUpdate for better performance
        app.add_systems(FixedUpdate, systems::process_zone_changes);
    }
}
