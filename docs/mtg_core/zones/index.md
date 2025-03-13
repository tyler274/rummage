# Game Zones

## Overview

Zones are distinct areas where cards can exist during a game of Magic: The Gathering. This section documents the core implementation of game zones in Rummage, which serve as the foundation for all MTG formats.

## Standard Game Zones

Magic: The Gathering has the following standard game zones:

1. **Library** - The player's deck of cards, face down and in a randomized order
2. **Hand** - Cards held by a player, visible only to that player (unless affected by card effects)
3. **Battlefield** - Where permanents (lands, creatures, artifacts, enchantments, planeswalkers) exist in play
4. **Graveyard** - Discard pile for destroyed, sacrificed, or discarded cards, face up
5. **Stack** - Where spells and abilities wait to resolve
6. **Exile** - Cards removed from the game, face up unless specified otherwise

Additionally, some formats introduce special zones:

- **Command Zone** - Used primarily in Commander format for commanders and emblems

## Core Zone Implementation

Each zone is implemented as an entity with components that track its contained cards. The zone system manages the movement of cards between zones according to the MTG rules.

### Zone Management Components

```rust
#[derive(Resource)]
pub struct ZoneManager {
    // Maps player entities to their personal zones (library, hand, graveyard)
    pub player_zones: HashMap<Entity, PlayerZones>,
    
    // Shared zones (battlefield, stack, exile)
    pub battlefield: Entity,
    pub stack: Entity,
    pub exile: Entity,
    
    // Format-specific zones can be added by plugins
    pub command_zone: Option<Entity>,
}

#[derive(Component)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub owner: Option<Entity>, // None for shared zones
    pub cards: Vec<Entity>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

pub struct PlayerZones {
    pub library: Entity,
    pub hand: Entity,
    pub graveyard: Entity,
}
```

## Zone Transitions

When a card moves from one zone to another, a zone transition occurs. These transitions are managed by the zone system and can trigger abilities based on the zones involved.

### Zone Transition Events

```rust
#[derive(Event)]
pub struct ZoneTransitionEvent {
    pub card: Entity,
    pub from_zone: Entity,
    pub to_zone: Entity,
    pub reason: ZoneTransitionReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneTransitionReason {
    Cast,
    Resolve,
    Destroy,
    Sacrifice,
    Discard,
    Draw,
    ReturnToHand,
    PutIntoPlay,
    Exile,
    ShuffleIntoLibrary,
    // ...and more
}
```

### Zone Transition System

```rust
pub fn handle_zone_transitions(
    mut commands: Commands,
    mut zone_events: EventReader<ZoneTransitionEvent>,
    mut zones: Query<&mut Zone>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
    cards: Query<&Card>,
) {
    for event in zone_events.iter() {
        // Remove card from source zone
        if let Ok(mut source_zone) = zones.get_mut(event.from_zone) {
            if let Some(index) = source_zone.cards.iter().position(|&c| c == event.card) {
                source_zone.cards.remove(index);
            }
        }
        
        // Add card to destination zone
        if let Ok(mut dest_zone) = zones.get_mut(event.to_zone) {
            dest_zone.cards.push(event.card);
        }
        
        // Check for triggered abilities based on zone transitions
        if let Ok(card) = cards.get(event.card) {
            // Process any "when this card enters/leaves a zone" abilities
            // ...
        }
    }
}
```

## Zone-Specific Rules

Each zone has specific rules that govern how cards interact within and with that zone:

### Library

- Cards are face down and in random order
- Players draw from the top
- Running out of cards to draw results in a loss

### Hand

- Cards are visible only to their owner
- Hand size is normally limited to 7 at end of turn
- Players discard down to maximum hand size during cleanup

### Battlefield

- Only permanents can exist on the battlefield
- Cards on the battlefield are affected by "summoning sickness"
- Most abilities can only be activated from the battlefield

### Graveyard

- Cards are face up and can be seen by all players
- Order of cards matters for some effects
- Some abilities function from the graveyard

### Stack

- Spells and abilities go on the stack before resolving
- The stack resolves in "last in, first out" order
- All players get priority between stack resolutions

### Exile

- Cards are face up by default
- Exiled cards are generally inaccessible
- Some effects allow interaction with exiled cards

## Format Extensions

Different formats may extend or modify the basic zone system:

- **Commander Format**: Adds the Command Zone for commanders
- **Planechase**: Adds a plane card zone
- **Archenemy**: Adds a scheme card zone

For format-specific zone mechanics like the Command Zone in Commander, see the respective format documentation.

---

Next: [Zone Transitions](zone_transitions.md) 