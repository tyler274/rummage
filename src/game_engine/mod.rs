// This module contains the core components for the MTG Commander game engine
// It follows the implementation plan outlined in docs/game_loop.md

mod actions;
pub mod combat;
pub mod commander;
mod phase;
pub mod politics;
mod priority;
mod stack;
pub mod state;
pub mod turns;
pub mod zones;

pub use actions::*;
pub use combat::*;
pub use commander::*;
pub use phase::*;
pub use politics::*;
pub use priority::*;
pub use stack::*;
pub use state::*;
pub use turns::*;
pub use zones::*;

use bevy::prelude::*;

/// Plugin that sets up the MTG Commander game engine
pub struct GameEnginePlugin;

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<StackItemResolvedEvent>()
            .add_event::<CheckStateBasedActionsEvent>()
            .add_event::<PlayerEliminatedEvent>();

        // Add core systems
        app.add_systems(Startup, setup_game_engine).add_systems(
            Update,
            (
                phase_transition_system,
                priority_system,
                stack_resolution_system,
                state_based_actions_system,
                trigger_state_based_actions_system,
                process_game_actions,
            ),
        );

        // Register turn-related systems and events
        turns::register_turn_systems(app);

        // Register zone-related systems and events
        zones::register_zone_systems(app);

        // Register commander-related systems and events
        commander::register_commander_systems(app);

        // Register combat-related systems and events
        combat::register_combat_systems(app);

        // Register politics-related systems and events
        politics::register_politics_systems(app);
    }
}

/// Initializes the core game engine resources
fn setup_game_engine(mut commands: Commands) {
    // Initialize the game state with default values
    commands.insert_resource(GameState::default());

    // Initialize the phase system starting at Beginning::Untap
    commands.insert_resource(Phase::Beginning(BeginningStep::Untap));

    // Initialize the priority system (no player has priority at start)
    commands.insert_resource(PrioritySystem::default());

    // Initialize an empty stack
    commands.insert_resource(GameStack::default());
}
