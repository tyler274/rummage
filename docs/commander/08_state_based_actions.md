# State-Based Actions

## Overview

State-Based Actions (SBAs) are automatic game rules that check and modify the game state whenever a player would receive priority. They handle fundamental game mechanics like creature death, legend rule, and player elimination. For Commander games, SBAs include special handling for commander damage and commander zone transitions as specified in the Magic: The Gathering Comprehensive Rules section 903.

## Core Components

### State-Based Action System

```rust
#[derive(Resource)]
pub struct StateBasedActionSystem {
    // Configuration
    pub enabled: bool,
    pub check_frequency: std::time::Duration,
    pub last_check_time: std::time::Instant,
    
    // Performance settings
    pub batch_size: usize,
    pub priority_categories: StateCheckPriority,
    
    // Game state tracking
    pub actions_performed_last_check: usize,
    pub state_is_clean: bool,
    
    // Commander-specific tracking
    pub commander_checks_enabled: bool,
    pub commander_damage_tracked: bool,
    pub commander_zone_transitions_enabled: bool,
}

#[derive(Default, Debug, Clone)]
pub struct StateCheckPriority {
    // Different categories of checks with priority ordering
    pub player_elimination: u8,
    pub creature_death: u8,
    pub card_movement: u8,
    pub counter_management: u8,
    pub attachment_validation: u8,
    pub legend_rule: u8,
    pub planeswalker_rule: u8,
    pub token_cleanup: u8,
    pub commander_damage: u8, // Commander-specific priority
    pub commander_zone_transitions: u8, // Commander-specific priority
}
```

### State Check Events

```rust
#[derive(Event)]
pub struct StateBasedActionCheckedEvent {
    pub actions_performed: usize,
    pub categories_modified: Vec<StateBasedActionCategory>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateBasedActionCategory {
    PlayerState,
    CreatureState,
    PlaneswalkerState,
    AttachmentState,
    CounterState,
    TokenState,
    ZoneState,
    LegendaryState,
    CommanderState, // Commander-specific category
}

#[derive(Event)]
pub struct GameStateModifiedEvent {
    pub source: GameStateChangeSource,
    pub requires_sba_check: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStateChangeSource {
    SpellResolution,
    AbilityResolution,
    CostPayment,
    PriorityAction,
    TurnBasedAction,
    CommanderDamage, // Commander-specific source
    CommanderZoneChange, // Commander-specific source
    Manual,
}

#[derive(Event)]
pub struct CheckStateBasedActionsEvent;

#[derive(Event)]
pub struct CommanderZoneChoiceEvent {
    pub commander: Entity,
    pub player: Entity,
    pub from_zone: Zone,
    pub to_zone: Zone,
    pub to_command_zone: bool,
}
```

## Key Systems

### Main State-Based Action System

```rust
fn state_based_action_system(
    mut commands: Commands,
    mut sba_system: ResMut<StateBasedActionSystem>,
    mut game_state: ResMut<CommanderGameState>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    time: Res<Time>,
    players: Query<(Entity, &Player, &mut CommanderPlayer)>,
    permanents: Query<(Entity, &Permanent, Option<&CreatureCard>)>,
    commanders: Query<(Entity, &CommanderCard)>,
    mut game_state_events: EventReader<GameStateModifiedEvent>,
    mut sba_events: EventWriter<StateBasedActionCheckedEvent>,
    mut zone_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    // Skip check if system is disabled
    if !sba_system.enabled {
        return;
    }
    
    // Check if it's time to run SBAs
    let current_time = time.elapsed();
    if current_time.duration_since(sba_system.last_check_time) < sba_system.check_frequency && 
       sba_system.state_is_clean {
        return;
    }
    
    // Update last check time
    sba_system.last_check_time = current_time;
    
    // Track modifications
    let mut actions_performed = 0;
    let mut categories_modified = Vec::new();
    
    // Check player state (including commander damage)
    if check_player_state(
        &mut commands,
        &players,
        &commanders,
        &game_state,
        &sba_system,
        &mut categories_modified,
    ) {
        actions_performed += 1;
    }
    
    // Check creature state
    if check_creature_state(
        &mut commands,
        &permanents,
        &mut zone_manager,
        &mut categories_modified,
    ) {
        actions_performed += 1;
    }
    
    // Commander-specific SBAs
    if sba_system.commander_checks_enabled {
        // Check commander zone transitions (rule 903.9a)
        if check_commander_zones(
            &mut commands,
            &mut cmd_zone_manager,
            &mut zone_manager,
            &mut zone_events,
            &mut categories_modified,
        ) {
            actions_performed += 1;
        }
    }
    
    // Additional checks...
    
    // Update tracking
    sba_system.actions_performed_last_check = actions_performed;
    sba_system.state_is_clean = actions_performed == 0;
    
    // Send event if actions were performed
    if actions_performed > 0 {
        sba_events.send(StateBasedActionCheckedEvent {
            actions_performed,
            categories_modified,
        });
    }
}
```

### Player State Checks (Including Commander Damage)

```rust
fn check_player_state(
    commands: &mut Commands,
    players: &Query<(Entity, &Player, &mut CommanderPlayer)>,
    commanders: &Query<(Entity, &CommanderCard)>,
    game_state: &CommanderGameState,
    sba_system: &StateBasedActionSystem,
    categories_modified: &mut Vec<StateBasedActionCategory>,
) -> bool {
    let mut state_modified = false;
    
    // Check each player's state
    for (entity, player, commander_player) in players.iter() {
        // Check for life <= 0
        if player.life <= 0 && !commander_player.has_lost {
            // Player loses the game
            commands.entity(entity).insert(PlayerEliminatedEvent {
                player: entity,
                reason: EliminationReason::LifeLoss,
            });
            state_modified = true;
        }
        
        // Check for other loss conditions
        // ...
        
        // Commander damage check (rule 903.10a)
        if sba_system.commander_damage_tracked {
            for (commander_entity, commander_damage) in &commander_player.commander_damage_received {
                if *commander_damage >= game_state.commander_damage_threshold {
                    // Player loses from commander damage
                    commands.entity(entity).insert(PlayerEliminatedEvent {
                        player: entity,
                        reason: EliminationReason::CommanderDamage(*commander_entity),
                    });
                    state_modified = true;
                    break;
                }
            }
        }
    }
    
    if state_modified {
        categories_modified.push(StateBasedActionCategory::PlayerState);
    }
    
    state_modified
}
```

### Commander Zone Checks

```rust
fn check_commander_zones(
    commands: &mut Commands,
    cmd_zone_manager: &mut ResMut<CommandZoneManager>,
    zone_manager: &mut ResMut<ZoneManager>,
    zone_events: &mut EventWriter<CommanderZoneChoiceEvent>,
    categories_modified: &mut Vec<StateBasedActionCategory>,
) -> bool {
    let mut state_modified = false;
    
    // Check for commanders in graveyard or exile (rule 903.9a)
    // First, collect all commanders in graveyards
    let commanders_in_graveyard: Vec<(Entity, Entity)> = zone_manager.graveyards
        .iter()
        .flat_map(|(&player, cards)| {
            cards.iter()
                .filter_map(|&card| {
                    if cmd_zone_manager.commander_zone_status.get(&card) == Some(&CommanderZoneLocation::Graveyard) {
                        Some((card, player))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    
    // Then, collect all commanders in exile
    let commanders_in_exile: Vec<(Entity, Entity)> = zone_manager.exiles
        .iter()
        .flat_map(|(&player, cards)| {
            cards.iter()
                .filter_map(|&card| {
                    if cmd_zone_manager.commander_zone_status.get(&card) == Some(&CommanderZoneLocation::Exile) {
                        Some((card, player))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    
    // Process graveyard commanders
    for (commander, player) in commanders_in_graveyard {
        if cmd_zone_manager.died_this_turn.contains(&commander) {
            // Send event to give player the choice to move commander to command zone
            zone_events.send(CommanderZoneChoiceEvent {
                commander,
                player,
                from_zone: Zone::Graveyard,
                to_zone: Zone::Graveyard, // Original destination
                to_command_zone: true, // Default to command zone, but player can choose
            });
            
            state_modified = true;
        }
    }
    
    // Process exile commanders
    for (commander, player) in commanders_in_exile {
        if cmd_zone_manager.exiled_this_turn.contains(&commander) {
            // Send event to give player the choice to move commander to command zone
            zone_events.send(CommanderZoneChoiceEvent {
                commander,
                player,
                from_zone: Zone::Exile,
                to_zone: Zone::Exile, // Original destination
                to_command_zone: true, // Default to command zone, but player can choose
            });
            
            state_modified = true;
        }
    }
    
    if state_modified {
        categories_modified.push(StateBasedActionCategory::CommanderState);
    }
    
    state_modified
}
```

### Commander Replacement Effect Handler

```rust
fn handle_commander_replacement_effects(
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    commanders: Query<Entity, With<CommanderCard>>,
    mut zone_choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    for event in zone_change_events.read() {
        // Check if the card is a commander
        if commanders.contains(event.card) {
            // Handle replacement effect for hand/library (rule 903.9b)
            if event.to_zone == Zone::Hand || event.to_zone == Zone::Library {
                // Trigger a choice event for the replacement effect
                zone_choice_events.send(CommanderZoneChoiceEvent {
                    commander: event.card,
                    player: event.controller,
                    from_zone: event.from_zone,
                    to_zone: event.to_zone,
                    to_command_zone: true, // Default to command zone, but player can choose
                });
            }
        }
    }
}
```

### Tracking Commander Damage

```rust
fn track_commander_damage(
    mut damage_events: EventReader<CombatDamageEvent>,
    commanders: Query<Entity, With<CommanderCard>>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    mut state_modified_events: EventWriter<GameStateModifiedEvent>,
) {
    for event in damage_events.read() {
        // Check if damage is from a commander
        if commanders.contains(event.source) && event.is_combat_damage {
            // Find the target player
            if let Ok((player_entity, mut commander_player)) = players.get_mut(event.target) {
                // Update commander damage tracking
                let damage_entry = commander_player.commander_damage_received
                    .entry(event.source)
                    .or_insert(0);
                *damage_entry += event.damage;
                
                // Trigger state-based action check
                state_modified_events.send(GameStateModifiedEvent {
                    source: GameStateChangeSource::CommanderDamage,
                    requires_sba_check: true,
                });
            }
        }
    }
}
```

## Commander-Specific State-Based Actions

According to rule 903.9a, when a commander is in a graveyard or in exile and that object was put into that zone since the last time state-based actions were checked, its owner may put it into the command zone as a state-based action.

```rust
fn trigger_state_based_actions_system(
    mut sba_checks: EventReader<GameStateModifiedEvent>,
    mut check_events: EventWriter<CheckStateBasedActionsEvent>,
) {
    for event in sba_checks.read() {
        if event.requires_sba_check {
            check_events.send(CheckStateBasedActionsEvent);
            break; // Only need one check
        }
    }
}
```

And according to rule 903.10a, a player who's been dealt 21 or more combat damage by the same commander over the course of the game loses the game.

```rust
fn check_commander_damage_loss(
    players: Query<(Entity, &CommanderPlayer)>,
    mut player_eliminated_events: EventWriter<PlayerEliminatedEvent>,
    game_state: Res<CommanderGameState>,
) {
    for (player_entity, commander_player) in players.iter() {
        // Skip players who already lost
        if commander_player.has_lost {
            continue;
        }
        
        // Check each commander damage source
        for (source_commander, damage) in &commander_player.commander_damage_received {
            if *damage >= game_state.commander_damage_threshold {
                // Player loses the game due to commander damage
                player_eliminated_events.send(PlayerEliminatedEvent {
                    player: player_entity,
                    reason: EliminationReason::CommanderDamage(*source_commander),
                });
                break;
            }
        }
    }
}
```

## Handling Melded Commanders

For melded commanders, there are special rules (903.9c):

```rust
fn handle_melded_commander_replacement(
    mut command_zone_events: EventReader<CommanderZoneChoiceEvent>,
    melded_commanders: Query<(Entity, &MeldedPermanent)>,
    commanders: Query<Entity, With<CommanderCard>>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
) {
    for event in command_zone_events.read() {
        // Check if this is a melded permanent where one part is a commander
        if let Ok((entity, meld_data)) = melded_commanders.get(event.commander) {
            if event.to_command_zone {
                // Find which part of the meld is the commander
                let commander_component = meld_data.components.iter()
                    .find(|&&comp| commanders.contains(comp))
                    .copied();
                
                if let Some(commander) = commander_component {
                    // Commander card goes to command zone
                    cmd_zone_manager.command_zones.get_mut(&event.player).unwrap()
                        .push(commander);
                    
                    // Other components go to the original destination zone
                    for &component in &meld_data.components {
                        if component != commander {
                            match event.to_zone {
                                Zone::Hand => {
                                    zone_manager.hands.get_mut(&event.player).unwrap()
                                        .push(component);
                                },
                                Zone::Library => {
                                    zone_manager.libraries.get_mut(&event.player).unwrap()
                                        .push(component);
                                },
                                Zone::Graveyard => {
                                    zone_manager.graveyards.get_mut(&event.player).unwrap()
                                        .push(component);
                                },
                                Zone::Exile => {
                                    zone_manager.exiles.get_mut(&event.player).unwrap()
                                        .push(component);
                                },
                                _ => {} // Other zones not relevant for this rule
                            }
                        }
                    }
                }
            }
        }
    }
}
```

## Integration with Other Systems

The State-Based Actions system coordinates with other modules:

1. **Priority System**: SBAs are checked each time a player would receive priority
2. **Command Zone**: For commander zone transitions (rule 903.9)
3. **Combat System**: For tracking commander damage (rule 903.10a)
4. **Player Management**: For applying commander damage loss conditions
5. **Zone Management**: For handling zone transitions

## Testing Strategy

1. **Unit Tests**:
   - Verify commander damage threshold triggers player loss
   - Test commander zone transitions as SBAs
   - Validate timing of SBA checks
   
2. **Integration Tests**:
   - Simulate complex commander damage scenarios
   - Test commander zone transitions in different contexts
   - Verify interaction with priority system
   - Test melded commander handling

## Performance Considerations

- Batch SBA checks to avoid performance issues in multiplayer games
- Prioritize checks based on likelihood of modification
- Optimize commander tracking for games with many commanders 

## Edge Cases in Commander State-Based Actions

### Complex Commander Zone Transitions

#### Multiple Commander Damage Sources

When tracking commander damage from multiple sources, special care is needed:

```rust
fn check_combined_commander_damage(
    players: Query<(Entity, &CommanderPlayer)>,
    game_state: Res<CommanderGameState>,
) -> bool {
    // Rule 903.10a only counts damage from the same commander
    // No need to check combined damage from different commanders
    
    for (player_entity, player) in players.iter() {
        let has_lethal_commander_damage = player.commander_damage_received
            .iter()
            .any(|(_, damage)| *damage >= CommanderRules::COMMANDER_DAMAGE_THRESHOLD);
            
        if has_lethal_commander_damage {
            return true;
        }
    }
    
    false
}
```

#### Multiple Commander Deaths in One Cycle

When multiple commanders die within the same SBA check cycle, all zone transition triggers must be handled sequentially:

```rust
fn handle_multiple_commander_deaths(
    mut commander_zone_events: EventReader<CommanderDiedEvent>,
    mut zone_choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    // Track all dead commanders for this SBA cycle
    let mut dead_commanders = Vec::new();
    
    for event in commander_zone_events.read() {
        dead_commanders.push((event.commander, event.owner));
    }
    
    // Process each commander individually
    for (commander, owner) in dead_commanders {
        zone_choice_events.send(CommanderZoneChoiceEvent {
            commander,
            player: owner,
            from_zone: Zone::Graveyard,
            to_zone: Zone::CommandZone,
            to_command_zone: true, // Default choice
        });
    }
}
```

### Face-Down Commanders

When a commander is turned face down (through effects like Morph, Disguise, or Ixidron):

```rust
fn handle_face_down_commander_tracking(
    mut commanders: Query<(Entity, &mut Commander, &FaceDownStatus)>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for event in damage_events.read() {
        // First check if source is a face-down commander
        if let Ok((entity, mut commander, face_down)) = commanders.get_mut(event.source) {
            if face_down.is_face_down && event.is_combat_damage {
                // Even face-down commanders accumulate commander damage
                if let Some(damage_entry) = commander
                    .damage_dealt
                    .iter_mut()
                    .find(|(p, _)| *p == event.target)
                {
                    damage_entry.1 += event.damage;
                } else {
                    commander.damage_dealt.push((event.target, event.damage));
                }
            }
        }
    }
}
```

### Partner Commanders and Commander Damage

For partner commanders (903.9d), damage from each partner is tracked separately:

```rust
fn check_partner_commander_damage(
    mut player_query: Query<(Entity, &CommanderPlayer)>,
    partner_commanders: Query<(Entity, &Commander), With<PartnerCommander>>,
) {
    for (player_entity, player) in player_query.iter_mut() {
        // Check each partner commander separately
        // Partner damage is NOT combined for the 21 damage threshold
        for (commander_entity, commander) in partner_commanders.iter() {
            if let Some((_, damage)) = commander
                .damage_dealt
                .iter()
                .find(|(p, _)| *p == player_entity)
            {
                if *damage >= CommanderRules::COMMANDER_DAMAGE_THRESHOLD {
                    // Player has lost to this specific partner commander
                    // Even if they haven't lost to the other partner
                    commands.spawn(PlayerEliminatedEvent {
                        player: player_entity,
                        reason: EliminationReason::CommanderDamage(commander_entity),
                    });
                    break;
                }
            }
        }
    }
}
```

### Simultaneous Player Elimination

When multiple players would be eliminated simultaneously:

```rust
fn handle_simultaneous_player_elimination(
    mut commands: Commands,
    mut elimination_queue: Local<Vec<(Entity, EliminationReason)>>,
    mut elimination_events: EventReader<PlayerEliminatedEvent>,
    game_state: Res<CommanderGameState>,
) {
    // Collect all elimination events for this cycle
    for event in elimination_events.read() {
        elimination_queue.push((event.player, event.reason));
    }
    
    if !elimination_queue.is_empty() {
        // Process in active player order (rule 800.4)
        let active_player_index = game_state.turn_order.active_player_index;
        let player_order = &game_state.turn_order.player_order;
        
        // Sort elimination queue by turn order
        elimination_queue.sort_by(|(player_a, _), (player_b, _)| {
            let a_pos = player_order.iter().position(|p| p == player_a).unwrap_or(usize::MAX);
            let b_pos = player_order.iter().position(|p| p == player_b).unwrap_or(usize::MAX);
            
            // Primary sort: distance from active player
            let a_dist = (a_pos + player_order.len() - active_player_index) % player_order.len();
            let b_dist = (b_pos + player_order.len() - active_player_index) % player_order.len();
            a_dist.cmp(&b_dist)
        });
        
        // Process each elimination
        for (player, reason) in elimination_queue.drain(..) {
            commands.spawn(ProcessPlayerEliminationEvent {
                player,
                reason,
            });
        }
    }
}
```

### Rule Interaction: Commander + Effects That Change Control

When a commander changes control temporarily:

```rust
fn handle_commander_control_change(
    mut commanders: Query<(Entity, &mut Commander, &ControlChangeStatus)>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for (entity, mut commander, control_status) in commanders.iter_mut() {
        if control_status.is_under_different_control {
            // The original owner is still tracked for commander damage purposes
            // Even if control has changed temporarily
            
            // For permanents under changed control that deal combat damage:
            // 1. Commander damage is still tracked normally
            // 2. The original owner must still be used for zone transition choices
            // 3. Commander tax is still based on the original owner's cast count
        }
    }
}
```

### Clone Effects on Commanders

When a commander is cloned (including by other player's spells):

```rust
fn handle_cloned_commanders(
    mut commands: Commands,
    clone_events: EventReader<CommanderClonedEvent>,
    commanders: Query<&Commander>,
) {
    for event in clone_events.read() {
        // The clone is NOT a commander (rule 903.3)
        // Only the original card has the commander designation
        
        if let Ok(original_commander) = commanders.get(event.original) {
            // Clone does not:
            // 1. Deal commander damage
            // 2. Have commander zone transition permissions
            // 3. Accrue commander tax
            
            // Ensure the clone does not have commander status
            commands.entity(event.clone).remove::<CommanderCard>();
        }
    }
}
```

### Rule Interaction: Commander + Replacement Effects

Complex interactions with replacement effects:

```rust
fn handle_commander_damage_replacement(
    mut commands: Commands,
    mut damage_events: EventReader<CombatDamageEvent>,
    commanders: Query<Entity, With<CommanderCard>>,
    replacement_effects: Query<(Entity, &DamageReplacementEffect)>,
) {
    for event in damage_events.read() {
        let is_commander_damage = commanders.contains(event.source) && event.is_combat_damage;
        
        if is_commander_damage {
            // Check for replacement effects that might modify the damage
            let mut replaced_damage = false;
            
            for (effect_entity, replacement) in replacement_effects.iter() {
                if replacement.applies_to(event.source, event.target) {
                    // Even if damage is prevented or redirected, it still counts 
                    // as commander damage from the original source
                    // The replacement only affects the game state damage, not the tracking
                    replaced_damage = true;
                    
                    // For prevention effects, we still need to record that commander 
                    // damage was attempted for various triggered abilities
                    if replacement.is_prevention() {
                        commands.spawn(CommanderDamagePreventedEvent {
                            source: event.source,
                            target: event.target,
                            amount: event.damage,
                            prevention_source: effect_entity,
                        });
                    }
                }
            }
        }
    }
}
```

### Commander Copy Effects and Type-Changing Effects

When a commander changes types or has its characteristics changed:

```rust
fn handle_commander_type_changes(
    mut commanders: Query<(Entity, &Commander, &TypeChangeStatus)>,
    mut damage_events: EventReader<CombatDamageEvent>,
) {
    for (entity, commander, type_status) in commanders.iter_mut() {
        // Even if a commander temporarily loses creature type
        // or gains other types, it still:
        // 1. Deals commander damage if combat damage is dealt
        // 2. Can return to command zone if it would go to graveyard/exile
        // 3. Maintains its commander status for all rules purposes

        // A commander that loses creature type can still deal commander
        // damage if it somehow deals combat damage
    }
}
```

### Merged Permanents Involving Commanders

For cases where a commander merges with another permanent:

```rust
fn handle_merged_commanders(
    merged_entities: Query<(Entity, &MergedPermanent)>,
    commanders: Query<&Commander>,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    mut cmd_zone_choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    for event in zone_change_events.read() {
        if let Ok((entity, merged)) = merged_entities.get(event.card) {
            // Check if any component is a commander
            let has_commander_component = merged.components.iter()
                .any(|component| commanders.contains(*component));
            
            if has_commander_component {
                // Handle complex merged permanent case
                // Rule 903.9c analog for merged permanents
                
                // Give player choice for commander component
                if event.to_zone == Zone::Graveyard || event.to_zone == Zone::Exile {
                    cmd_zone_choice_events.send(CommanderZoneChoiceEvent {
                        commander: entity,
                        player: event.controller,
                        from_zone: event.from_zone,
                        to_zone: event.to_zone,
                        to_command_zone: true, // Default
                        is_merged: true,
                        merged_components: merged.components.clone(),
                    });
                }
            }
        }
    }
}
```

### Token Commanders

When a token is copying a commander or created by a commander effect:

```rust
fn handle_token_commanders(
    token_commanders: Query<(Entity, &Commander, &TokenStatus)>,
) {
    // Token copies of commanders are not actually commanders (rule 903.3)
    // But tokens created by effects that specifically create commander tokens are
    
    for (entity, commander, token) in token_commanders.iter() {
        // Special case: tokens created by effects that explicitly make them commanders
        if token.is_special_commander_token {
            // These tokens can deal commander damage and have commander zone abilities
            // But cease to exist if they would change zones (due to token rules)
        } else {
            // Regular token copies of commanders are not commanders
            // and should be removed from commander tracking
        }
    }
}
```

## Handling Requirements vs. Restrictions in Multiplayer

In multiplayer Commander, many effects create requirements or restrictions:

```rust
fn handle_conflicting_requirements(
    creatures: Query<(Entity, &AttackRequirements, &AttackRestrictions)>,
    combat_state: Res<CombatState>,
    players: Query<(Entity, &CommanderPlayer)>,
) -> Vec<Entity> {
    // Per rule 508.1d, if there are conflicting requirements and restrictions,
    // the player must satisfy as many as possible
    
    let mut valid_attackers = Vec::new();
    
    for (entity, requirements, restrictions) in creatures.iter() {
        // Check if creature has both a requirement to attack
        // and a restriction against attacking
        let must_attack = requirements.forced_to_attack_if_able;
        let cannot_attack = restrictions.cannot_attack;
        
        if must_attack && cannot_attack {
            // Rule 508.1d: A restriction takes precedence over a requirement
            // So this creature won't attack
            continue;
        }
        
        // Handle "must attack specific player" vs "cannot attack that player"
        if let Some(required_defender) = requirements.must_attack_player {
            if restrictions.cannot_attack_players.contains(&required_defender) {
                // Cannot satisfy both, so restriction wins
                continue;
            }
            
            // Can satisfy both by attacking the required player
            valid_attackers.push(entity);
        }
        
        // Add creatures that can satisfy all requirements and restrictions
        if must_attack && !cannot_attack {
            // Ensure there is at least one player who can be attacked
            let can_attack_someone = players.iter()
                .any(|(player, _)| !restrictions.cannot_attack_players.contains(&player));
                
            if can_attack_someone {
                valid_attackers.push(entity);
            }
        }
    }
    
    valid_attackers
}
```

## Commander State-Based Actions Performance Optimizations

For large multiplayer games (5+ players), performance optimizations:

```rust
fn optimize_sba_checks(
    mut sba_system: ResMut<StateBasedActionSystem>,
    player_count: Res<PlayerCount>,
) {
    // For larger games, batch SBA checks to improve performance
    if player_count.count > 4 {
        // Increase batch size as player count grows
        sba_system.batch_size = player_count.count + 4;
        
        // Prioritize commander damage checks earlier with more players
        sba_system.priority_categories.commander_damage = 2;
        
        // Reduce frequency of complete checks in large games
        sba_system.check_frequency = std::time::Duration::from_millis(100 * player_count.count as u64);
    } else {
        // Default settings for 2-4 player games
        sba_system.batch_size = 4;
        sba_system.priority_categories.commander_damage = 5;
        sba_system.check_frequency = std::time::Duration::from_millis(50);
    }
}
```

## Testing Complex Edge Cases

Test vectors for complicated Commander-specific edge cases:

```rust
#[test]
fn test_complex_commander_interactions() {
    let mut app = App::new();
    
    // Setup Commander game with multiple players
    app.add_plugins(CommanderTestPlugin);
    
    // Complex test cases:
    
    // 1. Commander gets turned face down, deals damage, then face up
    // 2. Two commanders control-swapped, both deal damage
    // 3. Commander gets merged with non-commander, changes zones
    // 4. Partner commanders combined damage on one player
    // 5. Player gets eliminated by commander damage while controlling another player's commander
    // 6. Player with lethal commander damage and lethal regular damage simultaneously
    // 7. Command zone transition during a replacement effect resolution
    // 8. Multiple players eliminated simultaneously by different commanders
    
    // Run test logic...
}
``` 