# Commander Damage

## Overview

Commander Damage is a fundamental aspect of the Commander format. Any player who has taken 21 or more combat damage from a single commander loses the game. This unique rule adds an additional win condition and strategic layer to the multiplayer format. The implementation must accurately track, accumulate, and verify commander damage while handling edge cases like commander ownership changes or damage redirection effects.

## Core Implementation

```rust
// Player component with commander damage tracking
#[derive(Component)]
pub struct Player {
    pub life_total: i32,
    pub commander_damage: HashMap<Entity, u32>,
    // Other player state fields omitted
}

// System to process commander damage
pub fn commander_damage_system(
    mut player_query: Query<(Entity, &mut Player)>,
    combat_system: Res<CombatSystem>,
    commanders: Query<Entity, With<Commander>>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Collect any combat damage dealt by commanders
    let commander_damage_events = combat_system.combat_history
        .iter()
        .filter_map(|event| {
            if let CombatEvent::DamageDealt { source, target, amount, is_commander_damage } = event {
                if *is_commander_damage && commanders.contains(*source) {
                    return Some((*source, *target, *amount));
                }
            }
            None
        })
        .collect::<Vec<_>>();
    
    // Process the commander damage
    for (commander, target, amount) in commander_damage_events {
        for (player_entity, mut player) in player_query.iter_mut() {
            if player_entity == target {
                // Apply commander damage
                let previous_damage = player.commander_damage.get(&commander).copied().unwrap_or(0);
                let total_damage = previous_damage + amount;
                player.commander_damage.insert(commander, total_damage);
                
                // Check for commander damage loss condition
                if total_damage >= 21 {
                    game_events.send(GameEvent::PlayerLost {
                        player: player_entity,
                        reason: LossReason::CommanderDamage(commander),
                    });
                }
            }
        }
    }
}
```

## Damage Application System

The combat damage system needs special handling for commander damage:

```rust
pub fn apply_combat_damage_system(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut creature_query: Query<(Entity, &mut Creature, Option<&Commander>)>,
    mut player_query: Query<(Entity, &mut Player)>,
    mut planeswalker_query: Query<(Entity, &mut Planeswalker)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Only run during combat damage step
    if turn_manager.current_phase != Phase::Combat(CombatStep::CombatDamage) {
        return;
    }
    
    // Process each attacker that wasn't blocked
    for (attacker, attack_data) in combat_system.attackers.iter() {
        // Skip attackers that were blocked
        if combat_system.blockers.values().any(|block_data| 
            block_data.blocked_attackers.contains(attacker)) {
            continue;
        }
        
        // Get attacker data
        if let Ok((_, creature, commander)) = creature_query.get(*attacker) {
            let power = creature.power;
            let is_commander = commander.is_some();
            
            // Apply damage to defender
            let defender = attack_data.defender;
            
            // Check if defender is a player
            if let Ok((player_entity, mut player)) = player_query.get_mut(defender) {
                // Apply damage to player
                player.life_total -= power as i32;
                
                // Record damage event
                combat_system.combat_history.push_back(CombatEvent::DamageDealt {
                    source: *attacker,
                    target: player_entity,
                    amount: power,
                    is_commander_damage: is_commander,
                });
                
                // Check if player lost due to life total
                if player.life_total <= 0 {
                    game_events.send(GameEvent::PlayerLost {
                        player: player_entity,
                        reason: LossReason::ZeroLife,
                    });
                }
            }
            // Check if defender is a planeswalker
            else if let Ok((planeswalker_entity, mut planeswalker)) = planeswalker_query.get_mut(defender) {
                // Apply damage to planeswalker
                planeswalker.loyalty -= power as i32;
                
                // Record damage event
                combat_system.combat_history.push_back(CombatEvent::DamageDealt {
                    source: *attacker,
                    target: planeswalker_entity,
                    amount: power,
                    is_commander_damage: false, // Commander damage only applies to players
                });
                
                // Check if planeswalker was destroyed
                if planeswalker.loyalty <= 0 {
                    game_events.send(GameEvent::PlaneswalkerDestroyed {
                        planeswalker: planeswalker_entity,
                    });
                }
            }
        }
    }
    
    // Similar processing for blocked attackers
    // Implementation details omitted for brevity
}
```

## Handling Commander Identity Changes

When a commander changes identity (e.g., due to effects like [Sakashima of a Thousand Faces](https://scryfall.com/card/cmr/89/sakashima-of-a-thousand-faces)), the damage tracking needs to be maintained:

```rust
pub fn handle_commander_identity_change(
    mut commands: Commands,
    mut combat_system: ResMut<CombatSystem>,
    mut player_query: Query<&mut Player>,
    commander_query: Query<(Entity, &Commander, &OriginalEntity)>,
) {
    for (current_entity, commander, original) in commander_query.iter() {
        // If the commander has a different original entity
        if current_entity != original.0 {
            // Update all players' commander damage tracking
            for mut player in player_query.iter_mut() {
                if let Some(damage) = player.commander_damage.remove(&original.0) {
                    player.commander_damage.insert(current_entity, damage);
                }
            }
            
            // Update combat system tracking if needed
            for attack_data in combat_system.attackers.values_mut() {
                if attack_data.attacker == original.0 {
                    attack_data.attacker = current_entity;
                    attack_data.is_commander = true;
                }
            }
        }
    }
}
```

## UI Representation

The commander damage tracking needs to be visually represented to players:

```rust
pub fn update_commander_damage_ui_system(
    player_query: Query<(Entity, &Player)>,
    commander_query: Query<(Entity, &Commander, &Card)>,
    mut ui_state: ResMut<UiState>,
) {
    // Update UI representation of commander damage for each player
    for (player_entity, player) in player_query.iter() {
        let mut commander_damage_display = Vec::new();
        
        for (commander_entity, _, card) in commander_query.iter() {
            let damage = player.commander_damage.get(&commander_entity).copied().unwrap_or(0);
            
            commander_damage_display.push(CommanderDamageDisplay {
                commander_entity,
                commander_name: card.name.clone(),
                damage,
                progress: (damage as f32) / 21.0, // For progress bar visualization
                lethal: damage >= 21,
            });
        }
        
        // Sort by damage amount (highest first)
        commander_damage_display.sort_by(|a, b| b.damage.cmp(&a.damage));
        
        // Update UI state
        ui_state.player_commander_damage.insert(player_entity, commander_damage_display);
    }
}
```

## Edge Cases

### Damage Redirection

Some effects can redirect damage, which needs special handling for commander damage:

```rust
pub fn handle_damage_redirection(
    mut combat_system: ResMut<CombatSystem>,
    redirection_effects: Query<(Entity, &DamageRedirection)>,
) {
    // Check for redirection effects when processing damage events
    let mut redirected_damage = Vec::new();
    
    for event in combat_system.pending_combat_events.iter() {
        if let CombatEvent::DamageDealt { source, target, amount, is_commander_damage } = event {
            // Check for redirection effects affecting this target
            for (effect_entity, redirection) in redirection_effects.iter() {
                if redirection.target == *target {
                    // Redirect the damage
                    redirected_damage.push(CombatEvent::DamageDealt {
                        source: *source,
                        target: redirection.redirect_to,
                        amount: *amount,
                        // Important: Commander damage is preserved ONLY if damage source doesn't change
                        is_commander_damage: *is_commander_damage && redirection.preserve_source,
                    });
                }
            }
        }
    }
    
    // Add redirected damage events to combat history
    for event in redirected_damage {
        combat_system.combat_history.push_back(event);
    }
}
```

### Change of Control

When a commander changes control, the damage tracking needs to be maintained:

```rust
pub fn handle_commander_control_change(
    mut player_query: Query<&mut Player>,
    mut control_change_events: EventReader<ControlChangeEvent>,
    commander_query: Query<Entity, With<Commander>>,
) {
    // Process control change events
    for event in control_change_events.iter() {
        // Only care about commanders changing control
        if commander_query.contains(event.entity) {
            // Commander damage is tracked by entity, so no changes needed to players' tracking
            // The entity's controller changes but damage tracking by entity ID remains the same
            
            // Log the event for record-keeping
            info!("Commander {:?} changed control from {:?} to {:?}", 
                  event.entity, event.old_controller, event.new_controller);
        }
    }
}
```

### Double Strike and Commander Damage

Double strike creatures need special handling for commander damage:

```rust
pub fn handle_double_strike_commander_damage(
    mut combat_system: ResMut<CombatSystem>,
    creature_query: Query<(Entity, &Creature, Option<&Commander>)>,
) {
    // Get all double strike commanders
    let double_strike_commanders = creature_query.iter()
        .filter_map(|(entity, creature, commander)| {
            if commander.is_some() && creature.has_ability(Ability::Keyword(Keyword::DoubleStrike)) {
                Some(entity)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    
    // Special handling for double strike damage events
    let mut double_strike_events = Vec::new();
    
    for event in combat_system.combat_history.iter() {
        if let CombatEvent::DamageDealt { source, target, amount, is_commander_damage } = event {
            if *is_commander_damage && double_strike_commanders.contains(source) {
                // During first strike, we need to ensure this damage is tracked separately
                // from the regular damage that will come after
                if combat_system.active_combat_step == Some(CombatStep::FirstStrike) {
                    double_strike_events.push((*source, *target, *amount));
                }
            }
        }
    }
    
    // Record these events explicitly for commander damage tracking
    for (source, target, amount) in double_strike_events {
        info!("Tracking double strike first strike damage from commander {:?} to {:?}: {}", 
              source, target, amount);
    }
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Test basic commander damage tracking
    #[test]
    fn test_basic_commander_damage() {
        // Setup test environment
        let mut app = App::new();
        app.add_systems(Update, commander_damage_system);
        
        // Create test entities and components
        let commander = app.world.spawn((
            Card::default(),
            Commander,
            Creature { power: 5, toughness: 5, ..Default::default() },
        )).id();
        
        let player = app.world.spawn((
            Player {
                life_total: 40,
                commander_damage: HashMap::new(),
                ..Default::default()
            },
        )).id();
        
        // Create a combat system resource with damage history
        let mut combat_system = CombatSystem::default();
        combat_system.combat_history.push_back(CombatEvent::DamageDealt {
            source: commander,
            target: player,
            amount: 5,
            is_commander_damage: true,
        });
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Verify commander damage was tracked
        let player_entity = app.world.entity(player);
        let player_component = player_entity.get::<Player>().unwrap();
        assert_eq!(player_component.commander_damage.get(&commander), Some(&5));
    }
    
    // Test commander damage loss condition
    #[test]
    fn test_commander_damage_loss() {
        // Setup test environment with 21 commander damage
        // Test code omitted for brevity
        
        // Run the system
        app.update();
        
        // Verify loss event was sent
        let events = app.world.resource::<Events<GameEvent>>();
        let mut event_reader = EventReader::<GameEvent>::from_resource(events);
        let mut found_loss_event = false;
        
        for event in event_reader.iter() {
            if let GameEvent::PlayerLost { player: p, reason: LossReason::CommanderDamage(c) } = event {
                if *p == player && *c == commander {
                    found_loss_event = true;
                    break;
                }
            }
        }
        
        assert!(found_loss_event, "Player loss event not found");
    }
    
    // Test commander damage from multiple commanders
    #[test]
    fn test_multiple_commander_damage() {
        // Setup test with multiple commanders dealing damage
        // Test code omitted for brevity
        
        // Verify each commander's damage is tracked separately
        let player_entity = app.world.entity(player);
        let player_component = player_entity.get::<Player>().unwrap();
        assert_eq!(player_component.commander_damage.get(&commander1), Some(&15));
        assert_eq!(player_component.commander_damage.get(&commander2), Some(&10));
        
        // Verify player hasn't lost yet (no single commander has dealt 21+ damage)
        let events = app.world.resource::<Events<GameEvent>>();
        let mut event_reader = EventReader::<GameEvent>::from_resource(events);
        
        for event in event_reader.iter() {
            if let GameEvent::PlayerLost { player: _, reason: _ } = event {
                panic!("Player should not have lost yet");
            }
        }
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    // Test commander damage through complete combat
    #[test]
    fn test_commander_damage_through_combat() {
        // Setup full game environment
        let mut app = setup_test_game();
        
        // Setup a commander attacking a player
        let commander = /* create commander entity */;
        let player = /* create player entity */;
        
        // Setup the attack
        let mut combat_system = app.world.resource_mut::<CombatSystem>();
        combat_system.attackers.insert(commander, AttackData {
            attacker: commander,
            defender: player,
            is_commander: true,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        
        // Advance through combat damage step
        advance_to_combat_damage(&mut app);
        
        // Run the apply damage system
        app.update();
        
        // Verify commander damage was tracked
        let player_entity = app.world.entity(player);
        let player_component = player_entity.get::<Player>().unwrap();
        
        // Assuming a 5 power commander
        assert_eq!(player_component.commander_damage.get(&commander), Some(&5));
    }
    
    // Test commander damage accumulation over multiple turns
    #[test]
    fn test_commander_damage_accumulation() {
        // Setup game and play multiple turns with commander damage
        // Test code omitted for brevity
        
        // Verify accumulated commander damage
        let player_entity = app.world.entity(player);
        let player_component = player_entity.get::<Player>().unwrap();
        assert_eq!(player_component.commander_damage.get(&commander), Some(&15));
        
        // Deal more damage to hit 21
        // Test code omitted for brevity
        
        // Verify player lost
        let events = app.world.resource::<Events<GameEvent>>();
        let mut event_reader = EventReader::<GameEvent>::from_resource(events);
        let mut found_loss_event = false;
        
        for event in event_reader.iter() {
            if let GameEvent::PlayerLost { player: p, reason: LossReason::CommanderDamage(c) } = event {
                if *p == player && *c == commander {
                    found_loss_event = true;
                    break;
                }
            }
        }
        
        assert!(found_loss_event, "Player loss event not found");
    }
    
    // Test commander damage with protection/prevention
    #[test]
    fn test_commander_damage_prevention() {
        // Setup game with protection effects
        // Test code omitted for brevity
        
        // Verify commander damage was prevented
        let player_entity = app.world.entity(player);
        let player_component = player_entity.get::<Player>().unwrap();
        assert_eq!(player_component.commander_damage.get(&commander), Some(&0));
    }
}
```

## Performance Considerations

1. **HashMap Efficiency**: Commander damage is tracked using a HashMap which provides O(1) lookups, but care should be taken to avoid unnecessary cloning or rebuilding.

2. **Event Processing**: Combat damage events are processed in batches to minimize system overhead.

3. **UI Updates**: Commander damage UI updates should only happen when damage values change or during specific UI refresh cycles to avoid performance impact.

## Implementation Recommendations

1. **Persistent Storage**: In a networked game, commander damage should be persisted on the server and synchronized to clients.

2. **Undo Support**: The system should support undoing or adjusting commander damage in case of rule disputes or corrections.

3. **History Tracking**: Maintain a searchable history of commander damage events for game replay and verification.

4. **Visual Feedback**: Provide clear visual feedback when commander damage is getting close to lethal levels (e.g., at 15+ damage).

Commander damage tracking is a critical component of the Commander format and requires careful implementation to ensure game rules are correctly enforced while maintaining good performance even in complex game states. 