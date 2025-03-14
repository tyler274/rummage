# State Management

The state management system is responsible for tracking, updating, and maintaining the consistency of game state in Rummage. It represents all the information needed to describe the current state of a Magic: The Gathering game.

## State Components

Game state is divided into several major components:

### Player State

Players have various attributes tracked by the state system:

- **Life totals**: Current life points
- **Mana pool**: Available mana for casting spells
- **Hand**: Cards in the player's hand
- **Library**: Cards in the player's deck
- **Graveyard**: Cards in the player's discard pile
- **Commander zone**: Special zone for Commander cards

### Card State

Cards have multiple state properties:

- **Zone location**: Current game zone (hand, battlefield, etc.)
- **Physical state**: Tapped/untapped, face-up/face-down, etc.
- **Counters**: Various counters on the card
- **Attached entities**: Equipment, Auras, etc.
- **Temporary effects**: Effects currently modifying the card

### Game Flow State

The overall game flow has state:

- **Current turn**: Which player's turn it is
- **Phase/step**: Current phase and step in the turn
- **Priority holder**: Which player currently has priority
- **Stack**: Spells and abilities waiting to resolve

## ECS Implementation

State is implemented using Bevy's Entity Component System:

- **Entities**: Cards, players, and other game objects
- **Components**: State attributes attached to entities
- **Systems**: Logic that operates on components
- **Resources**: Global state shared across entities

## Example Implementation

Here's a simplified example of how player state might be implemented:

```rust
// Player life component
#[derive(Component)]
pub struct Life {
    pub current: u32,
    pub maximum: u32,
}

// Player mana pool component
#[derive(Component)]
pub struct ManaPool {
    pub white: u32,
    pub blue: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub colorless: u32,
}

// System to update life totals
fn update_life_system(
    mut player_query: Query<&mut Life>,
    mut life_events: EventReader<LifeChangeEvent>,
) {
    for event in life_events.read() {
        if let Ok(mut life) = player_query.get_mut(event.player_entity) {
            life.current = (life.current as i32 + event.amount).max(0) as u32;
        }
    }
}
```

## State Change Process

State changes follow a specific process:

1. **Event-driven**: Changes are triggered by events
2. **Validation**: Changes are validated against game rules
3. **Execution**: Changes are applied to components
4. **Side effects**: Changes may trigger additional events
5. **State-based actions**: After each change, state-based actions are checked

## Integration with Rules

The state system is tightly integrated with the [MTG rules implementation](../../mtg_rules/index.md):

- Rules define valid state transitions
- State-based actions maintain invariants
- Turn structure progresses through defined states

## Persistence

The state system supports:

- **Serialization**: Convert state to data format for storage
- **Deserialization**: Restore state from data
- **Snapshots**: Capture state at a point in time
- **Rollback**: Restore to a previous state if needed

## Networking

For multiplayer games, state is synchronized across clients. See [Networking](../../networking/gameplay/state/index.md) for details on how state is kept consistent across the network.

## Related Components

The state system works closely with:

- [Event System](../events/index.md): Events trigger state changes
- [Snapshot System](../../core_systems/snapshot/index.md): Captures and restores state
- [Card Effects](../../card_systems/effects/index.md): Effects modify state 