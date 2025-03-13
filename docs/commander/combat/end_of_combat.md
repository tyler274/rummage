# End of Combat

## Overview

The End of Combat step is the final phase of the combat sequence where "at end of combat" triggered abilities are put on the stack and resolved. This step also includes important cleanup activities that return the game state to normal after combat concludes. In Commander, this step is particularly important for handling multiplayer-specific combat interactions and resetting the combat state.

This document outlines the implementation details, edge cases, and testing strategies for the End of Combat step in our Commander engine.

## Core Concepts

### End of Combat Flow

The End of Combat step follows this general sequence:

1. "At end of combat" triggered abilities are put on the stack
2. Priority is passed to players in turn order
3. Once all players pass priority and the stack is empty, combat cleanup occurs:
   - "Until end of combat" effects end
   - Combat-specific statuses are removed
   - The game advances to the Postcombat Main Phase

### Combat Cleanup

After the End of Combat step is complete, several important cleanup tasks must occur:

1. Remove all attacking and blocking statuses from creatures
2. End "until end of combat" effects
3. Clear combat damage tracking from the current turn
4. Reset any combat-specific flags or state

## Implementation Design

### Data Structures

```rust
// Component for tracking "until end of combat" effects
#[derive(Component)]
struct UntilEndOfCombat {
    // Any data needed for the effect
}

// System resource for managing the end of combat step
struct EndOfCombatSystem {
    triggers_processed: bool,
}
```

### End of Combat System

```rust
fn end_of_combat_system(
    mut commands: Commands,
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    attacker_query: Query<Entity, With<Attacking>>,
    blocker_query: Query<Entity, With<Blocking>>,
    end_of_combat_effects: Query<Entity, With<UntilEndOfCombat>>,
    end_of_combat_triggers: Query<(Entity, &EndOfCombatTrigger)>,
    // Other system parameters
) {
    // Only run during end of combat step
    if !matches!(turn_manager.current_phase, Phase::Combat(CombatStep::End)) {
        return;
    }
    
    // Process "at end of combat" triggers if not already processed
    if !combat_system.end_of_combat.triggers_processed {
        for (entity, trigger) in end_of_combat_triggers.iter() {
            // Create trigger event
            commands.spawn(TriggerEvent {
                source: entity,
                trigger_type: TriggerType::EndOfCombat,
            });
        }
        
        combat_system.end_of_combat.triggers_processed = true;
        return; // Exit to allow triggers to be processed
    }
    
    // If we've processed triggers and the stack is empty, perform combat cleanup
    if combat_system.end_of_combat.triggers_processed && turn_manager.stack_is_empty() {
        // Remove attacking status from all attackers
        for entity in attacker_query.iter() {
            commands.entity(entity).remove::<Attacking>();
        }
        
        // Remove blocking status from all blockers
        for entity in blocker_query.iter() {
            commands.entity(entity).remove::<Blocking>();
        }
        
        // End "until end of combat" effects
        for entity in end_of_combat_effects.iter() {
            commands.entity(entity).remove::<UntilEndOfCombat>();
        }
        
        // Reset combat system state
        combat_system.reset();
        
        // Combat complete, ready to advance to postcombat main phase
    }
}
```

### Combat System Reset

```rust
impl CombatSystem {
    pub fn reset(&mut self) {
        self.attackers.clear();
        self.blockers.clear();
        self.damage_assignments.clear();
        self.first_strike_round_completed = false;
        self.begin_combat.triggers_processed = false;
        self.declare_attackers.triggers_processed = false;
        self.declare_blockers.triggers_processed = false;
        self.end_of_combat.triggers_processed = false;
    }
}
```

## Special Cases and Edge Scenarios

### Delayed End of Combat Triggers

Some effects create delayed triggered abilities that trigger at the end of combat:

```rust
fn create_delayed_end_of_combat_trigger(
    commands: &mut Commands,
    source: Entity,
    effect: impl Fn(&mut Commands) + Send + Sync + 'static
) {
    commands.spawn((
        DelayedTrigger {
            source,
            trigger_type: TriggerType::EndOfCombat,
            effect: Box::new(effect),
        },
    ));
}
```

### Regeneration Shield Cleanup

Regeneration shields that were used during combat should be removed:

```rust
fn cleanup_regeneration_shields(
    mut commands: Commands,
    regeneration_query: Query<(Entity, &RegenerationShield)>
) {
    for (entity, shield) in regeneration_query.iter() {
        if shield.used {
            commands.entity(entity).remove::<RegenerationShield>();
        }
    }
}
```

### "Until End of Combat" Effects

Effects that last until end of combat need special handling:

```rust
fn apply_until_end_of_combat_effect(
    commands: &mut Commands,
    target: Entity,
    effect_data: EffectData
) {
    // Apply the effect
    commands.entity(target).insert(effect_data.component);
    
    // Mark it to be removed at end of combat
    commands.entity(target).insert(UntilEndOfCombat {
        // Any necessary data
    });
}
```

### Phasing During End of Combat

Creatures that phase out during combat might need special handling during end of combat:

```rust
fn handle_phased_combat_participants(
    mut commands: Commands,
    phased_attackers: Query<Entity, (With<Attacking>, With<PhasedOut>)>,
    phased_blockers: Query<Entity, (With<Blocking>, With<PhasedOut>)>
) {
    // Remove combat status from phased creatures too
    for entity in phased_attackers.iter() {
        commands.entity(entity).remove::<Attacking>();
    }
    
    for entity in phased_blockers.iter() {
        commands.entity(entity).remove::<Blocking>();
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_basic_end_of_combat_cleanup() {
    // Set up test world
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, end_of_combat_system);
       
    // Create attackers and blockers
    let attacker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Attacking { defending: player_entity },
    )).id();
    
    let blocker = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Blocking { blocked_attackers: vec![attacker] },
    )).id();
    
    // Simulate end of combat step
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::End);
    app.world.resource_mut::<CombatSystem>().end_of_combat.triggers_processed = true;
    
    // Process end of combat
    app.update();
    
    // Verify cleanup
    assert!(!app.world.entity(attacker).contains::<Attacking>());
    assert!(!app.world.entity(blocker).contains::<Blocking>());
}

#[test]
fn test_end_of_combat_triggers() {
    // Test triggers that happen at end of combat
    // ...
}

#[test]
fn test_until_end_of_combat_effects() {
    // Test that effects with "until end of combat" duration are removed
    // ...
}
```

### Integration Tests

```rust
#[test]
fn test_full_combat_sequence_with_end_phase() {
    // Test a complete combat sequence from beginning to end
    // ...
}

#[test]
fn test_end_of_combat_with_replacement_effects() {
    // Test end of combat with replacement effects that modify cleanup
    // ...
}
```

### Edge Case Tests

```rust
#[test]
fn test_end_of_combat_with_phased_creatures() {
    // Test end of combat with phased creatures
    // ...
}

#[test]
fn test_end_of_combat_when_player_loses() {
    // Test what happens if a player loses during end of combat
    // ...
}

#[test]
fn test_end_of_combat_with_delayed_triggers() {
    // Test delayed triggers that happen at end of combat
    // ...
}
```

## Performance Considerations

1. **Batch Removal Operations**: Group similar removal operations together for better performance.

2. **Minimize Query Iterations**: Structure queries to minimize iterations over entities.

3. **State Reset Optimization**: Efficiently reset the combat state without unnecessary operations.

4. **Effect Tracking**: Use an efficient system for tracking and removing "until end of combat" effects.

## Conclusion

The End of Combat step is a critical transition point in the Commander game flow. Proper implementation ensures that combat-related statuses and effects are appropriately cleaned up, and the game state is correctly prepared for the next phase. By handling all "at end of combat" triggers and cleanup operations correctly, we create a robust and reliable combat system that maintains game state integrity while supporting all the complex interactions present in the Commander format. 