# Card Interaction Testing

This document explains the approach for testing card interactions in Rummage. It covers how to create robust tests for card effects, interactions between cards, and complex game scenarios.

## Testing Framework

Rummage provides a flexible testing framework centered around the `TestScenario` struct, which simplifies setting up test environments for card interactions. This framework allows developers to:

1. Create players with custom life totals
2. Add cards to specific zones (hand, battlefield, graveyard, etc.)
3. Play cards, including targeting
4. Resolve the stack
5. Verify game state after actions

## Example Test

Here's a simple example of testing a Lightning Bolt:

```rust
#[test]
fn lightning_bolt_deals_3_damage() {
    let mut test = TestScenario::new();
    let player1 = test.add_player(20);
    let player2 = test.add_player(20);
    let bolt = test.add_card_to_hand("Lightning Bolt", player1);
    
    // Play Lightning Bolt targeting player2
    test.play_card(bolt, Some(player2));
    test.resolve_top_of_stack();
    
    // Verify player2 lost 3 life
    assert_eq!(test.get_player_life(player2), 17);
}
```

## Testing Complex Interactions

For complex card interactions, the testing framework supports:

1. **Stack Interactions**: Testing cards that interact with the stack, like counterspells
2. **Zone Transitions**: Verifying cards move between zones correctly
3. **Targeting**: Testing complex targeting requirements and restrictions
4. **State-Based Actions**: Verifying state-based actions resolve correctly

Example of testing a counterspell:

```rust
#[test]
fn counterspell_counters_lightning_bolt() {
    let mut test = TestScenario::new();
    let player1 = test.add_player(20);
    let player2 = test.add_player(20);
    
    let bolt = test.add_card_to_hand("Lightning Bolt", player1);
    let counterspell = test.add_card_to_hand("Counterspell", player2);
    
    // Play Lightning Bolt targeting player2
    test.play_card(bolt, Some(player2));
    
    // In response, player2 casts Counterspell targeting Lightning Bolt
    test.play_card(counterspell, Some(bolt));
    
    // Resolve the stack from top to bottom
    test.resolve_top_of_stack(); // Counterspell resolves
    test.resolve_top_of_stack(); // Lightning Bolt tries to resolve but was countered
    
    // Verify player2 still has 20 life (Lightning Bolt was countered)
    assert_eq!(test.get_player_life(player2), 20);
}
```

## Best Practices

When writing tests for card interactions, follow these best practices:

1. **Test the rule, not the implementation**: Focus on testing the expected behavior according to MTG rules, not implementation details
2. **Cover edge cases**: Test unusual interactions and edge cases
3. **Test interactions with different card types**: Ensure cards interact correctly with different types (creatures, instants, etc.)
4. **Use realistic scenarios**: Create test scenarios that mimic real game situations
5. **Document test intent**: Clearly document what each test is verifying

## Extending the Testing Framework

The testing framework is designed to be extensible. To add support for testing new card mechanics:

1. Add new methods to the `TestScenario` struct
2. Implement simulation of the new mechanic
3. Add verification methods to check the result

## Future Enhancements

The testing framework is still under development. Planned enhancements include:

1. Support for more complex targeting scenarios
2. Better simulation of priority and the stack
3. Support for testing multiplayer interactions
4. Integration with snapshot testing

## Related Documentation

- [Effect Verification](effect_verification.md): How card effects are verified
- [Interaction Testing](interaction_testing.md): Testing complex card interactions
- [Snapshot Testing](../../core_systems/snapshot/testing.md): Using snapshots for card testing
- [End-to-End Testing](../../testing/end_to_end_testing.md): Testing cards in full game scenarios 