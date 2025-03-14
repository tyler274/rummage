# Beginning of Combat Step

## Overview

The Beginning of Combat step is the first step of the Combat Phase in Magic: The Gathering. This step serves as a transition between the pre-combat Main Phase and the declaration of attackers. It provides a crucial window for players to cast instants and activate abilities before attackers are declared. This document details the implementation of the Beginning of Combat step in our Commander game engine.

## Core Implementation

### Phase Structure

The Beginning of Combat step is implemented as part of the overall combat phase system:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatStep {
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    // Other phases...
    Combat(CombatStep),
    // Other phases...
}
```

### Beginning of Combat System

The core system that handles the Beginning of Combat step:

```rust
pub fn beginning_of_combat_system(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    mut game_events: EventWriter<GameEvent>,
    active_player: Query<Entity, With<ActivePlayer>>,
    mut next_phase: ResMut<NextState<Phase>>,
    mut priority_system: ResMut<PrioritySystem>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }

    // If this is the first time entering the step
    if !priority_system.priority_given {
        // Emit Beginning of Combat event
        let active_player_entity = active_player.single();
        game_events.send(GameEvent::BeginningOfCombat {
            player: active_player_entity,
        });
        
        // Clear any "until end of combat" effects from previous turns
        commands.run_system(clear_end_of_combat_effects);
        
        // Initialize any tracked combat data
        commands.insert_resource(CombatData::default());
        
        // Grant priority to active player
        priority_system.grant_initial_priority();
    }
    
    // If all players have passed priority without adding to the stack
    if priority_system.all_players_passed() && !priority_system.stack_changed {
        // Proceed to Declare Attackers step
        next_phase.set(Phase::Combat(CombatStep::DeclareAttackers));
        priority_system.priority_given = false;
    }
}
```

### Event Triggers

The beginning of combat step triggers various abilities:

```rust
pub fn beginning_of_combat_triggers(
    turn_manager: Res<TurnManager>,
    mut ability_triggers: ResMut<AbilityTriggerQueue>,
    trigger_sources: Query<(Entity, &AbilityTrigger)>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    // Process all "at beginning of combat" triggers
    for (entity, trigger) in trigger_sources.iter() {
        if let TriggerCondition::BeginningOfCombat { controller_only } = trigger.condition {
            let should_trigger = if controller_only {
                // Only trigger for the active player's permanents
                // Implementation details omitted
                true
            } else {
                // Trigger for all permanents with this trigger
                true
            };
            
            if should_trigger {
                ability_triggers.queue.push_back(AbilityTriggerEvent {
                    source: entity,
                    trigger: trigger.clone(),
                    targets: Vec::new(), // Will be filled later in target selection
                });
            }
        }
    }
}
```

## Multiplayer Considerations

In Commander, the Beginning of Combat step needs to handle multiplayer-specific considerations:

```rust
pub fn multiplayer_beginning_of_combat(
    turn_manager: Res<TurnManager>,
    player_query: Query<(Entity, &Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    // Notify all players about the beginning of combat
    let active_player = turn_manager.get_active_player();
    
    // Broadcast to all players
    for (player_entity, _) in player_query.iter() {
        game_events.send(GameEvent::PhaseChange {
            phase: Phase::Combat(CombatStep::BeginningOfCombat),
            player: active_player,
            notification_target: player_entity,
        });
    }
}
```

## Ability Types in Beginning of Combat

### Static Abilities

Static abilities that specifically affect combat are evaluated during this step:

```rust
pub fn evaluate_combat_static_abilities(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    static_ability_query: Query<(Entity, &StaticAbility)>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    // Apply "can't attack" effects
    for (entity, static_ability) in static_ability_query.iter() {
        match static_ability.effect {
            StaticEffect::CantAttack { target, condition } => {
                // Apply can't attack markers to appropriate creatures
                for (creature_entity, _, controllable) in creature_query.iter() {
                    if target.matches(creature_entity) && condition.is_met(creature_entity) {
                        commands.entity(creature_entity).insert(CantAttack {
                            source: entity,
                            duration: static_ability.duration,
                        });
                    }
                }
            },
            StaticEffect::CantBlock { target, condition } => {
                // Apply can't block markers to appropriate creatures
                for (creature_entity, _, controllable) in creature_query.iter() {
                    if target.matches(creature_entity) && condition.is_met(creature_entity) {
                        commands.entity(creature_entity).insert(CantBlock {
                            source: entity,
                            duration: static_ability.duration,
                        });
                    }
                }
            },
            // Other combat-relevant static effects...
            _ => {}
        }
    }
}
```

### Triggered Abilities

Abilities that trigger at the beginning of combat:

```rust
#[derive(Component)]
pub struct AbilityTrigger {
    pub condition: TriggerCondition,
    pub effect: TriggeredEffect,
}

#[derive(Clone)]
pub enum TriggerCondition {
    BeginningOfCombat { controller_only: bool },
    // Other trigger conditions...
}

// Example of a "beginning of combat" triggered ability
pub fn assemble_megatron_trigger(
    turn_manager: Res<TurnManager>,
    megatron_query: Query<(Entity, &Controllable), With<Megatron>>,
    mut game_events: EventWriter<GameEvent>,
    mut ability_triggers: ResMut<AbilityTriggerQueue>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    let active_player = turn_manager.get_active_player();
    
    for (entity, controllable) in megatron_query.iter() {
        // Only trigger for the active player's Megatron
        if controllable.controller == active_player {
            ability_triggers.queue.push_back(AbilityTriggerEvent {
                source: entity,
                trigger: AbilityTrigger {
                    condition: TriggerCondition::BeginningOfCombat { controller_only: true },
                    effect: TriggeredEffect::AssembleMegatron,
                },
                targets: Vec::new(),
            });
        }
    }
}
```

## Edge Cases and Special Interactions

### Cleanup Actions

Sometimes effects need to be cleaned up at the beginning of combat:

```rust
pub fn clear_end_of_combat_effects(
    mut commands: Commands,
    effect_query: Query<(Entity, &EndOfCombatEffect)>,
) {
    for (entity, effect) in effect_query.iter() {
        if effect.cleanup_at_next_beginning_of_combat {
            // Remove the effect component
            commands.entity(entity).remove::<EndOfCombatEffect>();
            
            // Apply any cleanup logic specific to this effect
            match effect.effect_type {
                EndOfCombatEffectType::TemporaryPowerBoost => {
                    commands.entity(entity).remove::<PowerToughnessModifier>();
                },
                EndOfCombatEffectType::TemporaryAbilityGrant => {
                    commands.entity(entity).remove::<GrantedAbility>();
                },
                // Other effect types...
            }
        }
    }
}
```

### "Until Your Next Combat" Effects

Some effects last until a player's next combat phase begins:

```rust
pub fn process_until_next_combat_effects(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    effect_query: Query<(Entity, &UntilNextCombatEffect, &Controllable)>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    let active_player = turn_manager.get_active_player();
    
    // Find and remove effects that should end
    for (entity, effect, controllable) in effect_query.iter() {
        if controllable.controller == active_player {
            commands.entity(entity).remove::<UntilNextCombatEffect>();
            
            // Apply any cleanup logic specific to this effect
            match effect.effect_type {
                // Implementation details...
                _ => {}
            }
        }
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_beginning_of_combat_triggers() {
        let mut app = App::new();
        app.add_systems(Update, beginning_of_combat_triggers);
        app.init_resource::<AbilityTriggerQueue>();
        
        // Create a simple trigger
        let trigger_entity = app.world.spawn((
            AbilityTrigger {
                condition: TriggerCondition::BeginningOfCombat { controller_only: false },
                effect: TriggeredEffect::DrawCard { count: 1 },
            },
        )).id();
        
        // Set up turn manager with Beginning of Combat phase
        let mut turn_manager = TurnManager::default();
        turn_manager.current_phase = Phase::Combat(CombatStep::BeginningOfCombat);
        app.insert_resource(turn_manager);
        
        // Run the system
        app.update();
        
        // Check if trigger was added to queue
        let ability_triggers = app.world.resource::<AbilityTriggerQueue>();
        assert_eq!(ability_triggers.queue.len(), 1);
        
        let trigger_event = ability_triggers.queue.front().unwrap();
        assert_eq!(trigger_event.source, trigger_entity);
    }
    
    #[test]
    fn test_beginning_of_combat_system_phase_progression() {
        let mut app = App::new();
        app.add_systems(Update, beginning_of_combat_system);
        app.add_event::<GameEvent>();
        app.init_resource::<PrioritySystem>();
        app.init_resource::<NextState<Phase>>();
        
        // Set up active player
        let active_player = app.world.spawn((Player::default(), ActivePlayer)).id();
        
        // Set up turn manager with Beginning of Combat phase
        let mut turn_manager = TurnManager::default();
        turn_manager.current_phase = Phase::Combat(CombatStep::BeginningOfCombat);
        app.insert_resource(turn_manager);
        
        // Set up priority system to indicate all players have passed
        let mut priority_system = PrioritySystem::default();
        priority_system.priority_given = true;
        priority_system.current_player_index = 0;
        priority_system.all_passed = true;
        priority_system.stack_changed = false;
        app.insert_resource(priority_system);
        
        // Run the system
        app.update();
        
        // Check that phase advanced to Declare Attackers
        let next_phase = app.world.resource::<NextState<Phase>>();
        assert_eq!(next_phase.0, Some(Phase::Combat(CombatStep::DeclareAttackers)));
    }
    
    // Additional unit tests...
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_beginning_of_combat_workflow() {
        let mut app = App::new();
        
        // Add all relevant systems
        app.add_systems(Update, (
            beginning_of_combat_system,
            beginning_of_combat_triggers,
            evaluate_combat_static_abilities,
            process_until_next_combat_effects,
            clear_end_of_combat_effects,
        ));
        
        // Set up game state
        // Implementation details omitted for brevity
        
        // Run several updates to simulate entire beginning of combat step
        for _ in 0..5 {
            app.update();
        }
        
        // Verify that all systems executed correctly
        // Implementation details omitted for brevity
    }
    
    // Additional integration tests...
}
```

## UI Considerations

During the Beginning of Combat step, the user interface should:

1. Clearly indicate the current phase (Beginning of Combat)
2. Show which player has priority
3. Highlight creatures that could potentially attack
4. Display any relevant triggered abilities that are waiting to go on the stack

This can be implemented with the following system:

```rust
pub fn update_beginning_of_combat_ui(
    turn_manager: Res<TurnManager>,
    priority_system: Res<PrioritySystem>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
    ability_triggers: Res<AbilityTriggerQueue>,
    mut ui_state: ResMut<UiState>,
) {
    // Only run during Beginning of Combat
    if turn_manager.current_phase != Phase::Combat(CombatStep::BeginningOfCombat) {
        return;
    }
    
    // Update phase display
    ui_state.current_phase_text = "Beginning of Combat".to_string();
    
    // Show player with priority
    ui_state.player_with_priority = priority_system.get_player_with_priority();
    
    // Highlight potential attackers
    let active_player = turn_manager.get_active_player();
    for (entity, creature, controllable) in creature_query.iter() {
        if controllable.controller == active_player && creature.can_attack() {
            ui_state.potential_attackers.insert(entity);
        }
    }
    
    // Display waiting triggers
    ui_state.pending_triggers = ability_triggers.queue.iter()
        .map(|trigger| (trigger.source, trigger.trigger.clone()))
        .collect();
}
```

## Performance Considerations

The Beginning of Combat step generally has fewer performance implications than subsequent combat steps, but we should still be mindful of:

1. Efficiently processing "beginning of combat" triggers, which could be numerous
2. Minimizing component queries by combining related operations
3. Only performing combat-specific calculations once per beginning of combat step

## Conclusion

The Beginning of Combat step, while often quickly passed through in many games, serves a crucial role in the overall combat structure. It provides the last opportunity for players to act before attackers are declared, and it's the time when many powerful combat-related triggered abilities occur. A robust implementation of this step ensures that all cards function correctly and players have appropriate opportunities to respond before the action of combat truly begins. 