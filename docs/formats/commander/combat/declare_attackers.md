# Declare Attackers Step

## Overview

The Declare Attackers step is where the active player selects which creatures will attack and which opponents or planeswalkers they will target. In Commander, this step is particularly complex due to the multiplayer nature of the format, allowing attacks against different opponents simultaneously. This document details the implementation of the Declare Attackers step in our game engine.

## Core Implementation

### Phase Structure

The Declare Attackers step follows the Beginning of Combat step in the combat phase sequence:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatStep {
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
}
```

### Declare Attackers System

The core system that handles the Declare Attackers step:

```rust
pub fn declare_attackers_system(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    mut game_events: EventWriter<GameEvent>,
    mut next_phase: ResMut<NextState<Phase>>,
    mut priority_system: ResMut<PrioritySystem>,
    mut combat_system: ResMut<CombatSystem>,
    mut attack_declarations: EventReader<AttackDeclarationEvent>,
) {
    // Only run during Declare Attackers step
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareAttackers) {
        return;
    }

    // If this is the first time entering the step
    if !priority_system.priority_given {
        // Emit Declare Attackers event
        let active_player = turn_manager.get_active_player();
        game_events.send(GameEvent::DeclareAttackersStep {
            player: active_player,
        });
        
        // Process attack requirements and restrictions
        commands.run_system(process_attack_requirements);
        
        // Grant priority to active player for attack declarations
        priority_system.grant_initial_priority();
    }
    
    // Process any attack declarations
    for event in attack_declarations.iter() {
        process_attack_declaration(&mut combat_system, event, &mut game_events);
    }
    
    // If active player has passed priority with attack declarations finalized
    if priority_system.active_player_passed && combat_system.attack_declarations_finalized {
        // Process post-declaration triggers
        commands.run_system(process_attack_triggers);
        
        // Reset priority for all players to respond to attacks
        priority_system.reset_with_active_player_priority();
    }
    
    // If all players have passed priority and the stack is empty
    if priority_system.all_players_passed() && priority_system.stack.is_empty() {
        // If at least one attacker was declared
        if !combat_system.attackers.is_empty() {
            // Proceed to Declare Blockers step
            next_phase.set(Phase::Combat(CombatStep::DeclareBlockers));
        } else {
            // Skip to End of Combat if no attackers
            next_phase.set(Phase::Combat(CombatStep::EndOfCombat));
        }
        priority_system.priority_given = false;
    }
}

// Helper function to process an attack declaration
fn process_attack_declaration(
    combat_system: &mut CombatSystem,
    event: &AttackDeclarationEvent,
    game_events: &mut EventWriter<GameEvent>,
) {
    let AttackDeclarationEvent { attacker, defender } = event;
    
    // Validate attack declaration
    if let Some(reason) = validate_attack(*attacker, *defender, combat_system) {
        game_events.send(GameEvent::InvalidAttackDeclaration {
            attacker: *attacker,
            defender: *defender,
            reason,
        });
        return;
    }
    
    // Record the attack in the combat system
    combat_system.attackers.insert(*attacker, AttackData {
        attacker: *attacker,
        defender: *defender,
        is_commander: false, // Will be updated by a separate system
        requirements: Vec::new(),
        restrictions: Vec::new(),
    });
    
    // Emit attack declaration event
    game_events.send(GameEvent::AttackDeclared {
        attacker: *attacker,
        defender: *defender,
    });
}
```

### Attack Validation

Attacks must be validated according to various rules and restrictions:

```rust
fn validate_attack(
    attacker: Entity,
    defender: Entity,
    combat_system: &CombatSystem,
) -> Option<String> {
    // Check if creature is able to attack
    if let Some(restrictions) = combat_system.attack_restrictions.get(&attacker) {
        for restriction in restrictions {
            match restriction {
                AttackRestriction::CantAttack => {
                    return Some("Creature cannot attack".to_string());
                },
                AttackRestriction::CantAttackPlayer(player) => {
                    if *player == defender {
                        return Some("Creature cannot attack this player".to_string());
                    }
                },
                AttackRestriction::CantAttackThisTurn => {
                    return Some("Creature cannot attack this turn".to_string());
                },
                // Other restrictions...
            }
        }
    }
    
    // Check defender-specific restrictions
    if let Some(restrictions) = combat_system.defender_restrictions.get(&defender) {
        for restriction in restrictions {
            match restriction {
                DefenderRestriction::CantBeAttacked => {
                    return Some("This player or planeswalker cannot be attacked".to_string());
                },
                DefenderRestriction::CantBeAttackedBy(condition) => {
                    if condition.matches(attacker) {
                        return Some("This creature cannot attack this defender".to_string());
                    }
                },
                // Other restrictions...
            }
        }
    }
    
    // All checks passed
    None
}
```

### Attack Requirements

Some creatures have requirements that dictate how they must attack:

```rust
pub fn process_attack_requirements(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
    active_player_query: Query<Entity, With<ActivePlayer>>,
    mut game_events: EventWriter<GameEvent>,
) {
    let active_player = active_player_query.single();
    
    // Process creatures with attack requirements
    for (entity, creature, controllable) in creature_query.iter() {
        // Only check creatures controlled by active player
        if controllable.controller != active_player {
            continue;
        }
        
        // Check if creature has attack requirements
        if let Some(requirements) = combat_system.attack_requirements.get(&entity) {
            for requirement in requirements {
                match requirement {
                    AttackRequirement::MustAttack => {
                        // Creature must attack if able
                        combat_system.add_required_attacker(entity);
                        
                        game_events.send(GameEvent::AttackRequirement {
                            creature: entity,
                            requirement: "Must attack if able".to_string(),
                        });
                    },
                    AttackRequirement::MustAttackSpecificPlayer(player) => {
                        // Creature must attack a specific player
                        combat_system.add_required_attacker(entity);
                        combat_system.add_required_defender(*player, entity);
                        
                        game_events.send(GameEvent::AttackRequirement {
                            creature: entity,
                            requirement: format!("Must attack player {:?} if able", player),
                        });
                    },
                    // Other requirements...
                }
            }
        }
    }
}
```

## Multiplayer Considerations

In Commander, the active player can attack multiple opponents in the same combat phase:

```rust
pub fn validate_multiplayer_attacks(
    combat_system: Res<CombatSystem>,
    turn_manager: Res<TurnManager>,
    player_query: Query<(Entity, &Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    let active_player = turn_manager.get_active_player();
    
    // Group attacks by defender
    let mut attacks_by_defender: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
    for (attacker, attack_data) in &combat_system.attackers {
        attacks_by_defender.entry(attack_data.defender)
            .or_insert_with(Vec::new)
            .push(*attacker);
    }
    
    // Verify all defenders are opponents
    for (defender, attackers) in &attacks_by_defender {
        // Skip planeswalkers (they're handled separately)
        if let Ok((_, player)) = player_query.get(*defender) {
            // Verify defender is not the active player
            if *defender == active_player {
                game_events.send(GameEvent::InvalidAttackTarget {
                    attacker: attackers[0], // Just report one of the attackers
                    defender: *defender,
                    reason: "Cannot attack yourself".to_string(),
                });
            }
        }
    }
    
    // Log all attack declarations for game history
    game_events.send(GameEvent::MultiplayerAttacksDeclared {
        active_player,
        attacks_by_defender: attacks_by_defender.clone(),
    });
}
```

## Commander-Specific Implementations

### Commander Attack Tracking

When a commander attacks, it needs to be specially tracked for commander damage:

```rust
pub fn track_commander_attacks(
    mut combat_system: ResMut<CombatSystem>,
    commander_query: Query<Entity, With<Commander>>,
) {
    // Find all commanders that are attacking
    for (attacker, attack_data) in combat_system.attackers.iter_mut() {
        if commander_query.contains(*attacker) {
            attack_data.is_commander = true;
        }
    }
}
```

### Goad Implementation

Goad is a Commander-specific mechanic that forces creatures to attack:

```rust
#[derive(Component)]
pub struct Goaded {
    pub source: Entity,
    pub until_end_of_turn: bool,
}

pub fn apply_goad_requirements(
    mut combat_system: ResMut<CombatSystem>,
    goaded_query: Query<(Entity, &Goaded, &Controllable)>,
    turn_manager: Res<TurnManager>,
) {
    let active_player = turn_manager.get_active_player();
    
    // Find all goaded creatures controlled by the active player
    for (entity, goaded, controllable) in goaded_query.iter() {
        if controllable.controller == active_player {
            // Add attack requirement - must attack if able
            combat_system.add_attack_requirement(entity, AttackRequirement::MustAttack);
            
            // Add attack restriction - can't attack the goad source
            combat_system.add_attack_restriction(entity, AttackRestriction::CantAttackPlayer(goaded.source));
        }
    }
}
```

## Triggered Abilities

### Attack Triggers

When attackers are declared, various triggered abilities might occur:

```rust
pub fn process_attack_triggers(
    turn_manager: Res<TurnManager>,
    combat_system: Res<CombatSystem>,
    mut ability_triggers: ResMut<AbilityTriggerQueue>,
    trigger_sources: Query<(Entity, &AbilityTrigger, &Controllable)>,
) {
    // Process "when this creature attacks" triggers
    for (attacker, _) in combat_system.attackers.iter() {
        if let Ok((entity, trigger, _)) = trigger_sources.get(*attacker) {
            if let TriggerCondition::WhenAttacks = trigger.condition {
                ability_triggers.queue.push_back(AbilityTriggerEvent {
                    source: entity,
                    trigger: trigger.clone(),
                    targets: Vec::new(),
                });
            }
        }
    }
    
    // Process "whenever a creature attacks" triggers
    for (entity, trigger, controllable) in trigger_sources.iter() {
        if let TriggerCondition::WheneverCreatureAttacks { controller_only } = trigger.condition {
            // Only consider triggers that should fire based on controller
            let should_trigger = if controller_only {
                // Check if any of the attacking creatures are controlled by this trigger's controller
                combat_system.attackers.iter().any(|(_, attack_data)| {
                    // Simplified for brevity, actual implementation would check creature controllers
                    true
                })
            } else {
                // Trigger for any attacking creature
                !combat_system.attackers.is_empty()
            };
            
            if should_trigger {
                ability_triggers.queue.push_back(AbilityTriggerEvent {
                    source: entity,
                    trigger: trigger.clone(),
                    targets: Vec::new(),
                });
            }
        }
    }
}
```

### Exert Mechanic

Exert is a mechanic that gives benefits in exchange for the creature not untapping:

```rust
#[derive(Component)]
pub struct Exert {
    pub duration: ExertDuration,
    pub effect: ExertEffect,
}

pub enum ExertDuration {
    NextUntapStep,
    NextUntapStepController,
}

pub fn handle_exert_choices(
    mut commands: Commands,
    mut exert_choices: EventReader<ExertChoiceEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in exert_choices.iter() {
        let ExertChoiceEvent { creature, exert } = event;
        
        if *exert {
            // Mark the creature as exerted
            commands.entity(*creature).insert(Exerted {
                until_next_untap_step: true,
            });
            
            // Apply exert effect
            // Implementation details omitted
            
            game_events.send(GameEvent::CreatureExerted {
                creature: *creature,
            });
        }
    }
}
```

## State Tracking

Once all attackers are declared, we need to update the game state:

```rust
pub fn update_creature_state_on_attack(
    mut commands: Commands,
    combat_system: Res<CombatSystem>,
    mut creature_query: Query<(Entity, &mut Creature)>,
) {
    // Update all attacking creatures
    for (attacker, attack_data) in combat_system.attackers.iter() {
        if let Ok((_, mut creature)) = creature_query.get_mut(*attacker) {
            // Mark creature as attacking
            creature.attacking = Some(attack_data.defender);
            
            // Add the Attacking component for faster queries
            commands.entity(*attacker).insert(Attacking {
                defender: attack_data.defender,
            });
            
            // Tap the creature unless it has vigilance
            if !creature.has_ability(CreatureAbility::Vigilance) {
                commands.entity(*attacker).insert(Tapped(true));
            }
        }
    }
}
```

## Edge Cases and Special Interactions

### Attack Redirection Effects

Some effects can redirect attacks to different players or planeswalkers:

```rust
pub fn handle_attack_redirection(
    mut combat_system: ResMut<CombatSystem>,
    redirection_effects: Query<(Entity, &AttackRedirection)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Process any attack redirection effects
    let redirections: Vec<(Entity, Entity, Entity)> = combat_system.attackers
        .iter()
        .filter_map(|(attacker, attack_data)| {
            for (effect_entity, redirection) in redirection_effects.iter() {
                if redirection.original_defender == attack_data.defender 
                   && redirection.applies_to(*attacker) {
                    return Some((*attacker, attack_data.defender, redirection.new_defender));
                }
            }
            None
        })
        .collect();
    
    // Apply redirections
    for (attacker, original_defender, new_defender) in redirections {
        if let Some(attack_data) = combat_system.attackers.get_mut(&attacker) {
            // Log the redirection
            game_events.send(GameEvent::AttackRedirected {
                attacker,
                original_defender,
                new_defender,
            });
            
            // Update the attack target
            attack_data.defender = new_defender;
        }
    }
}
```

### Attack Cost Effects

Some effects add costs to attacking:

```rust
pub fn handle_attack_costs(
    mut commands: Commands,
    mut combat_system: ResMut<CombatSystem>,
    cost_effects: Query<(Entity, &AttackCost)>,
    mut game_events: EventWriter<GameEvent>,
    mut mana_events: EventWriter<ManaPaymentEvent>,
) {
    // Process any attack cost effects
    let costs: Vec<(Entity, Entity, AttackCostType)> = combat_system.attackers
        .iter()
        .filter_map(|(attacker, attack_data)| {
            for (effect_entity, cost) in cost_effects.iter() {
                if cost.applies_to(*attacker, attack_data.defender) {
                    return Some((*attacker, effect_entity, cost.cost_type.clone()));
                }
            }
            None
        })
        .collect();
    
    // Apply costs
    for (attacker, cost_source, cost_type) in costs {
        match cost_type {
            AttackCostType::Mana(cost) => {
                // Request mana payment
                mana_events.send(ManaPaymentEvent {
                    source: attacker,
                    reason: PaymentReason::AttackCost { creature: attacker },
                    cost,
                });
            },
            AttackCostType::Life(amount) => {
                // Implementation for life payment
                // Details omitted
            },
            // Other cost types...
        }
        
        game_events.send(GameEvent::AttackCostApplied {
            attacker,
            cost_source,
            cost_description: format!("{:?}", cost_type),
        });
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
    fn test_attack_validation() {
        // Test with a creature that can attack
        let result = validate_attack(
            /* mock entities and combat system */
            Entity::from_raw(1),
            Entity::from_raw(2),
            &CombatSystem::default(),
        );
        assert_eq!(result, None, "Valid attack should return None");
        
        // Test with a creature that can't attack
        let mut combat_system = CombatSystem::default();
        combat_system.add_attack_restriction(
            Entity::from_raw(1),
            AttackRestriction::CantAttack,
        );
        
        let result = validate_attack(
            Entity::from_raw(1),
            Entity::from_raw(2),
            &combat_system,
        );
        assert!(result.is_some(), "Invalid attack should return an error message");
    }
    
    #[test]
    fn test_commander_attack_tracking() {
        let mut app = App::new();
        app.add_systems(Update, track_commander_attacks);
        
        // Set up a commander and a regular creature
        let commander = app.world.spawn((Creature::default(), Commander)).id();
        let regular_creature = app.world.spawn(Creature::default()).id();
        
        // Set up combat system with both attacking
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(commander, AttackData {
            attacker: commander,
            defender: Entity::from_raw(3),
            is_commander: false, // Should be updated by system
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        combat_system.attackers.insert(regular_creature, AttackData {
            attacker: regular_creature,
            defender: Entity::from_raw(3),
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Check commander status
        let combat_system = app.world.resource::<CombatSystem>();
        assert!(combat_system.attackers[&commander].is_commander, "Commander should be marked as such when attacking");
        assert!(!combat_system.attackers[&regular_creature].is_commander, "Regular creature should not be marked as a commander");
    }
    
    #[test]
    fn test_goad_mechanic() {
        let mut app = App::new();
        app.add_systems(Update, apply_goad_requirements);
        
        // Set up test environment with a goaded creature
        let active_player = app.world.spawn(Player::default()).id();
        let opponent = app.world.spawn(Player::default()).id();
        
        let goaded_creature = app.world.spawn((
            Creature::default(),
            Controllable { controller: active_player },
            Goaded { source: opponent, until_end_of_turn: true },
        )).id();
        
        // Set up turn manager
        let mut turn_manager = TurnManager::default();
        turn_manager.active_player_index = 0;
        turn_manager.player_order = vec![active_player, opponent];
        app.insert_resource(turn_manager);
        
        // Set up combat system
        let combat_system = CombatSystem::default();
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Check results
        let combat_system = app.world.resource::<CombatSystem>();
        
        // Goaded creature should be required to attack
        let has_must_attack = combat_system.attack_requirements.get(&goaded_creature)
            .map_or(false, |reqs| reqs.iter().any(|req| matches!(req, AttackRequirement::MustAttack)));
        assert!(has_must_attack, "Goaded creature should have MustAttack requirement");
        
        // Goaded creature should not be able to attack the goad source
        let has_cant_attack_source = combat_system.attack_restrictions.get(&goaded_creature)
            .map_or(false, |reqs| reqs.iter().any(|req| matches!(req, AttackRestriction::CantAttackPlayer(p) if *p == opponent)));
        assert!(has_cant_attack_source, "Goaded creature should not be able to attack goad source");
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
    fn test_declare_attackers_workflow() {
        let mut app = App::new();
        
        // Add all relevant systems
        app.add_systems(Update, (
            declare_attackers_system,
            process_attack_requirements,
            track_commander_attacks,
            process_attack_triggers,
            update_creature_state_on_attack,
        ));
        
        // Set up game state with players and creatures
        // Implementation details omitted for brevity
        
        // Simulate player declaring attackers
        app.world.resource_mut::<Events<AttackDeclarationEvent>>().send(
            AttackDeclarationEvent {
                attacker: Entity::from_raw(1),
                defender: Entity::from_raw(2),
            }
        );
        
        // Run update to process declarations
        app.update();
        
        // Verify attackers are properly recorded and state is updated
        // Implementation details omitted for brevity
    }
    
    #[test]
    fn test_multiplayer_attack_declarations() {
        let mut app = App::new();
        
        // Set up a multiplayer environment with three players
        let active_player = app.world.spawn(Player::default()).id();
        let opponent1 = app.world.spawn(Player::default()).id();
        let opponent2 = app.world.spawn(Player::default()).id();
        
        // Set up creatures for the active player
        let creature1 = app.world.spawn(Creature::default()).id();
        let creature2 = app.world.spawn(Creature::default()).id();
        
        // Set up turn manager
        let mut turn_manager = TurnManager::default();
        turn_manager.active_player_index = 0;
        turn_manager.player_order = vec![active_player, opponent1, opponent2];
        app.insert_resource(turn_manager);
        
        // Set up combat system with attacks against different opponents
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(creature1, AttackData {
            attacker: creature1,
            defender: opponent1,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        combat_system.attackers.insert(creature2, AttackData {
            attacker: creature2,
            defender: opponent2,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        app.insert_resource(combat_system);
        
        // Add validate_multiplayer_attacks system
        app.add_systems(Update, validate_multiplayer_attacks);
        app.add_event::<GameEvent>();
        
        // Run the system
        app.update();
        
        // Check for multiplayer attack event
        let mut found_event = false;
        let events = app.world.resource::<Events<GameEvent>>();
        let mut reader = events.get_reader();
        
        for event in reader.iter(events) {
            if let GameEvent::MultiplayerAttacksDeclared { .. } = event {
                found_event = true;
                break;
            }
        }
        
        assert!(found_event, "Multiplayer attack event should be emitted");
    }
    
    // Additional integration tests...
}
```

## UI Considerations

The UI during the Declare Attackers step needs to clearly communicate various states:

```rust
pub fn update_declare_attackers_ui(
    turn_manager: Res<TurnManager>,
    combat_system: Res<CombatSystem>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
    player_query: Query<Entity, With<Player>>,
    mut ui_state: ResMut<UiState>,
) {
    // Only run during Declare Attackers step
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareAttackers) {
        return;
    }
    
    // Update phase display
    ui_state.current_phase_text = "Declare Attackers".to_string();
    
    // Get active player
    let active_player = turn_manager.get_active_player();
    
    // Highlight potential attackers
    ui_state.potential_attackers.clear();
    for (entity, creature, controllable) in creature_query.iter() {
        if controllable.controller == active_player && creature.can_attack() {
            ui_state.potential_attackers.insert(entity);
            
            // Mark creatures that must attack
            if let Some(requirements) = combat_system.attack_requirements.get(&entity) {
                if requirements.iter().any(|req| matches!(req, AttackRequirement::MustAttack)) {
                    ui_state.creatures_with_requirements.insert(entity, "Must attack if able".to_string());
                }
            }
        }
    }
    
    // Highlight potential defenders
    ui_state.potential_defenders.clear();
    for entity in player_query.iter() {
        if entity != active_player {
            ui_state.potential_defenders.insert(entity);
        }
    }
    
    // Show current attack declarations
    ui_state.current_attacks.clear();
    for (attacker, attack_data) in combat_system.attackers.iter() {
        ui_state.current_attacks.insert(*attacker, attack_data.defender);
    }
}
```

## Performance Considerations

1. **Efficient Attack Validation**: The validation of attacks should be optimized to avoid redundant checks.

2. **Caching Attack Results**: Once attack declarations are finalized, the results can be cached for use in subsequent steps.

3. **Parallel Processing**: For games with many attackers, processing attack triggers could be done in parallel.

4. **Minimize Component Access**: Group related queries to minimize entity access operations.

## Conclusion

The Declare Attackers step is a critical part of the combat phase in Commander. A robust implementation ensures that all game rules are properly enforced, including multiplayer-specific mechanics like Goad. By handling attack declarations, restrictions, requirements, and triggers correctly, we provide the foundation for a smooth and accurate combat resolution process. 