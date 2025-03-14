# Commander Zone Transitions

## Overview

Zone transitions are a critical aspect of the Commander format, particularly with the special replacement effect that allows commanders to return to the Command Zone instead of going to certain other zones. This document details the implementation of these zone transition rules according to Magic: The Gathering Comprehensive Rules section 903.9.

## Commander Zone Transition Rules

According to the rules:

1. If a commander would be exiled or put into a hand, graveyard, or library from anywhere, its owner may choose to put it into the command zone instead. This is a replacement effect.
2. The choice is made as the zone change would occur.
3. If a commander is moved directly to the command zone in this way, its last known information is used to determine what happened to it.

## Implementation Components

### Zone Transition Events

```rust
#[derive(Event)]
pub struct ZoneTransitionEvent {
    pub entity: Entity,
    pub controller: Entity,
    pub source: Zone,
    pub destination: Zone,
    pub reason: ZoneChangeReason,
}

#[derive(Event)]
pub struct CommanderZoneChoiceEvent {
    pub commander: Entity,
    pub owner: Entity,
    pub from_zone: CommanderZoneLocation,
}

#[derive(Event)]
pub struct CommanderZoneChoiceResponseEvent {
    pub commander: Entity,
    pub owner: Entity,
    pub choose_command_zone: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneChangeReason {
    Cast,
    Resolve,
    PutOntoBattlefield,
    Destroy,
    Sacrifice,
    Exile,
    ReturnToHand,
    PutIntoGraveyard,
    PutIntoLibrary,
    PhasingOut,
    PhasingIn,
    StateBasedAction,
    Replacement,
    Other,
}
```

### Zone Transition System

```rust
pub fn handle_commander_zone_choice_responses(
    mut commands: Commands,
    mut command_zone: ResMut<CommandZoneManager>,
    mut response_events: EventReader<CommanderZoneChoiceResponseEvent>,
    mut state_actions: EventWriter<StateBasedActionEvent>,
) {
    for event in response_events.read() {
        if event.choose_command_zone {
            // Update commander location to Command Zone
            command_zone.commander_zone_status.insert(event.commander, CommanderZoneLocation::CommandZone);
            
            // Add to command zone list if not already there
            if let Some(zone) = command_zone.command_zones.get_mut(&event.owner) {
                if !zone.contains(&event.commander) {
                    zone.push(event.commander);
                }
            }
            
            // Trigger any state-based actions that might care about this
            state_actions.send(StateBasedActionEvent::CommanderZoneChanged(event.commander));
        }
    }
}
```

## Zone Transition Logic

The zone transition system follows this logical flow:

1. A card is about to change zones (triggered by a game event)
2. The system checks if the card is a commander
3. If it is, and the destination is hand, graveyard, library, or exile:
   a. A choice event is sent to the owner
   b. The game may need to wait for user input (or AI decision)
   c. The owner chooses whether to redirect to command zone
4. Based on the choice, the card either goes to the original destination or the command zone
5. State-based actions are triggered to handle any consequences

### Zone Transition Diagram

```
                   +-----------------+
                   | Zone Change     |
                   | Triggered       |
                   +--------+--------+
                            |
                   +--------v--------+
                   | Is it a         |
              No   | Commander?      |   Yes
          +--------+                 +--------+
          |        +-----------------+        |
          |                                   |
+---------v---------+               +---------v---------+
| Normal Zone       |               | Destination GY,   |   No
| Change Processing |               | Hand, Library,    +--------+
+-------------------+               | or Exile?         |        |
                                    +---------+---------+        |
                                              |                  |
                                              | Yes              |
                                    +---------v---------+  +-----v------+
                                    | Send Choice Event |  | Normal Zone|
                                    | to Owner          |  | Change     |
                                    +---------+---------+  +------------+
                                              |
                             +-----------+    |    +------------+
                             |           |    |    |            |
                      +------v------+    |    |  +-v-----------v--+
                      | Choose      |    |    |  | Choose Original |
                      | Command Zone|<---+----+->| Destination     |
                      +------+------+         |  +--------+--------+
                             |                |           |
               +-------------v-------------+  |  +--------v---------+
               | Move to Command Zone      |  |  | Move to Original |
               | Update Status             |  |  | Destination      |
               | Trigger State-Based       |  |  | Update Status    |
               | Actions                   |  |  | Trigger Effects  |
               +---------------------------+  |  +------------------+
                                              |
                                              v
                                    +-------------------+
                                    | Continue Game     |
                                    | Processing        |
                                    +-------------------+
```

## Special Cases

### Death Triggers

When a commander would die (go to the graveyard from the battlefield), it can create complications with death triggers:

```rust
pub fn handle_commander_death_triggers(
    command_zone: Res<CommandZoneManager>,
    mut death_events: EventReader<DeathEvent>,
    mut death_triggers: EventWriter<TriggerEvent>,
) {
    for event in death_events.read() {
        let entity = event.entity;
        
        // Check if this is a commander that was redirected to the command zone
        if command_zone.is_commander(entity) && 
           command_zone.commander_zone_status.get(&entity) == Some(&CommanderZoneLocation::CommandZone) &&
           command_zone.died_this_turn.contains(&entity) {
            
            // Create death trigger events even though it went to command zone
            death_triggers.send(TriggerEvent::Death {
                entity,
                from_battlefield: true,
                redirected_to_command_zone: true,
            });
        }
    }
}
```

### Multiple Zone Changes

If a commander would quickly change zones multiple times, each transition is handled separately:

```rust
pub fn handle_rapid_zone_changes(
    mut command_zone: ResMut<CommandZoneManager>,
    mut transition_events: EventReader<ZoneTransitionEvent>,
    mut choice_events: EventWriter<CommanderZoneChoiceEvent>,
) {
    // Track entities that have already had a choice offered this update
    let mut already_processed = HashSet::new();
    
    for event in transition_events.read() {
        // Skip non-commander entities
        if !command_zone.is_commander(event.entity) {
            continue;
        }
        
        // Skip if we already processed this entity this frame
        if already_processed.contains(&event.entity) {
            continue;
        }
        
        // Process as normal...
        
        // Mark as processed
        already_processed.insert(event.entity);
    }
}
```

The zone transition system is one of the most complex parts of the Commander implementation, as it needs to handle the unique replacement effects that define the format while preserving proper triggers and game state. 