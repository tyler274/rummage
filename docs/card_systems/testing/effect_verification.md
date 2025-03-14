# Effect Verification

This document outlines the methodologies and tools used to verify that card effects work correctly in Rummage. Effect verification is a critical part of ensuring that cards behave according to the Magic: The Gathering rules.

## Verification Approach

Effect verification follows a systematic approach:

1. **Define expected behavior**: Document how the effect should work
2. **Create test scenarios**: Design scenarios that test the effect
3. **Execute tests**: Run the tests and capture results
4. **Verify outcomes**: Compare results to expected behavior
5. **Document edge cases**: Record any special cases or interactions

## Test Scenario Builder

The test scenario builder is a key tool for effect verification:

```rust
// Example of using the test scenario builder
#[test]
fn verify_lightning_bolt_effect() {
    // Create a test scenario
    let mut scenario = TestScenario::new();
    
    // Set up players
    let player1 = scenario.add_player(20);
    let player2 = scenario.add_player(20);
    
    // Add cards to the game
    let bolt = scenario.add_card_to_hand("Lightning Bolt", player1);
    
    // Execute game actions
    scenario.play_card(bolt, Some(player2));
    scenario.resolve_top_of_stack();
    
    // Verify the effect
    assert_eq!(scenario.get_player_life(player2), 17);
}
```

## Effect Categories

Different effect categories require specific verification approaches:

### Damage Effects

- **Direct damage**: Verify damage is dealt to the correct target
- **Conditional damage**: Verify conditions are correctly evaluated
- **Damage prevention**: Verify prevention effects work correctly
- **Damage redirection**: Verify damage is redirected properly

### Card Movement Effects

- **Zone transitions**: Verify cards move between zones correctly
- **Search effects**: Verify search functionality works properly
- **Shuffle effects**: Verify randomization is applied
- **Reordering effects**: Verify cards are reordered correctly

### Modification Effects

- **Stat changes**: Verify power/toughness changes are applied
- **Ability changes**: Verify abilities are added/removed correctly
- **Type changes**: Verify type changes are applied correctly
- **Color changes**: Verify color changes are applied correctly

### Control Effects

- **Control changes**: Verify control changes hands correctly
- **Duration**: Verify control returns at the right time
- **Restrictions**: Verify restrictions on controlled permanents

## Verification Tools

Several tools assist in effect verification:

### State Snapshots

Snapshots capture the game state before and after an effect:

```rust
// Example of using state snapshots
#[test]
fn verify_wrath_of_god_effect() {
    let mut scenario = TestScenario::new();
    
    // Set up game state with creatures
    let player1 = scenario.add_player(20);
    let wrath = scenario.add_card_to_hand("Wrath of God", player1);
    scenario.add_card_to_battlefield("Grizzly Bears", player1);
    scenario.add_card_to_battlefield("Serra Angel", player1);
    
    // Take a snapshot before the effect
    let before_snapshot = scenario.create_snapshot();
    
    // Execute the effect
    scenario.play_card(wrath, None);
    scenario.resolve_top_of_stack();
    
    // Take a snapshot after the effect
    let after_snapshot = scenario.create_snapshot();
    
    // Verify all creatures are destroyed
    assert_eq!(
        after_snapshot.count_permanents_by_type(player1, CardType::Creature),
        0
    );
    
    // Verify the difference between snapshots
    let diff = before_snapshot.diff(&after_snapshot);
    assert!(diff.contains(SnapshotDiff::Destroyed { 
        card_name: "Grizzly Bears".to_string() 
    }));
}
```

### Event Trackers

Event trackers monitor events triggered during effect resolution:

```rust
// Example of using event trackers
#[test]
fn verify_lightning_helix_effect() {
    let mut scenario = TestScenario::new();
    
    // Set up the test
    let player1 = scenario.add_player(20);
    let player2 = scenario.add_player(20);
    let helix = scenario.add_card_to_hand("Lightning Helix", player1);
    
    // Create event trackers
    let mut damage_events = scenario.track_events::<DamageEvent>();
    let mut life_gain_events = scenario.track_events::<LifeGainEvent>();
    
    // Execute the effect
    scenario.play_card(helix, Some(player2));
    scenario.resolve_top_of_stack();
    
    // Verify damage event
    assert_eq!(damage_events.count(), 1);
    let damage_event = damage_events.last().unwrap();
    assert_eq!(damage_event.amount, 3);
    assert_eq!(damage_event.target, player2);
    
    // Verify life gain event
    assert_eq!(life_gain_events.count(), 1);
    let life_gain_event = life_gain_events.last().unwrap();
    assert_eq!(life_gain_event.amount, 3);
    assert_eq!(life_gain_event.player, player1);
}
```

### Rules Oracle

The rules oracle verifies that effects comply with the MTG comprehensive rules:

```rust
// Example of using the rules oracle
#[test]
fn verify_protection_effect() {
    let mut scenario = TestScenario::new();
    
    // Set up the test
    let player1 = scenario.add_player(20);
    let player2 = scenario.add_player(20);
    
    let creature = scenario.add_card_to_battlefield("Grizzly Bears", player1);
    let protection = scenario.add_card_to_hand("Gods Willing", player1);
    let bolt = scenario.add_card_to_hand("Lightning Bolt", player2);
    
    // Give protection from red
    scenario.play_card(protection, Some(creature));
    scenario.choose_option("Red"); // Choose red for protection
    scenario.resolve_top_of_stack();
    
    // Try to bolt the protected creature
    scenario.play_card(bolt, Some(creature));
    
    // Verify with rules oracle
    let oracle_result = scenario.check_rules_compliance();
    assert!(oracle_result.has_rule_violation(RuleViolation::InvalidTarget));
}
```

## Continuous Integration

Effect verification is integrated into the CI/CD pipeline:

- **Automated testing**: All effect tests run on each commit
- **Regression detection**: Changes that break effects are caught
- **Coverage tracking**: Ensures all effects are tested
- **Performance monitoring**: Tracks effect resolution performance

## Related Documentation

- [Interaction Testing](interaction_testing.md): Testing interactions between effects
- [Snapshot Testing](../../core_systems/snapshot/testing.md): Using snapshots for testing
- [Card Effects](../effects/index.md): How card effects are implemented 