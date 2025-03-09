# Command Zone Management

## Overview

The Command Zone is a unique game zone central to the Commander format. This module manages the Command Zone, Commander card movement between zones, and the special rules surrounding Commander cards. It integrates with the zone management and player systems to provide a complete implementation of Commander-specific mechanics.

## Core Components

### Command Zone Structure

```rust
#[derive(Resource)]
pub struct CommandZoneManager {
    // Maps player entity to their commander entities in the command zone
    pub command_zones: HashMap<Entity, Vec<Entity>>,
    
    // Tracks whether commanders are in the command zone or elsewhere
    pub commander_zone_status: HashMap<Entity, CommanderZoneLocation>,
    
    // Tracks the number of times a commander has moved to command zone
    pub zone_transition_count: HashMap<Entity, u32>,
    
    // Tracks commanders that died this turn (for "died this turn" triggers)
    pub died_this_turn: HashSet<Entity>,
    
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
    
    // Number of times cast from command zone (for tax calculation)
    pub cast_count: u32,
    
    // Commander's color identity (for deck validation)
    pub color_identity: ColorIdentity,
    
    // Commander specific flags
    pub is_partner: bool,
    pub is_background: bool,
    pub can_be_companion: bool,
    
    // Track if commander has dealt combat damage this turn
    pub dealt_combat_damage_this_turn: HashSet<Entity>,
    
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
    cards: Query<(Entity, &Card)>,
) {
    // Initialize command zones for each player
    for (player_entity, player) in players.iter_mut() {
        // Create empty command zone entry
        cmd_zone_manager.command_zones.insert(player_entity, Vec::new());
        
        // Get commander cards for this player
        for &commander_entity in &player.commander_entities {
            // Add commander to command zone
            cmd_zone_manager.command_zones
                .get_mut(&player_entity)
                .unwrap()
                .push(commander_entity);
                
            // Set initial zone status
            cmd_zone_manager.commander_zone_status
                .insert(commander_entity, CommanderZoneLocation::CommandZone);
                
            // Initialize transition count
            cmd_zone_manager.zone_transition_count
                .insert(commander_entity, 0);
                
            // Set up commander card component
            if let Ok((entity, card)) = cards.get(commander_entity) {
                // Extract color identity from card
                let color_identity = extract_color_identity(card);
                
                // Check for partner/background abilities
                let is_partner = card.rules_text.contains("Partner");
                let is_background = card.rules_text.contains("Background");
                
                // Create commander component
                let commander_card = CommanderCard {
                    owner: player_entity,
                    cast_count: 0,
                    color_identity,
                    is_partner,
                    is_background,
                    can_be_companion: false,
                    dealt_combat_damage_this_turn: HashSet::new(),
                    special_abilities: Vec::new(),
                };
                
                // Add commander component to card
                commands.entity(entity).insert(commander_card);
            }
        }
    }
    
    // Process partner commanders
    for (player_entity, player) in players.iter() {
        let commander_entities = player.commander_entities.clone();
        if commander_entities.len() == 2 {
            // Check if both have partner or one is a background
            let has_partners = commander_entities.iter().all(|&e| {
                if let Ok((_, cmd)) = cards.get_component::<CommanderCard>(e) {
                    cmd.is_partner
                } else {
                    false
                }
            });
            
            let has_background = commander_entities.iter().any(|&e| {
                if let Ok((_, cmd)) = cards.get_component::<CommanderCard>(e) {
                    cmd.is_background
                } else {
                    false
                }
            });
            
            if has_partners || has_background {
                // Link the partners or commander+background
                cmd_zone_manager.commander_partnerships.insert(
                    commander_entities[0], 
                    commander_entities[1]
                );
            } else {
                // Invalid pair - log error and handle
                error!("Invalid commander pair for player {}", player_entity.index());
            }
        }
    }
}
```

### Commander Movement System

```rust
fn handle_commander_zone_changes(
    mut commands: Commands,
    mut zone_events: EventReader<ZoneChangeEvent>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    commanders: Query<(Entity, &CommanderCard)>,
) {
    for event in zone_events.read() {
        // Only process commander cards
        if let Ok((entity, commander)) = commanders.get(event.card) {
            let owner = commander.owner;
            
            // Update zone status
            match event.destination {
                Zone::CommandZone => {
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::CommandZone);
                        
                    // Update player's commander in command zone flag
                    if let Ok((_, mut player)) = players.get_mut(owner) {
                        if let Some(index) = player.commander_entities.iter()
                            .position(|&e| e == entity) {
                            player.commander_in_command_zone[index] = true;
                        }
                    }
                },
                Zone::Battlefield => {
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Battlefield);
                },
                Zone::Graveyard => {
                    // Commander death triggers
                    if event.source == Zone::Battlefield {
                        cmd_zone_manager.died_this_turn.insert(entity);
                        
                        // Offer replacement effect to send to command zone
                        commands.spawn(CommanderZoneChoiceEvent {
                            commander: entity,
                            owner,
                            current_zone: Zone::Graveyard,
                            can_go_to_command_zone: true,
                        });
                    }
                    
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Graveyard);
                },
                Zone::Exile => {
                    // Offer replacement effect to send to command zone
                    commands.spawn(CommanderZoneChoiceEvent {
                        commander: entity,
                        owner,
                        current_zone: Zone::Exile,
                        can_go_to_command_zone: true,
                    });
                    
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Exile);
                },
                Zone::Hand => {
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Hand);
                },
                Zone::Library => {
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Library);
                },
                Zone::Stack => {
                    cmd_zone_manager.commander_zone_status
                        .insert(entity, CommanderZoneLocation::Stack);
                },
            }
            
            // Increase zone transition count if moved to command zone
            if event.destination == Zone::CommandZone {
                let count = cmd_zone_manager.zone_transition_count
                    .entry(entity)
                    .or_insert(0);
                *count += 1;
            }
        }
    }
}
```

### Commander Tax Calculation

```rust
fn calculate_commander_cost(
    commander: Entity,
    base_cost: Mana,
    cmd_zone_manager: &CommandZoneManager,
    players: &Query<(Entity, &CommanderPlayer)>,
) -> Mana {
    let mut final_cost = base_cost.clone();
    
    // Find the commander's owner and cast count
    for (_, commander_card) in players.iter() {
        if commander_card.owner == commander {
            // Add commander tax (2 generic mana per previous cast)
            let cast_count = commander_card.cast_count;
            final_cost.colorless += 2 * cast_count;
            break;
        }
    }
    
    final_cost
}
```

### Commander Damage Tracking

```rust
fn track_commander_damage(
    mut commands: Commands,
    mut damage_events: EventReader<CombatDamageEvent>,
    commanders: Query<(Entity, &CommanderCard)>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    game_state: Res<CommanderGameState>,
) {
    for event in damage_events.read() {
        // Only process commander combat damage
        if let Ok((commander_entity, commander)) = commanders.get(event.source) {
            if event.is_combat_damage && event.damage > 0 {
                // Find the damaged player
                if let Ok((damaged_player, mut player)) = players.get_mut(event.target) {
                    // Update commander damage for the player
                    let owner = commander.owner;
                    let entry = player.commander_damage_received
                        .entry(owner)
                        .or_insert(0);
                    *entry += event.damage;
                    
                    // Update tracking on the commander card
                    let mut cmd_card = commander.clone();
                    cmd_card.dealt_combat_damage_this_turn.insert(damaged_player);
                    commands.entity(commander_entity).insert(cmd_card);
                    
                    // Check for lethal commander damage
                    if *entry >= game_state.commander_damage_threshold {
                        commands.spawn(GameEvent::PlayerEliminated {
                            player: damaged_player,
                            reason: EliminationReason::CommanderDamage(owner),
                        });
                    }
                }
            }
        }
    }
}
```

### Commander Zone Choice Handling

```rust
fn handle_commander_zone_choices(
    mut commands: Commands,
    mut choice_events: EventReader<CommanderZoneChoiceEvent>,
    mut zone_manager: ResMut<ZoneManager>,
    mut cmd_zone_manager: ResMut<CommandZoneManager>,
    cards: Query<(Entity, &Card)>,
) {
    for event in choice_events.read() {
        // Typically in a real game the player would choose, 
        // but we'll implement the common choice to return to command zone
        if event.can_go_to_command_zone {
            // Remove from current zone
            match event.current_zone {
                Zone::Graveyard => {
                    let graveyard = zone_manager.graveyards
                        .get_mut(&event.owner)
                        .unwrap();
                    if let Some(pos) = graveyard.iter().position(|&e| e == event.commander) {
                        graveyard.swap_remove(pos);
                    }
                },
                Zone::Exile => {
                    let exile = zone_manager.exiles
                        .get_mut(&event.owner)
                        .unwrap();
                    if let Some(pos) = exile.iter().position(|&e| e == event.commander) {
                        exile.swap_remove(pos);
                    }
                },
                _ => { /* Handle other zones if needed */ }
            }
            
            // Add to command zone
            let command_zone = zone_manager.command_zones
                .get_mut(&event.owner)
                .unwrap();
            command_zone.push(event.commander);
            
            // Update zone status
            cmd_zone_manager.commander_zone_status
                .insert(event.commander, CommanderZoneLocation::CommandZone);
                
            // Send zone change event
            commands.spawn(ZoneChangeEvent {
                card: event.commander,
                source: event.current_zone,
                destination: Zone::CommandZone,
            });
        }
    }
}
```

## Special Commander Rules

### Color Identity Enforcement

```rust
fn validate_color_identity(
    deck: &Vec<Card>,
    commander_color_identity: &ColorIdentity,
) -> Vec<Card> {
    // Check each card against commander's color identity
    deck.iter()
        .filter(|card| {
            // Extract card's color identity
            let card_identity = extract_color_identity(card);
            
            // Card is valid if all its colors are included in commander's identity
            (card_identity.white == false || commander_color_identity.white == true) &&
            (card_identity.blue == false || commander_color_identity.blue == true) &&
            (card_identity.black == false || commander_color_identity.black == true) &&
            (card_identity.red == false || commander_color_identity.red == true) &&
            (card_identity.green == false || commander_color_identity.green == true)
        })
        .cloned()
        .collect()
}

fn extract_color_identity(card: &Card) -> ColorIdentity {
    let mut identity = ColorIdentity {
        white: card.cost.white > 0,
        blue: card.cost.blue > 0,
        black: card.cost.black > 0,
        red: card.cost.red > 0,
        green: card.cost.green > 0,
    };
    
    // Also check rules text for mana symbols
    // This is simplified and would need more complex text parsing
    if card.rules_text.contains("{W}") {
        identity.white = true;
    }
    if card.rules_text.contains("{U}") {
        identity.blue = true;
    }
    if card.rules_text.contains("{B}") {
        identity.black = true;
    }
    if card.rules_text.contains("{R}") {
        identity.red = true;
    }
    if card.rules_text.contains("{G}") {
        identity.green = true;
    }
    
    identity
}
```

## Integration Points

- **Game State Module**: Updates command zone state on game start/reset
- **Player Module**: Tracks commander-specific player data
- **Turn Structure**: Handles commander casting timing restrictions
- **Combat Module**: Processes commander combat damage
- **Zone Module**: Coordinates commander movement between zones

## Testing Strategy

1. **Unit Tests**:
   - Verify commander tax calculation
   - Test zone replacement effects
   - Validate color identity enforcement
   
2. **Integration Tests**:
   - Test full commander movement lifecycle
   - Verify commander damage tracking
   - Test partner/background handling

## Design Considerations

- Clear separation between command zone rules and general zone management
- Support for future Commander variants and rule changes
- Efficient tracking for commander-specific game state
- Proper handling of complex zone transitions 