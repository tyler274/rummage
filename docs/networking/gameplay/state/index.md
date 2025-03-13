# Game State Management in MTG Commander

This document outlines the approach to managing game state in the MTG Commander game engine's multiplayer implementation.

## Table of Contents

1. [Overview](#overview)
2. [Game State Components](#game-state-components)
3. [Implementation Approach](#implementation-approach)
4. [State Snapshots](#state-snapshots)
5. [State Synchronization](#state-synchronization)
6. [Deterministic State Updates](#deterministic-state-updates)
7. [Hidden Information](#hidden-information)
8. [Rollbacks and Recovery](#rollbacks-and-recovery)

## Overview

Proper game state management is critical for a multiplayer card game like Magic: The Gathering. The game state includes all information about the current game, including cards in various zones, player life totals, turn structure, and active effects. In a networked environment, this state must be synchronized across all clients while maintaining security and performance.

## Game State Components

The game state in MTG Commander consists of several key components:

1. **Zones**: Battlefield, hands, libraries, graveyards, exile, stack, and command zone
2. **Player Information**: Life totals, mana pools, commander damage, etc.
3. **Turn Structure**: Current phase, active player, priority player
4. **Effects**: Ongoing effects, delayed triggers, replacement effects
5. **Game Metadata**: Game ID, start time, game mode, etc.

## Implementation Approach

### Core Game State Structure

The game state is implemented as a collection of ECS components and resources:

```rust
// Core game state resource
#[derive(Resource)]
pub struct GameState {
    pub game_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub game_mode: GameMode,
    pub turn_number: u32,
    pub current_phase: Phase,
    pub active_player_id: PlayerId,
    pub priority_player_id: Option<PlayerId>,
    pub stack: Vec<StackItem>,
}

// Player component
#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
    pub client_id: ClientId,
    pub life_total: i32,
    pub mana_pool: ManaPool,
    pub commander_damage: HashMap<PlayerId, i32>,
}

// Zone components
#[derive(Component)]
pub struct Hand {
    pub cards: Vec<CardId>,
}

#[derive(Component)]
pub struct Library {
    pub cards: Vec<CardId>,
    pub top_revealed: bool,
}

#[derive(Component)]
pub struct Graveyard {
    pub cards: Vec<CardId>,
}

#[derive(Component)]
pub struct CommandZone {
    pub cards: Vec<CardId>,
}

// Battlefield is a shared resource
#[derive(Resource)]
pub struct Battlefield {
    pub permanents: Vec<Entity>,
}

// Card component
#[derive(Component)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub card_type: CardType,
    pub owner_id: PlayerId,
    pub controller_id: PlayerId,
    // Other card properties...
}
```

### State Replication with bevy_replicon

The game state is replicated using bevy_replicon, with careful control over what information is sent to each client:

```rust
// Register components for replication
fn register_replication(app: &mut App) {
    app.register_component_replication::<Player>()
        .register_component_replication::<Card>()
        // Only replicate public zone information
        .register_component_replication::<Graveyard>()
        .register_component_replication::<CommandZone>()
        // Register resources
        .register_resource_replication::<GameState>()
        .register_resource_replication::<Battlefield>();
        
    // Hand and Library require special handling for hidden information
    app.register_component_replication_with::<Hand>(
        RuleFns {
            serialize: |hand, ctx| {
                // Only send full hand to the owner
                if ctx.client_id == ctx.client_entity_map.get_client_id(hand.owner_entity) {
                    bincode::serialize(hand).ok()
                } else {
                    // Send only card count to other players
                    bincode::serialize(&HandInfo { card_count: hand.cards.len() }).ok()
                }
            },
            deserialize: |bytes, ctx| {
                // Handle deserialization based on what was sent
                // ...
            },
        }
    );
}
```

### State Synchronization

The game state is synchronized across clients using a combination of techniques:

1. **Initial State**: Full game state is sent when a client connects
2. **Incremental Updates**: Only changes are sent during gameplay
3. **Command-Based**: Player actions are sent as commands, not direct state changes
4. **Authoritative Server**: Server validates all commands before applying them

```rust
// System to process player commands
fn process_player_commands(
    mut commands: Commands,
    mut command_events: EventReader<PlayerCommand>,
    game_state: Res<GameState>,
    players: Query<(Entity, &Player)>,
    // Other queries...
) {
    for command in command_events.read() {
        // Validate the command
        if !validate_command(command, &game_state, &players) {
            continue;
        }
        
        // Apply the command to the game state
        match command {
            PlayerCommand::PlayCard { player_id, card_id, targets } => {
                // Handle playing a card
                // ...
            },
            PlayerCommand::ActivateAbility { permanent_id, ability_index, targets } => {
                // Handle activating an ability
                // ...
            },
            // Other command types...
        }
    }
}
```

## State Snapshots

In networked games, maintaining state consistency despite network disruptions is essential. Our MTG Commander implementation employs a comprehensive state rollback system for resilience:

- **Complete documentation:** [State Rollback and Recovery](rollback.md)
- Deterministic replay of game actions after network disruptions
- State snapshots at critical game moments
- RNG state preservation for consistent randomized outcomes
- Client-side prediction for responsive gameplay

The rollback system integrates tightly with our deterministic RNG implementation to ensure that random events like shuffling and coin flips remain consistent across network boundaries, even during recovery from disruptions.

## Deterministic State Updates

Maintaining state consistency is critical for a fair game experience. Several mechanisms ensure consistency:

1. **Sequence Numbers**: Commands are processed in order
2. **State Verification**: Periodic full state verification
3. **Reconciliation**: Automatic correction of client-server state differences
4. **Rollback**: Ability to roll back to a previous state if needed

```rust
// System to verify client state consistency
fn verify_client_state_consistency(
    mut server: ResMut<RepliconServer>,
    game_state: Res<GameState>,
    connected_clients: Res<ConnectedClients>,
) {
    // Periodically send state verification requests
    if game_state.turn_number % 5 == 0 && game_state.current_phase == Phase::Upkeep {
        for client_id in connected_clients.clients.keys() {
            // Generate state verification data
            let verification_data = generate_state_verification_data(&game_state);
            
            // Send verification request
            server.send_message(*client_id, StateVerificationRequest {
                turn: game_state.turn_number,
                verification_data,
            });
        }
    }
}
```

## Hidden Information

In networked games, it's important to protect sensitive information from unauthorized access. MTG Commander implements several mechanisms to hide sensitive information:

1. **Encryption**: All network communications are encrypted
2. **Access Control**: Only authorized clients can access certain game state information
3. **Data Masking**: Sensitive data is masked or obfuscated

## Rollbacks and Recovery

In networked games, maintaining state consistency despite network disruptions is essential. Our MTG Commander implementation employs a comprehensive state rollback system for resilience:

- **Complete documentation:** [State Rollback and Recovery](rollback.md)
- Deterministic replay of game actions after network disruptions
- State snapshots at critical game moments
- RNG state preservation for consistent randomized outcomes
- Client-side prediction for responsive gameplay

The rollback system integrates tightly with our deterministic RNG implementation to ensure that random events like shuffling and coin flips remain consistent across network boundaries, even during recovery from disruptions.

## Testing Game State Management

Testing the game state management system involves:

1. **Unit Tests**: Testing individual state components and transitions
2. **Integration Tests**: Testing state synchronization across multiple clients
3. **Stress Tests**: Testing state management under high load or poor network conditions

For detailed testing procedures, see the [Integration Testing Strategy](../../testing/integration/strategy.md).

## Future Enhancements

Planned improvements to game state management include:

- Enhanced state compression for better network performance
- More sophisticated state reconciliation algorithms
- Support for game state snapshots and replays
- Improved handling of complex card interactions

---

This documentation will be updated as game state management evolves.
