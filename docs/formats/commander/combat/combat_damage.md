# Combat Damage

## Overview

The Combat Damage step is the critical phase of combat where damage is assigned and dealt between attacking and blocking creatures, as well as to players and planeswalkers. In Commander, this step has additional significance due to the presence of Commander damage tracking and the multiplayer nature of the format.

This document outlines the implementation details, edge cases, and testing strategies for the Combat Damage step in our Commander engine.

## Core Concepts

### Combat Damage Flow

The Combat Damage step follows this general flow:

1. First, the active player assigns combat damage from their attacking creatures
2. Then, each defending player assigns combat damage from their blocking creatures
3. All damage is dealt simultaneously
4. State-based actions are checked, including:
   - Creatures with lethal damage are destroyed
   - Players with 0 or less life lose the game
   - Players with 21+ commander damage from a single commander lose the game
5. Combat damage triggers are placed on the stack
6. Priority passes to players in turn order

### First Strike and Double Strike

Combat damage may occur in two sub-steps when creatures with first strike or double strike are involved:

1. **First Strike Combat Damage** - Damage from creatures with first strike or double strike
2. **Regular Combat Damage** - Damage from all other creatures and second instance of damage from double strike creatures

## Implementation Design

### Data Structures

```rust
// Represents the damage assignment for a specific attacker or blocker
struct DamageAssignment {
    source: Entity,         // The entity dealing damage
    target: Entity,         // The entity receiving damage
    amount: u32,            // Amount of damage
    is_commander_damage: bool, // Whether this is commander damage
    is_combat_damage: bool, // Always true for combat damage step
}

// Component for tracking assigned damage
#[derive(Component)]
struct AssignedDamage {
    assignments: Vec<DamageAssignment>,
}

// System resource for managing the combat damage step
struct CombatDamageSystem {
    first_strike_round_completed: bool,
    damage_assignments: HashMap<Entity, Vec<DamageAssignment>>,
}
```

### Combat Damage System

```rust
fn combat_damage_system(
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
    // Check if we need to process first strike damage
    let current_step = match turn_manager.current_phase {
        Phase::Combat(CombatStep::FirstStrike) => CombatStep::FirstStrike,
        Phase::Combat(CombatStep::CombatDamage) => CombatStep::CombatDamage,
        _ => return, // Not in a combat damage step
    };
    
    // Determine which creatures deal damage in this step
    let (first_strike_creatures, regular_creatures) = creature_query
        .iter()
        .partition(|(_, _, _, first_strike, double_strike)| 
            first_strike.is_some() || double_strike.is_some()
        );
        
    // Process creatures that deal damage in this step
    let applicable_creatures = match current_step {
        CombatStep::FirstStrike => &first_strike_creatures,
        CombatStep::CombatDamage => {
            // In regular combat damage, double strike creatures deal damage again,
            // and creatures without first/double strike deal damage for the first time
            let mut creatures = Vec::new();
            creatures.extend(regular_creatures);
            creatures.extend(
                first_strike_creatures
                    .iter()
                    .filter(|(_, _, _, _, double_strike)| double_strike.is_some())
            );
            &creatures
        }
        _ => return,
    };
    
    // Assign and deal damage
    for (entity, creature, health, _, _) in applicable_creatures {
        // Determine damage amount and recipients
        let power = creature.power;
        
        if let Ok((_, attacking)) = attacker_query.get(*entity) {
            // Attacking creature deals damage
            assign_attacker_damage(
                *entity, 
                power, 
                attacking.defending, 
                &blocker_query, 
                &mut combat_system
            );
        } else if let Ok((_, blocking)) = blocker_query.get(*entity) {
            // Blocking creature deals damage
            assign_blocker_damage(
                *entity, 
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
    
    // If this was the first strike step, we'll continue to regular damage
    // If this was regular combat damage, we'll proceed to end of combat
    if current_step == CombatStep::FirstStrike {
        combat_system.first_strike_round_completed = true;
    } else {
        combat_system.first_strike_round_completed = false;
        // Clean up damage assignments
        combat_system.damage_assignments.clear();
    }
}
```

### Damage Assignment Functions

```rust
fn assign_attacker_damage(
    attacker: Entity,
    power: u32,
    defending: Entity,
    blocker_query: &Query<(Entity, &Blocking)>,
    combat_system: &mut ResMut<CombatSystem>
) {
    // Find blockers for this attacker
    let blockers = blocker_query
        .iter()
        .filter(|(_, blocking)| blocking.blocked_attackers.contains(&attacker))
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();
    
    if blockers.is_empty() {
        // Unblocked attacker - deal damage to player or planeswalker
        combat_system.damage_assignments.entry(attacker).or_default().push(
            DamageAssignment {
                source: attacker,
                target: defending,
                amount: power,
                is_commander_damage: true, // Check if attacker is a commander
                is_combat_damage: true,
            }
        );
    } else {
        // Attacker is blocked - distribute damage among blockers
        // (In a real implementation, this would handle player choices for damage assignment)
        
        // Simplified version: divide damage evenly
        let damage_per_blocker = power / blockers.len() as u32;
        let remainder = power % blockers.len() as u32;
        
        for (i, blocker) in blockers.iter().enumerate() {
            let damage = damage_per_blocker + if i < remainder as usize { 1 } else { 0 };
            
            combat_system.damage_assignments.entry(attacker).or_default().push(
                DamageAssignment {
                    source: attacker,
                    target: *blocker,
                    amount: damage,
                    is_commander_damage: false, // Commander damage only applies to players
                    is_combat_damage: true,
                }
            );
        }
    }
}

fn assign_blocker_damage(
    blocker: Entity,
    power: u32,
    blocked_attackers: &[Entity],
    combat_system: &mut ResMut<CombatSystem>
) {
    // Simplified version: divide damage evenly among attackers
    // (In a real implementation, this would handle player choices for damage assignment)
    
    let damage_per_attacker = power / blocked_attackers.len() as u32;
    let remainder = power % blocked_attackers.len() as u32;
    
    for (i, attacker) in blocked_attackers.iter().enumerate() {
        let damage = damage_per_attacker + if i < remainder as usize { 1 } else { 0 };
        
        combat_system.damage_assignments.entry(blocker).or_default().push(
            DamageAssignment {
                source: blocker,
                target: *attacker,
                amount: damage,
                is_commander_damage: false,
                is_combat_damage: true,
            }
        );
    }
}

fn process_damage_assignments(
    commands: &mut Commands,
    combat_system: &CombatSystem,
    commander_damage: &mut Query<&mut CommanderDamage>
) {
    // Process all damage assignments
    for (source, assignments) in &combat_system.damage_assignments {
        for assignment in assignments {
            // Deal damage to target
            commands.entity(assignment.target).insert(
                DamageReceived {
                    amount: assignment.amount,
                    source: assignment.source,
                    is_combat_damage: true,
                }
            );
            
            // If this is commander damage, update commander damage tracker
            if assignment.is_commander_damage {
                if let Ok(mut cmd_damage) = commander_damage.get_mut(assignment.target) {
                    cmd_damage.add_damage(assignment.source, assignment.amount);
                }
            }
            
            // Add damage event for triggers
            commands.spawn(DamageEvent {
                source: assignment.source,
                target: assignment.target,
                amount: assignment.amount,
                is_combat_damage: true,
            });
        }
    }
}
```

## Special Cases and Edge Scenarios

### Trample

When an attacking creature with trample is blocked, excess damage is dealt to the defending player:

```rust
fn assign_attacker_damage_with_trample(
    attacker: Entity,
    power: u32,
    defending: Entity,
    blocker_query: &Query<(Entity, &Blocking, &Health)>,
    trample_query: &Query<&Trample>,
    combat_system: &mut ResMut<CombatSystem>
) {
    // Find blockers for this attacker
    let blockers = blocker_query
        .iter()
        .filter(|(_, blocking, _)| blocking.blocked_attackers.contains(&attacker))
        .collect::<Vec<_>>();
    
    if blockers.is_empty() {
        // Handle unblocked attacker as normal
        // ...
    } else if trample_query.get(attacker).is_ok() {
        // Attacker has trample - assign lethal damage to each blocker
        let mut remaining_damage = power;
        
        for (blocker, _, health) in &blockers {
            let lethal_damage = health.current.min(remaining_damage);
            remaining_damage -= lethal_damage;
            
            // Assign lethal damage to blocker
            combat_system.damage_assignments.entry(attacker).or_default().push(
                DamageAssignment {
                    source: attacker,
                    target: *blocker,
                    amount: lethal_damage,
                    is_commander_damage: false,
                    is_combat_damage: true,
                }
            );
        }
        
        // Assign remaining damage to player
        if remaining_damage > 0 {
            combat_system.damage_assignments.entry(attacker).or_default().push(
                DamageAssignment {
                    source: attacker,
                    target: defending,
                    amount: remaining_damage,
                    is_commander_damage: true, // Check if attacker is a commander
                    is_combat_damage: true,
                }
            );
        }
    } else {
        // Handle normal blocked creature (no trample)
        // ...
    }
}
```

### Deathtouch

Creatures with deathtouch need to assign only 1 damage to be considered lethal:

```rust
fn is_lethal_damage(
    amount: u32,
    source: Entity,
    deathtouch_query: &Query<&Deathtouch>
) -> bool {
    if amount > 0 && deathtouch_query.get(source).is_ok() {
        return true;
    }
    amount > 0
}
```

### Damage Prevention and Replacement

Damage can be prevented or modified by various effects:

```rust
fn apply_damage_prevention_effects(
    assignment: &mut DamageAssignment,
    prevention_query: &Query<&PreventDamage>
) {
    if let Ok(prevent) = prevention_query.get(assignment.target) {
        let prevented = prevent.amount.min(assignment.amount);
        assignment.amount -= prevented;
    }
}
```

### Damage Redirection

Some effects can redirect damage to different entities:

```rust
fn apply_damage_redirection(
    assignment: &mut DamageAssignment,
    redirection_query: &Query<&RedirectDamage>
) {
    if let Ok(redirect) = redirection_query.get(assignment.target) {
        // Change the target of the damage
        assignment.target = redirect.new_target;
        // Adjust commander damage flag if needed
        assignment.is_commander_damage = is_commander_entity(redirect.new_target);
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_basic_combat_damage() {
    // Set up test world
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, combat_damage_system);
       
    // Create attacker and defender
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3, },
        Attacking { defending: player_entity },
    )).id();
    
    let defender = app.world.spawn((
        Player {},
        Health { current: 20, maximum: 20 },
        CommanderDamage::default(),
    )).id();
    
    // Process combat damage
    app.update();
    
    // Verify damage was dealt
    let health = app.world.get::<Health>(defender).unwrap();
    assert_eq!(health.current, 17);
}

#[test]
fn test_first_strike_damage() {
    // Set up test world with first strike creatures
    // ...
}

#[test]
fn test_blocked_creature_damage() {
    // Test damage assignment with blockers
    // ...
}

#[test]
fn test_commander_damage_tracking() {
    // Test commander damage accumulation
    // ...
}
```

### Integration Tests

```rust
#[test]
fn test_full_combat_sequence() {
    // Set up a complete combat sequence with attackers, blockers,
    // and damage calculation
    // ...
}

#[test]
fn test_combat_with_damage_prevention() {
    // Test combat with damage prevention effects
    // ...
}

#[test]
fn test_combat_with_replacement_effects() {
    // Test combat with damage replacement effects
    // ...
}
```

### Edge Case Tests

```rust
#[test]
fn test_damage_to_indestructible() {
    // Test damage to indestructible creatures
    // ...
}

#[test]
fn test_damage_with_deathtouch_and_trample() {
    // Test the interaction of deathtouch and trample
    // ...
}

#[test]
fn test_damage_redirection() {
    // Test redirection of combat damage
    // ...
}
```

## Performance Considerations

1. **Batch Damage Processing**: Process all damage assignments simultaneously for better performance.

2. **Damage Event Optimization**: Use a more efficient event system for damage events to avoid spawning entities.

3. **Damage Assignment Caching**: Cache damage assignments to avoid recalculating them.

4. **Parallel Processing**: Use parallel processing for damage assignment when appropriate.

## Conclusion

The Combat Damage step is a complex but crucial part of the Commander game engine. Proper implementation ensures fair and accurate damage calculation, especially for tracking commander damage which is a key victory condition in the format. By handling special abilities like first strike, double strike, trample, and deathtouch correctly, we create a robust combat system that faithfully represents the Magic: The Gathering rules while maintaining good performance. 