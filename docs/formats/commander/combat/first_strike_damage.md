# First Strike Damage

## Overview

The First Strike Damage step is a conditional sub-step within the Combat Damage step that occurs when at least one creature with first strike or double strike is involved in combat. During this step, only creatures with first strike or double strike deal combat damage, while other creatures wait until the regular Combat Damage step.

This document outlines the implementation details, edge cases, and testing strategies for the First Strike Damage step in our Commander engine.

## Core Concepts

### First Strike Damage Flow

The First Strike Damage step follows this general flow:

1. Check if any attacking or blocking creatures have first strike or double strike
2. If yes, create a dedicated First Strike Damage step before the regular Combat Damage step
3. Only creatures with first strike or double strike assign and deal damage in this step
4. State-based actions are checked
5. Triggers from first strike damage are put on the stack
6. Priority passes to players in turn order
7. Once all players pass priority and the stack is empty, the game proceeds to the regular Combat Damage step

### First Strike vs. Double Strike

The key difference between first strike and double strike:

- Creatures with **first strike** deal damage only during the First Strike Damage step
- Creatures with **double strike** deal damage in both the First Strike Damage step and the regular Combat Damage step

## Implementation Design

### Data Structures

```rust
// Components for first strike and double strike abilities
#[derive(Component)]
struct FirstStrike;

#[derive(Component)]
struct DoubleStrike;

// System resource for tracking the first strike step
struct FirstStrikeDamageSystem {
    triggers_processed: bool,
}
```

### First Strike Checking System

```rust
fn check_for_first_strike_step(
    mut turn_manager: ResMut<TurnManager>,
    first_strike_query: Query<Entity, Or<(With<FirstStrike>, With<DoubleStrike>)>>,
    combat_participants: Query<Entity, Or<(With<Attacking>, With<Blocking>)>>
) {
    // Only check when transitioning from Declare Blockers to Combat Damage
    if !matches!(turn_manager.current_phase, Phase::Combat(CombatStep::DeclareBlockers)) {
        return;
    }
    
    // Check if any creature in combat has first strike or double strike
    let has_first_strike = combat_participants.iter().any(|entity| {
        first_strike_query.contains(entity)
    });
    
    // If there are creatures with first strike, we'll use the FirstStrike step
    if has_first_strike {
        turn_manager.next_phase = Some(Phase::Combat(CombatStep::FirstStrike));
    } else {
        turn_manager.next_phase = Some(Phase::Combat(CombatStep::CombatDamage));
    }
}
```

### First Strike Damage System

```rust
fn first_strike_damage_system(
    mut commands: Commands,
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    creature_query: Query<(Entity, &Creature, &Health, Option<&FirstStrike>, Option<&DoubleStrike>)>,
    attacker_query: Query<(Entity, &Attacking)>,
    blocker_query: Query<(Entity, &Blocking)>,
    player_query: Query<(Entity, &Player, &Health)>,
    mut commander_damage: Query<&mut CommanderDamage>,
    // Other system parameters
) {
    // Only run during first strike damage step
    if !matches!(turn_manager.current_phase, Phase::Combat(CombatStep::FirstStrike)) {
        return;
    }
    
    // Find creatures with first strike or double strike that are in combat
    let first_strike_creatures = creature_query
        .iter()
        .filter(|(entity, _, _, first_strike, double_strike)| {
            (first_strike.is_some() || double_strike.is_some()) &&
            (attacker_query.contains(*entity) || blocker_query.contains(*entity))
        })
        .collect::<Vec<_>>();
        
    // Assign and deal damage
    for (entity, creature, _, _, _) in first_strike_creatures {
        // Determine damage amount and recipients
        let power = creature.power;
        
        if let Ok((_, attacking)) = attacker_query.get(entity) {
            // Attacking creature deals damage
            assign_attacker_damage(
                entity, 
                power, 
                attacking.defending, 
                &blocker_query, 
                &mut combat_system
            );
        } else if let Ok((_, blocking)) = blocker_query.get(entity) {
            // Blocking creature deals damage
            assign_blocker_damage(
                entity, 
                power, 
                &blocking.blocked_attackers, 
                &mut combat_system
            );
        }
    }
    
    // Deal all assigned damage simultaneously
    process_damage_assignments(&mut commands, &combat_system, &mut commander_damage);
    
    // Check for state-based actions
    check_state_based_actions(&mut commands, &player_query, &creature_query);
    
    // Record that first strike damage has been processed
    combat_system.first_strike_round_completed = true;
}
```

## Special Cases and Edge Scenarios

### Gaining/Losing First Strike During Combat

Creatures might gain or lose first strike abilities during combat:

```rust
fn handle_first_strike_changes(
    mut first_strike_system: ResMut<FirstStrikeDamageSystem>,
    turn_manager: Res<TurnManager>,
    added_first_strike: Query<Entity, Added<FirstStrike>>,
    removed_first_strike: Query<Entity, Without<FirstStrike>>,
    combat_participants: Query<Entity, Or<(With<Attacking>, With<Blocking>)>>,
    was_in_combat: Local<HashSet<Entity>>
) {
    // Check if any creature in combat gained or lost first strike
    let gained_first_strike = added_first_strike.iter()
        .filter(|&entity| combat_participants.contains(entity))
        .count() > 0;
        
    let lost_first_strike = removed_first_strike.iter()
        .filter(|&entity| was_in_combat.contains(&entity))
        .count() > 0;
        
    // If changes occurred during declare blockers step, we might need to adjust
    // whether a first strike step will occur
    if matches!(turn_manager.current_phase, Phase::Combat(CombatStep::DeclareBlockers)) {
        if gained_first_strike {
            first_strike_system.needed = true;
        }
    }
    
    // Update our tracking of combat participants
    was_in_combat.clear();
    for entity in combat_participants.iter() {
        was_in_combat.insert(entity);
    }
}
```

### Removing Creatures During First Strike

If a creature is destroyed by first strike damage, it doesn't deal regular combat damage:

```rust
fn track_destroyed_by_first_strike(
    combat_system: Res<CombatSystem>,
    mut destroyed_entities: ResMut<DestroyedEntities>,
    damage_events: Query<&DamageEvent>,
    creature_query: Query<&Health>
) {
    // Only track during first strike damage
    if !combat_system.first_strike_round_completed {
        return;
    }
    
    // Find creatures that received lethal damage during first strike
    for damage_event in damage_events.iter() {
        if let Ok(health) = creature_query.get(damage_event.target) {
            if health.current <= 0 {
                destroyed_entities.insert(damage_event.target);
            }
        }
    }
}
```

### Fast Effect Windows

Players get priority after first strike damage, allowing them to cast spells before regular damage:

```rust
fn handle_first_strike_priority(
    mut turn_manager: ResMut<TurnManager>,
    stack: Res<Stack>,
    combat_system: Res<CombatSystem>
) {
    // Only handle during first strike step
    if !matches!(turn_manager.current_phase, Phase::Combat(CombatStep::FirstStrike)) {
        return;
    }
    
    // If first strike damage has been dealt and the stack is empty,
    // we can proceed to regular combat damage
    if combat_system.first_strike_round_completed && stack.is_empty() {
        turn_manager.advance_phase();
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_first_strike_damage() {
    // Set up test world
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, first_strike_damage_system);
       
    // Create attacker with first strike
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Attacking { defending: player_entity },
        FirstStrike,
    )).id();
    
    // Create blocker without first strike
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Blocking { blocked_attackers: vec![attacker] },
        Health { current: 2, maximum: 2 },
    )).id();
    
    // Process first strike damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::FirstStrike);
    app.update();
    
    // Verify blocker received damage and was destroyed
    let health = app.world.get::<Health>(blocker).unwrap();
    assert_eq!(health.current, 0); // Received 3 damage from first strike attacker
    
    // Verify no damage was dealt to attacker yet (since blocker doesn't have first strike)
    let attacker_health = app.world.get::<Health>(attacker).unwrap();
    assert_eq!(attacker_health.current, attacker_health.maximum);
}

#[test]
fn test_double_strike_damage() {
    // Test that double strike creatures deal damage in both steps
    // ...
}

#[test]
fn test_first_strike_detection() {
    // Test that the system correctly detects when a first strike step is needed
    // ...
}
```

### Integration Tests

```rust
#[test]
fn test_first_strike_vs_regular_damage() {
    // Test complete sequence with both first strike and regular damage
    // ...
}

#[test]
fn test_gaining_first_strike_mid_combat() {
    // Test gaining first strike during combat
    // ...
}

#[test]
fn test_losing_first_strike_mid_combat() {
    // Test losing first strike during combat
    // ...
}
```

### Edge Case Tests

```rust
#[test]
fn test_first_strike_with_triggered_abilities() {
    // Test interaction between first strike damage and triggered abilities
    // ...
}

#[test]
fn test_first_strike_with_damage_prevention() {
    // Test first strike damage with damage prevention effects
    // ...
}

#[test]
fn test_first_strike_with_damage_redirection() {
    // Test first strike damage with redirection effects
    // ...
}
```

## Performance Considerations

1. **Conditional Step Creation**: Only create the First Strike Damage step when needed.

2. **Efficient Entity Filtering**: Optimize filtering of entities with first strike/double strike.

3. **Damage Assignment Optimization**: Reuse damage assignment logic between first strike and regular damage steps.

4. **State Tracking**: Efficiently track state between first strike and regular damage steps.

## Conclusion

The First Strike Damage step is a specialized combat sub-step that adds strategic depth to the combat system. By correctly implementing first strike and double strike mechanics, we ensure that these powerful combat abilities function as expected according to the Magic: The Gathering rules. The implementation must handle all edge cases, including mid-combat changes to first strike status, while maintaining good performance and seamless integration with the rest of the combat system. 