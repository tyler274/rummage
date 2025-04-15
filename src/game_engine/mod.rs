// This module contains the core components for the MTG Commander game engine
// It follows the implementation plan outlined in docs/game_loop.md

pub mod actions;
pub mod combat;
pub mod commander;
pub mod permanent;
pub mod phase;
pub mod politics;
pub mod priority;
pub mod save;
pub mod stack;
pub mod state;
pub mod tests;
pub mod turns;
pub mod zones;

// Import required types
use crate::menu::{GameMenuState, StateTransitionContext};
use crate::player::Player;

// Re-export important types for easier access
pub use actions::GameAction;
pub use combat::{CombatState, DeclareAttackersEvent, DeclareBlockersEvent};
pub use commander::{CombatDamageEvent, CommanderZoneChoiceEvent, PlayerEliminatedEvent};
pub use phase::Phase;
pub use priority::{
    EffectCounteredEvent, NextPhaseEvent, PassPriorityEvent, PrioritySystem, ResolveStackItemEvent,
};
pub use save::SaveLoadPlugin;
pub use stack::{GameStack, StackItemResolvedEvent};
pub use state::{CheckStateBasedActionsEvent, GameState};
pub use turns::{
    TurnEndEvent, TurnManager, TurnStartEvent, handle_turn_end, handle_turn_start,
    register_turn_systems,
};
pub use zones::{EntersBattlefieldEvent, ZoneChangeEvent, ZoneManager};

// Import the missing types
use crate::game_engine::actions::process_game_actions;
use crate::game_engine::combat::{
    AssignCombatDamageEvent, AttackerDeclaredEvent, BlockerDeclaredEvent, CombatBeginEvent,
    CombatDamageCompleteEvent, CombatEndEvent, CreatureAttacksEvent, CreatureBlockedEvent,
    CreatureBlocksEvent, DeclareAttackersStepBeginEvent, DeclareAttackersStepEndEvent,
    DeclareBlockersStepBeginEvent, DeclareBlockersStepEndEvent, assign_combat_damage_system,
    declare_attackers_system, declare_blockers_system, end_combat_system,
    handle_declare_attackers_event, handle_declare_blockers_event, initialize_combat_phase,
    process_combat_damage_system,
};
use crate::game_engine::commander::{CommandZone, CommandZoneManager};
use crate::game_engine::phase::{BeginningStep, phase_transition_system};
use crate::game_engine::politics::{
    ApplyCombatRestrictionEvent, GoadEvent, RemoveCombatRestrictionEvent,
};
use crate::game_engine::priority::{priority_passing_system, priority_system};

// Game Engine Plugin
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

/// Custom schedule for fixed timestep game logic updates
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct FixedGameLogicSet;

/// Condition function to check if the game state is InGame
pub fn game_state_condition(state: Res<State<GameMenuState>>) -> bool {
    *state.get() == GameMenuState::InGame
}

pub struct GameEnginePlugin;

impl Plugin for GameEnginePlugin {
    fn build(&self, app: &mut App) {
        // Core game state

        // Save/Load system
        app.add_plugins(SaveLoadPlugin);

        // First, add the essential resources
        if !app.world().contains_resource::<Phase>() {
            app.insert_resource(Phase::default());
        }

        // Initialize essential resources that should always be available
        app.init_resource::<CombatState>()
            .init_resource::<GameStack>()
            .init_resource::<PrioritySystem>()
            .init_resource::<GameState>();

        // Register all game logic systems in the FixedUpdate schedule
        // This ensures they run at a fixed timestep decoupled from the frame rate
        // Only run these systems when in the InGame state
        app.add_systems(
            FixedUpdate,
            (
                // Core game systems
                phase_transition_system,
                priority_system,
                priority_passing_system,
                stack::stack_resolution_system,
                state::state_based_actions_system,
                state::trigger_state_based_actions_system,
                process_game_actions,
                // Turn systems
                handle_turn_start,
                handle_turn_end,
                // Combat systems in sequence
                initialize_combat_phase,
                handle_declare_attackers_event,
                declare_attackers_system,
                handle_declare_blockers_event,
                declare_blockers_system,
                assign_combat_damage_system,
                process_combat_damage_system,
                end_combat_system,
            )
                .run_if(in_state(GameMenuState::InGame)),
        );

        // Register events
        app.add_event::<GameAction>()
            .add_event::<StackItemResolvedEvent>()
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
            .add_event::<CombatDamageCompleteEvent>()
            // Register priority events
            .add_event::<PassPriorityEvent>()
            .add_event::<ResolveStackItemEvent>()
            .add_event::<NextPhaseEvent>()
            .add_event::<EffectCounteredEvent>()
            // Register battlefield events
            .add_event::<EntersBattlefieldEvent>()
            // Register politics events
            .add_event::<GoadEvent>()
            .add_event::<ApplyCombatRestrictionEvent>()
            .add_event::<RemoveCombatRestrictionEvent>();

        // Add game resources initialization during OnEnter(GameMenuState::InGame)
        app.add_systems(
            OnEnter(GameMenuState::InGame),
            (setup_players, setup_game_engine.after(setup_players)),
        );

        // Register zone systems
        zones::register_zone_systems(app);
        // Register turn systems
        register_turn_systems(app);
        // Register commander systems
        commander::register_commander_systems(app);

        // Allow politics systems to register additional systems
        politics::register_politics_systems(app);

        app.add_plugins(zones::ZonesPlugin)
            .add_plugins(permanent::PermanentPlugin);
    }
}

// SystemParam struct to group game engine resources
#[derive(SystemParam)]
struct GameEngineResources<'w> {
    combat_state: ResMut<'w, CombatState>,
    game_stack: ResMut<'w, GameStack>,
    priority_system: ResMut<'w, PrioritySystem>,
    game_state: ResMut<'w, GameState>,
}

/// Spawns initial player entities
fn setup_players(mut commands: Commands) {
    info!("Spawning initial players...");
    // Spawn one placeholder player for now
    // TODO: Replace with actual player setup logic (e.g., based on config or lobby)
    commands.spawn((
        Player {
            player_index: 0,               // Set the player index
            name: "Player 1".to_string(),  // Provide name
            life: 40,                      // Provide starting life (Commander format)
            mana_pool: Default::default(), // Provide default mana pool
        },
        Name::new("Player 1"), // Optional: for debugging
    ));
    info!("Initial players spawned.");
}

/// Initializes the core game engine resources
fn setup_game_engine(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    context: Res<StateTransitionContext>,
    turn_manager: Option<Res<TurnManager>>,
    mut resources: GameEngineResources, // Use the SystemParam struct
) {
    // Skip initialization if we're coming from the pause menu and already have a turn manager
    if context.from_pause_menu && turn_manager.is_some() {
        info!("Resuming from pause menu, skipping game engine initialization");
        return;
    }

    info!("Initializing game engine resources...");

    // Initialize the phase system starting at Beginning::Untap
    // Do this first to ensure other systems that depend on it work correctly
    commands.insert_resource(Phase::Beginning(BeginningStep::Untap));

    // Get all player entities
    let players: Vec<Entity> = player_query.iter().collect();
    if players.is_empty() {
        // This case should ideally not happen now, but good to handle defensively
        error!("No players found during game engine setup! TurnManager initialization skipped.");
        // We probably shouldn't proceed without players
        return;
    } else {
        info!("Found {} players for initialization.", players.len());
    }

    // Initialize turn manager with player list
    let mut turn_manager_instance = TurnManager::default();
    turn_manager_instance.initialize(players.clone());
    commands.insert_resource(turn_manager_instance);

    // Initialize zone manager
    commands.insert_resource(ZoneManager::default());

    // Initialize the commander zone and manager
    commands.insert_resource(CommandZone::default());
    commands.insert_resource(CommandZoneManager::default());

    // Reset any existing resources to their default states using the SystemParam struct
    *resources.combat_state = CombatState::default();
    *resources.game_stack = GameStack::default();
    *resources.priority_system = PrioritySystem::default();
    *resources.game_state = GameState::default();

    info!("Game engine resources initialized successfully.");
}

/// Register core game engine systems with the app
#[allow(dead_code)]
pub fn register_game_engine(app: &mut App) {
    // We'll use these functions which already register the systems for each module
    turns::register_turn_systems(app);
    zones::register_zone_systems(app);

    // Add the stack system
    app.init_resource::<GameStack>();

    // Add the priority system
    app.init_resource::<PrioritySystem>();

    // Add all game systems to FixedUpdate schedule for consistent timing
    app.add_systems(
        FixedUpdate,
        (
            // Core game systems
            phase_transition_system,
            priority_system,
            priority_passing_system,
            stack::stack_resolution_system,
            state::state_based_actions_system,
            state::trigger_state_based_actions_system,
            process_game_actions,
            // Combat systems
            initialize_combat_phase,
            handle_declare_attackers_event,
            declare_attackers_system,
            handle_declare_blockers_event,
            declare_blockers_system,
            assign_combat_damage_system,
            process_combat_damage_system,
            end_combat_system,
        )
            .run_if(in_state(GameMenuState::InGame)),
    );

    // Register commander systems
    commander::register_commander_systems(app);
}
