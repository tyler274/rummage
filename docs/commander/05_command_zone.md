# Command Zone Management

## Overview

The Command Zone is a unique game zone central to the Commander format. This module manages the Command Zone, Commander card movement between zones, and the special rules surrounding Commander cards. It integrates with the zone management and player systems to provide a complete implementation of Commander-specific mechanics according to the official Magic: The Gathering Comprehensive Rules section 903.

## Core Components

### Command Zone Structure

```rust
#[derive(Resource)]
pub struct CommandZoneManager {
    // Maps player entity to their commander entities in the command zone
    pub command_zones: HashMap<Entity, Vec<Entity>>,
    
    // Tracks whether commanders are in the command zone or elsewhere
    pub commander_zone_status: HashMap<Entity, CommanderZoneLocation>,
    
    // Tracks the number of times each commander has been cast from the command zone
    pub cast_count: HashMap<Entity, u32>,
    
    // Tracks commanders that died/were exiled this turn (for state-based actions)
    pub died_this_turn: HashSet<Entity>,
    pub exiled_this_turn: HashSet<Entity>,
    
    // Track partner commanders and backgrounds
    pub commander_partnerships: HashMap<Entity, Entity>,
    pub backgrounds: HashMap<Entity, Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommanderZoneLocation {
    CommandZone,
    Battlefield,
    Library,
    Hand,
    Graveyard,
    Exile,
    Stack,
    Limbo, // For temporary transitions and special cases
}
```

### Commander Card Component

```rust
#[derive(Component, Debug, Clone)]
pub struct CommanderCard {
    // Reference to the player who owns this commander
    pub owner: Entity,
    
    // Commander's color identity (for deck validation)
    pub color_identity: ColorIdentity,
    
    // Commander specific flags
    pub is_partner: bool,
    pub is_background: bool,
    pub can_be_companion: bool,
    
    // Track if commander has dealt combat damage to each player this turn
    pub dealt_combat_damage_this_turn: HashSet<Entity>,
    pub total_commander_damage_dealt: HashMap<Entity, u32>,
    
    // Commander-specific abilities
    pub special_abilities: Vec<CommanderAbility>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommanderAbility {
    Eminence,
    Lieutenant,
    CommanderNinjutsu,
    Partner,
    FriendForever,
    ChooseBackdrop,
    Companion(CompanionRestriction),
    CustomAbility(String),
}
```

## Key Systems

### Command Zone Initialization

```rust
fn initialize_command_zone(
    mut commands: Commands,
    mut game_state: ResMut<CommanderGameState>,
    mut players: Query<(Entity, &CommanderPlayer)>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    cards: Query<(Entity, &Card, Option<&CommanderCard>)>,
) {
    // Initialize command zones for each player (rule 903.6)
    for (player_entity, player) in players.iter_mut() {
        // Create empty command zone entry
        cmd_zone_manager.command_zones.insert(player_entity, Vec::new());
        
        // Get commander cards for this player and put them in command zone
        for commander_entity in &player.commander_entities {
            if let Ok((entity, card, _)) = cards.get(*commander_entity) {
                // Add commander to command zone
                cmd_zone_manager.command_zones.get_mut(&player_entity).unwrap().push(entity);
                
                // Set zone status
                cmd_zone_manager.commander_zone_status.insert(entity, CommanderZoneLocation::CommandZone);
                
                // Initialize cast count
                cmd_zone_manager.cast_count.insert(entity, 0);
                
                // Additional commander initialization
                // ...
            }
        }
    }
}
```

### Commander Zone Transfer System

```rust
fn handle_commander_zone_change(
    mut command_zone_events: EventReader<CommanderZoneChoiceEvent>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in command_zone_events.read() {
        let commander_entity = event.commander;
        let player_entity = event.player;
        
        // Handle state-based action for commander in graveyard or exile (rule 903.9a)
        if event.from_zone == Zone::Graveyard || event.from_zone == Zone::Exile {
            // Commander can go to command zone as a state-based action
            if event.to_command_zone {
                // Remove from current zone
                match event.from_zone {
                    Zone::Graveyard => {
                        if let Some(graveyard) = zone_manager.graveyards.get_mut(&player_entity) {
                            if let Some(idx) = graveyard.iter().position(|&e| e == commander_entity) {
                                graveyard.swap_remove(idx);
                            }
                        }
                        cmd_zone_manager.died_this_turn.remove(&commander_entity);
                    },
                    Zone::Exile => {
                        if let Some(exile) = zone_manager.exiles.get_mut(&player_entity) {
                            if let Some(idx) = exile.iter().position(|&e| e == commander_entity) {
                                exile.swap_remove(idx);
                            }
                        }
                        cmd_zone_manager.exiled_this_turn.remove(&commander_entity);
                    },
                    _ => {}
                }
                
                // Add to command zone
                cmd_zone_manager.command_zones.get_mut(&player_entity).unwrap().push(commander_entity);
                cmd_zone_manager.commander_zone_status.insert(commander_entity, CommanderZoneLocation::CommandZone);
                
                // Send event for successful zone change
                game_events.send(GameEvent::ZoneChanged {
                    card: commander_entity,
                    from: event.from_zone,
                    to: Zone::CommandZone,
                    controller: player_entity,
                });
            }
        } 
        // Handle replacement effect for commander going to hand or library (rule 903.9b)
        else if event.from_zone == Zone::Battlefield && 
               (event.to_zone == Zone::Hand || event.to_zone == Zone::Library) {
            if event.to_command_zone {
                // This is a replacement effect - commander never goes to hand/library
                
                // Remove from battlefield
                if let Some(battlefield) = zone_manager.battlefields.get_mut(&player_entity) {
                    if let Some(idx) = battlefield.iter().position(|&e| e == commander_entity) {
                        battlefield.swap_remove(idx);
                    }
                }
                
                // Add to command zone directly
                cmd_zone_manager.command_zones.get_mut(&player_entity).unwrap().push(commander_entity);
                cmd_zone_manager.commander_zone_status.insert(commander_entity, CommanderZoneLocation::CommandZone);
                
                // Send event for successful zone change
                game_events.send(GameEvent::ZoneChanged {
                    card: commander_entity,
                    from: event.from_zone,
                    to: Zone::CommandZone,
                    controller: player_entity,
                });
            } else {
                // Commander goes to original destination
                match event.to_zone {
                    Zone::Hand => {
                        zone_manager.hands.get_mut(&player_entity).unwrap().push(commander_entity);
                        cmd_zone_manager.commander_zone_status.insert(commander_entity, CommanderZoneLocation::Hand);
                    },
                    Zone::Library => {
                        zone_manager.libraries.get_mut(&player_entity).unwrap().push(commander_entity);
                        cmd_zone_manager.commander_zone_status.insert(commander_entity, CommanderZoneLocation::Library);
                    },
                    _ => {}
                }
            }
        }
    }
}
```

### Commander Death State-Based Action

```rust
fn check_commander_death_state_based_action(
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut zone_manager: ResMut<ZoneManager>,
    mut choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    // Create a list of commanders to check, to avoid mutability issues
    let commanders_to_check: Vec<(Entity, Entity, Zone)> = cmd_zone_manager.died_this_turn
        .iter()
        .filter_map(|&commander_entity| {
            // Find the owner
            for (player, commanders) in &cmd_zone_manager.command_zones {
                if commanders.contains(&commander_entity) {
                    return Some((commander_entity, *player, Zone::Graveyard));
                }
            }
            None
        })
        .collect();
    
    // Same for exiled commanders
    let exiled_commanders: Vec<(Entity, Entity, Zone)> = cmd_zone_manager.exiled_this_turn
        .iter()
        .filter_map(|&commander_entity| {
            // Find the owner
            for (player, commanders) in &cmd_zone_manager.command_zones {
                if commanders.contains(&commander_entity) {
                    return Some((commander_entity, *player, Zone::Exile));
                }
            }
            None
        })
        .collect();
    
    // Process all dead/exiled commanders
    let all_commanders = [commanders_to_check, exiled_commanders].concat();
    
    for (commander_entity, player_entity, from_zone) in all_commanders {
        // Create a zone choice event (rule 903.9a)
        choice_events.send(CommanderZoneChoiceEvent {
            commander: commander_entity,
            player: player_entity,
            from_zone,
            to_zone: from_zone, // Original destination
            to_command_zone: true, // Default to move to command zone, but player can choose
        });
    }
}
```

### Commander Replacement Effect System

```rust
fn commander_replacement_effect(
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    commanders: Query<Entity, With<CommanderCard>>,
    mut choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    for event in zone_change_events.read() {
        // Check if this is a commander
        if commanders.contains(event.card) {
            // Handle replacement effect for hand/library (rule 903.9b)
            if event.to_zone == Zone::Hand || event.to_zone == Zone::Library {
                // Trigger choice event for replacement effect
                choice_events.send(CommanderZoneChoiceEvent {
                    commander: event.card,
                    player: event.controller,
                    from_zone: event.from_zone,
                    to_zone: event.to_zone,
                    to_command_zone: true, // Default to move to command zone, but player can choose
                });
            }
            
            // Track graveyard and exile for SBA (rule 903.9a)
            if event.to_zone == Zone::Graveyard {
                cmd_zone_manager.died_this_turn.insert(event.card);
                cmd_zone_manager.commander_zone_status.insert(event.card, CommanderZoneLocation::Graveyard);
            } else if event.to_zone == Zone::Exile {
                cmd_zone_manager.exiled_this_turn.insert(event.card);
                cmd_zone_manager.commander_zone_status.insert(event.card, CommanderZoneLocation::Exile);
            }
        }
    }
}
```

### Commander Tax Calculation

```rust
fn calculate_commander_tax(
    cmd_zone_manager: Res<CommandZoneManager>,
    commander_entity: Entity,
) -> u32 {
    // Get the number of times this commander has been cast (rule 903.8)
    let cast_count = cmd_zone_manager.cast_count.get(&commander_entity).copied().unwrap_or(0);
    
    // Each previous cast adds {2} to the cost
    cast_count * 2
}
```

### Commander Casting System

```rust
fn handle_commander_cast(
    mut commands: Commands,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cast_events: EventReader<CommanderCastEvent>,
) {
    for event in cast_events.read() {
        let commander_entity = event.commander;
        let player_entity = event.player;
        
        // Update cast count for commander tax (rule 903.8)
        let cast_count = cmd_zone_manager.cast_count.entry(commander_entity).or_insert(0);
        *cast_count += 1;
        
        // Update zone status
        cmd_zone_manager.commander_zone_status.insert(commander_entity, CommanderZoneLocation::Stack);
        
        // Move from command zone to stack
        if let Some(command_zone) = cmd_zone_manager.command_zones.get_mut(&player_entity) {
            if let Some(idx) = command_zone.iter().position(|&e| e == commander_entity) {
                command_zone.swap_remove(idx);
                
                // Add to stack (simplified, actual implementation in stack.rs)
                zone_manager.stack.push(StackItem {
                    entity: commander_entity,
                    item_type: StackItemType::Spell(SpellType::Commander),
                    controller: player_entity,
                    source: commander_entity,
                    source_zone: Zone::CommandZone,
                    targets: Vec::new(),
                    // Additional stack item fields...
                });
            }
        }
    }
}
```

### Partner Commander System

```rust
fn handle_partner_commanders(
    mut commands: Commands,
    partners: Query<(Entity, &CommanderCard), With<PartnerAbility>>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    // Find all partner commanders and link them
    for (player_entity, player) in players.iter() {
        let player_commanders: Vec<Entity> = player.commander_entities.clone();
        
        if player_commanders.len() == 2 {
            // Both might be partners, validate
            let mut valid_partners = true;
            
            for &cmd_entity in &player_commanders {
                if let Ok((_, commander_card)) = partners.get(cmd_entity) {
                    if !commander_card.is_partner {
                        valid_partners = false;
                        break;
                    }
                } else {
                    valid_partners = false;
                    break;
                }
            }
            
            // Link the partners if valid
            if valid_partners {
                cmd_zone_manager.commander_partnerships.insert(player_commanders[0], player_commanders[1]);
                cmd_zone_manager.commander_partnerships.insert(player_commanders[1], player_commanders[0]);
            }
        }
    }
}
```

## Integrating with Game State

The Command Zone module integrates with other systems:

1. **Game State**: Commanders start in the command zone (rule 903.6)
2. **Zone System**: Special rules for commander movement (rule 903.9)
3. **Stack System**: Casting commanders from command zone with tax (rule 903.8)
4. **Combat System**: Tracking commander damage (rule 903.10a)
5. **State-Based Actions**: Supporting death/exile zone transitions (rule 903.9a)

## Handling Melded and Merged Commanders

As per rule 903.9c, special handling is needed for melded or merged commanders:

```rust
fn handle_melded_commander_zone_change(
    mut zone_events: EventReader<CommanderZoneChoiceEvent>,
    melded_commanders: Query<(Entity, &MeldedPermanent)>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
) {
    for event in zone_events.read() {
        // Check if this is a melded commander
        if let Ok((entity, melded)) = melded_commanders.get(event.commander) {
            if event.to_command_zone && 
               (event.to_zone == Zone::Hand || event.to_zone == Zone::Library) {
                // Only the commander component goes to command zone (rule 903.9c)
                let commander_component = melded.components.iter()
                    .find(|&&comp| cmd_zone_manager.command_zones.values().flatten().any(|&cmd| cmd == comp))
                    .copied();
                
                if let Some(commander_card) = commander_component {
                    // Move just the commander to command zone
                    cmd_zone_manager.command_zones.get_mut(&event.player).unwrap().push(commander_card);
                    cmd_zone_manager.commander_zone_status.insert(commander_card, CommanderZoneLocation::CommandZone);
                    
                    // Move other components to appropriate zone
                    for &component in melded.components.iter().filter(|&&c| Some(c) != commander_component) {
                        match event.to_zone {
                            Zone::Hand => {
                                zone_manager.hands.get_mut(&event.player).unwrap().push(component);
                            },
                            Zone::Library => {
                                zone_manager.libraries.get_mut(&event.player).unwrap().push(component);
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
```

## Testing Strategy

1. **Unit Tests**:
   - Verify commander tax calculation
   - Test command zone transitions
   - Validate state-based actions for commanders
   
2. **Integration Tests**:
   - Test commander casting with tax increase
   - Simulate commander deaths and zone choices
   - Verify partner commander mechanics
   - Test melded commander handling

## Performance Considerations

- Optimize commander zone tracking for games with many commanders (partners, etc.)
- Batch process state-based action checks
- Use flags to quickly identify commanders in different zones

## Design Considerations

- Clear separation between command zone rules and general zone management
- Support for future Commander variants and rule changes
- Efficient tracking for commander-specific game state
- Proper handling of complex zone transitions 