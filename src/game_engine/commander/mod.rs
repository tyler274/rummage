pub mod components;
pub mod events;
pub mod resources;
pub mod rules;
pub mod systems;

// Re-export the core components and types for easier access
pub use components::{Commander, EliminationReason};
pub use events::{CombatDamageEvent, CommanderZoneChoiceEvent, PlayerEliminatedEvent};
pub use resources::{CommandZone, CommandZoneManager};
pub use systems::{
    check_commander_damage_loss, handle_commander_zone_change, process_commander_zone_choices,
    record_commander_damage, track_commander_damage,
};

use bevy::prelude::*;

/// Register all Commander-related systems and events
pub fn register_commander_systems(app: &mut App) {
    app.add_event::<CommanderZoneChoiceEvent>()
        .add_event::<PlayerEliminatedEvent>()
        .add_event::<CombatDamageEvent>()
        .add_systems(
            Update,
            (
                track_commander_damage,
                handle_commander_zone_change,
                process_commander_zone_choices,
                check_commander_damage_loss,
                record_commander_damage,
            )
                .run_if(crate::game_engine::game_state_condition),
        );
}
