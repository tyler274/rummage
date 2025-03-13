# Phasing During Combat - Test Cases

## Overview

Phasing is a complex mechanic where permanents temporarily leave and re-enter the game without triggering "leaves the battlefield" or "enters the battlefield" effects. When a creature phases out during combat, several edge cases can occur that require special handling.

This document outlines test cases for phasing interactions during combat in our Commander engine.

## Test Case: Attacker Phases Out

### Test: Attacking Creature Phases Out During Declare Attackers

```rust
#[test]
fn test_attacker_phases_out_after_declare_attackers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_attackers_system, phasing_system));
       
    // Create attacker and defending player
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        Attacking { defending: defender_entity },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Phase out the attacker
    app.world.spawn(PhaseOutCommand { target: attacker });
    
    // Process phasing
    app.update();
    
    // Verify the creature is phased out but still marked as attacking
    assert!(app.world.get::<PhasedOut>(attacker).is_some());
    assert!(app.world.get::<Attacking>(attacker).is_some());
    
    // When combat damage is processed, it should not deal damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify no damage was dealt to defender
    let health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(health.current, 20);
}
```

### Test: Phased Out Attacker at End of Combat

```rust
#[test]
fn test_phased_out_attacker_at_end_of_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (end_of_combat_system, phasing_system));
       
    // Create phased out attacker
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        Attacking { defending: defender_entity },
        PhasedOut {},
    )).id();
    
    // Process end of combat
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::End);
    app.world.resource_mut::<CombatSystem>().end_of_combat.triggers_processed = true;
    app.update();
    
    // Verify attacking status is removed, even though creature is phased out
    assert!(!app.world.entity(attacker).contains::<Attacking>());
}
```

## Test Case: Blocker Phases Out

### Test: Blocking Creature Phases Out During Declare Blockers

```rust
#[test]
fn test_blocker_phases_out_after_declare_blockers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_blockers_system, phasing_system, combat_damage_system));
       
    // Create attacker, blocker, and player
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        Attacking { defending: defender_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2, },
        Blocking { blocked_attackers: vec![attacker] },
    )).id();
    
    let defender_entity = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
    )).id();
    
    // Phase out the blocker
    app.world.spawn(PhaseOutCommand { target: blocker });
    
    // Process phasing
    app.update();
    
    // Verify the creature is phased out but still marked as blocking
    assert!(app.world.get::<PhasedOut>(blocker).is_some());
    assert!(app.world.get::<Blocking>(blocker).is_some());
    
    // When combat damage is processed, attacker should deal damage to player
    // since phased out blocker cannot block
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify damage was dealt to defender (attacker was effectively unblocked)
    let health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(health.current, 17);
}
```

## Test Case: Phasing During Combat Damage

### Test: Creature Phases Out in Response to Damage

```rust
#[test]
fn test_creature_phases_out_in_response_to_damage() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, phasing_system));
       
    // Create attacker and blocker
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        Attacking { defending: defender_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2, },
        Blocking { blocked_attackers: vec![attacker] },
        Health { current: 2, maximum: 2 },
    )).id();
    
    // Simulate player responding to damage by phasing out the blocker
    // (in a real implementation, this would be triggered by priority passing)
    app.world.spawn(PhaseOutCommand { 
        target: blocker,
        trigger_condition: TriggerCondition::BeforeDamage,
    });
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify blocker phased out and did not receive damage
    assert!(app.world.get::<PhasedOut>(blocker).is_some());
    let health = app.world.get::<Health>(blocker).unwrap();
    assert_eq!(health.current, 2); // Health unchanged
    
    // Verify attacker did not receive damage either
    let attacker_health = app.world.get::<Health>(attacker).unwrap();
    assert_eq!(attacker_health.current, attacker_health.maximum);
}
```

## Test Case: Phasing In During Combat

### Test: Creature Phases In During Combat

```rust
#[test]
fn test_creature_phases_in_during_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (phasing_system, combat_system));
       
    // Create phased out creature
    let creature = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        PhasedOut {},
    )).id();
    
    // Set up phase in trigger for declare blockers step
    app.world.spawn(PhaseInCommand { 
        target: creature,
        trigger_phase: Phase::Combat(CombatStep::DeclareBlockers),
    });
    
    // Advance to declare blockers
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareBlockers);
    app.update();
    
    // Verify creature phased in
    assert!(app.world.get::<PhasedOut>(creature).is_none());
    
    // Verify creature cannot be declared as a blocker this combat
    // (creatures that phase in are treated as though they just entered the battlefield)
    let can_block = app.world.get_resource::<CombatSystem>().unwrap()
        .can_block(creature, attacker_entity);
    assert!(!can_block);
}
```

## Test Case: Phasing with Auras and Equipment

### Test: Equipped Creature Phases Out

```rust
#[test]
fn test_equipped_creature_phases_out_during_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (phasing_system, combat_damage_system));
       
    // Create creature with equipment
    let creature = app.world.spawn((
        Creature { power: 2, toughness: 2, },
        Attacking { defending: defender_entity },
    )).id();
    
    let equipment = app.world.spawn((
        Equipment { 
            equipped_to: creature,
            power_bonus: 2,
            toughness_bonus: 0,
        },
        Attached { attached_to: creature },
    )).id();
    
    // Phase out the creature
    app.world.spawn(PhaseOutCommand { target: creature });
    app.update();
    
    // Verify creature and equipment are both phased out
    assert!(app.world.get::<PhasedOut>(creature).is_some());
    assert!(app.world.get::<PhasedOut>(equipment).is_some());
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify no damage was dealt
    let health = app.world.get::<Health>(defender_entity).unwrap();
    assert_eq!(health.current, 20);
    
    // Phase in during a later turn
    app.world.spawn(PhaseInCommand { 
        target: creature,
        trigger_phase: Phase::Main(MainPhaseStep::First),
    });
    
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Main(MainPhaseStep::First);
    app.update();
    
    // Verify both creature and equipment phased in and are still attached
    assert!(app.world.get::<PhasedOut>(creature).is_none());
    assert!(app.world.get::<PhasedOut>(equipment).is_none());
    assert_eq!(app.world.get::<Equipment>(equipment).unwrap().equipped_to, creature);
    assert_eq!(app.world.get::<Attached>(equipment).unwrap().attached_to, creature);
}
```

## Integration with Turn Structure

### Test: Phasing and Turn Phases

```rust
#[test]
fn test_phasing_respects_turn_structure() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (phasing_system, turn_system));
       
    // Create creature that phases out
    let creature = app.world.spawn((
        Creature { power: 3, toughness: 3, },
    )).id();
    
    // Phase out every other turn
    app.world.spawn(RecurringPhaseCommand { 
        target: creature,
        phase_out_on: TurnCondition::ActivePlayerTurn,
        phase_in_on: TurnCondition::NonActivePlayerTurn,
    });
    
    // Set active player
    app.world.resource_mut::<TurnManager>().active_player = player1_entity;
    app.update();
    
    // Verify creature is phased out on active player's turn
    assert!(app.world.get::<PhasedOut>(creature).is_some());
    
    // Change active player
    app.world.resource_mut::<TurnManager>().active_player = player2_entity;
    app.update();
    
    // Verify creature phases in on non-active player's turn
    assert!(app.world.get::<PhasedOut>(creature).is_none());
}
```

## Performance Considerations for Phasing Tests

1. **Batch Phasing Operations**: Group phasing operations to minimize entity access.

2. **Efficient Phase Tracking**: Use a more efficient system for tracking phased entities.

3. **Minimize Query Iterations**: Structure queries to minimize iterations through phased entities.

## Test Coverage Checklist

- [x] Attackers phasing out during declare attackers
- [x] Attackers phasing out during combat damage
- [x] Blockers phasing out during declare blockers
- [x] Blockers phasing out during combat damage
- [x] Creatures phasing in during combat
- [x] End of combat cleanup with phased entities
- [x] Phasing with attached permanents (equipment, auras)
- [x] Turn structure integration with phasing

## Additional Edge Cases to Consider

1. Multiple creatures phasing in/out simultaneously
2. Phasing commander in and out (commander damage tracking)
3. Phasing and "enters the battlefield" replacement effects
4. Phasing and state-based actions 