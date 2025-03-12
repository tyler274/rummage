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
pub use priority::*;
pub use stack::*;
pub use state::*;
pub use turns::{
    TurnEndEvent, TurnManager, TurnStartEvent, handle_untap_step, turn_end_system,
    turn_start_system,
};
pub use zones::*;

use crate::menu::{GameMenuState, state::StateTransitionContext};
use crate::player::Player;
use bevy::prelude::*;

/// Plugin that sets up the MTG Commander game engine
pub struct GameEnginePlugin;

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<StackItemResolvedEvent>()
            .add_event::<CheckStateBasedActionsEvent>()
            .add_event::<PlayerEliminatedEvent>()
            .add_event::<CommanderZoneChoiceEvent>()
            .add_event::<CombatDamageEvent>()
            .add_event::<ZoneChangeEvent>()
            .add_event::<TurnStartEvent>()
            .add_event::<TurnEndEvent>()
            .add_event::<DeclareAttackersEvent>()
            .add_event::<DeclareBlockersEvent>()
            .add_event::<AssignCombatDamageEvent>()
            .add_event::<AttackerDeclaredEvent>()
            .add_event::<BlockerDeclaredEvent>()
            .add_event::<CombatBeginEvent>()
            .add_event::<CombatEndEvent>()
            .add_event::<DeclareAttackersStepBeginEvent>()
            .add_event::<DeclareAttackersStepEndEvent>()
            .add_event::<DeclareBlockersStepBeginEvent>()
            .add_event::<DeclareBlockersStepEndEvent>()
            .add_event::<CreatureAttacksEvent>()
            .add_event::<CreatureBlocksEvent>()
            .add_event::<CreatureBlockedEvent>()
            .add_event::<CombatDamageCompleteEvent>();

        // Add game resources initialization during OnEnter(GameMenuState::InGame)
        app.add_systems(OnEnter(GameMenuState::InGame), setup_game_engine);

        // Register each system individually with the condition
        // Core systems
        app.add_systems(
            Update,
            phase_transition_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            priority_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            stack_resolution_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            state_based_actions_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            trigger_state_based_actions_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            process_game_actions.run_if(in_state(GameMenuState::InGame)),
        );

        // Turn systems
        app.add_systems(
            Update,
            turn_start_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            turn_end_system.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            handle_untap_step.run_if(in_state(GameMenuState::InGame)),
        );

        // Commander systems
        app.add_systems(
            Update,
            track_commander_damage.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            handle_commander_zone_change.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            process_commander_zone_choices.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            check_commander_damage_loss.run_if(in_state(GameMenuState::InGame)),
        );
        app.add_systems(
            Update,
            record_commander_damage.run_if(in_state(GameMenuState::InGame)),
        );

        // Combat systems
        app.add_systems(
            Update,
            (
                initialize_combat_phase,
                handle_declare_attackers_event.after(initialize_combat_phase),
                declare_attackers_system.after(handle_declare_attackers_event),
                handle_declare_blockers_event.after(declare_attackers_system),
                declare_blockers_system.after(handle_declare_blockers_event),
                assign_combat_damage_system.after(declare_blockers_system),
                process_combat_damage_system.after(assign_combat_damage_system),
                end_combat_system.after(process_combat_damage_system),
            )
                .run_if(in_state(GameMenuState::InGame)),
        );

        // Allow politics systems to register additional systems
        politics::register_politics_systems(app);
    }
}

/// Condition function to check if the game state is InGame
fn game_state_condition(state: Res<State<GameMenuState>>) -> bool {
    *state.get() == GameMenuState::InGame
}

/// Initializes the core game engine resources
fn setup_game_engine(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    context: Res<StateTransitionContext>,
    turn_manager: Option<Res<TurnManager>>,
) {
    // Skip initialization if we're coming from the pause menu and already have a turn manager
    if context.from_pause_menu && turn_manager.is_some() {
        info!("Resuming from pause menu, skipping game engine initialization");
        return;
    }

    info!("Initializing game engine resources...");

    // Get all player entities
    let players: Vec<Entity> = player_query.iter().collect();

    // Initialize the phase system starting at Beginning::Untap
    commands.insert_resource(Phase::Beginning(BeginningStep::Untap));

    // Initialize the priority system (no player has priority at start)
    commands.insert_resource(PrioritySystem::default());

    // Initialize an empty stack
    commands.insert_resource(GameStack::default());

    // Initialize zone manager
    commands.insert_resource(ZoneManager::default());

    // Initialize combat state
    commands.insert_resource(CombatState::default());

    // Initialize turn manager with player list
    let mut turn_manager = TurnManager::default();
    turn_manager.initialize(players);
    commands.insert_resource(turn_manager);

    // Initialize the commander zone and manager
    commands.insert_resource(CommandZone::default());
    commands.insert_resource(CommandZoneManager::default());

    // Initialize game state
    commands.insert_resource(GameState::default());
}
