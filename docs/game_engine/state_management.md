# Game State Management

This document describes how game state is managed and updated throughout the Magic: The Gathering game.

## State Components

The game state is composed of several interconnected components:

- **Player State**: Life totals, mana pools, and other player-specific information
- **Card State**: Location, orientation, counters, and other card-specific states
- **Stack**: Spells and abilities waiting to resolve
- **Zones**: Organization of cards in different game areas
- **Turn Structure**: Current phase, step, and active player
- **Combat State**: Attackers, blockers, and damage assignment

## State Management Architecture

Rummage uses an entity-component-system (ECS) architecture through Bevy:

1. Game objects are represented as entities
2. State is stored in components
3. Game logic is implemented in systems
4. Events drive state transitions

## State Synchronization

For multiplayer games, state is synchronized across clients:

- Authoritative server maintains the true game state
- Clients maintain a local representation of the state
- State updates are sent as change events
- Conflict resolution handles edge cases

## Integration with UI

The game state integrates with UI elements:

- [Card Visualization](../game_gui/cards/index.md): UI reflects card state
- [Drag and Drop](../game_gui/interaction/drag_and_drop.md): UI interactions update state
- [Targeting](../game_gui/interaction/targeting.md): Targeting validates against state

## State Persistence

Game state can be:

- Saved for resuming games later
- Snapshotted for replay or analysis
- Rolled back for handling certain effects

## Implementation Example

```rust
// Example of updating card state when it's played
fn play_card_system(
    mut commands: Commands,
    mut event_reader: EventReader<PlayCardEvent>,
    mut game_state: ResMut<GameState>,
    cards: Query<(Entity, &Card)>,
) {
    for event in event_reader.read() {
        // Move card to battlefield zone
        game_state.move_card_to_zone(event.card_entity, Zone::Battlefield);
        
        // Update card state
        commands.entity(event.card_entity).insert(CardPlayedState {
            played_this_turn: true,
            controller: event.player_entity,
        });
        
        // Trigger appropriate effects
        commands.spawn(CardPlayedEffectEvent {
            card_entity: event.card_entity,
            player_entity: event.player_entity,
        });
    }
}
```

For more details on game rules implementation, see [MTG Rules Implementation](../mtg_rules/index.md). 