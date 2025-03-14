# Unit Testing

Unit testing in Rummage focuses on verifying the correct implementation of individual components, systems, and rules in isolation. This approach allows us to identify and fix issues early in the development process.

## Overview

Unit tests in Rummage:
- Test small, isolated pieces of functionality
- Are fast and deterministic
- Use mocks and test doubles when needed
- Validate specific MTG rule implementations

## Testing MTG Rules

Testing Magic: The Gathering rules is a critical aspect of unit testing in Rummage. Each rule must be thoroughly tested to ensure it behaves exactly as defined in the official rules.

### Rule Testing Approach

When testing MTG rules:

1. **Identify the Rule**: Clearly document which rule from the comprehensive rules is being tested
2. **Define Test Cases**: Create test cases covering normal behavior, edge cases, and interactions
3. **Deterministic Setup**: Use seeded random number generation for tests involving randomness
4. **Verify Outcomes**: Assert the expected game state after rule application

### Example: Testing State-Based Actions

```rust
#[test]
fn test_creature_dies_from_zero_toughness() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(TestingPlugins);
    
    // Create test entities
    let player = app.world.spawn(PlayerMarker).id();
    let battlefield = app.world.spawn(BattlefieldMarker).id();
    
    // Create a 1/1 creature
    let creature = app.world.spawn((
        CardMarker,
        Creature { power: 1, toughness: 1 },
        InZone { zone: battlefield },
        Controller { player },
    )).id();
    
    // Apply -1/-1 effect
    app.world.send_event(ApplyEffect {
        target: creature,
        effect: Effect::ModifyPowerToughness(-1, -1),
    });
    app.update();
    
    // Apply state-based actions
    app.world.send_event(CheckStateBasedActions);
    app.update();
    
    // Verify creature is now in graveyard
    let card_zone = app.world.get::<InZone>(creature).unwrap();
    let graveyard = app.world.entity(player)
                       .get::<Graveyard>()
                       .unwrap()
                       .zone_entity;
    
    assert_eq!(card_zone.zone, graveyard, "Creature should be moved to graveyard");
}
```

## Component Testing

Unit tests also verify the behavior of individual components:

```rust
#[test]
fn test_mana_cost_component() {
    // Setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    // Test parsing
    let cost = ManaCost::from_string("{2}{W}{U}");
    
    // Verify
    assert_eq!(cost.generic, 2);
    assert_eq!(cost.white, 1);
    assert_eq!(cost.blue, 1);
    assert_eq!(cost.black, 0);
    assert_eq!(cost.red, 0);
    assert_eq!(cost.green, 0);
}
```

## System Testing

Testing individual systems ensures they correctly transform game state:

```rust
#[test]
fn test_draw_card_system() {
    // Setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, draw_card_system);
    
    // Create player with library and hand
    let library = app.world.spawn(LibraryZone).id();
    let hand = app.world.spawn(HandZone).id();
    
    let player = app.world.spawn((
        PlayerMarker,
        PlayerZones { library, hand },
    )).id();
    
    // Add card to library
    let card = app.world.spawn((
        CardMarker,
        InZone { zone: library },
    )).id();
    
    // Trigger draw card
    app.world.send_event(DrawCardEvent { player });
    app.update();
    
    // Verify card moved to hand
    let card_zone = app.world.get::<InZone>(card).unwrap();
    assert_eq!(card_zone.zone, hand, "Card should move from library to hand");
}
```

## Best Practices

When writing unit tests for Rummage:

1. **Use Test Plugins**: Create specialized plugin sets for testing
2. **Seed Random Generation**: Use `bevy_rand` with deterministic seeds
3. **Test State Transitions**: Verify before and after states
4. **Document Test Purpose**: Clearly document which rule or component is being tested
5. **Test Edge Cases**: Ensure rules handle unusual situations correctly
6. **Isolate Systems**: Test systems in isolation using mocks when necessary

## Tools for Unit Testing

Rummage uses several tools to facilitate unit testing:

- **Bevy Test Harness**: Simplified app setup for testing
- **Test Resources**: Common test data and configurations
- **Snapshot Verification**: Comparing game states against expected outcomes
- **Parameterized Tests**: Testing multiple cases with different inputs

## Related Documentation

For more information on testing in Rummage, see:

- [Integration Testing](integration_testing.md)
- [End-to-End Testing](end_to_end_testing.md)
- [Snapshot Testing](../core_systems/snapshot/testing.md) 