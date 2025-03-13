# Networking Implementation with bevy_replicon

This document outlines the architecture and implementation details for adding multiplayer networking support to our MTG Commander game engine using `bevy_replicon`.

## Table of Contents

1. [Overview](#overview)
2. [Setup and Dependencies](#setup-and-dependencies)
3. [Architecture](#architecture)
4. [Server Implementation](#server-implementation)
5. [Client Implementation](#client-implementation)
6. [Replication Strategy](#replication-strategy)
7. [Game State Synchronization](#game-state-synchronization)
8. [Networking Events](#networking-events)
9. [Security Considerations](#security-considerations)
10. [Testing and Debugging](#testing-and-debugging)

## Overview

`bevy_replicon` is a high-level networking library built on top of `bevy_renet` that provides a client-server replication system for Bevy. It handles entity replication, RPC (Remote Procedure Calls), and event synchronization between the server and connected clients.

Our implementation will focus on creating a robust, secure, and efficient networking layer that supports the complex state management required for MTG Commander games while maintaining the game rules integrity.

## Setup and Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
# Existing dependencies...
bevy_replicon = "0.15.0"
bevy_quinnet = { version = "0.6.0", optional = true }
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
```

## Architecture

Our networking architecture follows a client-server model:

1. **Server**: Maintains authoritative game state, handles game logic, and broadcasts state changes to clients
2. **Clients**: Connect to the server, send player actions, and render game state received from the server

### Key Components:

- **NetworkingPlugin**: Initializes all networking systems and components
- **ServerPlugin**: Handles server-specific logic when running as a server
- **ClientPlugin**: Handles client-specific logic when running as a client
- **ReplicationSet**: Defines which components should be replicated from server to clients
- **NetworkedActions**: Server-validated actions that clients can request

## Server Implementation

The server is the authoritative source of truth for game state and handles:

1. Game session creation and management
2. Player connections and authentication
3. Processing game actions and maintaining game rules
4. Broadcasting state updates to connected clients

```rust
// src/networking/server.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::game_engine::GameAction;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bevy_replicon::prelude::ServerPlugin::default())
            .add_systems(Startup, setup_server)
            .add_systems(Update, (
                handle_player_connections,
                process_action_requests,
                validate_game_state,
                broadcast_game_events,
            ));
    }
}

fn setup_server(mut commands: Commands) {
    // Initialize server resources
    commands.insert_resource(GameServer::new());
    
    // Add server-specific systems and resources
    // ...
}

fn handle_player_connections(
    mut commands: Commands,
    mut server: ResMut<Server>,
    mut connection_events: EventReader<ConnectionEvent>,
    mut clients: ResMut<Clients>,
) {
    // Handle new player connections and disconnections
    // ...
}

fn process_action_requests(
    mut commands: Commands,
    mut server: ResMut<Server>,
    mut action_requests: EventReader<ClientActionRequest>,
    game_state: Res<GameState>,
) {
    // Process and validate client action requests
    // ...
}
```

## Client Implementation

The client is responsible for:

1. Connecting to the server
2. Sending player inputs and action requests
3. Rendering the replicated game state
4. Providing feedback for connection status

```rust
// src/networking/client.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::game_engine::GameAction;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(bevy_replicon::prelude::ClientPlugin::default())
            .add_systems(Startup, setup_client)
            .add_systems(Update, (
                handle_connection_status,
                send_player_actions,
                apply_server_updates,
            ));
    }
}

fn setup_client(mut commands: Commands) {
    // Initialize client resources
    commands.insert_resource(GameClient::new());
    
    // Add client-specific systems and resources
    // ...
}

fn handle_connection_status(
    mut connection_events: EventReader<ConnectionEvent>,
    mut client_state: ResMut<ClientState>,
) {
    // Update UI based on connection status
    // ...
}

fn send_player_actions(
    mut client: ResMut<Client>,
    mut action_queue: ResMut<ActionQueue>,
    input: Res<Input<KeyCode>>,
) {
    // Send player actions to the server
    // ...
}
```

## Replication Strategy

We need to carefully consider which components should be replicated and how to maintain game state integrity.

### Server-to-Client Replication:

```rust
// src/networking/replication.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::card::Card;
use crate::player::Player;
use crate::game_engine::{GameState, Phase, PrioritySystem};

pub fn register_replication_components(app: &mut App) {
    app
        // Register components that should be replicated
        .replicate::<Card>()
        .replicate::<Player>()
        .replicate::<GameState>()
        .replicate::<Phase>()
        .replicate::<PrioritySystem>()
        // Register custom events for replication
        .replicate_event::<GameAction>();
}

#[derive(Component, Serialize, Deserialize, Default)]
pub struct NetworkedEntity;

#[derive(Component, Serialize, Deserialize)]
pub struct OwnerOnly {
    pub player_id: u64,
}
```

## Game State Synchronization

MTG requires careful synchronization of complex game states:

### Card Visibility and Privacy:

```rust
// Example of handling card visibility for networked games
// src/networking/visibility.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::card::Card;
use crate::game_engine::zones::{Zone, ZoneType};

// Components for visibility control
#[derive(Component)]
pub struct VisibleTo {
    pub player_ids: Vec<u64>,
}

// System to update card visibility based on game rules
fn update_card_visibility(
    mut commands: Commands,
    mut cards: Query<(Entity, &Card, &Zone, Option<&VisibleTo>)>,
    players: Query<(Entity, &Player)>,
) {
    for (entity, card, zone, visible_to) in &mut cards {
        match zone.zone_type {
            ZoneType::Hand => {
                // Only visible to owner
                let owner_id = card.owner.id();
                commands.entity(entity).insert(VisibleTo {
                    player_ids: vec![owner_id],
                });
            },
            ZoneType::Battlefield => {
                // Visible to all players
                commands.entity(entity).remove::<VisibleTo>();
            },
            // Handle other zone types...
        }
    }
}
```

## Networking Events

Define custom events for networking-related actions:

```rust
// src/networking/events.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct ClientActionRequest {
    pub player_id: u64,
    pub action_type: NetworkedActionType,
    pub targets: Vec<Entity>,
    // Additional parameters specific to the action type
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NetworkedActionType {
    PlayLand,
    CastSpell,
    ActivateAbility { ability_index: usize },
    PassPriority,
    MulliganDecision { keep: bool },
    CommanderZoneChoice { to_command_zone: bool },
    DeclareAttackers { attackers: Vec<Entity> },
    DeclareBlockers { blockers: Vec<(Entity, Entity)> },
    // Additional action types as needed
}

// Implement server-to-client events for game updates
#[derive(Event, Serialize, Deserialize, Clone, Debug)]
pub struct GameStateUpdate {
    pub active_player: Entity,
    pub phase: Phase,
    pub priority_player: Option<Entity>,
    // Additional state information
}
```

## Security Considerations

Security is crucial for card games to prevent cheating:

1. **Server Authority**: The server is the sole authority for game state. All client actions must be validated by the server.

2. **Action Validation**: Each client action must be validated against the current game state and rules.

3. **Anti-Cheat Measures**:
   - Hidden information (hand cards) should only be sent to the appropriate player
   - Random events (shuffling, coin flips) should be performed on the server
   - Rate-limiting client requests to prevent DoS attacks

4. **Reconnection Handling**: Players should be able to reconnect to games in progress.

## Testing and Debugging

For effective testing and debugging of the networking implementation:

1. **Local Testing**: Simulate a networked environment on a single machine
   
```rust
// src/networking/testing.rs
pub fn setup_local_test_environment(app: &mut App) {
    app
        .add_plugins(ServerPlugin)
        .add_plugins(ClientPlugin)
        .add_systems(Startup, spawn_test_clients);
}
```

2. **Integration Tests**: Dedicated tests for network functionality

3. **Network Condition Simulation**: Test under various network conditions (latency, packet loss)

4. **Logging and Monitoring**: Comprehensive logging of network events

```rust
fn log_network_events(
    connection_events: EventReader<ConnectionEvent>,
    client_events: EventReader<ClientActionRequest>,
    server_events: EventReader<GameStateUpdate>,
) {
    for event in connection_events.read() {
        info!("Connection event: {:?}", event);
    }
    
    // Log other events...
}
```

---

This document provides a high-level overview of implementing networking with bevy_replicon for the MTG Commander game engine. Implementation details will need to be adjusted based on specific game requirements and the evolving architecture of the game. 