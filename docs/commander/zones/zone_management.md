# Zone Management in Commander

## Overview

Zone management in Commander follows the standard Magic: The Gathering zone structure but with special considerations for the Command Zone and commander-specific zone transitions. This document covers the implementation of zone management in the Commander format.

## Game Zones

Commander uses all standard Magic: The Gathering zones plus the Command Zone:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    Battlefield,
    Stack,
    Hand,
    Library,
    Graveyard,
    Exile,
    Command,
    Limbo, // Transitional zone
}
```

## Zone Manager Implementation

The Zone Manager keeps track of all entities in each zone:

```rust
#[derive(Resource)]
pub struct ZoneManager {
    // Maps zone type to a set of entities in that zone
    pub zones: HashMap<Zone, HashSet<Entity>>,
    
    // Maps entity to its current zone
    pub entity_zones: HashMap<Entity, Zone>,
    
    // Maps player to their personal zones (hand, library, etc.)
    pub player_zones: HashMap<Entity, HashMap<Zone, HashSet<Entity>>>,
}
```

## Key Systems

### Zone Initialization

```rust
pub fn initialize_zones(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    players: Query<Entity, With<Player>>,
) {
    // Initialize global zones
    zone_manager.zones.insert(Zone::Battlefield, HashSet::new());
    zone_manager.zones.insert(Zone::Stack, HashSet::new());
    zone_manager.zones.insert(Zone::Exile, HashSet::new());
    zone_manager.zones.insert(Zone::Command, HashSet::new());
    zone_manager.zones.insert(Zone::Limbo, HashSet::new());
    
    // Initialize player-specific zones
    for player in players.iter() {
        let mut player_zones = HashMap::new();
        player_zones.insert(Zone::Hand, HashSet::new());
        player_zones.insert(Zone::Library, HashSet::new());
        player_zones.insert(Zone::Graveyard, HashSet::new());
        
        zone_manager.player_zones.insert(player, player_zones);
    }
}
```

### Zone Transition System

```rust
pub fn handle_zone_transitions(
    mut commands: Commands,
    mut zone_manager: ResMut<ZoneManager>,
    mut events: EventReader<ZoneTransitionEvent>,
    mut commander_events: EventWriter<CommanderZoneTransitionEvent>,
    command_zone: Res<CommandZoneManager>,
) {
    for event in events.read() {
        let entity = event.entity;
        let controller = event.controller;
        let source = event.source;
        let destination = event.destination;
        
        // Check if entity is a commander for special handling
        let is_commander = command_zone.is_commander(entity);
        
        // Special case for commander - emit commander-specific event
        if is_commander {
            commander_events.send(CommanderZoneTransitionEvent {
                commander: entity,
                controller,
                source,
                destination,
                reason: event.reason,
            });
            
            // Let the commander system handle it
            continue;
        }
        
        // Normal zone transition handling
        handle_normal_zone_transition(
            &mut zone_manager, 
            entity, 
            controller, 
            source, 
            destination
        );
    }
}

fn handle_normal_zone_transition(
    zone_manager: &mut ZoneManager,
    entity: Entity,
    controller: Entity,
    source: Zone,
    destination: Zone,
) {
    // Remove from source zone
    match source {
        Zone::Battlefield | Zone::Stack | Zone::Exile | Zone::Command | Zone::Limbo => {
            if let Some(zone_entities) = zone_manager.zones.get_mut(&source) {
                zone_entities.remove(&entity);
            }
        },
        Zone::Hand | Zone::Library | Zone::Graveyard => {
            if let Some(player_zones) = zone_manager.player_zones.get_mut(&controller) {
                if let Some(zone_entities) = player_zones.get_mut(&source) {
                    zone_entities.remove(&entity);
                }
            }
        }
    }
    
    // Add to destination zone
    match destination {
        Zone::Battlefield | Zone::Stack | Zone::Exile | Zone::Command | Zone::Limbo => {
            if let Some(zone_entities) = zone_manager.zones.get_mut(&destination) {
                zone_entities.insert(entity);
            }
        },
        Zone::Hand | Zone::Library | Zone::Graveyard => {
            if let Some(player_zones) = zone_manager.player_zones.get_mut(&controller) {
                if let Some(zone_entities) = player_zones.get_mut(&destination) {
                    zone_entities.insert(entity);
                }
            }
        }
    }
    
    // Update entity's current zone
    zone_manager.entity_zones.insert(entity, destination);
}
```

## Command Zone Integration

The Zone Manager integrates with the Command Zone manager to handle special Commander-specific zone transitions:

```rust
pub fn sync_command_zone(
    mut zone_manager: ResMut<ZoneManager>,
    command_zone: Res<CommandZoneManager>,
) {
    // Get all entities currently in the command zone
    let mut command_zone_entities = HashSet::new();
    
    for player_commanders in command_zone.command_zones.values() {
        for commander in player_commanders.iter() {
            command_zone_entities.insert(*commander);
        }
    }
    
    // Update the zone manager's command zone
    if let Some(zone_entities) = zone_manager.zones.get_mut(&Zone::Command) {
        *zone_entities = command_zone_entities;
    }
    
    // Update entity_zones for all commanders
    for (commander, location) in command_zone.commander_zone_status.iter() {
        if *location == CommanderZoneLocation::CommandZone {
            zone_manager.entity_zones.insert(*commander, Zone::Command);
        }
    }
}
```

## Special Commander Zone Considerations

### Casting From Command Zone

```rust
pub fn enable_command_zone_casting(
    mut commands: Commands,
    zone_manager: Res<ZoneManager>,
    command_zone: Res<CommandZoneManager>,
    mut players: Query<(Entity, &mut PlayerActions)>,
) {
    for (player, mut actions) in players.iter_mut() {
        // Get player's commanders in the command zone
        let commanders = command_zone
            .command_zones
            .get(&player)
            .cloned()
            .unwrap_or_default();
        
        // Add castable action for each commander
        for commander in commanders {
            if let Some(tax) = command_zone.cast_count.get(&commander) {
                actions.castable_from_command_zone.insert(
                    commander, 
                    *tax * 2 // 2 mana per previous cast
                );
            }
        }
    }
}
```

### Zone Visibility

Zone visibility in Commander follows standard MTG rules with players only seeing their own hands and libraries:

```rust
pub fn update_zone_visibility(
    mut commands: Commands,
    zone_manager: Res<ZoneManager>,
    players: Query<Entity, With<Player>>,
) {
    for player in players.iter() {
        // Add components for visible zones
        let mut visible_entities = HashSet::new();
        
        // Public zones (visible to all)
        for zone in [Zone::Battlefield, Zone::Stack, Zone::Exile, Zone::Command].iter() {
            if let Some(zone_entities) = zone_manager.zones.get(zone) {
                visible_entities.extend(zone_entities.iter());
            }
        }
        
        // All players' graveyards are public
        for (other_player, zones) in zone_manager.player_zones.iter() {
            if let Some(graveyard) = zones.get(&Zone::Graveyard) {
                visible_entities.extend(graveyard.iter());
            }
        }
        
        // Player's own hand and library (top card only if revealed)
        if let Some(player_zones) = zone_manager.player_zones.get(&player) {
            if let Some(hand) = player_zones.get(&Zone::Hand) {
                visible_entities.extend(hand.iter());
            }
            
            // Other private zone logic...
        }
        
        // Update visibility components
        commands.entity(player).insert(ZoneVisibility {
            visible_entities: visible_entities.into_iter().collect(),
        });
    }
}
```

The Zone Management system provides a complete implementation of Magic: The Gathering zones with special handling for Commander-specific mechanics, particularly related to the Command Zone and commander movement between zones. 