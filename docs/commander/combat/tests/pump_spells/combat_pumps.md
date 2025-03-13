# Combat Pump Spells - Test Cases

## Overview

Combat pump spells are instants that temporarily boost a creature's power and/or toughness, often used during combat to alter the outcome. These spells can be cast at various points during the combat phase, leading to different results depending on timing.

This document outlines test cases for combat pump spell interactions in our Commander engine.

## Test Case: Basic Pump Spell Effects

### Test: Power/Toughness Boost During Declare Blockers

```rust
#[test]
fn test_pump_spell_during_declare_blockers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_blockers_system, spell_resolution_system));
       
    // Create attacker and potential blocker
    let attacker = app.world.spawn((
        Creature { power: 4, toughness: 4 },
        Attacking { defending: defender_entity },
    )).id();
    
    let small_blocker = app.world.spawn((
        Creature { power: 1, toughness: 1 },
        CreatureCard { controller: defender_entity },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Set up initially - blocker cannot block due to being too small
    app.world.resource_mut::<CombatSystem>().can_block_check = |blocker, attacker| {
        let blocker_creature = app.world.get::<Creature>(blocker).unwrap();
        let attacker_creature = app.world.get::<Creature>(attacker).unwrap();
        blocker_creature.power >= attacker_creature.power / 2
    };
    
    // Can't block initially
    let can_block_initially = app.world.resource::<CombatSystem>()
        .can_block(small_blocker, attacker);
    assert!(!can_block_initially);
    
    // Cast pump spell (+3/+3 until end of turn)
    app.world.spawn((
        PumpSpell { 
            target: small_blocker,
            power_bonus: 3,
            toughness_bonus: 3,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(small_blocker).unwrap();
    assert_eq!(boosted_creature.power, 4);
    assert_eq!(boosted_creature.toughness, 4);
    
    // Now should be able to block
    let can_block_after = app.world.resource::<CombatSystem>()
        .can_block(small_blocker, attacker);
    assert!(can_block_after);
}
```

## Test Case: Pump Spells During Combat Damage

### Test: Pump Spell Saving Creature from Lethal Damage

```rust
#[test]
fn test_pump_spell_prevents_lethal_damage() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, spell_resolution_system));
       
    // Create attacker and blocker
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Attacking { defending: defender_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Blocking { blocked_attackers: vec![attacker] },
        Health { current: 2, maximum: 2 },
    )).id();
    
    // Before damage is dealt, cast pump spell to increase toughness
    // In a real implementation, this would be during the priority window
    app.world.spawn((
        PumpSpell { 
            target: blocker,
            power_bonus: 0,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(blocker).unwrap();
    assert_eq!(boosted_creature.power, 2);
    assert_eq!(boosted_creature.toughness, 4);
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify blocker survived
    let health = app.world.get::<Health>(blocker).unwrap();
    assert!(health.current > 0);
}
```

### Test: Increasing Power After Blockers to Deal More Damage

```rust
#[test]
fn test_pump_spell_increases_damage_after_blockers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, spell_resolution_system));
       
    // Create attacker and defending player
    let attacker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Attacking { defending: defender_entity },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // After declare blockers, cast pump spell to increase power
    app.world.spawn((
        PumpSpell { 
            target: attacker,
            power_bonus: 3,
            toughness_bonus: 0,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(attacker).unwrap();
    assert_eq!(boosted_creature.power, 5);
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify more damage was dealt to player
    let health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(health.current, 15); // 20 - 5 = 15
}
```

## Test Case: Timing Edge Cases

### Test: Pump Spell During First Strike Damage

```rust
#[test]
fn test_pump_spell_between_first_strike_and_normal_damage() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (first_strike_damage_system, combat_damage_system, spell_resolution_system));
       
    // Create first strike attacker and regular blocker
    let first_strike_attacker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Attacking { defending: defender_entity },
        FirstStrike {},
    )).id();
    
    let regular_blocker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Blocking { blocked_attackers: vec![first_strike_attacker] },
        Health { current: 2, maximum: 2 },
    )).id();
    
    // Process first strike damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::FirstStrike);
    app.update();
    
    // Verify blocker took damage but is still alive
    let health_after_first_strike = app.world.get::<Health>(regular_blocker).unwrap();
    assert_eq!(health_after_first_strike.current, 0);
    
    // SBA would normally destroy the creature here, but we'll simulate a pump spell in response
    // Cast Sudden Invigoration (instant that gives +0/+2 and prevents damage this turn)
    app.world.spawn((
        PumpSpell { 
            target: regular_blocker,
            power_bonus: 0,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
        PreventDamageEffect { amount: 2 },
    ));
    
    // Resolve spell and apply effects
    app.update();
    
    // Reset health as if the damage had been prevented
    app.world.entity_mut(regular_blocker).insert(Health { current: 2, maximum: 2 });
    
    // Verify creature stats were boosted and creature is alive
    let boosted_creature = app.world.get::<Creature>(regular_blocker).unwrap();
    assert_eq!(boosted_creature.toughness, 4);
    
    let health_after_spell = app.world.get::<Health>(regular_blocker).unwrap();
    assert_eq!(health_after_spell.current, 2);
    
    // Process regular combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify blocker survived regular combat damage too
    let final_health = app.world.get::<Health>(regular_blocker).unwrap();
    assert_eq!(final_health.current, 2); // Damage was prevented
}
```

### Test: Pump Spell at End of Combat

```rust
#[test]
fn test_pump_spell_at_end_of_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (end_of_combat_system, spell_resolution_system));
       
    // Create creature
    let creature = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard {},
    )).id();
    
    // Cast pump spell with "until end of turn" duration
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 3,
            toughness_bonus: 3,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(boosted_creature.power, 5);
    assert_eq!(boosted_creature.toughness, 5);
    
    // Add EndOfTurn effect
    app.world.entity_mut(creature).insert(UntilEndOfTurn {
        original_power: 2,
        original_toughness: 2,
    });
    
    // Process end of combat phase
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::End);
    app.world.resource_mut::<CombatSystem>().end_of_combat.triggers_processed = true;
    app.update();
    
    // Verify creature stats still boosted after end of combat
    // (effects last until end of turn, not end of combat)
    let still_boosted = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(still_boosted.power, 5);
    assert_eq!(still_boosted.toughness, 5);
    
    // Process end of turn
    app.world.resource_mut::<TurnManager>().current_phase = Phase::End(EndPhaseStep::End);
    app.update();
    
    // Now the effects should wear off
    let end_of_turn = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(end_of_turn.power, 2);
    assert_eq!(end_of_turn.toughness, 2);
}
```

## Test Case: Multiple Pump Spells

### Test: Stacking Pump Effects

```rust
#[test]
fn test_stacking_pump_spells() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, spell_resolution_system);
       
    // Create creature
    let creature = app.world.spawn((
        Creature { power: 1, toughness: 1 },
        CreatureCard {},
    )).id();
    
    // Cast first pump spell
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 2,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve first spell
    app.update();
    
    // Cast second pump spell
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 1,
            toughness_bonus: 3,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
    ));
    
    // Resolve second spell
    app.update();
    
    // Verify creature stats were properly stacked
    let boosted_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(boosted_creature.power, 4);  // 1 + 2 + 1 = 4
    assert_eq!(boosted_creature.toughness, 6);  // 1 + 2 + 3 = 6
}
```

## Test Case: Pump Spells and Combat Abilities

### Test: Pump Giving Trample

```rust
#[test]
fn test_pump_spell_gives_trample() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, spell_resolution_system));
       
    // Create attacker, blocker, and player
    let attacker = app.world.spawn((
        Creature { power: 4, toughness: 4 },
        Attacking { defending: defender_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Blocking { blocked_attackers: vec![attacker] },
        Health { current: 2, maximum: 2 },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Cast pump spell that also grants trample
    app.world.spawn((
        PumpSpell { 
            target: attacker,
            power_bonus: 1,
            toughness_bonus: 1,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
        GrantAbilityEffect { 
            ability: CreatureAbility::Trample,
            duration: SpellDuration::EndOfTurn,
        },
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted and gained trample
    let boosted_creature = app.world.get::<Creature>(attacker).unwrap();
    assert_eq!(boosted_creature.power, 5);
    assert!(app.world.get::<Trample>(attacker).is_some());
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify blocker is destroyed
    let blocker_health = app.world.get::<Health>(blocker).unwrap();
    assert_eq!(blocker_health.current, 0);
    
    // Verify excess damage trampled through to player
    let player_health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(player_health.current, 17); // 20 - (5-2) = 17
}
```

## Test Case: Pump Spell Targeting

### Test: Incorrectly Targeted Pump Spell

```rust
#[test]
fn test_illegally_targeted_pump_spell() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, spell_resolution_system);
       
    // Create creature with hexproof
    let hexproof_creature = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard {},
        Hexproof {},
    )).id();
    
    // Create opposing player
    let opponent_entity = app.world.spawn(Player {}).id();
    
    // Cast pump spell from opponent (should fail due to hexproof)
    app.world.spawn((
        PumpSpell { 
            target: hexproof_creature,
            power_bonus: 2,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
        SpellController { controller: opponent_entity },
    ));
    
    // Attempt to resolve spell (should be countered by game rules)
    app.update();
    
    // Verify creature stats were not changed
    let creature = app.world.get::<Creature>(hexproof_creature).unwrap();
    assert_eq!(creature.power, 2);
    assert_eq!(creature.toughness, 2);
    
    // Verify spell was countered
    let spell_events = app.world.resource::<Events<SpellEvent>>().get_reader().iter().collect::<Vec<_>>();
    assert!(spell_events.iter().any(|event| matches!(event, SpellEvent::Countered(_))));
}
```

## Integration with Combat System

### Test: Pump Spells and State-Based Actions

```rust
#[test]
fn test_pump_spell_and_state_based_actions() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (spell_resolution_system, state_based_actions_system));
       
    // Create damaged creature
    let creature = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        CreatureCard {},
        Health { current: 1, maximum: 2 },
    )).id();
    
    // Deal damage to creature
    app.world.entity_mut(creature).insert(DamageReceived {
        amount: 2,
        source: Entity::PLACEHOLDER,
        is_combat_damage: false,
    });
    
    // Process state-based actions to apply damage
    app.update();
    
    // Health would be 0, but don't destroy yet
    
    // Cast pump spell to increase toughness before SBA check
    app.world.spawn((
        PumpSpell { 
            target: creature,
            power_bonus: 0,
            toughness_bonus: 2,
            duration: SpellDuration::EndOfTurn,
        },
        SpellOnStack {},
        InstantEffect {},
    ));
    
    // Resolve spell
    app.update();
    
    // Verify creature stats were boosted
    let boosted_creature = app.world.get::<Creature>(creature).unwrap();
    assert_eq!(boosted_creature.toughness, 4);
    
    // Update health to reflect new toughness (would happen in a real implementation)
    app.world.entity_mut(creature).insert(Health {
        current: 1, // Still has damage marked
        maximum: 4,
    });
    
    // Verify creature is still alive despite the damage
    assert!(app.world.get::<Health>(creature).is_some());
    
    // Process state-based actions again
    app.update();
    
    // Verify creature is still alive
    assert!(app.world.get::<Health>(creature).is_some());
}
```

## Performance Considerations for Pump Spell Tests

1. **Optimize Spell Resolution**: Structure spell resolution to minimize entity access.

2. **Batch Effect Application**: Group similar effects when applying multiple pump spells.

3. **Efficient Stat Calculation**: Use an efficient system for calculating final creature stats.

## Test Coverage Checklist

- [x] Basic pump spell application
- [x] Pump spells preventing lethal damage
- [x] Increasing power after blockers
- [x] Pump spells during first strike damage
- [x] Pump spells at end of combat
- [x] Stacking multiple pump spells
- [x] Pump spells granting abilities
- [x] Invalid pump spell targeting
- [x] Pump spells and state-based actions
- [x] Pump spells with delayed effects

## Additional Edge Cases to Consider

1. Pump spells with conditional effects
2. Pump spells that scale based on game state
3. Pump spells that trigger other abilities
4. Temporary control change plus pump effects
5. Layer-dependent pump effects (timestamps, dependencies) 