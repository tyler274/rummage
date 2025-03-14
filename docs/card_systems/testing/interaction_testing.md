# Interaction Testing

This document outlines the methodologies and tools used to test interactions between cards in Rummage. Interaction testing is crucial for ensuring that complex card combinations work correctly according to Magic: The Gathering rules.

## Interaction Complexity

Card interactions in Magic: The Gathering can be extremely complex due to:

- **Layering system**: Effects apply in a specific order
- **Timing rules**: Effects happen at specific times
- **Replacement effects**: Effects that replace other effects
- **Prevention effects**: Effects that prevent other effects
- **Triggered abilities**: Abilities that trigger based on events
- **State-based actions**: Automatic game rules

## Testing Approach

Interaction testing follows a systematic approach:

1. **Identify interaction pairs**: Determine which cards interact
2. **Document expected behavior**: Define how the interaction should work
3. **Create test scenarios**: Design scenarios that test the interaction
4. **Execute tests**: Run the tests and capture results
5. **Verify outcomes**: Compare results to expected behavior

## Interaction Test Framework

The interaction test framework provides tools for testing card interactions:

```rust
// Example of testing card interactions
#[test]
fn test_humility_and_opalescence_interaction() {
    let mut scenario = TestScenario::new();
    
    // Set up players
    let player = scenario.add_player(20);
    
    // Add the interacting cards
    let humility = scenario.add_card_to_battlefield("Humility", player);
    let opalescence = scenario.add_card_to_battlefield("Opalescence", player);
    
    // Add a test enchantment
    let test_enchantment = scenario.add_card_to_battlefield("Pacifism", player);
    
    // Verify the interaction effects
    
    // 1. Check that Humility is a 4/4 creature with no abilities
    let humility_card = scenario.get_permanent(humility);
    assert_eq!(humility_card.power, 4);
    assert_eq!(humility_card.toughness, 4);
    assert!(humility_card.has_no_abilities());
    
    // 2. Check that Opalescence is a 4/4 creature with no abilities
    let opalescence_card = scenario.get_permanent(opalescence);
    assert_eq!(opalescence_card.power, 4);
    assert_eq!(opalescence_card.toughness, 4);
    assert!(opalescence_card.has_no_abilities());
    
    // 3. Check that Pacifism is a 4/4 creature with no abilities
    let pacifism_card = scenario.get_permanent(test_enchantment);
    assert_eq!(pacifism_card.power, 4);
    assert_eq!(pacifism_card.toughness, 4);
    assert!(pacifism_card.has_no_abilities());
    
    // 4. Check that creatures on the battlefield are not affected by Pacifism
    let test_creature = scenario.add_card_to_battlefield("Grizzly Bears", player);
    let creature_card = scenario.get_permanent(test_creature);
    assert!(creature_card.can_attack());
}
```

## Interaction Categories

Different categories of interactions require specific testing approaches:

### Layer Interactions

Testing interactions between effects in different layers:

```rust
#[test]
fn test_layer_interactions() {
    let mut scenario = TestScenario::new();
    let player = scenario.add_player(20);
    
    // Layer 4 (Type) and Layer 7 (P/T) interaction
    let bear = scenario.add_card_to_battlefield("Grizzly Bears", player);
    let mutation = scenario.add_card_to_hand("Artificial Evolution", player);
    let giant_growth = scenario.add_card_to_hand("Giant Growth", player);
    
    // Change creature type (Layer 4)
    scenario.play_card(mutation, Some(bear));
    scenario.choose_text("Bear");
    scenario.choose_text("Elephant");
    scenario.resolve_top_of_stack();
    
    // Boost power/toughness (Layer 7)
    scenario.play_card(giant_growth, Some(bear));
    scenario.resolve_top_of_stack();
    
    // Verify the creature is now a 5/5 Elephant
    let bear_card = scenario.get_permanent(bear);
    assert_eq!(bear_card.power, 5);
    assert_eq!(bear_card.toughness, 5);
    assert!(bear_card.type_line.contains("Elephant"));
    assert!(!bear_card.type_line.contains("Bear"));
}
```

### Timing Interactions

Testing interactions with specific timing requirements:

```rust
#[test]
fn test_timing_interactions() {
    let mut scenario = TestScenario::new();
    let player1 = scenario.add_player(20);
    let player2 = scenario.add_player(20);
    
    // Set up cards
    let creature = scenario.add_card_to_battlefield("Grizzly Bears", player1);
    let counterspell = scenario.add_card_to_hand("Counterspell", player2);
    let bolt = scenario.add_card_to_hand("Lightning Bolt", player1);
    
    // Player 1 casts Lightning Bolt
    scenario.play_card(bolt, Some(player2));
    
    // Player 2 responds with Counterspell
    scenario.pass_priority(player1);
    scenario.play_card(counterspell, Some(bolt));
    
    // Resolve the stack
    scenario.resolve_stack();
    
    // Verify Counterspell countered Lightning Bolt
    assert!(scenario.is_in_zone(bolt, Zone::Graveyard));
    assert!(scenario.is_in_zone(counterspell, Zone::Graveyard));
    assert_eq!(scenario.get_player_life(player2), 20); // Bolt was countered
}
```

### Replacement Effect Interactions

Testing interactions between replacement effects:

```rust
#[test]
fn test_replacement_effect_interactions() {
    let mut scenario = TestScenario::new();
    let player = scenario.add_player(20);
    
    // Set up cards
    let creature = scenario.add_card_to_battlefield("Grizzly Bears", player);
    let prevention = scenario.add_card_to_battlefield("Circle of Protection: Red", player);
    let redirection = scenario.add_card_to_battlefield("Deflecting Palm", player);
    let bolt = scenario.add_card_to_hand("Lightning Bolt", player);
    
    // Activate Circle of Protection
    scenario.activate_ability(prevention, 0);
    scenario.pay_mana("{1}");
    scenario.resolve_top_of_stack();
    
    // Cast Lightning Bolt
    scenario.play_card(bolt, Some(player));
    
    // Choose which replacement effect to apply first
    scenario.choose_replacement_effect(redirection);
    
    // Resolve the stack
    scenario.resolve_stack();
    
    // Verify Deflecting Palm redirected the damage
    assert_eq!(scenario.get_player_life(player), 17); // Took 3 damage from redirection
}
```

## Interaction Test Matrix

Complex interactions are organized in a test matrix:

- **Row cards**: First card in the interaction
- **Column cards**: Second card in the interaction
- **Cell tests**: Tests for the specific interaction

This ensures comprehensive coverage of card interactions.

## Automated Interaction Discovery

The system can automatically discover potential interactions:

```rust
// Example of automated interaction discovery
#[test]
fn discover_interactions() {
    let interaction_finder = InteractionFinder::new();
    
    // Find cards that interact with "Humility"
    let interactions = interaction_finder.find_interactions("Humility");
    
    // Verify known interactions are discovered
    assert!(interactions.contains("Opalescence"));
    assert!(interactions.contains("Nature's Revolt"));
    assert!(interactions.contains("Dryad Arbor"));
    
    // Generate tests for each interaction
    for interaction in interactions {
        let test = interaction_finder.generate_test("Humility", &interaction);
        test.run();
    }
}
```

## Edge Case Testing

Interaction testing includes edge cases:

- **Multiple simultaneous effects**: Many effects applying at once
- **Circular dependencies**: Effects that depend on each other
- **Priority edge cases**: Complex priority passing scenarios
- **Zone transition timing**: Effects during zone transitions

## Related Documentation

- [Effect Verification](effect_verification.md): Testing individual card effects
- [Complex Interactions](../effects/complex_interactions.md): How complex interactions are implemented
- [Layering System](../../mtg_rules/layering.md): Rules for effect layering 