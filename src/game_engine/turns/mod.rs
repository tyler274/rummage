mod builder;
mod controller;
mod events;
mod manager;
mod systems;

pub use builder::*;
pub use controller::*;
pub use events::*;
pub use manager::*;
pub use systems::*;

// Register all turn-related systems with the app
pub fn register_turn_systems(app: &mut bevy::prelude::App) {
    // Make sure the Phase resource is registered
    if !app
        .world()
        .contains_resource::<crate::game_engine::phase::types::Phase>()
    {
        app.insert_resource(crate::game_engine::phase::types::Phase::default());
    }

    app.add_event::<TurnStartEvent>()
        .add_event::<TurnEndEvent>()
        .init_resource::<TurnManager>();

    // We don't register these systems here as they're registered directly in GameEnginePlugin
    // Instead, we just ensure the needed resources and events are available
}
