use bevy::prelude::*;
use rummage::game_engine::*;
use rummage::player::Player;

// Helper function to set up a test app with two players
fn setup_test_app() -> (App, Entity, Entity) {
    let mut app = App::new();

    // Manually create player entities
    let mut world = app.world_mut();
    let player1 = world.spawn(Player::default()).id();
    let player2 = world.spawn(Player::default()).id();

    (app, player1, player2)
}

/// Test to verify the phase transition system works correctly
#[test]
fn test_phase_transitions() {
    // Set up test app
    let (mut app, player1, player2) = setup_test_app();

    // Initialize resources
    let mut game_state = GameState::default();
    game_state.set_turn_order(vec![player1, player2]);

    // Add resources to the app
    app.insert_resource(game_state)
        .insert_resource(PhaseState::default())
        .add_event::<PhaseTransitionEvent>()
        .insert_resource(PrioritySystem::default())
        .insert_resource(GameStack::default());

    // Add the phase transition system
    app.add_systems(Update, phase_transition_system);

    // Get the initial phase
    let initial_phase = app.world().resource::<PhaseState>().current_phase;
    assert_eq!(initial_phase, Phase::Beginning(BeginningStep::Untap));

    // Send a phase transition event
    let mut events = app
        .world_mut()
        .resource_mut::<Events<PhaseTransitionEvent>>();
    events.send(PhaseTransitionEvent {
        new_phase: Phase::Beginning(BeginningStep::Upkeep),
        new_step: Some(Step::Beginning(BeginningStep::Upkeep)),
    });

    // Run the systems to process the event
    app.update();

    // Verify the phase has advanced
    let new_phase_state = app.world().resource::<PhaseState>();
    assert_eq!(
        new_phase_state.current_phase,
        Phase::Beginning(BeginningStep::Upkeep)
    );
}

/// Test to verify the priority system works correctly
#[test]
fn test_priority_handling() {
    let mut app = App::new();

    // Manually create player entities
    let mut world = app.world_mut();
    let player1 = world.spawn(Player::default()).id();
    let player2 = world.spawn(Player::default()).id();
    let player3 = world.spawn(Player::default()).id();
    let players = vec![player1, player2, player3];

    // Initialize resources
    let mut game_state = GameState::default();
    game_state.set_turn_order(players.clone());

    let mut priority_system_res = PrioritySystem::default();
    priority_system_res.initialize(&players, player1);

    // Add resources to the app
    app.insert_resource(game_state)
        .insert_resource(PhaseState {
            current_phase: Phase::Precombat(PrecombatStep::Main),
            current_step: Some(Step::Precombat(PrecombatStep::Main)),
        })
        .insert_resource(priority_system_res)
        .insert_resource(GameStack::default());

    // Verify initial state
    let initial_priority = app.world().resource::<PrioritySystem>();
    assert_eq!(initial_priority.active_player, player1);

    // Pass priority
    {
        let mut priority = app.world_mut().resource_mut::<PrioritySystem>();
        priority.pass_priority();
    }

    // Verify priority has passed to the next player without running a system
    let new_priority = app.world().resource::<PrioritySystem>();
    assert_eq!(new_priority.active_player, player2);
}

/// Test to verify stack resolution
#[test]
fn test_stack_resolution() {
    let mut app = App::new();

    // Register the event
    app.add_event::<StackItemResolvedEvent>();

    // Manually create player entities
    let mut world = app.world_mut();
    let player1 = world.spawn(Player::default()).id();
    let player2 = world.spawn(Player::default()).id();
    let players = vec![player1, player2];

    // Initialize resources
    let mut game_state = GameState::default();
    game_state.set_turn_order(players.clone());

    let mut priority_system = PrioritySystem::default();
    priority_system.initialize(&players, player1);
    priority_system.all_players_passed = true;

    // Create and add a test effect to the stack
    #[derive(Debug)]
    struct TestEffect {
        controller: Entity,
    }

    impl Effect for TestEffect {
        fn resolve(&self, _commands: &mut Commands) {
            // This would do something in a real effect
        }

        fn controller(&self) -> Entity {
            self.controller
        }

        fn targets(&self) -> Vec<Entity> {
            vec![]
        }
    }

    let mut stack = GameStack::default();
    stack.push(Box::new(TestEffect {
        controller: player1,
    }));

    // Add resources to the app
    app.insert_resource(game_state)
        .insert_resource(PhaseState {
            current_phase: Phase::Precombat(PrecombatStep::Main),
            current_step: Some(Step::Precombat(PrecombatStep::Main)),
        })
        .insert_resource(priority_system)
        .insert_resource(stack);

    // Add the stack resolution system
    app.add_systems(Update, stack_resolution_system);

    // Verify initial state
    let initial_stack = app.world().resource::<GameStack>();
    assert_eq!(initial_stack.len(), 1);

    // Run systems to resolve the stack
    app.update();

    // Verify the stack item was resolved
    let new_stack = app.world().resource::<GameStack>();
    assert_eq!(new_stack.len(), 0);
    assert!(new_stack.is_empty());
}

/// Test to verify phase allows proper actions
#[test]
fn test_phase_action_permissions() {
    // Test various phases and check their action permissions
    let untap_phase = Phase::Beginning(BeginningStep::Untap);
    let upkeep_phase = Phase::Beginning(BeginningStep::Upkeep);
    let main_phase = Phase::Precombat(PrecombatStep::Main);
    let combat_phase = Phase::Combat(CombatStep::DeclareAttackers);

    // Check which phases allow actions
    assert!(!untap_phase.allows_actions());
    assert!(upkeep_phase.allows_actions());
    assert!(main_phase.allows_actions());
    assert!(combat_phase.allows_actions());

    // Check which phases allow sorcery-speed effects
    assert!(!untap_phase.allows_sorcery_speed());
    assert!(!upkeep_phase.allows_sorcery_speed());
    assert!(main_phase.allows_sorcery_speed());
    assert!(!combat_phase.allows_sorcery_speed());
}

/// Test to verify turn progression
#[test]
fn test_turn_progression() {
    let mut app = App::new();

    // Manually create player entities
    let mut world = app.world_mut();
    let player1 = world.spawn(Player::default()).id();
    let player2 = world.spawn(Player::default()).id();
    let players = vec![player1, player2];

    // Initialize resources
    let mut game_state = GameState::default();
    game_state.set_turn_order(players.clone());

    // Add resources to the app
    app.insert_resource(game_state)
        .insert_resource(PhaseState {
            current_phase: Phase::Ending(EndingStep::Cleanup),
            current_step: Some(Step::Ending(EndingStep::Cleanup)),
        })
        .add_event::<PhaseTransitionEvent>()
        .insert_resource(PrioritySystem {
            all_players_passed: true,
            ..PrioritySystem::default()
        })
        .insert_resource(GameStack::default());

    // Add the phase transition system
    app.add_systems(Update, phase_transition_system);

    // Verify initial state
    let initial_game_state = app.world().resource::<GameState>();
    assert_eq!(initial_game_state.turn_number, 1);
    assert_eq!(initial_game_state.active_player, player1);

    // Send a phase transition event to move to the next turn
    let mut events = app
        .world_mut()
        .resource_mut::<Events<PhaseTransitionEvent>>();
    events.send(PhaseTransitionEvent {
        new_phase: Phase::Beginning(BeginningStep::Untap),
        new_step: Some(Step::Beginning(BeginningStep::Untap)),
    });

    // Run systems to process the event
    app.update();

    // Verify the phase has changed
    let new_phase_state = app.world().resource::<PhaseState>();
    assert_eq!(
        new_phase_state.current_phase,
        Phase::Beginning(BeginningStep::Untap)
    );
}
