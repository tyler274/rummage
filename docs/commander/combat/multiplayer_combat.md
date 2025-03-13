# Multiplayer Combat

## Overview

Commander is inherently a multiplayer format, which introduces unique complexities to the combat system. Unlike one-on-one matches, multiplayer combat allows a player to attack multiple opponents simultaneously and introduces political elements that affect combat decisions. This document details how the multiplayer aspects of combat are implemented, tested, and verified in our game engine.

## Core Multiplayer Combat Features

### Multiple Attack Targets

In Commander, the active player can declare attackers against different opponents in the same combat phase. This is implemented through the attack declaration system:

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
        if !player_query.contains(*defender) {
            continue;
        }
        
        // Verify defender is not the active player
        if *defender == active_player {
            game_events.send(GameEvent::InvalidAttackTarget {
                attacker: attackers[0], // Just report one of the attackers
                defender: *defender,
                reason: "Cannot attack yourself".to_string(),
            });
        }
    }
    
    // Log all attack declarations for game history
    game_events.send(GameEvent::MultiplayerAttacksDeclared {
        active_player,
        attacks_by_defender: attacks_by_defender.clone(),
    });
}
```

### Defending Player Choice

When a player is being attacked, they alone make blocking decisions for those attackers:

```rust
pub fn handle_multiplayer_blocks(
    mut combat_system: ResMut<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut block_events: EventReader<BlockDeclarationEvent>,
    player_query: Query<Entity, With<Player>>,
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
    
    // Process block declarations from each defending player
    for event in block_events.iter() {
        let BlockDeclarationEvent { player, blocker, attacker } = event;
        
        // Verify the player is declaring blocks against attackers targeting them
        if let Some(attackers) = attackers_by_defender.get(player) {
            if attackers.contains(attacker) {
                // This is a valid block declaration
                if let Some(block_data) = combat_system.blockers.get_mut(blocker) {
                    block_data.blocked_attackers.push(*attacker);
                } else {
                    combat_system.blockers.insert(*blocker, BlockData {
                        blocker: *blocker,
                        blocked_attackers: vec![*attacker],
                        requirements: Vec::new(),
                        restrictions: Vec::new(),
                    });
                }
            } else {
                // Player trying to block an attacker not targeting them
                warn!("Player {:?} tried to block attacker {:?} not targeting them", player, attacker);
            }
        }
    }
}
```

## Political Game Elements

### Goad Mechanic

Goad is a Commander-focused mechanic that forces creatures to attack players other than you. This is implemented as follows:

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

pub fn cleanup_goad_effects(
    mut commands: Commands,
    goaded_query: Query<(Entity, &Goaded)>,
    turn_manager: Res<TurnManager>,
) {
    // Only run during end step
    if turn_manager.current_phase != Phase::Ending(EndingStep::End) {
        return;
    }
    
    // Remove goad effects that last until end of turn
    for (entity, goaded) in goaded_query.iter() {
        if goaded.until_end_of_turn {
            commands.entity(entity).remove::<Goaded>();
        }
    }
}
```

### Monarch Mechanic

The monarch is another multiplayer-focused mechanic that encourages combat:

```rust
#[derive(Resource)]
pub struct Monarch(pub Option<Entity>);

pub fn monarch_attack_trigger(
    mut monarch: ResMut<Monarch>,
    combat_system: Res<CombatSystem>,
    turn_manager: Res<TurnManager>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Only check at combat damage step
    if turn_manager.current_phase != Phase::Combat(CombatStep::CombatDamage) {
        return;
    }
    
    // Find players who were dealt combat damage
    let mut damaged_players = HashSet::new();
    
    for event in combat_system.combat_history.iter() {
        if let CombatEvent::DamageDealt { source, target, amount, .. } = event {
            if *amount > 0 && combat_system.attackers.contains_key(source) {
                damaged_players.insert(*target);
            }
        }
    }
    
    // Check if the monarch was dealt damage
    if let Some(current_monarch) = monarch.0 {
        if damaged_players.contains(&current_monarch) {
            // Find who dealt damage to the monarch
            for event in combat_system.combat_history.iter() {
                if let CombatEvent::DamageDealt { source, target, amount, .. } = event {
                    if *amount > 0 && *target == current_monarch && combat_system.attackers.contains_key(source) {
                        // Get the controller of the attacking creature
                        if let Some(controller) = get_controller(*source) {
                            if controller != current_monarch {
                                // Change the monarch
                                monarch.0 = Some(controller);
                                
                                game_events.send(GameEvent::MonarchChanged {
                                    old_monarch: current_monarch,
                                    new_monarch: controller,
                                    reason: "Combat damage".to_string(),
                                });
                                
                                // Only change once (first damage)
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## Multiplayer Damage Tracking

In multiplayer games, damage source tracking becomes more important, especially for commander damage:

```rust
pub fn track_multiplayer_combat_damage(
    combat_system: Res<CombatSystem>,
    mut player_query: Query<(Entity, &mut Player)>,
    commander_query: Query<Entity, With<Commander>>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Extract all combat damage events
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
    
    // Process commander damage
    for (source, target, amount, is_commander_damage) in damage_events {
        // Only process if the source is a commander and target is a player
        if is_commander_damage && commander_query.contains(source) {
            for (player_entity, mut player) in player_query.iter_mut() {
                if player_entity == target {
                    // Update commander damage tracking
                    let previous_damage = player.commander_damage.get(&source).copied().unwrap_or(0);
                    let new_damage = previous_damage + amount;
                    player.commander_damage.insert(source, new_damage);
                    
                    // Check for commander damage loss condition
                    if new_damage >= 21 {
                        game_events.send(GameEvent::PlayerLost {
                            player: player_entity,
                            reason: LossReason::CommanderDamage(source),
                        });
                    }
                }
            }
        }
    }
}
```

## Teamwork Mechanics

Some Commander variants include team play, which requires special handling:

```rust
#[derive(Component)]
pub struct Team(pub u32);

pub fn validate_team_attacks(
    combat_system: Res<CombatSystem>,
    team_query: Query<(Entity, &Team)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Create team mappings
    let mut player_teams = HashMap::new();
    for (entity, team) in team_query.iter() {
        player_teams.insert(entity, team.0);
    }
    
    // Check for attacks against teammates
    for (attacker, attack_data) in &combat_system.attackers {
        if let Some(attacker_controller) = get_controller(*attacker) {
            if let (Some(attacker_team), Some(defender_team)) = (
                player_teams.get(&attacker_controller),
                player_teams.get(&attack_data.defender)
            ) {
                if attacker_team == defender_team {
                    // This is an attack against a teammate
                    game_events.send(GameEvent::TeamAttack {
                        attacker: *attacker,
                        defender: attack_data.defender,
                        team: *attacker_team,
                    });
                }
            }
        }
    }
}
```

## Multiplayer Edge Cases

### Player Elimination During Combat

If a player is eliminated during combat, any attackers targeting them need to be handled:

```rust
pub fn handle_player_elimination_during_combat(
    mut combat_system: ResMut<CombatSystem>,
    mut player_elimination_events: EventReader<PlayerEliminatedEvent>,
    mut commands: Commands,
) {
    for event in player_elimination_events.iter() {
        let eliminated_player = event.player;
        
        // Remove any attackers targeting the eliminated player
        let attackers_to_remove: Vec<Entity> = combat_system.attackers
            .iter()
            .filter_map(|(attacker, attack_data)| {
                if attack_data.defender == eliminated_player {
                    Some(*attacker)
                } else {
                    None
                }
            })
            .collect();
        
        for attacker in attackers_to_remove {
            combat_system.attackers.remove(&attacker);
            
            // Update creature component to no longer be attacking
            if let Some(mut creature) = commands.get_entity(attacker) {
                creature.insert(Creature {
                    attacking: None,
                    // Other fields preserved...
                    ..Default::default() // This would be replaced with actual preservation
                });
            }
        }
        
        // Remove any blockers controlled by the eliminated player
        let blockers_to_remove: Vec<Entity> = combat_system.blockers
            .iter()
            .filter_map(|(blocker, _)| {
                if get_controller(*blocker) == Some(eliminated_player) {
                    Some(*blocker)
                } else {
                    None
                }
            })
            .collect();
        
        for blocker in blockers_to_remove {
            combat_system.blockers.remove(&blocker);
            
            // Update creature component to no longer be blocking
            if let Some(mut creature) = commands.get_entity(blocker) {
                creature.insert(Creature {
                    blocking: Vec::new(),
                    // Other fields preserved...
                    ..Default::default() // This would be replaced with actual preservation
                });
            }
        }
    }
}
```

### Redirect Attack Effects

Some cards can redirect attacks to different players:

```rust
pub fn handle_attack_redirection(
    mut combat_system: ResMut<CombatSystem>,
    redirection_effects: Query<(Entity, &AttackRedirection)>,
) {
    // Process any attack redirection effects
    let redirections: Vec<(Entity, Entity)> = combat_system.attackers
        .iter()
        .filter_map(|(attacker, attack_data)| {
            for (_, redirection) in redirection_effects.iter() {
                if redirection.original_defender == attack_data.defender 
                   && redirection.applies_to(*attacker) {
                    return Some((*attacker, redirection.new_defender));
                }
            }
            None
        })
        .collect();
    
    // Apply redirections
    for (attacker, new_defender) in redirections {
        if let Some(attack_data) = combat_system.attackers.get_mut(&attacker) {
            attack_data.defender = new_defender;
        }
    }
}

#[derive(Component)]
pub struct AttackRedirection {
    pub original_defender: Entity,
    pub new_defender: Entity,
    pub condition: RedirectionCondition,
}

impl AttackRedirection {
    pub fn applies_to(&self, attacker: Entity) -> bool {
        match &self.condition {
            RedirectionCondition::AllAttackers => true,
            RedirectionCondition::AttackerWithPower { operator, value } => {
                // Implementation details omitted
                true
            },
            // Other conditions...
            _ => false,
        }
    }
}
```

## Testing Strategy

### Multiplayer Combat Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_multiple_attack_targets() {
        let mut app = App::new();
        app.add_systems(Update, validate_multiplayer_attacks);
        app.add_event::<GameEvent>();
        
        // Set up test environment
        let active_player = app.world.spawn(Player::default()).id();
        let opponent1 = app.world.spawn(Player::default()).id();
        let opponent2 = app.world.spawn(Player::default()).id();
        
        let attacker1 = app.world.spawn(Creature::default()).id();
        let attacker2 = app.world.spawn(Creature::default()).id();
        
        // Create turn manager
        let mut turn_manager = TurnManager::default();
        turn_manager.active_player_index = 0;
        turn_manager.player_order = vec![active_player, opponent1, opponent2];
        app.insert_resource(turn_manager);
        
        // Create combat system with attacks against multiple players
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(attacker1, AttackData {
            attacker: attacker1,
            defender: opponent1,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        combat_system.attackers.insert(attacker2, AttackData {
            attacker: attacker2,
            defender: opponent2,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Check for multiplay attack event
        let mut event_reader = app.world.resource_mut::<Events<GameEvent>>();
        let mut found_event = false;
        
        for event in event_reader.iter() {
            if let GameEvent::MultiplayerAttacksDeclared { .. } = event {
                found_event = true;
                break;
            }
        }
        
        assert!(found_event, "Multiplayer attack event not emitted");
    }
    
    #[test]
    fn test_goad_mechanic() {
        let mut app = App::new();
        app.add_systems(Update, apply_goad_requirements);
        
        // Set up test environment
        let active_player = app.world.spawn(Player::default()).id();
        let opponent1 = app.world.spawn(Player::default()).id();
        let opponent2 = app.world.spawn(Player::default()).id();
        
        // Create goaded creature
        let goaded_creature = app.world.spawn((
            Creature::default(),
            Controllable { controller: active_player },
            Goaded { source: opponent1, until_end_of_turn: true },
        )).id();
        
        // Create turn manager
        let mut turn_manager = TurnManager::default();
        turn_manager.active_player_index = 0;
        turn_manager.player_order = vec![active_player, opponent1, opponent2];
        app.insert_resource(turn_manager);
        
        // Create combat system
        let combat_system = CombatSystem::default();
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Verify goad requirements were applied
        let combat_system = app.world.resource::<CombatSystem>();
        
        // Check that creature must attack
        let has_must_attack = combat_system.attack_requirements.get(&goaded_creature)
            .map_or(false, |reqs| reqs.iter().any(|req| matches!(req, AttackRequirement::MustAttack)));
        assert!(has_must_attack, "Goaded creature should have MustAttack requirement");
        
        // Check that creature can't attack goad source
        let has_cant_attack_source = combat_system.attack_restrictions.get(&goaded_creature)
            .map_or(false, |reqs| reqs.iter().any(|req| matches!(req, AttackRestriction::CantAttackPlayer(p) if *p == opponent1)));
        assert!(has_cant_attack_source, "Goaded creature should not be able to attack goad source");
    }
    
    #[test]
    fn test_monarch_mechanic() {
        let mut app = App::new();
        app.add_systems(Update, monarch_attack_trigger);
        app.add_event::<GameEvent>();
        
        // Set up test environment
        let player1 = app.world.spawn(Player::default()).id();
        let player2 = app.world.spawn(Player::default()).id();
        
        let attacker = app.world.spawn((
            Creature::default(),
            Controllable { controller: player2 },
        )).id();
        
        // Create turn manager with combat phase
        let mut turn_manager = TurnManager::default();
        turn_manager.current_phase = Phase::Combat(CombatStep::CombatDamage);
        app.insert_resource(turn_manager);
        
        // Set player1 as monarch
        app.insert_resource(Monarch(Some(player1)));
        
        // Create combat system with attack history
        let mut combat_system = CombatSystem::default();
        combat_system.attackers.insert(attacker, AttackData {
            attacker,
            defender: player1,
            is_commander: false,
            requirements: Vec::new(),
            restrictions: Vec::new(),
        });
        combat_system.combat_history.push_back(CombatEvent::DamageDealt {
            source: attacker,
            target: player1,
            amount: 3,
            is_commander_damage: false,
        });
        app.insert_resource(combat_system);
        
        // Run the system
        app.update();
        
        // Verify monarch changed
        let monarch = app.world.resource::<Monarch>();
        assert_eq!(monarch.0, Some(player2), "Monarch should have changed to player2");
    }
    
    // Additional tests...
}
```

### Multiplayer Combat Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_multiplayer_combat_sequence() {
        let mut builder = CombatScenarioBuilder::new();
        
        // Add multiple opponents
        let opponent1 = builder.add_player();
        let opponent2 = builder.add_player();
        
        // Add attackers for active player
        let attacker1 = builder.add_attacker(3, 3, builder.active_player, false);
        let attacker2 = builder.add_attacker(2, 2, builder.active_player, true); // Commander
        
        // Add blocker for first opponent
        let blocker = builder.add_blocker(2, 2, opponent1);
        
        // Declare attacks against multiple opponents
        builder.declare_attacks(vec![
            (attacker1, opponent1),
            (attacker2, opponent2),
        ]);
        
        // Declare blocks
        builder.declare_blocks(vec![
            (blocker, vec![attacker1]),
        ]);
        
        // Execute combat
        let result = builder.execute();
        
        // Verify results
        
        // Opponent1 should have lost no life (blocked) but blocker should be dead
        assert_eq!(result.player_life[&opponent1], 40);
        let blocker_status = result.creature_status.get(&blocker).unwrap();
        assert!(blocker_status.destroyed);
        
        // Opponent2 should have taken commander damage
        assert_eq!(result.player_life[&opponent2], 40 - 2);
        assert_eq!(result.commander_damage[&opponent2][&attacker2], 2);
    }
    
    #[test]
    fn test_player_elimination_during_combat() {
        let mut app = App::new();
        
        // Setup full game environment with multiple players
        // Implementation details omitted for brevity
        
        // Simulate combat damage that eliminates a player
        // Implementation details omitted for brevity
        
        // Verify that all attacks targeting the eliminated player are removed
        // Implementation details omitted for brevity
    }
    
    // Additional integration tests...
}
```

### Multiplayer Combat System Tests

```rust
#[cfg(test)]
mod system_tests {
    use super::*;
    
    #[test]
    fn test_full_multiplayer_game() {
        let mut app = App::new();
        
        // Setup a 4-player Commander game
        let players = setup_four_player_game(&mut app);
        
        // Play through several turns with complex political interactions
        for _ in 0..10 {
            app.update();
            
            // Add various political actions between updates
            // (goad effects, attack redirection, monarch changes, etc.)
            // Implementation details omitted for brevity
        }
        
        // Verify multiplayer combat interactions worked as expected
        // Implementation details omitted for brevity
    }
    
    // Helper functions for system tests
    fn setup_four_player_game(app: &mut App) -> Vec<Entity> {
        // Implementation details omitted for brevity
        Vec::new()
    }
    
    // Additional system tests...
}
```

## Performance Considerations

Multiplayer combat introduces additional computational complexity:

```rust
// Optimize attack target lookup
pub fn optimize_multiplayer_combat(
    mut combat_system: ResMut<CombatSystem>,
) {
    // Build attack target lookup table for faster validation
    let mut attack_target_map: HashMap<Entity, HashSet<Entity>> = HashMap::new();
    
    for (attacker, attack_data) in &combat_system.attackers {
        attack_target_map.entry(attack_data.defender)
            .or_insert_with(HashSet::new)
            .insert(*attacker);
    }
    
    combat_system.attack_target_map = attack_target_map;
}
```

## UI Considerations

Multiplayer combat requires special attention to the user interface:

```rust
pub fn update_multiplayer_combat_ui(
    combat_system: Res<CombatSystem>,
    player_query: Query<(Entity, &Player)>,
    mut ui_state: ResMut<UiState>,
) {
    // Group attacks by defender for UI display
    let mut attacks_by_defender: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
    for (attacker, attack_data) in &combat_system.attackers {
        attacks_by_defender.entry(attack_data.defender)
            .or_insert_with(Vec::new)
            .push(*attacker);
    }
    
    // Update UI state for each player
    for (player_entity, _) in player_query.iter() {
        // Display incoming attacks for this player
        if let Some(attackers) = attacks_by_defender.get(&player_entity) {
            ui_state.player_incoming_attacks.insert(player_entity, attackers.clone());
        } else {
            ui_state.player_incoming_attacks.insert(player_entity, Vec::new());
        }
    }
}
```

## Networking Considerations

In a networked multiplayer game, commander-specific combat events need special handling:

```rust
pub fn replicate_multiplayer_combat_state(
    combat_system: Res<CombatSystem>,
    monarch: Res<Monarch>,
    mut replication: ResMut<Replication>,
) {
    // Replicate critical combat state
    replication.replicate_resource::<CombatSystem>();
    replication.replicate_resource::<Monarch>();
    
    // Replicate all combat-relevant components
    for (entity, _) in combat_system.attackers.iter() {
        replication.replicate_entity(*entity);
    }
    
    for (entity, _) in combat_system.blockers.iter() {
        replication.replicate_entity(*entity);
    }
}
```

## Conclusion

Multiplayer combat in Commander adds significant complexity but also creates rich strategic gameplay. By properly implementing the systems described in this document, we ensure that players can fully experience the political dynamics and multiplayer interactions that make Commander such a beloved format. The implementation handles all the edge cases and unique mechanics while maintaining good performance even with four or more players. 