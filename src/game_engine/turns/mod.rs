mod builder;
mod controller;
mod events;
mod manager;
mod systems;

// Re-export types for external use
pub use controller::PermanentController;
pub use events::{TurnEndEvent, TurnEventTracker, TurnStartEvent};
pub use manager::TurnManager;
pub use systems::{handle_turn_end, handle_turn_start, handle_untap_step};

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
