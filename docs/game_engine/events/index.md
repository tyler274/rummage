# Event System

The Rummage event system is a core component of the game engine that enables communication between different systems and components in a decoupled manner. Events are used to notify about changes in game state, trigger effects, and coordinate actions between different parts of the codebase.

## Event Architecture

The event system is built on Bevy's native event system, which provides:

- **Type-safe events**: Each event is a strongly-typed struct
- **Single-frame lifetime**: Events are processed within a single frame by default
- **Multiple readers**: Multiple systems can respond to the same events
- **Ordered processing**: Events are processed in a deterministic order

## Core Event Types

Rummage implements several categories of events:

### Game Flow Events

Events that control the progression of the game:

- `TurnStartEvent`: Signals the start of a new turn
- `PhaseChangeEvent`: Indicates a change in the current phase
- `StepChangeEvent`: Indicates a change in the current step
- `PriorityPassedEvent`: Signals when priority is passed between players

### Card Events

Events related to card actions:

- `CardPlayedEvent`: Triggered when a card is played from hand
- `CardMovedEvent`: Signals when a card changes zones
- `CardStateChangedEvent`: Indicates a change in a card's state (tapped, etc.)
- `CardTargetedEvent`: Triggered when a card is targeted by a spell or ability

### Player Events

Events related to player actions and state:

- `PlayerDamageEvent`: Signals when a player takes damage
- `PlayerGainLifeEvent`: Triggered when a player gains life
- `PlayerDrawCardEvent`: Indicates a card draw
- `PlayerLosesEvent`: Signals when a player loses the game

## Custom Events

Game mechanics and card effects often require custom events. These can be created by defining a new event struct:

```rust
#[derive(Event)]
pub struct ExileCardEvent {
    pub card_entity: Entity,
    pub source_entity: Option<Entity>,
    pub until_end_of_turn: bool,
}
```

## Event Processing

Events are processed using Bevy's event readers:

```rust
fn process_card_played_events(
    mut event_reader: EventReader<CardPlayedEvent>,
    mut commands: Commands,
    // other system parameters
) {
    for event in event_reader.read() {
        // React to the card being played
        // Trigger effects, update state, etc.
    }
}
```

## Event Broadcasting

Systems can broadcast events using Bevy's event writers:

```rust
fn play_card_system(
    mut commands: Commands,
    mut event_writer: EventWriter<CardPlayedEvent>,
    // other system parameters
) {
    // Logic to determine a card is played
    
    // Broadcast the event
    event_writer.send(CardPlayedEvent {
        card_entity,
        player_entity,
        from_zone: Zone::Hand,
    });
}
```

## Event-Driven Architecture

The event system enables an event-driven architecture where:

1. Actions in the game broadcast events
2. Systems listen for relevant events
3. Events trigger state changes and additional events
4. Complex interactions emerge from simple event chains

This approach simplifies the implementation of MTG's complex rules and card interactions.

## Integration

The event system integrates with:

- [State Management](../state/index.md): Events trigger state changes
- [MTG Rules](../../mtg_rules/index.md): Rules are implemented as event handlers
- [Network System](../../networking/index.md): Events are serialized for network play

For more information on implementing card effects using events, see [Card Effects](../../card_systems/effects/index.md). 