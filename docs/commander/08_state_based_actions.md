# State-Based Actions

## Overview

State-Based Actions (SBAs) are automatic game rules that check and modify the game state whenever a player would receive priority. They handle fundamental game mechanics like creature death, legend rule, and player elimination. For Commander games with many players, SBAs require optimized implementation to efficiently handle complex board states.

## Core Components

### State-Based Actions Resource

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
    Manual,
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
    time: Res<Time>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    permanents: Query<(Entity, &Permanent, Option<&CreatureCard>)>,
    mut game_state_events: EventReader<GameStateModifiedEvent>,
    mut sba_events: EventWriter<StateBasedActionCheckedEvent>,
) {
    // Skip check if system is disabled
    if !sba_system.enabled {
        return;
    }
    
    // Determine if check is needed
    let force_check = game_state_events.read().any(|e| e.requires_sba_check);
    let time_check = time.elapsed() - sba_system.last_check_time >= sba_system.check_frequency;
    
    if !(force_check || time_check) {
        return;
    }
    
    // Record check time
    sba_system.last_check_time = time.elapsed();
    
    // Reset tracking
    sba_system.actions_performed_last_check = 0;
    let mut categories_modified = Vec::new();
    sba_system.state_is_clean = false;
    
    // Perform checks until state is clean (no more actions needed)
    let mut iterations = 0;
    let max_iterations = 10; // Safety limit
    
    while !sba_system.state_is_clean && iterations < max_iterations {
        // Track actions performed in this iteration
        let mut actions_this_iteration = 0;
        
        // Check player state (life total, empty library, etc.)
        let player_actions = check_player_state(
            &mut commands, 
            &mut game_state, 
            &players
        );
        
        if player_actions > 0 {
            actions_this_iteration += player_actions;
            categories_modified.push(StateBasedActionCategory::PlayerState);
        }
        
        // Check creature state (toughness <= 0, lethal damage, etc.)
        let creature_actions = check_creature_state(
            &mut commands, 
            &permanents, 
            &mut zone_manager
        );
        
        if creature_actions > 0 {
            actions_this_iteration += creature_actions;
            categories_modified.push(StateBasedActionCategory::CreatureState);
        }
        
        // Check planeswalker state (0 loyalty)
        let planeswalker_actions = check_planeswalker_state(
            &mut commands, 
            &permanents, 
            &mut zone_manager
        );
        
        if planeswalker_actions > 0 {
            actions_this_iteration += planeswalker_actions;
            categories_modified.push(StateBasedActionCategory::PlaneswalkerState);
        }
        
        // Check attachment validity (Auras, Equipment)
        let attachment_actions = check_attachment_validity(
            &mut commands, 
            &permanents, 
            &mut zone_manager
        );
        
        if attachment_actions > 0 {
            actions_this_iteration += attachment_actions;
            categories_modified.push(StateBasedActionCategory::AttachmentState);
        }
        
        // Check legendary permanent uniqueness
        let legend_actions = check_legend_rule(
            &mut commands, 
            &permanents, 
            &mut zone_manager,
            &players
        );
        
        if legend_actions > 0 {
            actions_this_iteration += legend_actions;
            categories_modified.push(StateBasedActionCategory::LegendaryState);
        }
        
        // Other state checks as needed...
        
        // Update tracking
        sba_system.actions_performed_last_check += actions_this_iteration;
        
        // If no actions were performed, state is clean
        if actions_this_iteration == 0 {
            sba_system.state_is_clean = true;
        }
        
        iterations += 1;
    }
    
    // Notify that state-based actions were checked
    if sba_system.actions_performed_last_check > 0 {
        sba_events.send(StateBasedActionCheckedEvent {
            actions_performed: sba_system.actions_performed_last_check,
            categories_modified,
        });
    }
}
```

### Player State Check

```rust
fn check_player_state(
    commands: &mut Commands,
    game_state: &mut CommanderGameState,
    players: &Query<(Entity, &CommanderPlayer)>,
) -> usize {
    let mut actions_performed = 0;
    
    for (entity, player) in players.iter() {
        // Check life total
        if player.life <= 0 && !player.has_lost {
            // Player loses the game
            commands.spawn(GameEvent::PlayerEliminated {
                player: entity,
                reason: EliminationReason::LifeLoss,
            });
            actions_performed += 1;
        }
        
        // Check poison counters (10+ means loss)
        if player.poison_counters >= 10 && !player.has_lost {
            commands.spawn(GameEvent::PlayerEliminated {
                player: entity,
                reason: EliminationReason::Poison,
            });
            actions_performed += 1;
        }
        
        // Check if player tried to draw from empty library
        if player.attempted_draw_from_empty_library && !player.has_lost {
            commands.spawn(GameEvent::PlayerEliminated {
                player: entity,
                reason: EliminationReason::EmptyLibrary,
            });
            actions_performed += 1;
        }
        
        // Special Commander rule: Check if all commanders are in command zone
        // and player has no other permanents or cards in hand, signifying a restart
        if should_concede_commander(entity, player, game_state) {
            commands.spawn(GameEvent::PlayerEliminated {
                player: entity,
                reason: EliminationReason::Conceded,
            });
            actions_performed += 1;
        }
    }
    
    actions_performed
}

fn should_concede_commander(
    player_entity: Entity,
    player: &CommanderPlayer,
    game_state: &CommanderGameState,
) -> bool {
    // This would implement Commander-specific concession detection
    // For example, if all commanders are in command zone and player has
    // zero permanents and empty hand, they might be trying to restart
    false
}
```

### Creature State Check

```rust
fn check_creature_state(
    commands: &mut Commands,
    permanents: &Query<(Entity, &Permanent, Option<&CreatureCard>)>,
    zone_manager: &mut ZoneManager,
) -> usize {
    let mut actions_performed = 0;
    let mut creatures_to_destroy = Vec::new();
    
    // Check all creatures
    for (entity, permanent, creature_opt) in permanents.iter() {
        if let Some(creature) = creature_opt {
            let current_toughness = creature.toughness as i64 + creature.toughness_modifier;
            
            // Check for 0 or less toughness
            if current_toughness <= 0 {
                creatures_to_destroy.push((entity, permanent.controller));
                actions_performed += 1;
            }
            
            // Check for lethal damage
            if permanent.damage >= creature.toughness as u64 {
                creatures_to_destroy.push((entity, permanent.controller));
                actions_performed += 1;
            }
        }
    }
    
    // Process all creatures that need to be destroyed
    for (entity, controller) in creatures_to_destroy {
        commands.spawn(PermanentDestroyedEvent {
            permanent: entity,
            controller,
            reason: DestructionReason::StateBased,
        });
        
        // Move to appropriate zone (normally graveyard)
        move_to_graveyard(commands, zone_manager, entity, controller);
    }
    
    actions_performed
}

fn move_to_graveyard(
    commands: &mut Commands,
    zone_manager: &mut ZoneManager,
    entity: Entity,
    controller: Entity,
) {
    // Implementation would remove from battlefield and add to graveyard
    if let Some(battlefield) = zone_manager.battlefields.get_mut(&controller) {
        if let Some(pos) = battlefield.iter().position(|&e| e == entity) {
            battlefield.swap_remove(pos);
            
            // Add to graveyard
            zone_manager.graveyards
                .entry(controller)
                .or_default()
                .push(entity);
                
            // Send zone change event
            commands.spawn(ZoneChangeEvent {
                card: entity,
                source: Zone::Battlefield,
                destination: Zone::Graveyard,
            });
        }
    }
}
```

### Legendary Permanent Check (Legend Rule)

```rust
fn check_legend_rule(
    commands: &mut Commands,
    permanents: &Query<(Entity, &Permanent)>,
    zone_manager: &mut ZoneManager,
    players: &Query<(Entity, &CommanderPlayer)>,
) -> usize {
    let mut actions_performed = 0;
    
    // Group legendary permanents by name and controller
    let mut legend_groups: HashMap<(String, Entity), Vec<Entity>> = HashMap::new();
    
    for (entity, permanent) in permanents.iter() {
        if permanent.is_legendary {
            legend_groups
                .entry((permanent.name.clone(), permanent.controller))
                .or_default()
                .push(entity);
        }
    }
    
    // Check each group for duplicates
    for ((name, controller), entities) in legend_groups.iter() {
        if entities.len() > 1 {
            // Controller must choose one to keep
            commands.spawn(LegendRuleChoiceEvent {
                controller: *controller,
                permanents: entities.clone(),
                name: name.clone(),
            });
            
            actions_performed += 1;
        }
    }
    
    actions_performed
}
```

### Attachment Validity Check

```rust
fn check_attachment_validity(
    commands: &mut Commands,
    permanents: &Query<(Entity, &Permanent)>,
    zone_manager: &mut ZoneManager,
) -> usize {
    let mut actions_performed = 0;
    let mut attachments_to_drop = Vec::new();
    
    // Check all permanents with attachments
    for (entity, permanent) in permanents.iter() {
        if permanent.is_aura || permanent.is_equipment {
            let attached_to = permanent.attached_to;
            
            if let Some(target) = attached_to {
                // Check if attachment target is invalid
                if !is_valid_attachment_target(entity, target, permanent, permanents) {
                    attachments_to_drop.push((entity, permanent.controller));
                    actions_performed += 1;
                }
            } else if permanent.is_aura {
                // Auras must be attached
                attachments_to_drop.push((entity, permanent.controller));
                actions_performed += 1;
            }
        }
    }
    
    // Process invalid attachments
    for (entity, controller) in attachments_to_drop {
        // For Auras, put in graveyard
        if is_aura(entity, permanents) {
            move_to_graveyard(commands, zone_manager, entity, controller);
        } 
        // For Equipment, unattach but leave on battlefield
        else if is_equipment(entity, permanents) {
            unattach_equipment(commands, entity);
        }
    }
    
    actions_performed
}

fn is_valid_attachment_target(
    attachment: Entity,
    target: Entity,
    attachment_permanent: &Permanent,
    permanents: &Query<(Entity, &Permanent)>,
) -> bool {
    // Check if target still exists
    if let Ok((_, target_permanent)) = permanents.get(target) {
        // Check if target has protection
        if has_protection_from(target_permanent, attachment_permanent) {
            return false;
        }
        
        // Check if target is still a valid attachment target
        // This would check if equipment can equip this type of creature, etc.
        
        return true;
    }
    
    false
}
```

## Commander-Specific State-Based Actions

### Commander Damage Check

```rust
fn check_commander_damage(
    commands: &mut Commands,
    players: &Query<(Entity, &CommanderPlayer)>,
    game_state: &CommanderGameState,
) -> usize {
    let mut actions_performed = 0;
    
    for (entity, player) in players.iter() {
        // Check commander damage from each opponent
        for (source_player, damage) in player.commander_damage_received.iter() {
            if *damage >= game_state.commander_damage_threshold && !player.has_lost {
                commands.spawn(GameEvent::PlayerEliminated {
                    player: entity,
                    reason: EliminationReason::CommanderDamage(*source_player),
                });
                actions_performed += 1;
                break; // Only need to eliminate once
            }
        }
    }
    
    actions_performed
}
```

### Commander Zone Change Check

```rust
fn handle_commander_zone_changes(
    commands: &mut Commands,
    players: &Query<(Entity, &CommanderPlayer)>,
    cmd_zone_manager: &CommandZoneManager,
    zone_events: &mut EventReader<ZoneChangeEvent>,
) -> usize {
    let mut actions_performed = 0;
    
    for event in zone_events.read() {
        // Check if the card is a commander
        for (player_entity, player) in players.iter() {
            if player.commander_entities.contains(&event.card) {
                // If commander changed zones to graveyard or exile
                if event.destination == Zone::Graveyard || event.destination == Zone::Exile {
                    // Offer player choice to move to command zone
                    commands.spawn(CommanderZoneChoiceEvent {
                        commander: event.card,
                        owner: player_entity,
                        current_zone: event.destination,
                        can_go_to_command_zone: true,
                    });
                    
                    actions_performed += 1;
                }
            }
        }
    }
    
    actions_performed
}
```

## Optimization for Multiplayer

```rust
fn optimize_sba_checks(
    sba_system: &mut StateBasedActionSystem,
    player_count: usize,
) {
    // Adjust check frequency based on player count
    let base_frequency = std::time::Duration::from_millis(200);
    let player_factor = (player_count as f32 / 4.0).max(1.0);
    
    sba_system.check_frequency = base_frequency.mul_f32(player_factor);
    
    // Adjust batch size for larger games
    sba_system.batch_size = player_count * 5;
    
    // Prioritize critical checks in large games
    if player_count > 6 {
        sba_system.priority_categories.player_elimination = 1;
        sba_system.priority_categories.creature_death = 2;
        sba_system.priority_categories.legend_rule = 3;
        // Lower priority for less critical checks
    }
}
```

## Integration Points

- **Game State Module**: Receives updates from state-based actions
- **Player Module**: Processes player eliminations
- **Combat Module**: Provides damage information for creature checks
- **Zone Management**: Coordinates card movement between zones
- **Command Zone**: Special handling for Commander cards

## Testing Strategy

1. **Unit Tests**:
   - Test each individual state-based action
   - Verify correct detection of each game state condition
   - Test action ordering and priority
   
2. **Integration Tests**:
   - Test interactions between different state-based actions
   - Verify persistence of state modifications
   - Test performance with large game states
   - Validate Commander-specific rules

## Performance Considerations

For Commander games with many players:

- Batch processing of similar state checks
- Priority-based checking (check most critical conditions first)
- Efficient data structures for quick state lookups
- Only check relevant parts of state modified since last check
- Throttling check frequency based on game complexity 