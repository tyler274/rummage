# Snapshot Testing

This document provides an overview of snapshot testing in the Rummage project. For detailed implementation of the snapshot system itself, please see [Snapshot System Testing](../core_systems/snapshot/testing.md).

## Overview

Snapshot testing is a critical component of Rummage's testing strategy, allowing us to capture, store, and compare game states at different points in time. This approach is especially valuable for testing a complex rules engine like Magic: The Gathering, where many interactions can affect the game state in subtle ways.

## Benefits of Snapshot Testing

- **Deterministic Testing**: Ensures game logic produces consistent, reproducible results
- **Regression Prevention**: Quickly identifies when changes affect existing functionality
- **Complex State Verification**: Makes it easier to verify complex game states
- **Interaction Testing**: Facilitates testing of multi-step card interactions

## Snapshot Testing in MTG

In the context of Magic: The Gathering, snapshot testing helps verify:

1. **Rule Correctness**: Ensures rules are applied correctly
2. **Card Interactions**: Validates complex interactions between cards
3. **Format-Specific Rules**: Tests special rules for formats like Commander

## Integration with Other Testing Types

Snapshot testing complements other testing approaches:

- **Unit Tests**: Verify individual components in isolation
- **Integration Tests**: Test how components work together
- **End-to-End Tests**: Test the entire game flow

## Using Snapshots in Tests

```rust
#[test]
fn test_lightning_bolt_damage() {
    // Set up the game state
    let mut game = setup_test_game();
    let player = game.add_player(20); // Player with 20 life
    
    // Cast Lightning Bolt targeting the player
    let lightning_bolt = game.create_card("Lightning Bolt");
    game.cast(lightning_bolt, Some(player));
    
    // Take a snapshot before resolution
    let pre_resolution = game.create_snapshot();
    
    // Resolve the spell
    game.resolve_top_of_stack();
    
    // Take a snapshot after resolution
    let post_resolution = game.create_snapshot();
    
    // Verify the player's life total decreased by 3
    assert_eq!(pre_resolution.get_player_life(player), 20);
    assert_eq!(post_resolution.get_player_life(player), 17);
    
    // Save the snapshots for future regression tests
    game.save_snapshot("lightning_bolt_pre", pre_resolution);
    game.save_snapshot("lightning_bolt_post", post_resolution);
}
```

## Best Practices

1. **Targeted Snapshots**: Capture only the relevant parts of game state
2. **Clear Naming**: Use descriptive names for snapshots
3. **Minimal Setup**: Keep test setup as simple as possible
4. **Deterministic Inputs**: Ensure tests have consistent inputs (e.g., fix RNG seeds)
5. **Review Changes**: Carefully review snapshot changes in pull requests

## Related Documentation

- [Snapshot System Overview](../core_systems/snapshot/overview.md)
- [Snapshot System Implementation](../core_systems/snapshot/implementation.md)
- [Snapshot System Testing](../core_systems/snapshot/testing.md)
- [Integration Testing](integration_testing.md)
- [End-to-End Testing](end_to_end_testing.md) 