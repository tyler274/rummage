# Integration Testing: Networking and Game Engine

This document focuses on the critical integration testing needed between the networking layer and the core MTG Commander game engine. Ensuring these two complex systems work together seamlessly is essential for a robust online implementation.

## Table of Contents

1. [Integration Test Goals](#integration-test-goals)
2. [Test Architecture](#test-architecture)
3. [State Synchronization Tests](#state-synchronization-tests)
4. [MTG-Specific Network Integration Tests](#mtg-specific-network-integration-tests)
5. [End-to-End Gameplay Tests](#end-to-end-gameplay-tests)
6. [Test Harness Implementation](#test-harness-implementation)
7. [Continuous Integration Strategy](#continuous-integration-strategy)
8. [Automated Test Generation](#automated-test-generation)

## Integration Test Goals

Integration testing for our networked MTG Commander implementation focuses on:

1. **Seamless Interaction**: Verifying the networking code and game engine interact without errors
2. **Game State Integrity**: Ensuring game state remains consistent across server and clients
3. **Rules Enforcement**: Confirming game rules are correctly enforced in multiplayer contexts
4. **Performance**: Measuring and validating performance under realistic gameplay conditions
5. **Robustness**: Testing recovery from network disruptions and other failures

## Test Architecture

Our integration test architecture follows a layered approach:

```
┌─────────────────────────────────────┐
│       End-to-End Gameplay Tests     │
├─────────────────────────────────────┤
│   MTG-Specific Network Integration  │
├─────────────────────────────────────┤
│      State Synchronization Tests    │
├─────────────────────────────────────┤
│  Component Integration Tests        │
└─────────────────────────────────────┘
```

Each test layer builds on the previous, starting with component integration and progressing to full gameplay scenarios.

### Implementation Structure

```rust
// src/tests/integration/mod.rs
pub mod networking_game_integration {
    mod component_integration;
    mod state_synchronization;
    mod mtg_specific;
    mod end_to_end;
    
    pub use component_integration::*;
    pub use state_synchronization::*;
    pub use mtg_specific::*;
    pub use end_to_end::*;
}
```

## State Synchronization Tests

These tests verify that game state is correctly synchronized between server and clients.

### Game State Replication Test

```rust
#[test]
fn test_game_state_replication() {
    let mut app = setup_integration_test(4);
    
    // Configure initial game state
    app.world.resource_mut::<TestController>()
        .setup_game_state(TestGameState::MidGame);
    
    // Ensure all clients start with synchronized state
    app.update_n_times(10);
    
    // Execute a complex game action on the server
    app.world.resource_mut::<TestController>()
        .execute_server_action(ServerAction::ResolveComplexEffect {
            effect_id: 123,
            targets: vec![1, 2, 3],
            values: vec![5, 10, 15]
        });
    
    // Allow time for replication
    app.update_n_times(10);
    
    // Verify all clients received the updated state
    let server_state = extract_server_game_state(&app);
    
    for client_id in 1..=4 {
        let client_state = extract_client_game_state(&app, client_id);
        
        // Check public state is identical
        assert_eq!(
            server_state.public_state,
            client_state.public_state,
            "Client {} has inconsistent public state", 
            client_id
        );
        
        // Check player-specific private state
        assert_eq!(
            server_state.player_states[client_id].private_view,
            client_state.private_state,
            "Client {} has inconsistent private state", 
            client_id
        );
    }
}
```

### Incremental Update Test

Verify that incremental updates are correctly applied:

```rust
#[test]
fn test_incremental_updates() {
    let mut app = setup_integration_test(4);
    
    // Track update messages sent
    let mut update_tracker = UpdateTracker::new();
    app.insert_resource(update_tracker);
    app.add_systems(Update, track_network_updates);
    
    // Execute sequence of small game actions
    let actions = [
        GameAction::DrawCard(1),
        GameAction::PlayLand(1, 101),
        GameAction::PassPriority(1),
        GameAction::PassPriority(2),
        GameAction::CastSpell(3, 302, vec![]),
    ];
    
    for action in &actions {
        app.world.resource_mut::<TestController>()
            .execute_action(action.clone());
        app.update_n_times(5);
    }
    
    // Retrieve update tracking data
    let update_tracker = app.world.resource::<UpdateTracker>();
    
    // Verify we sent incremental updates (not full state)
    assert!(update_tracker.full_state_updates < actions.len());
    assert!(update_tracker.incremental_updates > 0);
    
    // Verify game state is consistent
    verify_game_state_consistency(&app);
}
```

## MTG-Specific Network Integration Tests

These tests focus on MTG-specific game mechanics and their network integration.

### Priority Passing Test

```rust
#[test]
fn test_networked_priority_system() {
    let mut app = setup_integration_test(4);
    
    // Initialize game state for testing priority
    setup_priority_test_state(&mut app);
    
    // Track current priority holder
    let mut expected_priority = 1; // Start with player 1
    
    // Pass priority around the table
    for _ in 0..8 {  // 2 full cycles
        // Verify current priority
        let priority_system = app.world.resource::<PrioritySystem>();
        assert_eq!(
            priority_system.current_player, 
            expected_priority,
            "Incorrect priority holder"
        );
        
        // Current player passes priority
        app.world.resource_mut::<TestController>()
            .execute_action(GameAction::PassPriority(expected_priority));
        app.update_n_times(5);
        
        // Update expected priority (cycle 1->2->3->4->1)
        expected_priority = expected_priority % 4 + 1;
    }
    
    // Now test a player casting a spell with priority
    let priority_holder = expected_priority;
    
    // Execute cast spell action
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::CastSpell(
            priority_holder, 
            123, // Card ID
            vec![] // No targets
        ));
    app.update_n_times(5);
    
    // Verify spell was cast and on the stack
    let game_state = extract_server_game_state(&app);
    assert!(game_state.stack.contains_spell(123));
    
    // Verify priority passed to next player
    let priority_system = app.world.resource::<PrioritySystem>();
    assert_eq!(
        priority_system.current_player,
        priority_holder % 4 + 1,
        "Priority didn't pass after spell cast"
    );
}
```

### Hidden Information Test

```rust
#[test]
fn test_networked_hidden_information() {
    let mut app = setup_integration_test(4);
    
    // Setup a player with cards in hand
    app.world.resource_mut::<TestController>()
        .setup_player_hand(1, vec![101, 102, 103]);
    
    app.update_n_times(5);
    
    // Verify only player 1 can see their hand contents
    for client_id in 1..=4 {
        let client_state = extract_client_game_state(&app, client_id);
        
        if client_id == 1 {
            // Player 1 should see all cards in their hand
            assert_eq!(client_state.player_hand.len(), 3);
            assert!(client_state.player_hand.contains(&101));
            assert!(client_state.player_hand.contains(&102));
            assert!(client_state.player_hand.contains(&103));
        } else {
            // Other players should only see card backs/count
            assert_eq!(client_state.opponents[0].hand_size, 3);
            assert!(!client_state.can_see_card(101));
            assert!(!client_state.can_see_card(102));
            assert!(!client_state.can_see_card(103));
        }
    }
    
    // Test revealing a card to all players
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::RevealCard(1, 102));
    
    app.update_n_times(5);
    
    // Verify all clients can now see the revealed card
    for client_id in 1..=4 {
        let client_state = extract_client_game_state(&app, client_id);
        assert!(client_state.can_see_card(102));
        
        // Other cards still hidden from opponents
        if client_id != 1 {
            assert!(!client_state.can_see_card(101));
            assert!(!client_state.can_see_card(103));
        }
    }
}
```

### Stack Resolution Test

```rust
#[test]
fn test_networked_stack_resolution() {
    let mut app = setup_integration_test(4);
    
    // Setup initial game state with empty stack
    setup_stack_test_state(&mut app);
    
    // Player 1 casts a spell
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::CastSpell(1, 101, vec![]));
    app.update_n_times(5);
    
    // Player 2 responds with their own spell
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::CastSpell(2, 201, vec![101])); // Targeting first spell
    app.update_n_times(5);
    
    // Player 3 and 4 pass priority
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::PassPriority(3));
    app.update_n_times(5);
    
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::PassPriority(4));
    app.update_n_times(5);
    
    // Back to player 1, who passes
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::PassPriority(1));
    app.update_n_times(5);
    
    // Player 2 passes - this should resolve their spell
    app.world.resource_mut::<TestController>()
        .execute_action(GameAction::PassPriority(2));
    app.update_n_times(10); // More updates for resolution
    
    // Verify player 2's spell resolved
    let game_state = extract_server_game_state(&app);
    assert!(!game_state.stack.contains_spell(201));
    
    // Check if the effect was applied - in this case, counter player 1's spell
    assert!(!game_state.stack.contains_spell(101));
    
    // Verify all clients see the empty stack
    for client_id in 1..=4 {
        let client_state = extract_client_game_state(&app, client_id);
        assert_eq!(client_state.stack.spells.len(), 0);
    }
}
```

## End-to-End Gameplay Tests

These tests simulate full gameplay scenarios to verify the complete integrated system.

### Full Turn Cycle Test

```rust
#[test]
fn test_complete_turn_cycle() {
    let mut app = setup_integration_test(4);
    
    // Setup game state with predetermined decks and hands
    setup_deterministic_game_start(&mut app);
    
    // Execute a full turn cycle
    let turn_actions = generate_full_turn_actions(1); // For player 1's turn
    
    for action in turn_actions {
        app.world.resource_mut::<TestController>()
            .execute_action(action);
        app.update_n_times(5);
    }
    
    // Verify turn passed to next player
    let turn_system = app.world.resource::<TurnSystem>();
    assert_eq!(turn_system.active_player, 2);
    assert_eq!(turn_system.phase, Phase::Untap);
    
    // Verify all game state is consistent
    verify_game_state_consistency(&app);
    
    // Check that specific expected game actions occurred
    let game_state = extract_server_game_state(&app);
    assert!(game_state.turn_history.contains_action_by_player(1, ActionType::PlayLand));
    assert!(game_state.turn_history.contains_action_by_player(1, ActionType::CastSpell));
    
    // Verify expected cards moved zones correctly
    assert!(game_state.battlefield.contains_card_controlled_by(
        played_land_id, 1
    ));
    assert!(game_state.graveyard.contains_card(cast_spell_id));
}
```

### Multiplayer Interaction Test

```rust
#[test]
fn test_multiplayer_interaction() {
    let mut app = setup_integration_test(4);
    
    // Setup mid-game state with interesting board presence
    setup_complex_board_state(&mut app);
    
    // Execute a complex multiplayer interaction:
    // Player 1 attacks player 3
    // Player 2 interferes with a combat trick
    // Player 4 counters player 2's spell
    
    let interaction_sequence = [
        // Player 1 declares attack
        GameAction::DeclareAttackers(1, vec![(creature_id_1, 3)]),
        
        // Priority passes to player 2 who casts combat trick
        GameAction::CastSpell(2, combat_trick_id, vec![creature_id_1]),
        
        // Player 3 passes
        GameAction::PassPriority(3),
        
        // Player 4 counters the combat trick
        GameAction::CastSpell(4, counterspell_id, vec![combat_trick_id]),
        
        // Priority passes around
        GameAction::PassPriority(1),
        GameAction::PassPriority(2),
        GameAction::PassPriority(3),
        GameAction::PassPriority(4),
        
        // Counterspell resolves, and damages goes through
    ];
    
    for action in &interaction_sequence {
        app.world.resource_mut::<TestController>()
            .execute_action(action.clone());
        app.update_n_times(5);
    }
    
    // Allow time for full resolution
    app.update_n_times(20);
    
    // Verify final state reflects expected outcome
    let game_state = extract_server_game_state(&app);
    
    // Combat trick should be countered
    assert!(game_state.graveyard.contains_card(combat_trick_id));
    assert!(game_state.graveyard.contains_card(counterspell_id));
    
    // Player 3 should have taken damage
    let player3_life = game_state.players[2].life;
    assert!(player3_life < STARTING_LIFE);
    
    // Verify all clients have consistent view of the outcome
    verify_game_state_consistency(&app);
}
```

## Test Harness Implementation

Here's a sketch of the integration test harness structure:

```rust
pub struct IntegrationTestContext {
    pub app: App,
    pub network_monitor: NetworkMonitor,
    pub test_actions: Vec<TestAction>,
    pub verification_points: Vec<VerificationPoint>,
}

impl IntegrationTestContext {
    pub fn new(num_players: usize) -> Self {
        let mut app = App::new();
        
        // Add minimal plugins for testing
        app.add_plugins(MinimalPlugins)
            .add_plugins(RepliconServerPlugin::default())
            .add_plugins(RepliconClientPlugin::default());
            
        // Add MTG game systems
        app.add_plugins(MTGCommanderGamePlugin)
            .add_plugins(TestControllerPlugin);
        
        // Setup network monitoring
        let network_monitor = NetworkMonitor::new();
        app.insert_resource(network_monitor.clone());
        app.add_systems(Update, monitor_network_traffic);
        
        // Initialize game with specified players
        app.world.resource_mut::<TestController>()
            .initialize_game(num_players);
            
        Self {
            app,
            network_monitor,
            test_actions: Vec::new(),
            verification_points: Vec::new(),
        }
    }
    
    pub fn queue_action(&mut self, action: TestAction) {
        self.test_actions.push(action);
    }
    
    pub fn add_verification_point(&mut self, point: VerificationPoint) {
        self.verification_points.push(point);
    }
    
    pub fn run_test(&mut self) -> TestResult {
        // Execute all queued actions
        for action in &self.test_actions {
            self.app.world.resource_mut::<TestController>()
                .execute_action(action.clone());
            self.app.update_n_times(5);
        }
        
        // Check all verification points
        let mut results = Vec::new();
        for point in &self.verification_points {
            let result = verify_point(&self.app, point);
            results.push(result);
        }
        
        TestResult {
            passed: results.iter().all(|r| r.passed),
            verification_results: results,
            network_stats: self.network_monitor.get_stats(),
        }
    }
}
```

## Continuous Integration Strategy

Our CI pipeline for integration testing uses a matrix approach:

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  integration-tests:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        test-category: [
          'component-integration',
          'state-synchronization',
          'mtg-specific',
          'end-to-end'
        ]
        num-players: [2, 3, 4]
      
    steps:
    - uses: actions/checkout@v2
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Build
      run: cargo build --verbose
    
    - name: Run integration tests
      run: |
        cargo test --verbose \
        networking_game_integration::${{ matrix.test-category }}::* \
        -- --test-threads=1 \
        --ignored \
        --exact \
        -Z unstable-options \
        --include-ignored \
        --env NUM_PLAYERS=${{ matrix.num-players }}
```

## Automated Test Generation

We use property-based testing to generate valid game scenarios:

```rust
#[test]
fn test_generated_game_scenarios() {
    // Configure test generation parameters
    let generator_config = ScenarioGeneratorConfig {
        num_players: 4,
        min_turns: 3,
        max_turns: 10,
        complexity: TestComplexity::Medium,
        focus_areas: vec![
            FocusArea::Combat,
            FocusArea::SpellInteraction,
            FocusArea::CommanderMechanics,
        ],
    };
    
    let generator = ScenarioGenerator::new(generator_config);
    
    // Generate 5 different test scenarios
    for i in 0..5 {
        let scenario = generator.generate_scenario(i); // Use i as seed
        
        // Create fresh test context
        let mut test_context = IntegrationTestContext::new(scenario.num_players);
        
        // Setup initial state
        test_context.app.world.resource_mut::<TestController>()
            .setup_scenario(&scenario);
        
        // Queue all scenario actions
        for action in &scenario.actions {
            test_context.queue_action(action.clone());
        }
        
        // Add verification points from scenario
        for verification in &scenario.verifications {
            test_context.add_verification_point(verification.clone());
        }
        
        // Run the test
        let result = test_context.run_test();
        
        // Verify all checks passed
        assert!(
            result.passed,
            "Generated scenario {} failed: {:?}",
            i,
            result.failed_verifications()
        );
    }
}
```

---

This integration testing approach ensures that our networking code and MTG Commander game engine work together seamlessly, providing a robust foundation for online play. By combining structured test scenarios with automated generation, we can comprehensively test the complex interactions between game rules and network communication. 