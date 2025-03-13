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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommanderZoneLocation {
    CommandZone,
    Battlefield,
    Graveyard,
    Exile,
    Library,
    Hand,
    Stack,
    Limbo, // Transitional state
}
```

### Commander Components

```rust
#[derive(Component)]
pub struct Commander {
    pub owner: Entity,
    pub partner: Option<Entity>,
    pub background: Option<Entity>,
    pub color_identity: ColorIdentity,
}

#[derive(Component)]
pub struct CommanderCastable {
    pub base_cost: ManaCost,
    pub current_tax: u32,
}
```

## Key Systems

### Command Zone Initialization

```rust
pub fn initialize_command_zone(
    mut commands: Commands,
    mut command_zone: ResMut<CommandZoneManager>,
    players: Query<Entity, With<Player>>,
    commanders: Query<(Entity, &Commander)>,
) {
    // Create empty command zone entries for each player
    for player in players.iter() {
        command_zone.command_zones.insert(player, Vec::new());
    }
    
    // Place all commanders in their owners' command zones
    for (entity, commander) in commanders.iter() {
        if let Some(zone_list) = command_zone.command_zones.get_mut(&commander.owner) {
            zone_list.push(entity);
        }
        
        // Set initial zone status
        command_zone.commander_zone_status.insert(entity, CommanderZoneLocation::CommandZone);
        
        // Initialize cast count for commander tax
        command_zone.cast_count.insert(entity, 0);
        
        // Add castable component with initial tax of zero
        if let Ok(card) = commanders.get_component::<Card>(entity) {
            commands.entity(entity).insert(CommanderCastable {
                base_cost: card.cost.clone(),
                current_tax: 0,
            });
        }
    }
    
    // Set up partner relationships
    for (entity, commander) in commanders.iter() {
        if let Some(partner) = commander.partner {
            command_zone.commander_partnerships.insert(entity, partner);
        }
        
        if let Some(background) = commander.background {
            command_zone.backgrounds.insert(entity, background);
        }
    }
}
```

### Commander Casting System

```rust
pub fn handle_commander_cast(
    mut commands: Commands,
    mut command_zone: ResMut<CommandZoneManager>,
    mut cast_events: EventReader<CastFromCommandZoneEvent>,
    mut castable: Query<&mut CommanderCastable>,
) {
    for event in cast_events.read() {
        let commander_entity = event.commander;
        
        // Update zone status
        command_zone.commander_zone_status.insert(commander_entity, CommanderZoneLocation::Stack);
        
        // Increment cast count for commander tax
        if let Some(count) = command_zone.cast_count.get_mut(&commander_entity) {
            *count += 1;
            
            // Update tax amount for next cast
            if let Ok(mut commander_castable) = castable.get_mut(commander_entity) {
                commander_castable.current_tax = *count * 2; // 2 mana per previous cast
            }
        }
        
        // Remove from command zone list
        if let Some(player_zone) = command_zone.command_zones.get_mut(&event.player) {
            if let Some(pos) = player_zone.iter().position(|&c| c == commander_entity) {
                player_zone.swap_remove(pos);
            }
        }
    }
}
```

### Zone Transition Handler

```rust
pub fn handle_commander_zone_transitions(
    mut commands: Commands,
    mut command_zone: ResMut<CommandZoneManager>,
    mut zone_events: EventReader<ZoneTransitionEvent>,
    mut choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    for event in zone_events.read() {
        // Only process events for commander entities
        if !command_zone.commander_zone_status.contains_key(&event.entity) {
            continue;
        }
        
        let destination = match event.destination {
            Zone::Graveyard => CommanderZoneLocation::Graveyard,
            Zone::Exile => CommanderZoneLocation::Exile,
            Zone::Library => CommanderZoneLocation::Library,
            Zone::Hand => CommanderZoneLocation::Hand,
            Zone::Battlefield => CommanderZoneLocation::Battlefield,
            Zone::Stack => CommanderZoneLocation::Stack,
            // Handle other zones...
            _ => continue,
        };
        
        // Record death/exile for state-based actions
        if destination == CommanderZoneLocation::Graveyard {
            command_zone.died_this_turn.insert(event.entity);
        } else if destination == CommanderZoneLocation::Exile {
            command_zone.exiled_this_turn.insert(event.entity);
        }
        
        // Update commander location
        command_zone.commander_zone_status.insert(event.entity, destination);
        
        // If moving to graveyard, exile, library, or hand, offer replacement to command zone
        if matches!(destination, 
            CommanderZoneLocation::Graveyard | 
            CommanderZoneLocation::Exile | 
            CommanderZoneLocation::Library | 
            CommanderZoneLocation::Hand) 
        {
            // Find owner
            let owner = commanders.get_component::<Commander>(event.entity)
                .map(|c| c.owner)
                .unwrap_or(event.controller);
                
            // Send choice event to owner
            choice_events.send(CommanderZoneChoiceEvent {
                commander: event.entity,
                owner,
                from_zone: destination,
            });
        }
    }
}
```

## Command Zone API

The Command Zone module provides a public API for other modules to interact with commander-specific functionality:

```rust
impl CommandZoneManager {
    // Get the current tax for a commander
    pub fn get_commander_tax(&self, commander: Entity) -> u32 {
        self.cast_count.get(&commander).copied().unwrap_or(0) * 2
    }
    
    // Check if an entity is a commander
    pub fn is_commander(&self, entity: Entity) -> bool {
        self.commander_zone_status.contains_key(&entity)
    }
    
    // Move a commander to the command zone
    pub fn move_to_command_zone(&mut self, commander: Entity, owner: Entity) {
        // Update status
        self.commander_zone_status.insert(commander, CommanderZoneLocation::CommandZone);
        
        // Add to command zone list
        if let Some(zone) = self.command_zones.get_mut(&owner) {
            if !zone.contains(&commander) {
                zone.push(commander);
            }
        }
    }
    
    // Get all commanders for a player
    pub fn get_player_commanders(&self, player: Entity) -> Vec<Entity> {
        self.command_zones.get(&player)
            .cloned()
            .unwrap_or_default()
    }
}
```

The Command Zone management module is central to the Commander format, as it implements the unique rules that define the format, including the command zone, commander casting with tax, and zone transition replacements. 