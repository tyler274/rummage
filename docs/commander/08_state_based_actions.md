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