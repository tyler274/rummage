# Declare Blockers Step

## Overview

The Declare Blockers step is the phase of combat where defending players assign their creatures to block attacking creatures. In Commander, this step can be particularly complex due to the multiplayer nature of the format, where multiple players may be defending against attacks simultaneously. This document outlines the implementation of the Declare Blockers step in our game engine.

## Core Implementation

### Phase Structure

The Declare Blockers step follows the Declare Attackers step in the combat phase sequence:

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

### Declare Blockers System

The core system that handles the Declare Blockers step:

```rust
pub fn declare_blockers_system(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    mut game_events: EventWriter<GameEvent>,
    mut next_phase: ResMut<NextState<Phase>>,
    mut priority_system: ResMut<PrioritySystem>,
    combat_system: Res<CombatSystem>,
    mut block_declarations: EventReader<BlockDeclarationEvent>,
) {
    // Only run during Declare Blockers step
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareBlockers) {
        return;
    }

    // If this is the first time entering the step
    if !priority_system.priority_given {
        // Emit Declare Blockers event
        game_events.send(GameEvent::DeclareBlockersStep);
        
        // Process block requirements and restrictions
        commands.run_system(process_block_requirements);
        
        // Create turn order for block declarations
        priority_system.set_player_order_for_blockers(combat_system.attacked_players());
        
        // Grant priority to first defending player
        priority_system.grant_initial_priority();
    }
    
    // Process any block declarations
    for event in block_declarations.iter() {
        process_block_declaration(&mut commands, event, &combat_system, &mut game_events);
    }
    
    // If all defending players have passed priority with block declarations finalized
    if priority_system.all_defenders_passed && combat_system.block_declarations_finalized {
        // Process post-declaration triggers
        commands.run_system(process_block_triggers);
        
        // Reset priority for all players to respond to blocks
        priority_system.reset_with_active_player_priority();
    }
    
    // If all players have passed priority and the stack is empty
    if priority_system.all_players_passed() && priority_system.stack.is_empty() {
        // Proceed to Combat Damage step
        next_phase.set(Phase::Combat(CombatStep::CombatDamage));
        priority_system.priority_given = false;
    }
}

// Helper function to process a block declaration
fn process_block_declaration(
    commands: &mut Commands,
    event: &BlockDeclarationEvent,
    combat_system: &CombatSystem,
    game_events: &mut EventWriter<GameEvent>,
) {
    let BlockDeclarationEvent { blocker, attackers } = event;
    
    // Validate block declaration
    if let Some(reason) = validate_block(*blocker, attackers, combat_system) {
        game_events.send(GameEvent::InvalidBlockDeclaration {
            blocker: *blocker,
            attackers: attackers.clone(),
            reason,
        });
        return;
    }
    
    // Record the block in the combat system
    commands.entity(*blocker).insert(Blocking {
        blocked_attackers: attackers.clone(),
    });
    
    // Emit block declaration event
    for attacker in attackers {
        game_events.send(GameEvent::BlockDeclared {
            blocker: *blocker,
            attacker: *attacker,
        });
    }
}
```

### Block Validation

Blocks must be validated according to various rules and restrictions:

```rust
fn validate_block(
    blocker: Entity,
    attackers: &[Entity],
    combat_system: &CombatSystem,
) -> Option<String> {
    // Check if creature can block at all
    if let Some(restrictions) = combat_system.block_restrictions.get(&blocker) {
        for restriction in restrictions {
            match restriction {
                BlockRestriction::CantBlock => {
                    return Some("Creature cannot block".to_string());
                },
                // Other general block restrictions...
            }
        }
    }
    
    // Check if creature can block multiple attackers
    if attackers.len() > 1 {
        let can_block_multiple = check_can_block_multiple(blocker, combat_system);
        if !can_block_multiple {
            return Some("Creature cannot block multiple attackers".to_string());
        }
    }
    
    // Check attacker-specific restrictions
    for attacker in attackers {
        // Check if this attacker can be blocked by this blocker
        if let Some(restrictions) = combat_system.attacker_restrictions.get(attacker) {
            for restriction in restrictions {
                match restriction {
                    AttackerRestriction::CantBeBlocked => {
                        return Some(format!("Attacker {} cannot be blocked", attacker.index()));
                    },
                    AttackerRestriction::CantBeBlockedBy(condition) => {
                        if condition.matches(blocker) {
                            return Some(format!("Attacker {} cannot be blocked by this creature", attacker.index()));
                        }
                    },
                    // Other attacker-specific restrictions...
                }
            }
        }
    }
    
    // Check special blocking requirements
    if let Some(requirements) = combat_system.attacker_block_requirements.get(&attackers[0]) {
        for requirement in requirements {
            match requirement {
                BlockRequirement::MustBeBlockedByAtLeast(count) => {
                    if combat_system.get_blockers_for_attacker(attackers[0]).len() + 1 < *count {
                        // This single blocker is not enough to satisfy the requirement
                        // Note: In a real implementation, this would need to check if the requirement
                        // could be satisfied with other declared blocks
                        return Some(format!("Attacker requires at least {} blockers", count));
                    }
                },
                // Other block requirements...
            }
        }
    }
    
    // All checks passed
    None
}

// Helper function to check if a creature can block multiple attackers
fn check_can_block_multiple(
    blocker: Entity,
    combat_system: &CombatSystem,
) -> bool {
    // Check if creature has a special ability that allows blocking multiple attackers
    if let Some(special_abilities) = combat_system.creature_special_abilities.get(&blocker) {
        if special_abilities.contains(&SpecialAbility::CanBlockAdditionalCreature(1)) {
            return true;
        }
        if special_abilities.contains(&SpecialAbility::CanBlockAnyNumber) {
            return true;
        }
    }
    
    // By default, creatures can only block one attacker
    false
}
```

## Block Requirements

Some effects in the game can force creatures to block:

```rust
pub fn process_block_requirements(
    combat_system: Res<CombatSystem>,
    mut game_events: EventWriter<GameEvent>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
    player_query: Query<(Entity, &Player)>,
) {
    // Identify all players who are being attacked
    let attacked_players: HashSet<Entity> = combat_system.attackers
        .iter()
        .filter_map(|(_, attack_data)| {
            if player_query.contains(attack_data.defender) {
                Some(attack_data.defender)
            } else {
                None
            }
        })
        .collect();
    
    // Process creatures with block requirements
    for (entity, creature, controllable) in creature_query.iter() {
        // Only check creatures controlled by players being attacked
        if !attacked_players.contains(&controllable.controller) {
            continue;
        }
        
        // Check if creature has block requirements
        if let Some(requirements) = combat_system.block_requirements.get(&entity) {
            for requirement in requirements {
                match requirement {
                    BlockRequirement::MustBlock => {
                        // Creature must block if able
                        game_events.send(GameEvent::BlockRequirement {
                            creature: entity,
                            requirement: "Must block if able".to_string(),
                        });
                    },
                    BlockRequirement::MustBlockAttacker(attacker) => {
                        // Creature must block a specific attacker
                        if combat_system.attackers.contains_key(attacker) {
                            game_events.send(GameEvent::BlockRequirement {
                                creature: entity,
                                requirement: format!("Must block attacker {:?} if able", attacker),
                            });
                        }
                    },
                    // Other requirements...
                }
            }
        }
    }
}
```

## Evasion Abilities

Evasion abilities like Flying, Menace, etc. are essential to the blocking rules:

```rust
pub fn apply_evasion_restrictions(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
) {
    // Process flying
    for (entity, creature) in creature_query.iter() {
        if creature.has_ability(CreatureAbility::Flying) {
            // Flying creatures can only be blocked by creatures with flying or reach
            combat_system.add_attacker_restriction(
                entity,
                AttackerRestriction::CantBeBlockedBy(BlockerCondition::NotFlyingOrReach),
            );
        }
        
        if creature.has_ability(CreatureAbility::Menace) {
            // Menace creatures must be blocked by at least two creatures
            combat_system.add_attacker_block_requirement(
                entity,
                BlockRequirement::MustBeBlockedByAtLeast(2),
            );
        }
        
        if creature.has_ability(CreatureAbility::Fear) {
            // Fear creatures can only be blocked by black or artifact creatures
            combat_system.add_attacker_restriction(
                entity,
                AttackerRestriction::CantBeBlockedBy(BlockerCondition::NotBlackOrArtifact),
            );
        }
        
        // Implement other evasion abilities
    }
}
```

## Multiplayer Considerations

In multiplayer Commander games, multiple players might need to declare blockers:

```rust
pub fn handle_multiplayer_blocks(
    combat_system: Res<CombatSystem>,
    turn_manager: Res<TurnManager>,
    player_query: Query<(Entity, &Player)>,
    mut priority_system: ResMut<PrioritySystem>,
) {
    // Group attackers by defending player
    let mut attackers_by_defender: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
    for (attacker, attack_data) in &combat_system.attackers {
        if player_query.contains(attack_data.defender) {
            attackers_by_defender.entry(attack_data.defender)
                .or_insert_with(Vec::new)
                .push(*attacker);
        }
    }
    
    // Set up priority for each defending player in turn order
    let mut defender_order = Vec::new();
    for player_idx in 0..turn_manager.player_order.len() {
        let player_entity = turn_manager.player_order[player_idx];
        if attackers_by_defender.contains_key(&player_entity) {
            defender_order.push(player_entity);
        }
    }
    
    // Update priority system with defender order
    priority_system.defender_order = defender_order;
}
```

## Special Blocking Rules

### Multiple Blockers

When a single attacker is blocked by multiple creatures:

```rust
pub fn handle_multiple_blockers(
    mut combat_system: ResMut<CombatSystem>,
    blocking_query: Query<(Entity, &Blocking)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Build a map of attackers to their blockers
    let mut blockers_by_attacker: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
    for (blocker, blocking) in blocking_query.iter() {
        for attacker in &blocking.blocked_attackers {
            blockers_by_attacker.entry(*attacker)
                .or_insert_with(Vec::new)
                .push(blocker);
        }
    }
    
    // Process attackers with multiple blockers
    for (attacker, blockers) in blockers_by_attacker.iter() {
        if blockers.len() > 1 {
            // Attacker's controller declares damage assignment order
            game_events.send(GameEvent::DamageAssignmentOrderNeeded {
                attacker: *attacker,
                blockers: blockers.clone(),
            });
        }
    }
}
```

### Triggered Abilities

When blockers are declared, various triggered abilities might occur:

```rust
pub fn process_block_triggers(
    combat_system: Res<CombatSystem>,
    blocking_query: Query<(Entity, &Blocking)>,
    mut ability_triggers: ResMut<AbilityTriggerQueue>,
    trigger_sources: Query<(Entity, &AbilityTrigger)>,
) {
    // Create attacker to blocker map
    let mut blockers_by_attacker: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
    for (blocker, blocking) in blocking_query.iter() {
        for attacker in &blocking.blocked_attackers {
            blockers_by_attacker.entry(*attacker)
                .or_insert_with(Vec::new)
                .push(blocker);
        }
    }
    
    // Process "when this creature blocks" triggers
    for (blocker, blocking) in blocking_query.iter() {
        if let Ok((entity, trigger)) = trigger_sources.get(blocker) {
            if let TriggerCondition::WhenBlocks = trigger.condition {
                for attacker in &blocking.blocked_attackers {
                    ability_triggers.queue.push_back(AbilityTriggerEvent {
                        source: entity,
                        trigger: trigger.clone(),
                        targets: vec![*attacker], // The attacker is the target
                    });
                }
            }
        }
    }
    
    // Process "when this creature becomes blocked" triggers
    for (attacker, blockers) in blockers_by_attacker.iter() {
        if let Ok((entity, trigger)) = trigger_sources.get(*attacker) {
            if let TriggerCondition::WhenBecomesBlocked = trigger.condition {
                ability_triggers.queue.push_back(AbilityTriggerEvent {
                    source: entity,
                    trigger: trigger.clone(),
                    targets: blockers.clone(), // All blockers are targets
                });
            }
        }
    }
    
    // Process "whenever a creature blocks" triggers
    for (entity, trigger) in trigger_sources.iter() {
        if let TriggerCondition::WheneverCreatureBlocks { controller_only } = trigger.condition {
            let should_trigger = if controller_only {
                // Only trigger for creatures controlled by the same player
                // Implementation details omitted for brevity
                true
            } else {
                // Trigger for any blocking creature
                !blocking_query.is_empty()
            };
            
            if should_trigger {
                ability_triggers.queue.push_back(AbilityTriggerEvent {
                    source: entity,
                    trigger: trigger.clone(),
                    targets: Vec::new(), // No specific targets
                });
            }
        }
    }
}
```

## State Tracking

Once all blockers are declared, we need to update the game state:

```rust
pub fn update_creature_state_on_block(
    mut commands: Commands,
    blocking_query: Query<(Entity, &Blocking, &mut Creature)>,
) {
    // Update all blocking creatures
    for (entity, blocking, mut creature) in blocking_query.iter_mut() {
        // Mark creature as blocking
        creature.blocking = blocking.blocked_attackers.clone();
        
        // Tap the creature if it has vigilance for blocking
        // Note: This is not a standard MTG rule but could be a house rule or special card effect
        if creature.has_ability(CreatureAbility::VigilanceForBlocking) {
            commands.entity(entity).insert(Tapped(true));
        }
    }
}
```

## Edge Cases and Special Interactions

### Banding

Banding is a complex ability that affects blocks:

```rust
pub fn handle_banding(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    blocking_query: Query<(Entity, &Blocking)>,
) {
    // Find all creatures with banding
    let banding_creatures: HashSet<Entity> = creature_query
        .iter()
        .filter_map(|(entity, creature)| {
            if creature.has_ability(CreatureAbility::Banding) {
                Some(entity)
            } else {
                None
            }
        })
        .collect();
    
    // Process attackers blocked by banding creatures
    let mut defenders_control_damage_assignment: HashSet<Entity> = HashSet::new();
    
    for (blocker, blocking) in blocking_query.iter() {
        if banding_creatures.contains(&blocker) {
            // If a banding creature is blocking, the defender controls damage assignment
            for attacker in &blocking.blocked_attackers {
                defenders_control_damage_assignment.insert(*attacker);
            }
        }
    }
    
    // Update combat system
    for attacker in defenders_control_damage_assignment {
        combat_system.add_attacker_flag(attacker, AttackerFlag::DefenderControlsDamageAssignment);
    }
}
```

### Protection

Protection affects blocking in various ways:

```rust
pub fn apply_protection_block_restrictions(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature, &Protection)>,
) {
    for (entity, _, protection) in creature_query.iter() {
        match protection.from {
            ProtectionFrom::Color(color) => {
                // Creature with protection from a color can't be blocked by creatures of that color
                combat_system.add_attacker_restriction(
                    entity,
                    AttackerRestriction::CantBeBlockedBy(BlockerCondition::HasColor(color)),
                );
                
                // Creature with protection can't block creatures of that color
                combat_system.add_block_restriction(
                    entity,
                    BlockRestriction::CantBlockCreature(AttackerCondition::HasColor(color)),
                );
            },
            ProtectionFrom::Type(card_type) => {
                // Similar restrictions for protection from a type
                // Implementation details omitted for brevity
            },
            // Other protection types...
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
    fn test_block_validation() {
        // Test with a creature that can block
        let result = validate_block(
            /* mock entities and combat system */
            Entity::from_raw(1),
            &[Entity::from_raw(2)],
            &CombatSystem::default(),
        );
        assert_eq!(result, None, "Valid block should return None");
        
        // Test with a creature that can't block
        let mut combat_system = CombatSystem::default();
        combat_system.add_block_restriction(
            Entity::from_raw(1),
            BlockRestriction::CantBlock,
        );
        
        let result = validate_block(
            Entity::from_raw(1),
            &[Entity::from_raw(2)],
            &combat_system,
        );
        assert!(result.is_some(), "Invalid block should return an error message");
    }
    
    #[test]
    fn test_flying_restriction() {
        let mut app = App::new();
        app.add_systems(Update, apply_evasion_restrictions);
        
        // Create a flying creature
        let flying_creature = app.world.spawn((
            Creature {
                abilities: vec![CreatureAbility::Flying],
                ..Default::default()
            },
        )).id();
        
        // Set up combat system
        let combat_system = CombatSystem::default();
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Check if flying restriction was applied
        let combat_system = app.world.resource::<CombatSystem>();
        
        let has_flying_restriction = combat_system.attacker_restrictions.get(&flying_creature)
            .map_or(false, |reqs| reqs.iter().any(|req| matches!(req, AttackerRestriction::CantBeBlockedBy(BlockerCondition::NotFlyingOrReach))));
        
        assert!(has_flying_restriction, "Flying creature should have CantBeBlockedBy restriction");
    }
    
    #[test]
    fn test_multiple_blockers() {
        let mut app = App::new();
        app.add_systems(Update, handle_multiple_blockers);
        app.add_event::<GameEvent>();
        
        // Set up an attacker and multiple blockers
        let attacker = app.world.spawn(Creature::default()).id();
        let blocker1 = app.world.spawn((
            Creature::default(),
            Blocking { blocked_attackers: vec![attacker] },
        )).id();
        let blocker2 = app.world.spawn((
            Creature::default(),
            Blocking { blocked_attackers: vec![attacker] },
        )).id();
        
        // Set up combat system
        let combat_system = CombatSystem::default();
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Check for damage assignment order event
        let mut event_reader = app.world.resource_mut::<Events<GameEvent>>();
        let mut found_event = false;
        
        for event in event_reader.iter() {
            if let GameEvent::DamageAssignmentOrderNeeded { attacker: a, blockers } = event {
                if *a == attacker && blockers.contains(&blocker1) && blockers.contains(&blocker2) {
                    found_event = true;
                    break;
                }
            }
        }
        
        assert!(found_event, "Damage assignment order event should be emitted for multiple blockers");
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
    fn test_declare_blockers_workflow() {
        let mut app = App::new();
        
        // Add all relevant systems
        app.add_systems(Update, (
            declare_blockers_system,
            process_block_requirements,
            apply_evasion_restrictions,
            handle_multiple_blockers,
            process_block_triggers,
            update_creature_state_on_block,
        ));
        
        // Set up game state with attackers and defenders
        // Implementation details omitted for brevity
        
        // Simulate player declaring blockers
        app.world.resource_mut::<Events<BlockDeclarationEvent>>().send(
            BlockDeclarationEvent {
                blocker: Entity::from_raw(1),
                attackers: vec![Entity::from_raw(2)],
            }
        );
        
        // Run update to process declarations
        app.update();
        
        // Verify blockers are properly recorded and state is updated
        // Implementation details omitted for brevity
    }
    
    #[test]
    fn test_evasion_abilities_interaction() {
        let mut app = App::new();
        
        // Set up attacker with flying
        let flying_attacker = app.world.spawn((
            Creature {
                abilities: vec![CreatureAbility::Flying],
                ..Default::default()
            },
        )).id();
        
        // Set up potential blockers: one with flying, one with reach, one with neither
        let flying_blocker = app.world.spawn((
            Creature {
                abilities: vec![CreatureAbility::Flying],
                ..Default::default()
            },
        )).id();
        
        let reach_blocker = app.world.spawn((
            Creature {
                abilities: vec![CreatureAbility::Reach],
                ..Default::default()
            },
        )).id();
        
        let normal_blocker = app.world.spawn(Creature::default()).id();
        
        // Set up combat system with the attacker
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(flying_attacker, AttackData {
            attacker: flying_attacker,
            defender: Entity::from_raw(10), // Some player
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        app.insert_resource(combat_system);
        
        // Add relevant systems
        app.add_systems(Update, apply_evasion_restrictions);
        
        // Run the system
        app.update();
        
        // Try to declare blocks with each blocker
        let can_flying_block = validate_block(flying_blocker, &[flying_attacker], &app.world.resource::<CombatSystem>());
        let can_reach_block = validate_block(reach_blocker, &[flying_attacker], &app.world.resource::<CombatSystem>());
        let can_normal_block = validate_block(normal_blocker, &[flying_attacker], &app.world.resource::<CombatSystem>());
        
        // Verify results
        assert_eq!(can_flying_block, None, "Flying creature should be able to block flying attacker");
        assert_eq!(can_reach_block, None, "Reach creature should be able to block flying attacker");
        assert!(can_normal_block.is_some(), "Normal creature should not be able to block flying attacker");
    }
    
    // Additional integration tests...
}
```

## UI Considerations

The UI during the Declare Blockers step needs to clearly communicate various states:

```rust
pub fn update_declare_blockers_ui(
    turn_manager: Res<TurnManager>,
    combat_system: Res<CombatSystem>,
    creature_query: Query<(Entity, &Creature, &Controllable)>,
    attacking_query: Query<Entity, With<Attacking>>,
    blocking_query: Query<(Entity, &Blocking)>,
    mut ui_state: ResMut<UiState>,
) {
    // Only run during Declare Blockers step
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareBlockers) {
        return;
    }
    
    // Update phase display
    ui_state.current_phase_text = "Declare Blockers".to_string();
    
    // Get active player and current player with priority
    let active_player = turn_manager.get_active_player();
    let current_player = turn_manager.get_current_player();
    
    // Highlight attackers
    ui_state.attackers.clear();
    for attacker in attacking_query.iter() {
        ui_state.attackers.insert(attacker);
    }
    
    // Highlight potential blockers for current player
    ui_state.potential_blockers.clear();
    for (entity, creature, controllable) in creature_query.iter() {
        if controllable.controller == current_player && creature.can_block() {
            ui_state.potential_blockers.insert(entity);
            
            // Mark creatures that must block
            if let Some(requirements) = combat_system.block_requirements.get(&entity) {
                if requirements.iter().any(|req| matches!(req, BlockRequirement::MustBlock)) {
                    ui_state.creatures_with_requirements.insert(entity, "Must block if able".to_string());
                }
            }
        }
    }
    
    // Show current block declarations
    ui_state.current_blocks.clear();
    for (blocker, blocking) in blocking_query.iter() {
        ui_state.current_blocks.insert(blocker, blocking.blocked_attackers.clone());
    }
    
    // Highlight legal blocks
    ui_state.legal_blocks.clear();
    for attacker in attacking_query.iter() {
        let legal_blockers = creature_query
            .iter()
            .filter_map(|(entity, creature, controllable)| {
                if controllable.controller == current_player && 
                   validate_block(entity, &[attacker], &combat_system).is_none() {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        
        ui_state.legal_blocks.insert(attacker, legal_blockers);
    }
}
```

## Performance Considerations

1. **Efficient Block Validation**: The validation of blocks should be optimized to avoid redundant checks.

2. **Caching Block Results**: Once block declarations are finalized, the results can be cached for use in subsequent steps.

3. **Minimize Entity Queries**: Group related queries to minimize entity access operations.

4. **Parallel Processing**: For games with many blockers, processing block triggers could be done in parallel.

## Conclusion

The Declare Blockers step is a critical part of the combat phase in Commander, particularly in multiplayer games where multiple players may be defending simultaneously. A robust implementation ensures that all game rules are properly enforced, including evasion abilities, protection effects, and multi-blocker scenarios. By handling block declarations, restrictions, requirements, and triggers correctly, we provide the foundation for accurate combat resolution in the following steps. 