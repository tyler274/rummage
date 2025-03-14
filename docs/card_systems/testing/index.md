# Testing Cards

Card testing is a critical aspect of Rummage development, ensuring that cards function correctly according to Magic: The Gathering rules. This document outlines the approach and methodologies for testing cards in the system.

## Testing Philosophy

The card testing system follows these principles:

- **Comprehensive coverage**: All cards should be tested for correct behavior
- **Rule compliance**: Cards must behave according to official MTG rules
- **Edge case handling**: Tests should cover uncommon interactions
- **Regression prevention**: Tests should catch regressions when code changes
- **Documentation**: Tests serve as documentation for expected behavior

## Testing Categories

Cards are tested across several categories:

### Functional Testing

- **Basic functionality**: Core card mechanics work as expected
- **Rules text adherence**: Card behaves according to its rules text
- **Zone transitions**: Card behaves correctly when moving between zones
- **State changes**: Card responds correctly to changes in game state

### Interaction Testing

- **Card-to-card interactions**: How cards interact with other cards
- **Rules interactions**: How cards interact with game rules
- **Timing interactions**: How cards behave with timing-sensitive effects
- **Priority interactions**: How cards interact with the priority system

### Edge Case Testing

- **Corner cases**: Unusual but valid game states
- **Rules exceptions**: Cards that modify or break standard rules
- **Multiple effects**: Cards with multiple simultaneous effects
- **Layer interactions**: How cards behave with the layer system

## Testing Tools

The testing system provides several tools:

- **Scenario builder**: Create specific game states for testing
- **Action sequencer**: Execute a series of game actions
- **State validator**: Verify game state after actions
- **Snapshot comparison**: Compare game states before and after effects
- **Rules oracle**: Validate behavior against rules database

## Example Test

Here's an example of a card test using the testing framework:

```rust
#[test]
fn lightning_bolt_deals_3_damage() {
    // Arrange: Set up the test scenario
    let mut test = TestScenario::new();
    let player1 = test.add_player(20);
    let player2 = test.add_player(20);
    let bolt = test.add_card_to_hand("Lightning Bolt", player1);
    
    // Act: Execute the actions
    test.play_card(bolt, Some(player2));
    test.resolve_top_of_stack();
    
    // Assert: Verify the outcome
    assert_eq!(test.get_player_life(player2), 17);
}
```

## Testing Standards

All cards must pass these testing standards:

- **Functionality tests**: Basic functionality works
- **Interaction tests**: Works with related card types
- **Rules compliance**: Follows comprehensive rules
- **Performance**: Doesn't cause performance issues
- **Visual correctness**: Renders correctly

## Continuous Integration

Card tests are run as part of the continuous integration pipeline:

- Tests run on every pull request
- Cards with failed tests cannot be merged
- Test coverage is tracked and reported

## Related Documentation

- [Effect Verification](effect_verification.md): How card effects are verified
- [Interaction Testing](interaction_testing.md): Testing complex card interactions
- [Snapshot Testing](../../core_systems/snapshot/testing.md): Using snapshots for card testing
- [End-to-End Testing](../../testing/end_to_end_testing.md): Testing cards in full game scenarios 