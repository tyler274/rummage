# Combat Abilities

## Overview

Magic: The Gathering includes numerous abilities that specifically interact with combat. In Commander, these abilities gain additional complexity due to the multiplayer nature of the format and the special rules surrounding commander damage. This document details how combat-specific abilities are implemented and tested in our game engine.

## Combat Keywords

Combat keywords are implemented through a combination of component flags and systems that check for these keywords during the appropriate combat steps.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    // Combat-specific keywords
    FirstStrike,
    DoubleStrike,
    Deathtouch,
    Trample,
    Vigilance,
    Menace,
    Flying,
    Reach,
    Indestructible,
    Lifelink,
    // Other keywords...
}

// Component that stores a creature's abilities
#[derive(Component, Clone)]
pub struct Abilities(pub Vec<Ability>);

// Different ability types
#[derive(Debug, Clone)]
pub enum Ability {
    Keyword(Keyword),
    Triggered(Trigger, Effect),
    Activated(ActivationCost, Effect),
    Static(StaticEffect),
    // Other ability types...
}

// Implement utility functions for checking abilities
impl Creature {
    pub fn has_ability(&self, ability: Ability) -> bool {
        // Implementation details omitted for brevity
        false
    }
    
    pub fn has_keyword(&self, keyword: Keyword) -> bool {
        // Check if the creature has the specified keyword
        if let Ok(abilities) = abilities_query.get(self.entity) {
            abilities.0.iter().any(|ability| {
                if let Ability::Keyword(kw) = ability {
                    *kw == keyword
                } else {
                    false
                }
            })
        } else {
            false
        }
    }
}
```

## Keyword Implementation Systems

Each combat keyword has a dedicated system that implements its effect:

### First Strike and Double Strike

```rust
pub fn first_strike_damage_system(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    creature_query: Query<(Entity, &Creature)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Only run during the first strike damage step
    if turn_manager.current_phase != Phase::Combat(CombatStep::FirstStrike) {
        return;
    }
    
    // Get all attackers with first strike or double strike
    let first_strike_attackers = combat_system.attackers
        .iter()
        .filter_map(|(attacker, attack_data)| {
            if let Ok((entity, creature)) = creature_query.get(*attacker) {
                if creature.has_keyword(Keyword::FirstStrike) || 
                   creature.has_keyword(Keyword::DoubleStrike) {
                    return Some((attacker, attack_data));
                }
            }
            None
        })
        .collect::<Vec<_>>();
    
    // Get all blockers with first strike or double strike
    let first_strike_blockers = combat_system.blockers
        .iter()
        .filter_map(|(blocker, block_data)| {
            if let Ok((entity, creature)) = creature_query.get(*blocker) {
                if creature.has_keyword(Keyword::FirstStrike) || 
                   creature.has_keyword(Keyword::DoubleStrike) {
                    return Some((blocker, block_data));
                }
            }
            None
        })
        .collect::<Vec<_>>();
    
    // Process first strike combat damage
    // Implementation details omitted for brevity
    
    // Emit event for first strike damage
    game_events.send(GameEvent::FirstStrikeDamageDealt {
        attackers: first_strike_attackers.len(),
        blockers: first_strike_blockers.len(),
    });
}
```

### Deathtouch

```rust
pub fn apply_deathtouch_system(
    combat_system: Res<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    mut commands: Commands,
) {
    // Find all creatures with deathtouch that dealt damage
    let deathtouch_creatures = combat_system.combat_history
        .iter()
        .filter_map(|event| {
            if let CombatEvent::DamageDealt { source, target, amount, .. } = event {
                if *amount > 0 {
                    if let Ok((entity, creature)) = creature_query.get(*source) {
                        if creature.has_keyword(Keyword::Deathtouch) {
                            return Some((source, target));
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    
    // Apply destroy effect to creatures that were damaged by deathtouch
    for (source, target) in deathtouch_creatures {
        // Only apply to creatures, not players or planeswalkers
        if creature_query.contains(*target) {
            // Mark creature as destroyed
            commands.entity(*target).insert(Destroyed {
                source: *source,
                reason: DestructionReason::Deathtouch,
            });
        }
    }
}
```

### Trample

```rust
pub fn apply_trample_damage_system(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    blocker_toughness_query: Query<&Creature>,
    player_query: Query<(Entity, &mut Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Find all creatures with trample that are being blocked
    for (attacker, attack_data) in combat_system.attackers.iter() {
        // Only process if the attacker has trample
        if let Ok((entity, creature)) = creature_query.get(*attacker) {
            if !creature.has_keyword(Keyword::Trample) {
                continue;
            }
            
            // Check if this attacker is blocked
            let blockers: Vec<Entity> = combat_system.blockers.iter()
                .filter_map(|(blocker, block_data)| {
                    if block_data.blocked_attackers.contains(attacker) {
                        Some(*blocker)
                    } else {
                        None
                    }
                })
                .collect();
            
            // Only apply trample if the creature is blocked
            if !blockers.is_empty() {
                // Calculate total blocker toughness
                let total_blocker_toughness: u32 = blockers.iter()
                    .filter_map(|blocker| {
                        blocker_toughness_query.get(*blocker).ok().map(|creature| creature.toughness)
                    })
                    .sum();
                
                // Calculate trample damage (attacker power - total blocker toughness)
                let trample_damage = creature.power.saturating_sub(total_blocker_toughness);
                
                // Apply trample damage to defending player if there's excess damage
                if trample_damage > 0 {
                    // Only apply to players, not planeswalkers
                    for (player_entity, mut player) in player_query.iter_mut() {
                        if player_entity == attack_data.defender {
                            // Apply the damage
                            player.life_total -= trample_damage as i32;
                            
                            // Record the damage event
                            combat_system.combat_history.push_back(CombatEvent::DamageDealt {
                                source: *attacker,
                                target: player_entity,
                                amount: trample_damage,
                                is_commander_damage: attack_data.is_commander,
                            });
                            
                            // Check if player lost from trample damage
                            if player.life_total <= 0 {
                                game_events.send(GameEvent::PlayerLost {
                                    player: player_entity,
                                    reason: LossReason::ZeroLife,
                                });
                            }
                            
                            break;
                        }
                    }
                }
            }
        }
    }
}
```

### Flying and Reach

```rust
pub fn validate_blocks_flying_system(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    mut block_events: EventWriter<BlockValidationEvent>,
) {
    // Check for illegal blocks involving flying creatures
    let mut illegal_blocks = Vec::new();
    
    for (blocker, block_data) in combat_system.blockers.iter() {
        // Get blocker details
        if let Ok((blocker_entity, blocker_creature)) = creature_query.get(*blocker) {
            // Check if blocker has reach or flying
            let can_block_flying = blocker_creature.has_keyword(Keyword::Flying) || 
                                  blocker_creature.has_keyword(Keyword::Reach);
            
            // Check all attackers this creature is blocking
            for attacker in &block_data.blocked_attackers {
                if let Ok((attacker_entity, attacker_creature)) = creature_query.get(*attacker) {
                    // If attacker has flying and blocker can't block flying, this is illegal
                    if attacker_creature.has_keyword(Keyword::Flying) && !can_block_flying {
                        illegal_blocks.push((*blocker, *attacker));
                    }
                }
            }
        }
    }
    
    // Remove illegal blocks
    for (blocker, attacker) in illegal_blocks {
        // Remove the block relationship
        if let Some(mut block_data) = combat_system.blockers.get_mut(&blocker) {
            block_data.blocked_attackers.retain(|a| *a != attacker);
            
            // If no more attackers, remove blocker entry
            if block_data.blocked_attackers.is_empty() {
                combat_system.blockers.remove(&blocker);
            }
        }
        
        // Emit event for illegal block
        block_events.send(BlockValidationEvent::IllegalBlock {
            blocker,
            attacker,
            reason: "Can't block flying".to_string(),
        });
    }
}
```

### Menace

```rust
pub fn validate_blocks_menace_system(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    mut block_events: EventWriter<BlockValidationEvent>,
) {
    // Find attackers with menace that are being blocked by only one creature
    let menace_violations = combat_system.attackers
        .iter()
        .filter_map(|(attacker, attack_data)| {
            // Check if attacker has menace
            if let Ok((entity, creature)) = creature_query.get(*attacker) {
                if creature.has_keyword(Keyword::Menace) {
                    // Count blockers for this attacker
                    let blocker_count = combat_system.blockers
                        .values()
                        .filter(|block_data| block_data.blocked_attackers.contains(attacker))
                        .count();
                    
                    // Menace requires at least 2 blockers
                    if blocker_count == 1 {
                        // Find the single blocker
                        let blocker = combat_system.blockers
                            .iter()
                            .find_map(|(blocker, block_data)| {
                                if block_data.blocked_attackers.contains(attacker) {
                                    Some(*blocker)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        
                        return Some((*attacker, blocker));
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    
    // Remove blocks that violate menace
    for (attacker, blocker) in menace_violations {
        // Remove the block relationship
        if let Some(mut block_data) = combat_system.blockers.get_mut(&blocker) {
            block_data.blocked_attackers.retain(|a| *a != attacker);
            
            // If no more attackers, remove blocker entry
            if block_data.blocked_attackers.is_empty() {
                combat_system.blockers.remove(&blocker);
            }
        }
        
        // Emit event for illegal block
        block_events.send(BlockValidationEvent::IllegalBlock {
            blocker,
            attacker,
            reason: "Menace requires at least two blockers".to_string(),
        });
    }
}
```

### Vigilance

```rust
pub fn apply_vigilance_system(
    combat_system: Res<CombatSystem>,
    creature_query: Query<(Entity, &Creature)>,
    mut commands: Commands,
) {
    // Find attackers with vigilance and ensure they don't get tapped
    for (attacker, _) in combat_system.attackers.iter() {
        if let Ok((entity, creature)) = creature_query.get(*attacker) {
            if creature.has_keyword(Keyword::Vigilance) {
                // Remove the Tapped component if it exists
                commands.entity(*attacker).remove::<Tapped>();
            }
        }
    }
}
```

## Triggered Combat Abilities

Triggered abilities are a major part of combat in Magic. These are implemented using a trigger system:

```rust
#[derive(Debug, Clone)]
pub enum Trigger {
    // Combat-related triggers
    WhenAttacks { conditions: Vec<TriggerCondition> },
    WhenBlocks { conditions: Vec<TriggerCondition> },
    WhenBlocked { conditions: Vec<TriggerCondition> },
    WhenDealsCombat { conditions: Vec<TriggerCondition> },
    WheneverCreatureAttacks { conditions: Vec<TriggerCondition> },
    WheneverCreatureBlocks { conditions: Vec<TriggerCondition> },
    BeginningOfCombat { controller_condition: ControllerCondition },
    EndOfCombat { controller_condition: ControllerCondition },
    // Other triggers...
}

pub fn combat_trigger_system(
    combat_system: Res<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut stack: ResMut<Stack>,
    entity_query: Query<(Entity, &Card, &Abilities)>,
) {
    // Only run during the appropriate combat step
    if !matches!(turn_manager.current_phase, 
        Phase::Combat(CombatStep::DeclareAttackers) | 
        Phase::Combat(CombatStep::DeclareBlockers) | 
        Phase::Combat(CombatStep::CombatDamage)) {
        return;
    }
    
    // Collect triggered abilities based on the current combat step
    let mut triggered_abilities = Vec::new();
    
    // Check all entities with abilities
    for (entity, card, abilities) in entity_query.iter() {
        for ability in &abilities.0 {
            if let Ability::Triggered(trigger, effect) = ability {
                match trigger {
                    // Check if we're in declare attackers and this is a "when attacks" trigger
                    Trigger::WhenAttacks { conditions } 
                        if turn_manager.current_phase == Phase::Combat(CombatStep::DeclareAttackers) => {
                        // Check if this entity is attacking
                        if let Some(attack_data) = combat_system.attackers.get(&entity) {
                            // Check if conditions are met
                            if all_conditions_met(conditions, entity, attack_data.defender) {
                                triggered_abilities.push((entity, effect.clone(), get_controller(entity)));
                            }
                        }
                    },
                    
                    // Similar checks for other triggers...
                    _ => {}
                }
            }
        }
    }
    
    // Add triggered abilities to the stack in APNAP order
    let active_player = turn_manager.get_active_player();
    let ordered_triggers = order_triggers_by_apnap(triggered_abilities, active_player);
    
    for (source, effect, controller) in ordered_triggers {
        stack.push(StackItem::Ability {
            source,
            effect,
            controller,
        });
    }
}
```

## Special Combat Abilities

### Ninjutsu

```rust
pub fn handle_ninjutsu_system(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut commands: Commands,
    card_query: Query<(Entity, &Card, &Abilities, Option<&Commander>)>,
    mut activation_events: EventReader<NinjutsuActivationEvent>,
    mut creature_query: Query<(Entity, &mut Creature)>,
) {
    // Only active during declare attackers step after attacks are declared
    if turn_manager.current_phase != Phase::Combat(CombatStep::DeclareAttackers) 
       || combat_system.attackers.is_empty() {
        return;
    }
    
    // Process ninjutsu activations
    for event in activation_events.iter() {
        if let Ok((entity, card, abilities, commander)) = card_query.get(event.ninja) {
            // Verify card has ninjutsu or commander ninjutsu
            let has_ninjutsu = abilities.0.iter().any(|ability| {
                matches!(ability, Ability::Activated(ActivationCost::Ninjutsu(_), _))
            });
            
            let has_commander_ninjutsu = commander.is_some() && abilities.0.iter().any(|ability| {
                matches!(ability, Ability::Activated(ActivationCost::CommanderNinjutsu(_), _))
            });
            
            if has_ninjutsu || has_commander_ninjutsu {
                // Verify unblocked attacker
                if !combat_system.attackers.contains_key(&event.unblocked_attacker) {
                    continue;
                }
                
                // Verify attacker is unblocked
                let is_blocked = combat_system.blockers.values().any(|block_data| {
                    block_data.blocked_attackers.contains(&event.unblocked_attacker)
                });
                
                if is_blocked {
                    continue;
                }
                
                // Get defender
                let defender = combat_system.attackers[&event.unblocked_attacker].defender;
                
                // Return unblocked attacker to hand
                // Implementation details omitted for brevity
                
                // Put ninja onto battlefield tapped and attacking
                // Implementation details omitted for brevity
                
                // Update combat system
                combat_system.attackers.remove(&event.unblocked_attacker);
                combat_system.attackers.insert(event.ninja, AttackData {
                    attacker: event.ninja,
                    defender,
                    is_commander: commander.is_some(),
                    requirements: Vec::new(),
                    restrictions: Vec::new(),
                });
            }
        }
    }
}
```

### Protection

```rust
pub fn apply_protection_system(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature, &ActiveEffects)>,
    mut combat_events: EventWriter<CombatEvent>,
) {
    // Check for protection effects
    let mut invalid_blocks = Vec::new();
    let mut prevented_damage = Vec::new();
    
    // Check blocks against protection
    for (blocker, block_data) in combat_system.blockers.iter() {
        if let Ok((blocker_entity, _, effects)) = creature_query.get(*blocker) {
            for attacker in &block_data.blocked_attackers {
                if has_protection_from(*blocker, *attacker, &creature_query) {
                    invalid_blocks.push((*blocker, *attacker));
                }
            }
        }
    }
    
    // Check damage against protection
    let damage_events = combat_system.combat_history
        .iter()
        .filter_map(|event| {
            if let CombatEvent::DamageDealt { source, target, amount, is_commander_damage } = event {
                Some((*source, *target, *amount, *is_commander_damage))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    for (source, target, amount, is_commander_damage) in damage_events {
        if has_protection_from(target, source, &creature_query) {
            prevented_damage.push((source, target, amount, is_commander_damage));
        }
    }
    
    // Remove invalid blocks
    for (blocker, attacker) in invalid_blocks {
        if let Some(mut block_data) = combat_system.blockers.get_mut(&blocker) {
            block_data.blocked_attackers.retain(|a| *a != attacker);
        }
    }
    
    // Record damage prevention events
    for (source, target, amount, is_commander_damage) in prevented_damage {
        combat_events.send(CombatEvent::DamagePrevented {
            source,
            target,
            amount,
            reason: PreventionReason::Protection,
        });
    }
}

// Helper function to check if an entity has protection from another
fn has_protection_from(
    protected: Entity,
    source: Entity,
    creature_query: &Query<(Entity, &Creature, &ActiveEffects)>,
) -> bool {
    if let Ok((_, _, effects)) = creature_query.get(protected) {
        for effect in &effects.0 {
            match effect {
                Effect::ProtectionFromColor(color) => {
                    // Check if source has the protected color
                    if let Ok((_, source_creature, _)) = creature_query.get(source) {
                        if source_creature.colors.contains(color) {
                            return true;
                        }
                    }
                },
                Effect::ProtectionFromCreatureType(creature_type) => {
                    // Check if source has the protected creature type
                    if let Ok((_, source_creature, _)) = creature_query.get(source) {
                        if source_creature.types.contains(creature_type) {
                            return true;
                        }
                    }
                },
                Effect::ProtectionFromPlayer(player) => {
                    // Check if source is controlled by the protected player
                    // Implementation details omitted for brevity
                },
                Effect::ProtectionFromEverything => {
                    return true;
                },
                _ => {}
            }
        }
    }
    
    false
}
```

## Edge Cases

### Multiple Combats

Some cards create additional combat phases. These need special handling:

```rust
pub fn handle_additional_combat_phases(
    mut turn_manager: ResMut<TurnManager>,
    mut combat_system: ResMut<CombatSystem>,
    mut creature_query: Query<(Entity, &mut Creature)>,
    mut phase_events: EventReader<PhaseEvent>,
) {
    // Check for additional combat phase events
    for event in phase_events.iter() {
        if let PhaseEvent::AdditionalPhase { phase, source } = event {
            if matches!(phase, Phase::Combat(_)) {
                // Reset combat state for the new phase
                combat_system.reset();
                combat_system.is_additional_combat_phase = true;
                
                // Reset attacked/blocked flags on creatures
                for (entity, mut creature) in creature_query.iter_mut() {
                    creature.attacking = None;
                    creature.blocking.clear();
                }
                
                // Update turn manager
                turn_manager.additional_phases.push_back((*phase, *source));
            }
        }
    }
}
```

### Changing Abilities During Combat

Creatures can gain or lose abilities during combat:

```rust
pub fn update_combat_abilities_system(
    mut creature_query: Query<(Entity, &mut Creature, &ActiveEffects)>,
    mut combat_system: ResMut<CombatSystem>,
    mut ability_events: EventReader<AbilityChangeEvent>,
) {
    // Process ability change events
    for event in ability_events.iter() {
        if let AbilityChangeEvent::GainedKeyword { entity, keyword } = event {
            // Update entity if it's involved in combat
            if combat_system.attackers.contains_key(entity) || 
               combat_system.blockers.contains_key(entity) {
                // Special handling for combat-relevant keywords
                match keyword {
                    Keyword::FirstStrike | Keyword::DoubleStrike => {
                        // May need to recalculate damage for first strike step
                        if combat_system.active_combat_step == Some(CombatStep::DeclareBlockers) {
                            // Flag that first strike needs to be checked
                            combat_system.first_strike_recalculation_needed = true;
                        }
                    },
                    Keyword::Flying => {
                        // May need to recalculate blocks for flying
                        if let Some(block_data) = combat_system.blockers.get(entity) {
                            // Check if this creature is now blocking a flyer illegally
                            // Implementation details omitted for brevity
                        }
                    },
                    Keyword::Vigilance => {
                        // If creature already attacked, untap it
                        if combat_system.attackers.contains_key(entity) {
                            // Implementation details omitted for brevity
                        }
                    },
                    // Handle other keywords...
                    _ => {}
                }
            }
        }
        else if let AbilityChangeEvent::LostKeyword { entity, keyword } = event {
            // Similar handling for losing keywords
            // Implementation details omitted for brevity
        }
    }
}
```

## Testing Strategy

### Unit Tests for Combat Abilities

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deathtouch() {
        let mut app = App::new();
        app.add_systems(Update, apply_deathtouch_system);
        
        // Setup test entities with deathtouch
        let attacker = app.world.spawn((
            Card::default(),
            Creature { power: 1, toughness: 1, ..Default::default() },
            Abilities(vec![Ability::Keyword(Keyword::Deathtouch)]),
        )).id();
        
        let blocker = app.world.spawn((
            Card::default(),
            Creature { power: 5, toughness: 5, ..Default::default() },
        )).id();
        
        // Create combat history with damage event
        let mut combat_system = CombatSystem::default();
        combat_system.combat_history.push_back(CombatEvent::DamageDealt {
            source: attacker,
            target: blocker,
            amount: 1,
            is_commander_damage: false,
        });
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Verify blocker was destroyed by deathtouch
        assert!(app.world.entity(blocker).contains::<Destroyed>());
    }
    
    #[test]
    fn test_trample() {
        let mut app = App::new();
        app.add_systems(Update, apply_trample_damage_system);
        
        // Setup test entities
        let player = app.world.spawn((
            Player { life_total: 40, commander_damage: HashMap::new(), ..Default::default() },
        )).id();
        
        let attacker = app.world.spawn((
            Card::default(),
            Creature { power: 5, toughness: 5, ..Default::default() },
            Abilities(vec![Ability::Keyword(Keyword::Trample)]),
        )).id();
        
        let blocker = app.world.spawn((
            Card::default(),
            Creature { power: 2, toughness: 2, ..Default::default() },
        )).id();
        
        // Create combat system with attacker and blocker
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(attacker, AttackData {
            attacker,
            defender: player,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        combat_system.blockers.insert(blocker, BlockData {
            blocker,
            blocked_attackers: vec![attacker],
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Verify trample damage was dealt to player (5 power - 2 toughness = 3 damage)
        let player_component = app.world.entity(player).get::<Player>().unwrap();
        assert_eq!(player_component.life_total, 40 - 3);
        
        // Verify combat history contains damage event
        let combat_system = app.world.resource::<CombatSystem>();
        assert!(combat_system.combat_history.iter().any(|event| {
            matches!(event, CombatEvent::DamageDealt { source, target, amount, .. } 
                if *source == attacker && *target == player && *amount == 3)
        }));
    }
    
    #[test]
    fn test_flying_and_reach() {
        let mut app = App::new();
        app.add_systems(Update, validate_blocks_flying_system);
        
        // Setup test entities
        let flying_creature = app.world.spawn((
            Card::default(),
            Creature { power: 3, toughness: 3, ..Default::default() },
            Abilities(vec![Ability::Keyword(Keyword::Flying)]),
        )).id();
        
        let normal_creature = app.world.spawn((
            Card::default(),
            Creature { power: 2, toughness: 2, ..Default::default() },
        )).id();
        
        let reach_creature = app.world.spawn((
            Card::default(),
            Creature { power: 1, toughness: 4, ..Default::default() },
            Abilities(vec![Ability::Keyword(Keyword::Reach)]),
        )).id();
        
        // Create combat system with illegal block
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(flying_creature, AttackData {
            attacker: flying_creature,
            defender: Entity::from_raw(999), // Dummy defender
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        combat_system.blockers.insert(normal_creature, BlockData {
            blocker: normal_creature,
            blocked_attackers: vec![flying_creature],
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        combat_system.blockers.insert(reach_creature, BlockData {
            blocker: reach_creature,
            blocked_attackers: vec![flying_creature],
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        app.insert_resource(combat_system);
        
        // Set up event listener
        app.add_event::<BlockValidationEvent>();
        
        // Run the system
        app.update();
        
        // Verify normal creature can't block the flyer
        let combat_system = app.world.resource::<CombatSystem>();
        assert!(!combat_system.blockers.get(&normal_creature)
            .map_or(false, |data| data.blocked_attackers.contains(&flying_creature)));
        
        // Verify reach creature can still block the flyer
        assert!(combat_system.blockers.get(&reach_creature)
            .map_or(false, |data| data.blocked_attackers.contains(&flying_creature)));
    }
    
    // Additional tests...
}
```

### Integration Tests for Combat Abilities

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_keyword_interactions() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Setup a combat scenario with multiple keyword interactions
        let opponent = builder.add_player();
        
        // Flying attacker with lifelink and double strike
        let attacker = builder.add_attacker(3, 3, builder.active_player, false);
        builder.add_effect(attacker, Effect::Keyword(Keyword::Flying));
        builder.add_effect(attacker, Effect::Keyword(Keyword::Lifelink));
        builder.add_effect(attacker, Effect::Keyword(Keyword::DoubleStrike));
        
        // Normal blocker can't block flying
        let blocker1 = builder.add_blocker(2, 2, opponent);
        
        // Reach blocker can block flying
        let blocker2 = builder.add_blocker(1, 1, opponent);
        builder.add_effect(blocker2, Effect::Keyword(Keyword::Reach));
        
        // Set up attacks and blocks
        builder.declare_attacks(vec![(attacker, opponent)]);
        builder.declare_blocks(vec![(blocker2, vec![attacker])]);
        
        // Execute combat
        let result = builder.execute();
        
        // Verify: 
        // 1. First strike damage killed blocker2
        // 2. Double strike allowed second hit to player
        // 3. Lifelink gained life
        
        // Check blocker died
        let blocker2_status = result.creature_status.get(&blocker2).unwrap();
        assert!(blocker2_status.destroyed);
        
        // Check player took damage from second hit (double strike)
        assert_eq!(result.player_life[&opponent], 40 - 3);
        
        // Check controller gained life from lifelink
        let active_player_life = result.player_life[&builder.active_player];
        assert_eq!(active_player_life, 40 + 6); // 3 damage twice from double strike
    }
    
    #[test]
    fn test_triggered_abilities() {
        // Test implementation for combat-triggered abilities
        // Implementation details omitted for brevity
    }
    
    #[test]
    fn test_ninjutsu() {
        // Test implementation for ninjutsu
        // Implementation details omitted for brevity
    }
    
    // Additional integration tests...
}
```

## Combos and Interactions

The combat system needs to handle complex ability interactions:

```rust
pub fn handle_ability_interactions(
    combat_system: Res<CombatSystem>,
    entity_query: Query<(Entity, &Creature, &ActiveEffects)>,
    mut destroy_events: EventWriter<DestroyEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    // Track relevant ability combinations
    let mut indestructible_entities = HashSet::new();
    let mut damage_redirection_map = HashMap::new();
    let mut regeneration_shields = HashMap::new();
    
    // Collect all combat-relevant abilities
    for (entity, _, effects) in entity_query.iter() {
        for effect in &effects.0 {
            match effect {
                Effect::Indestructible => {
                    indestructible_entities.insert(entity);
                },
                Effect::RedirectDamage { target } => {
                    damage_redirection_map.insert(entity, *target);
                },
                Effect::RegenerationShield => {
                    regeneration_shields.entry(entity).or_insert(0) += 1;
                },
                // Handle other effects
                _ => {}
            }
        }
    }
    
    // Process combat damage events considering special abilities
    for event in combat_system.combat_history.iter() {
        if let CombatEvent::DamageDealt { source, target, amount, is_commander_damage } = event {
            // Check for damage redirection
            let actual_target = damage_redirection_map.get(target).copied().unwrap_or(*target);
            
            // Check if damage would be lethal
            if let Ok((_, creature, _)) = entity_query.get(actual_target) {
                if creature.toughness <= creature.damage + amount {
                    // Check for indestructible
                    if indestructible_entities.contains(&actual_target) {
                        // Creature survives despite lethal damage
                        continue;
                    }
                    
                    // Check for regeneration shield
                    if let Some(shields) = regeneration_shields.get_mut(&actual_target) {
                        if *shields > 0 {
                            // Use a regeneration shield
                            *shields -= 1;
                            
                            // Emit regeneration event
                            // Implementation details omitted
                            
                            continue;
                        }
                    }
                    
                    // No protection, creature is destroyed
                    destroy_events.send(DestroyEvent {
                        entity: actual_target,
                        source: *source,
                        reason: DestructionReason::LethalDamage,
                    });
                }
            }
        }
    }
}
```

## Conclusion

Combat abilities add significant complexity to the Commander format, requiring careful implementation and testing. The systems described here work together to handle the various keywords, triggers, and special cases that can arise during combat. By properly implementing and testing these abilities, we ensure that the combat system correctly follows the rules of Magic: The Gathering while providing the strategic depth that makes Commander such a popular format. 