# Complex Combat Edge Cases - Test Documentation

## Overview

This document outlines test cases for complex interactions in combat that involve multiple mechanics working together. These scenarios test the robustness of our Commander engine when dealing with intricate card interactions and edge cases.

## Test Case: Multiple Triggers During Combat

### Test: Death Triggers During Combat

```rust
#[test]
fn test_multiple_death_triggers_during_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, trigger_system, state_based_actions_system));
       
    // Create creatures with death triggers
    let creature1 = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Attacking { defending: defender_entity },
        Health { current: 3, maximum: 3 },
        DiesTrigger { 
            effect: TriggerEffect::DealDamage { 
                amount: 2,
                target: TargetType::Player(defender_entity),
            }
        },
    )).id();
    
    let creature2 = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Blocking { blocked_attackers: vec![creature1] },
        Health { current: 3, maximum: 3 },
        DiesTrigger { 
            effect: TriggerEffect::GainLife { 
                amount: 2,
                target: TargetType::Player(controller_entity),
            }
        },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    let controller_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Check state-based actions
    app.update();
    
    // Process death triggers
    app.update();
    
    // Verify both creatures died
    assert!(app.world.get::<Health>(creature1).is_none());
    assert!(app.world.get::<Health>(creature2).is_none());
    
    // Verify both death triggers resolved
    let defender_health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(defender_health.current, 18); // 20 - 2 from death trigger
    
    let controller_health = app.world.get::<Health>(controller_entity).unwrap();
    assert_eq!(controller_health.current, 22); // 20 + 2 from death trigger
}
```

## Test Case: Replacement Effects During Combat

### Test: Damage Prevention/Redirection

```rust
#[test]
fn test_damage_replacement_effects() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, replacement_effects_system));
       
    // Create attacker and blocker
    let attacker = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Attacking { defending: defender_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Blocking { blocked_attackers: vec![attacker] },
        Health { current: 3, maximum: 3 },
        // Replacement effect: Redirect half of all damage to controller (rounded up)
        DamageReplacementEffect { 
            effect_type: ReplacementType::Redirect { 
                percentage: 50,
                target: TargetType::Player(controller_entity),
                round_up: true,
            }
        },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    let controller_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify damage was redirected
    let blocker_health = app.world.get::<Health>(blocker).unwrap();
    assert_eq!(blocker_health.current, 1); // 3 - (5 - 3) = 1
    
    let controller_health = app.world.get::<Health>(controller_entity).unwrap();
    assert_eq!(controller_health.current, 17); // 20 - 3 (redirected damage) = 17
    
    // Verify attacker also took damage
    let attacker_health = app.world.get::<Health>(attacker).unwrap();
    assert_eq!(attacker_health.current, 2); // 5 - 3 = 2
}
```

## Test Case: Layer-Dependent Effects

### Test: Characteristic-Defining Abilities and Pump Spells

```rust
#[test]
fn test_layer_dependent_effects() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (spell_resolution_system, characteristic_layer_system));
       
    // Create creature with CDA (power/toughness equal to cards in hand)
    let creature = app.world.spawn((
        CreatureCard {},
        // Base power/toughness are handled in layer 7a
        CharacteristicDefiningAbility { 
            affects: Characteristic::PowerToughness,
            calculation: Box::new(|world, entity| {
                // We'll mock this for the test - cards in hand = 4
                let cards_in_hand = 4;
                (cards_in_hand, cards_in_hand)
            }),
        },
    )).id();
    
    // Process CDA to establish base power/toughness
    app.update();
    
    // Verify creature has power/toughness equal to cards in hand
    let creature_stats = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(creature_stats.power, 4);
    assert_eq!(creature_stats.toughness, 4);
    
    // Cast pump spell (+2/+2) - this applies in layer 7c
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 2,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted correctly
    let boosted_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(boosted_creature.power, 6);  // 4 + 2 = 6
    assert_eq!(boosted_creature.toughness, 6);  // 4 + 2 = 6
    
    // Discard a card - affects layer 7a CDA
    app.world.spawn(DiscardCardCommand {
        player: Entity::PLACEHOLDER,
        count: 1,
    });
    app.update();
    
    // Verify stats update correctly (CDA now sees 3 cards, then +2/+2 applies)
    let updated_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(updated_creature.power, 5);  // 3 + 2 = 5
    assert_eq!(updated_creature.toughness, 5);  // 3 + 2 = 5
}
```

## Test Case: Changing Controllers During Combat

### Test: Creature Changes Controller During Combat

```rust
#[test]
fn test_creature_changes_controller_during_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, spell_resolution_system, controller_system));
       
    let player1 = app.world.spawn(Player {}).id();
    let player2 = app.world.spawn(Player {}).id();
    
    // Create attacking creature controlled by player 1
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Attacking { defending: player2 },
        CreatureCard { controller: player1 },
    )).id();
    
    // Cast control-changing spell before damage
    app.world.spawn((
        ControlChangeSpell { 
            target: attacker,
            new_controller: player2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature changed controllers
    let creature_card = app.world.get::<CreatureCard>(attacker).unwrap();
    assert_eq!(creature_card.controller, player2);
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify creature is still attacking the same player
    assert!(app.world.get::<Attacking>(attacker).is_some());
    assert_eq!(app.world.get::<Attacking>(attacker).unwrap().defending, player2);
    
    // Verify no damage was dealt (can't attack yourself)
    let player2_health = app.world.get::<Health>(player2).unwrap();
    assert_eq!(player2_health.current, player2_health.maximum);
}
```

## Test Case: Triggered Abilities Affecting Combat

### Test: Attack Triggers Modifying Other Creatures

```rust
#[test]
fn test_attack_trigger_modifying_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_attackers_system, trigger_system, combat_damage_system));
       
    let player1 = app.world.spawn(Player {}).id();
    let player2 = app.world.spawn(Player {
        health: Health { current: 20, maximum: 20 },
    }).id();
    
    // Create creature with attack trigger
    let battle_leader = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard { controller: player1 },
        AttackTrigger { 
            effect: TriggerEffect::PumpCreatures { 
                target_filter: TargetFilter::Controlled { controller: player1 },
                power_bonus: 1,
                toughness_bonus: 1,
                duration: SpellDuration::EndOfTurn,
            }
        },
    )).id();
    
    // Create another creature that will benefit from the trigger
    let other_attacker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard { controller: player1 },
    )).id();
    
    // Declare attackers
    app.world.entity_mut(battle_leader).insert(Attacking { defending: player2 });
    app.world.entity_mut(other_attacker).insert(Attacking { defending: player2 });
    
    // Process attack triggers
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareAttackers);
    app.update();
    
    // Verify both creatures were boosted
    let battle_leader_stats = app.world.get::<Creature>(battle_leader).unwrap();
    assert_eq!(battle_leader_stats.power, 3);  // 2 + 1 = 3
    
    let other_attacker_stats = app.world.get::<Creature>(other_attacker).unwrap();
    assert_eq!(other_attacker_stats.power, 3);  // 2 + 1 = 3
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify damage with the boost
    let player2_health = app.world.get::<Health>(player2).unwrap();
    assert_eq!(player2_health.current, 14);  // 20 - 3 - 3 = 14
}
```

## Test Case: Damage Doubling and Prevention Interactions

### Test: Damage Doubling with Prevention Shield

```rust
#[test]
fn test_damage_doubling_with_prevention() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, replacement_effects_system));
       
    // Create attacker with damage doubling effect
    let attacker = app.world.spawn((
        Creature { power: 4, toughness: 4 },
        Attacking { defending: defender_entity },
        DamageReplacementEffect { 
            effect_type: ReplacementType::Double { 
                condition: DamageCondition::DealingCombatDamage,
            }
        },
    )).id();
    
    // Create defender with protection shield
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
        PreventDamage {
            amount: 5,
            source_filter: None,
        },
    )).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // The original 4 damage is doubled to 8, then 5 is prevented
    let defender_health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(defender_health.current, 17);  // 20 - (8 - 5) = 17
}
```

## Test Case: Indestructible in Combat

### Test: Indestructible Creature with Lethal Damage

```rust
#[test]
fn test_indestructible_with_lethal_damage() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, state_based_actions_system));
       
    // Create indestructible attacker
    let indestructible_attacker = app.world.spawn((
        Creature { power: 4, toughness: 4 },
        Attacking { defending: defender_entity },
        Health { current: 4, maximum: 4 },
        Indestructible {},
    )).id();
    
    // Create blocker with deathtouch
    let deathtouch_blocker = app.world.spawn((
        Creature { power: 1, toughness: 1 },
        Blocking { blocked_attackers: vec![indestructible_attacker] },
        Health { current: 1, maximum: 1 },
        Deathtouch {},
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Check state-based actions
    app.update();
    
    // Verify blocker died
    assert!(app.world.get::<Health>(deathtouch_blocker).is_none());
    
    // Verify indestructible attacker is damaged but alive
    let attacker_health = app.world.get::<Health>(indestructible_attacker).unwrap();
    assert_eq!(attacker_health.current, 3);  // 4 - 1 = 3
    assert!(app.world.entity(indestructible_attacker).contains::<Indestructible>());
    
    // Verify it can still be in further combats despite lethal deathtouch damage
    let attacker_health = app.world.get::<Health>(indestructible_attacker).unwrap();
    app.world.entity_mut(indestructible_attacker).insert(
        DamageReceived {
            amount: 10,
            source: deathtouch_blocker,
            is_combat_damage: true,
        }
    );
    
    // Process more damage
    app.update();
    
    // Check state-based actions
    app.update();
    
    // Verify indestructible creature is still alive despite lethal damage
    assert!(app.world.get::<Health>(indestructible_attacker).is_some());
}
```

## Test Case: Phasing and Pump Spell Interactions

### Test: Phasing Out After Pump Spell

```rust
#[test]
fn test_phasing_after_pump_spell() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (spell_resolution_system, phasing_system, end_of_turn_system));
       
    // Create creature
    let creature = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard {},
    )).id();
    
    // Cast pump spell
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 2,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(boosted_creature.power, 4);
    assert_eq!(boosted_creature.toughness, 4);
    
    // Phase out the creature
    app.world.entity_mut(creature).insert(PhasedOut {});
    
    // Process end of turn
    app.world.resource_mut::<TurnManager>().current_phase = Phase::End(EndPhaseStep::End);
    app.update();
    
    // The creature is phased out, so end of turn effects shouldn't affect it
    
    // Phase the creature back in
    app.world.entity_mut(creature).remove::<PhasedOut>();
    
    // Now the pump effect should still be active (since it didn't wear off while phased out)
    let still_boosted = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(still_boosted.power, 4);
    assert_eq!(still_boosted.toughness, 4);
    
    // Process another end of turn
    app.update();
    
    // Now the effect should expire
    let end_of_turn = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(end_of_turn.power, 2);
    assert_eq!(end_of_turn.toughness, 2);
}
```

## Performance Considerations for Edge Case Tests

1. **Efficient Interaction Processing**: Ensure complex interaction processing is optimized.

2. **Minimize Redundant Query Operations**: Structure queries to minimize redundant operations.

3. **Layer System Optimization**: Efficiently process layer-dependent effects in the correct order.

## Test Coverage Checklist

- [x] Multiple death triggers during combat
- [x] Damage replacement/redirection effects
- [x] Layer-dependent ability interactions
- [x] Controller change during combat
- [x] Attack triggers modifying other creatures
- [x] Damage doubling with prevention
- [x] Indestructible creatures with lethal damage
- [x] Phasing and pump spell interactions

## Additional Considerations

1. Tests should verify both immediate effects and downstream consequences
2. Multiple mechanics should be tested in combination to ensure correct interactions
3. Order-dependent effects should be tested with different sequencing
4. Extreme edge cases should be included to stress-test the system 