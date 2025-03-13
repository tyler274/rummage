# Game Engine Testing Guide

## Overview

This guide outlines the comprehensive testing approach for the Rummage Magic: The Gathering Commander game engine. Testing a complex game engine requires multiple layers of validation to ensure rules are correctly implemented and interactions work as expected.

## Testing Philosophies

The Rummage testing strategy follows these core philosophies:

1. **Rules-First Testing**: Test cases are derived directly from the MTG Comprehensive Rules
2. **Isolation and Integration**: Test components in isolation before testing their interactions
3. **Edge Case Coverage**: Explicitly test corner cases and unusual card interactions
4. **Performance Validation**: Ensure the engine performs well under various game conditions
5. **Reproducibility**: All tests should be deterministic and repeatable

## Testing Layers

The game engine testing is structured in layers:

### 1. Unit Tests

Unit tests validate individual components in isolation. Each module should have comprehensive unit tests covering:

- Basic functionality
- Edge cases
- Error handling
- Public interface contracts

Example for the stack system:

```rust
#[test]
fn test_stack_push_and_resolve() {
    let mut app = App::new();
    // Setup minimal test environment
    app.add_plugins(MinimalPlugins)
       .add_plugin(StackPlugin);
    
    // Add a spell to the stack
    let spell_entity = app.world.spawn_empty().id();
    app.world.send_event(StackPushEvent {
        entity: spell_entity,
        source: None,
    });
    
    // Run systems
    app.update();
    
    // Verify stack state
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    
    // Resolve the top item
    app.world.send_event(StackResolveTopEvent);
    app.update();
    
    // Verify stack is empty
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 0);
}
```

### 2. Integration Tests

Integration tests verify the interaction between multiple components. Key integration scenarios include:

- Turn structure and phase progression
- Combat resolution
- Stack interaction with permanents
- Zone transitions

Example integration test:

```rust
#[test]
fn test_spell_cast_and_resolve_integration() {
    let mut app = App::new();
    // Setup more complete environment
    app.add_plugins(GameEngineTestPlugins)
       .add_systems(Startup, setup_test_game);
    
    // Create test player and card
    let (player, card) = setup_test_player_with_card(&mut app);
    
    // Cast the spell
    app.world.send_event(CastSpellEvent {
        player,
        card,
        targets: Vec::new(),
        mode: CastMode::Normal,
    });
    
    // Run systems to process the cast
    app.update();
    
    // Verify card moved to stack
    let stack = app.world.resource::<Stack>();
    assert!(stack.contains(card));
    
    // Resolve the stack
    resolve_stack_completely(&mut app);
    
    // Verify expected outcome based on card type
    let card_type = app.world.get::<CardType>(card).unwrap();
    match *card_type {
        CardType::Creature => {
            // Verify creature entered battlefield
            let battlefield = app.world.resource::<Battlefield>();
            assert!(battlefield.contains(card));
        },
        CardType::Sorcery => {
            // Verify sorcery went to graveyard
            let graveyard = get_player_graveyard(&app, player);
            assert!(graveyard.contains(card));
        },
        // Handle other card types
        _ => {}
    }
}
```

### 3. End-to-End Tests

End-to-end tests simulate complete game scenarios to validate the engine as a whole:

```rust
#[test]
fn test_full_turn_cycle() {
    let mut app = App::new();
    // Setup complete game environment
    app.add_plugins(GameEnginePlugins)
       .add_systems(Startup, setup_full_game);
    
    // Get active player
    let active_player = get_active_player(&app);
    
    // Progress through each phase of a turn
    progress_to_phase(&mut app, Phase::Untap);
    // Verify untap actions occurred
    
    progress_to_phase(&mut app, Phase::Upkeep);
    // Verify upkeep triggers happened
    
    progress_to_phase(&mut app, Phase::Draw);
    // Verify draw occurred
    
    progress_to_phase(&mut app, Phase::Main1);
    // Play a land and cast a spell
    play_land(&mut app, active_player);
    cast_test_spell(&mut app, active_player);
    
    progress_to_phase(&mut app, Phase::Combat);
    // Declare attackers and blockers
    declare_all_attackers(&mut app, active_player);
    declare_blockers(&mut app, get_defending_player(&app));
    
    progress_to_phase(&mut app, Phase::Main2);
    // Cast another spell
    
    progress_to_phase(&mut app, Phase::End);
    // Verify end step triggers
    
    progress_to_phase(&mut app, Phase::Cleanup);
    // Verify cleanup actions
    
    // Verify turn passed to next player
    app.update();
    assert_ne!(active_player, get_active_player(&app));
}
```

## Testing Specific Systems

### Stack and Priority System

The stack and priority system requires detailed testing:

1. **Stack Order Tests**
   - Test LIFO (Last In, First Out) resolution order
   - Verify split second interactions
   - Test interrupted resolution (new items added during resolution)

2. **Priority Tests**
   - Test priority passing in turn order
   - Verify holding and passing priority
   - Test shortcuts when all players pass priority

3. **Response Window Tests**
   - Test response opportunities for each player
   - Verify timing restrictions (sorcery speed vs. instant speed)
   - Test response chains with multiple spells

### Combat System

Combat system testing should include:

1. **Attack Phase Tests**
   - Test declaration constraints
   - Verify "attacks each turn if able" effects
   - Test combat-triggered abilities

2. **Blocking Tests**
   - Test valid blocker determination
   - Verify "must block if able" effects
   - Test multiple blockers assignment

3. **Damage Assignment Tests**
   - Test normal damage assignment
   - Verify trample damage assignment
   - Test replacement and prevention effects

### Zones System

Zone transitions require comprehensive testing:

1. **Basic Zone Transfer Tests**
   - Test all possible zone-to-zone movements
   - Verify appropriate triggers fire
   - Test replacement effects on zone transfers

2. **Replacement Effect Tests**
   - Test "instead of going to graveyard" effects
   - Verify commanders going to command zone
   - Test exile replacements

3. **Hidden Zone Tests**
   - Test library as a hidden zone
   - Verify hand information visibility rules
   - Test face-down exile interactions

### Turn Structure Tests

Turn structure testing should cover:

1. **Phase Progression Tests**
   - Test normal phase progression
   - Verify phase skipping effects
   - Test additional phases/steps

2. **Additional Turn Tests**
   - Test extra turn creation
   - Verify nested extra turns
   - Test turn-ending effects

3. **Special Turn Rule Tests**
   - Test first turn draw rule
   - Verify time limits per phase
   - Test simultaneous turn effects (Two-Headed Giant)

## Testing Tools and Utilities

The testing framework provides several utilities to facilitate testing:

### Test Game Setup

```rust
/// Sets up a test game with specified parameters
pub fn setup_test_game(
    app: &mut App,
    player_count: usize,
    starting_life: i32,
    starting_hand_size: usize,
) {
    // Setup test game with given parameters
}
```

### Game State Assertions

```rust
/// Asserts the expected game state
pub fn assert_game_state(
    app: &App, 
    expected_active_player: Entity,
    expected_phase: Phase,
    expected_stack_size: usize,
) {
    // Verify game state matches expectations
}
```

### Card Creation Utilities

```rust
/// Creates a test card with specified characteristics
pub fn create_test_card(
    app: &mut App,
    name: &str,
    card_type: CardType,
    mana_cost: &str,
) -> Entity {
    // Create and return a test card entity
}
```

## Test Data Management

Rummage uses structured test data to drive complex test scenarios:

1. **Test Deck Files**
   - JSON/TOML files defining test decks
   - Special test-only cards with predictable behavior
   - Scenario-specific deck configurations

2. **Test Scenario Definitions**
   - Predefined board states for targeted testing
   - Scripted action sequences
   - Expected outcome definitions

3. **Card Interaction Databases**
   - Known complex card interactions
   - Rules clarifications implemented as tests
   - Regression test coverage

## Performance Testing

Game engine performance testing includes:

1. **Benchmark Tests**
   - Core operation benchmarks (card draws, stack resolution)
   - Complex board state handling
   - Memory usage tracking

2. **Scaling Tests**
   - Large token count handling
   - Many triggered abilities firing simultaneously
   - Large deck/graveyard performance

3. **Engine Stress Tests**
   - Heavy interaction chains
   - Extreme game states
   - Recovery from invalid states

## Test Logging and Debugging

Test environments provide enhanced logging:

```rust
// Enable detailed test logging
#[test]
fn test_with_detailed_logging() {
    // Setup test-specific logger
    let _logger = TestLogger::start();
    
    // Test steps will now have detailed logging
    // ...
    
    // Examine logs for debugging
    let logs = _logger.get_logs();
    assert!(logs.contains("Expected debug information"));
}
```

## Continuous Integration

Our CI pipeline runs the following test suites:

1. Fast unit tests on every commit
2. Integration tests before merge
3. Full end-to-end test suite nightly
4. Performance benchmarks weekly

## Test Coverage Goals

The Rummage test suite aims for:

- 95%+ line coverage for critical game rule systems
- 90%+ branch coverage for game logic
- 100% coverage of publicly documented interactions
- All reported bugs covered by regression tests 