# bevy_replicon Implementation Details

This document provides detailed implementation guidelines for integrating bevy_replicon into our MTG Commander game engine.

## Table of Contents

1. [Project Structure](#project-structure)
2. [Core Components](#core-components)
3. [Server Implementation](#server-implementation)
4. [Client Implementation](#client-implementation)
5. [Serialization Strategy](#serialization-strategy)
6. [Game-Specific Replication](#game-specific-replication)
7. [Testing Strategy](#testing-strategy)
8. [Random Number Generator Synchronization](#random-number-generator-synchronization)

## Project Structure

The networking implementation will be structured as follows:

```
src/
└── networking/
    ├── mod.rs               # Module exports and plugin registration
    ├── plugin.rs            # Main NetworkingPlugin
    ├── server/
    │   ├── mod.rs           # Server module exports
    │   ├── plugin.rs        # ServerPlugin implementation
    │   ├── systems.rs       # Server systems
    │   ├── events.rs        # Server-specific events
    │   └── resources.rs     # Server resources
    ├── client/
    │   ├── mod.rs           # Client module exports
    │   ├── plugin.rs        # ClientPlugin implementation
    │   ├── systems.rs       # Client systems
    │   ├── events.rs        # Client-specific events
    │   └── resources.rs     # Client resources
    ├── replication/
    │   ├── mod.rs           # Replication module exports
    │   ├── components.rs    # Replicable components
    │   ├── registry.rs      # Component and event registration
    │   └── visibility.rs    # Visibility control
    ├── protocol/
    │   ├── mod.rs           # Protocol module exports
    │   ├── actions.rs       # Networked actions
    │   └── messages.rs      # Custom messages
    └── testing/
        ├── mod.rs           # Testing module exports
        ├── simulation.rs    # Network simulation
        └── diagnostics.rs   # Diagnostics tools
```

## Core Components

### Networking Plugin

```rust
// src/networking/plugin.rs
use bevy::prelude::*;
use crate::networking::server::ServerPlugin;
use crate::networking::client::ClientPlugin;
use crate::networking::replication::ReplicationPlugin;

/// Configuration for the networking plugin
#[derive(Resource)]
pub struct NetworkingConfig {
    /// Whether this instance is running as a server
    pub is_server: bool,
    /// Whether this instance is running as a client
    pub is_client: bool,
    /// Server address to connect to (client only)
    pub server_address: Option<String>,
    /// Server port to host on (server) or connect to (client)
    pub port: u16,
    /// Maximum number of clients that can connect (server only)
    pub max_clients: usize,
}

impl Default for NetworkingConfig {
    fn default() -> Self {
        Self {
            is_server: false,
            is_client: true,
            server_address: None,
            port: 5000,
            max_clients: 4,
        }
    }
}

/// Main plugin for networking functionality
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        // Add core networking resources
        app.init_resource::<NetworkingConfig>();
        
        // Add replication plugin
        app.add_plugins(ReplicationPlugin);
        
        // Add server or client plugins based on configuration
        let config = app.world.resource::<NetworkingConfig>();
        
        if config.is_server {
            app.add_plugins(ServerPlugin);
        }
        
        if config.is_client {
            app.add_plugins(ClientPlugin);
        }
    }
}
```

## Server Implementation

### Server Resource

```rust
// src/networking/server/resources.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use std::collections::HashMap;

/// Resource for managing the server state
#[derive(Resource)]
pub struct GameServer {
    /// Maps client IDs to player entities
    pub client_player_map: HashMap<ClientId, Entity>,
    /// Maps player entities to client IDs
    pub player_client_map: HashMap<Entity, ClientId>,
    /// Current game session ID
    pub session_id: String,
    /// Whether the server is accepting new connections
    pub accepting_connections: bool,
    /// Server status
    pub status: ServerStatus,
}

/// Server status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    /// Server is starting up
    Starting,
    /// Server is waiting for players to connect
    WaitingForPlayers,
    /// Game is in progress
    GameInProgress,
    /// Game has ended
    GameEnded,
}

impl Default for GameServer {
    fn default() -> Self {
        Self {
            client_player_map: HashMap::new(),
            player_client_map: HashMap::new(),
            session_id: uuid::Uuid::new_v4().to_string(),
            accepting_connections: true,
            status: ServerStatus::Starting,
        }
    }
}
```

### Server Systems

```rust
// src/networking/server/systems.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::networking::server::resources::*;
use crate::networking::protocol::actions::*;
use crate::game_engine::GameAction;

/// Set up server resources
pub fn setup_server(mut commands: Commands) {
    commands.insert_resource(GameServer::default());
}

/// Handle player connections and disconnections
pub fn handle_player_connections(
    mut commands: Commands,
    mut server: ResMut<GameServer>,
    mut server_events: EventReader<ServerEvent>,
    mut connected_clients: ResMut<ConnectedClients>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client connected: {:?}", client_id);
                
                // Create player entity for the client
                if server.accepting_connections {
                    let player_entity = commands.spawn_empty().id();
                    
                    // Map client to player
                    server.client_player_map.insert(*client_id, player_entity);
                    server.player_client_map.insert(player_entity, *client_id);
                    
                    // Start replication for this client
                    commands.add(StartReplication {
                        client_id: *client_id,
                    });
                    
                    // Add player to replicated clients list
                    commands.entity(player_entity).insert(ReplicatedClient {
                        client_id: *client_id,
                    });
                }
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client disconnected: {:?}, reason: {:?}", client_id, reason);
                
                // Remove player entity and mappings
                if let Some(player_entity) = server.client_player_map.remove(client_id) {
                    server.player_client_map.remove(&player_entity);
                    
                    // Handle player disconnection in game logic
                    // (e.g., mark player as AFK, save their state for reconnection, etc.)
                }
            }
        }
    }
}

/// Process action requests from clients
pub fn process_action_requests(
    mut commands: Commands,
    mut action_requests: EventReader<ClientActionRequest>,
    server: Res<GameServer>,
    game_state: Res<crate::game_engine::state::GameState>,
    mut game_actions: EventWriter<GameAction>,
) {
    for request in action_requests.read() {
        // Validate client ID to player mapping
        if let Some(player_entity) = server.client_player_map.get(&request.client_id) {
            match &request.action {
                NetworkedAction::PlayLand { card_id } => {
                    // Validate action against game rules
                    if game_state.can_play_land(*player_entity) {
                        // Convert to game action
                        game_actions.send(GameAction::PlayLand {
                            player: *player_entity,
                            land_card: *card_id,
                        });
                    }
                }
                NetworkedAction::CastSpell { card_id, targets, mana_payment } => {
                    // Validate spell casting
                    if game_state.can_cast_spell(*player_entity, *card_id) {
                        game_actions.send(GameAction::CastSpell {
                            player: *player_entity,
                            spell_card: *card_id,
                            targets: targets.clone(),
                            mana_payment: mana_payment.clone(),
                        });
                    }
                }
                // Handle other action types...
                _ => {}
            }
        }
    }
}
```

## Client Implementation

### Client Resources

```rust
// src/networking/client/resources.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;

/// Resource for managing the client state
#[derive(Resource)]
pub struct GameClient {
    /// The client's local player entity
    pub local_player: Option<Entity>,
    /// Local player ID
    pub local_player_id: Option<u64>,
    /// Connection status
    pub connection_status: ConnectionStatus,
    /// Action queue for batching actions
    pub action_queue: Vec<NetworkedAction>,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Not connected to a server
    Disconnected,
    /// Attempting to connect to a server
    Connecting,
    /// Connected and authenticated
    Connected,
    /// Connection error occurred
    Error(ConnectionError),
}

/// Connection error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionError {
    /// Failed to connect to the server
    ConnectionFailed,
    /// Connected but authentication failed
    AuthenticationFailed,
    /// Disconnected unexpectedly
    Disconnected,
    /// Timeout waiting for server response
    Timeout,
}

impl Default for GameClient {
    fn default() -> Self {
        Self {
            local_player: None,
            local_player_id: None,
            connection_status: ConnectionStatus::Disconnected,
            action_queue: Vec::new(),
        }
    }
}
```

### Client Systems

```rust
// src/networking/client/systems.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::networking::client::resources::*;
use crate::networking::protocol::actions::*;
use crate::player::Player;

/// Set up client resources
pub fn setup_client(mut commands: Commands) {
    commands.insert_resource(GameClient::default());
}

/// Handle connection status changes
pub fn handle_connection_status(
    mut client: ResMut<GameClient>,
    mut client_status: ResMut<RepliconClientStatus>,
) {
    match *client_status {
        RepliconClientStatus::Connecting => {
            client.connection_status = ConnectionStatus::Connecting;
        }
        RepliconClientStatus::Connected => {
            client.connection_status = ConnectionStatus::Connected;
        }
        RepliconClientStatus::Disconnected => {
            client.connection_status = ConnectionStatus::Disconnected;
        }
    }
}

/// Update local player reference
pub fn update_local_player(
    mut client: ResMut<GameClient>,
    players: Query<(Entity, &Player, &ReplicatedClient)>,
    replicon_client: Res<RepliconClient>,
) {
    // Find the player entity that belongs to this client
    if client.local_player.is_none() {
        for (entity, _player, replicated_client) in &players {
            if replicated_client.client_id == replicon_client.id() {
                client.local_player = Some(entity);
                client.local_player_id = Some(replicon_client.id().0);
                break;
            }
        }
    }
}

/// Send actions to the server
pub fn send_player_actions(
    mut client: ResMut<GameClient>,
    mut action_requests: EventWriter<ClientActionRequest>,
    replicon_client: Res<RepliconClient>,
) {
    // Process queued actions
    for action in client.action_queue.drain(..) {
        let request = ClientActionRequest {
            client_id: replicon_client.id(),
            action: action,
        };
        
        action_requests.send(request);
    }
}

/// Handle player input and queue actions
pub fn handle_player_input(
    mut client: ResMut<GameClient>,
    input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    card_interaction: Res<CardInteraction>,
) {
    // Example: Queue an action based on player input
    if mouse_input.just_pressed(MouseButton::Left) && card_interaction.selected_card.is_some() {
        if input.pressed(KeyCode::ShiftLeft) {
            // Queue a cast spell action
            client.action_queue.push(NetworkedAction::CastSpell {
                card_id: card_interaction.selected_card.unwrap(),
                targets: card_interaction.targets.clone(),
                mana_payment: card_interaction.proposed_payment.clone(),
            });
        } else {
            // Queue a different action based on the card and context
            // ...
        }
    }
}
```

## Serialization Strategy

For efficient serialization, we need to implement Serialize/Deserialize traits for all networked components:

```rust
// src/networking/replication/components.rs
use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::game_engine::phase::Phase;
use crate::player::Player;
use crate::mana::Mana;

// Make sure all replicated components implement Serialize and Deserialize
// For complex types, consider custom serialization for efficiency

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct NetworkedPlayer {
    pub name: String,
    pub life: i32,
    pub mana_pool: Mana,
    // Include only data that needs to be networked
    // Omit any large or game-state-derived data
}

impl From<&Player> for NetworkedPlayer {
    fn from(player: &Player) -> Self {
        Self {
            name: player.name.clone(),
            life: player.life,
            mana_pool: player.mana_pool.clone(),
        }
    }
}

// Custom serialization for Entity references to handle cross-client referencing
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NetworkedEntity {
    pub id: u64, // Stable ID that can be used across the network
}

// System to maintain Entity <-> NetworkedEntity mappings
pub fn update_entity_mappings(
    mut commands: Commands,
    new_entities: Query<Entity, Added<Replicated>>,
    mut entity_map: ResMut<EntityMap>,
) {
    for entity in &new_entities {
        // Generate stable network ID
        let network_id = entity_map.next_id();
        
        // Save mapping
        entity_map.insert(entity, network_id);
        
        // Add networked ID component
        commands.entity(entity).insert(NetworkedId(network_id));
    }
}
```

## Game-Specific Replication

### MTG Card Replication

Special care needs to be taken for replicating MTG cards, as they have hidden information:

```rust
// src/networking/replication/visibility.rs
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::card::Card;
use crate::game_engine::zones::{Zone, ZoneType};

// System to update client visibility for cards
pub fn update_card_visibility(
    mut commands: Commands,
    cards: Query<(Entity, &Card, &Zone)>,
    players: Query<(Entity, &ReplicatedClient)>,
    server: Res<RepliconServer>,
    replication_rules: Res<ReplicationRules>,
) {
    for (card_entity, card, zone) in &cards {
        match zone.zone_type {
            ZoneType::Hand => {
                // Only the owner can see cards in hand
                let owner_client_id = get_client_id_for_player(card.owner, &players);
                
                if let Some(owner_id) = owner_client_id {
                    // Use ClientVisibility to control which clients can see this entity
                    commands.entity(card_entity).insert(ClientVisibility {
                        policy: VisibilityPolicy::Blacklist,
                        client_ids: players
                            .iter()
                            .filter_map(|(_, client)| {
                                if client.client_id != owner_id {
                                    Some(client.client_id)
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    });
                }
            },
            ZoneType::Library => {
                // No player can see library cards (except the top in some cases)
                commands.entity(card_entity).insert(ClientVisibility {
                    policy: VisibilityPolicy::Blacklist,
                    client_ids: players
                        .iter()
                        .map(|(_, client)| client.client_id)
                        .collect(),
                });
            },
            ZoneType::Battlefield => {
                // All players can see battlefield cards
                commands.entity(card_entity).remove::<ClientVisibility>();
            },
            ZoneType::Graveyard | ZoneType::Exile | ZoneType::Command => {
                // All players can see these zones
                commands.entity(card_entity).remove::<ClientVisibility>();
            },
            // Handle other zones...
        }
    }
}

// Helper function to get client ID for a player entity
fn get_client_id_for_player(
    player_entity: Entity,
    players: &Query<(Entity, &ReplicatedClient)>,
) -> Option<ClientId> {
    players
        .iter()
        .find_map(|(entity, client)| {
            if entity == player_entity {
                Some(client.client_id)
            } else {
                None
            }
        })
}
```

## Testing Strategy

### Network Simulation

```rust
// src/networking/testing/simulation.rs
use bevy::prelude::*;
use crate::networking::plugin::NetworkingPlugin;
use crate::networking::server::ServerPlugin;
use crate::networking::client::ClientPlugin;

/// Plugin for testing the networking in a local environment
pub struct NetworkTestPlugin;

impl Plugin for NetworkTestPlugin {
    fn build(&self, app: &mut App) {
        // Create a local server and client
        app.insert_resource(NetworkingConfig {
            is_server: true, // This instance acts as both server and client
            is_client: true,
            server_address: Some("127.0.0.1".to_string()),
            port: 5000,
            max_clients: 4,
        });
        
        app.add_plugins(NetworkingPlugin);
        
        // Add systems for simulating network conditions
        app.add_systems(Update, simulate_network_conditions);
    }
}

/// System to simulate various network conditions for testing
pub fn simulate_network_conditions(
    mut server: ResMut<RepliconServer>,
    mut client: ResMut<RepliconClient>,
    network_simulation: Res<NetworkSimulation>,
) {
    // Simulate latency
    if let Some(latency) = network_simulation.latency {
        // Delay processing of messages
        std::thread::sleep(std::time::Duration::from_millis(latency));
    }
    
    // Simulate packet loss
    if let Some(packet_loss) = network_simulation.packet_loss {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < packet_loss {
            // Simulate packet loss by not processing some messages
            // This would require modifications to the underlying transport layer
        }
    }
}

/// Resource for configuring network simulation
#[derive(Resource)]
pub struct NetworkSimulation {
    /// Simulated latency in milliseconds
    pub latency: Option<u64>,
    /// Packet loss rate (0.0 to 1.0)
    pub packet_loss: Option<f32>,
    /// Jitter in milliseconds
    pub jitter: Option<u64>,
}

impl Default for NetworkSimulation {
    fn default() -> Self {
        Self {
            latency: None,
            packet_loss: None,
            jitter: None,
        }
    }
}
```

## Integration with Game Loop

```rust
// src/networking/integration.rs
use bevy::prelude::*;
use crate::networking::plugin::NetworkingPlugin;
use crate::game_engine::state::GameState;

/// System to initialize networking based on game mode
pub fn initialize_networking(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut app_config: ResMut<NetworkingConfig>,
) {
    match game_state.mode {
        GameMode::SinglePlayer => {
            // No networking needed
            app_config.is_server = false;
            app_config.is_client = false;
        },
        GameMode::HostMultiplayer => {
            // Host acts as both server and client
            app_config.is_server = true;
            app_config.is_client = true;
            app_config.server_address = Some("0.0.0.0".to_string()); // Bind to all interfaces
        },
        GameMode::JoinMultiplayer { server_address } => {
            // Client-only mode
            app_config.is_server = false;
            app_config.is_client = true;
            app_config.server_address = Some(server_address.clone());
        }
    }
}
```

## Random Number Generator Synchronization

### Overview

Deterministic random number generation is critical for multiplayer games to ensure that all clients produce identical results when processing the same game actions. This section outlines how to use `bevy_rand` and `bevy_prng` to maintain synchronized RNG state across network boundaries.

```rust
// src/networking/rng/plugin.rs
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use crate::networking::server::resources::GameServer;

/// Plugin for handling RNG synchronization in networked games
pub struct NetworkedRngPlugin;

impl Plugin for NetworkedRngPlugin {
    fn build(&self, app: &mut App) {
        // Register the WyRand PRNG with bevy_rand
        app.add_plugins(EntropyPlugin::<WyRand>::default())
            .add_systems(Update, synchronize_rng_state)
            .add_systems(PostUpdate, handle_rng_state_replication);
    }
}

/// Resource to track the RNG state for replication
#[derive(Resource)]
pub struct RngStateTracker {
    /// The current state of the global RNG
    pub global_state: Vec<u8>,
    /// Last synchronization timestamp
    pub last_sync: f32,
    /// Whether the RNG state has changed since last sync
    pub dirty: bool,
}

impl Default for RngStateTracker {
    fn default() -> Self {
        Self {
            global_state: Vec::new(),
            last_sync: 0.0,
            dirty: false,
        }
    }
}

/// System to capture RNG state for replication
pub fn synchronize_rng_state(
    mut rng: GlobalEntropy<WyRand>,
    mut state_tracker: ResMut<RngStateTracker>,
    time: Res<Time>,
) {
    // Only sync periodically to reduce network traffic
    if time.elapsed_seconds() - state_tracker.last_sync > 5.0 {
        // Serialize the RNG state
        if let Some(serialized_state) = rng.try_serialize_state() {
            state_tracker.global_state = serialized_state;
            state_tracker.last_sync = time.elapsed_seconds();
            state_tracker.dirty = true;
        }
    }
}

/// System to handle replication of RNG state to clients
pub fn handle_rng_state_replication(
    server: Option<Res<GameServer>>,
    rng_state: Res<RngStateTracker>,
    mut client: ResMut<RepliconClient>,
    mut server_res: ResMut<RepliconServer>,
) {
    // Only the server should send RNG state updates
    if let Some(server) = server {
        if rng_state.dirty {
            // Send RNG state to all clients
            for client_id in server.client_player_map.keys() {
                server_res.send_message(
                    *client_id,
                    ServerChannel::Reliable,
                    bincode::serialize(&RngStateMessage {
                        state: rng_state.global_state.clone(),
                        timestamp: rng_state.last_sync,
                    }).unwrap(),
                );
            }
        }
    }
}

/// Message for RNG state synchronization
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RngStateMessage {
    /// Serialized RNG state
    pub state: Vec<u8>,
    /// Timestamp of the state
    pub timestamp: f32,
}
```

### Player-Specific RNG Components

Each player should have their own RNG component that is deterministically seeded from the global source:

```rust
// src/player/components/player_rng.rs
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;

/// Component for player-specific randomization
#[derive(Component)]
pub struct PlayerRng {
    /// The player's RNG component
    pub rng: Entropy<WyRand>,
    /// Whether this RNG is remotely controlled
    pub is_remote: bool,
}

/// System to initialize player RNGs
pub fn setup_player_rngs(
    mut commands: Commands,
    players: Query<(Entity, &Player), Without<PlayerRng>>,
    mut global_rng: GlobalEntropy<WyRand>,
    server: Option<Res<GameServer>>,
) {
    for (entity, player) in players.iter() {
        // On the server, create a new RNG for each player
        let is_remote = server.is_none() || !server.unwrap().is_server_player(entity);
        
        // Fork from the global RNG to maintain determinism
        commands.entity(entity).insert(PlayerRng {
            rng: global_rng.fork_rng(),
            is_remote,
        });
    }
}
```

### Client-Side RNG Management

Clients need to apply RNG state updates from the server:

```rust
// src/networking/client/systems.rs
use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::prelude::*;
use crate::networking::rng::RngStateMessage;

/// System to handle RNG state updates from server
pub fn handle_rng_state_update(
    mut client_rng: GlobalEntropy<WyRand>,
    mut incoming_messages: EventReader<NetworkMessage>,
) {
    for message in incoming_messages.read() {
        if let Ok(rng_message) = bincode::deserialize::<RngStateMessage>(&message.data) {
            // Apply the server's RNG state
            client_rng.deserialize_state(&rng_message.state).expect("Failed to deserialize RNG state");
            
            // Now client and server have synchronized RNG state
        }
    }
}
```

### Deterministic Usage in Game Actions

To ensure deterministic behavior, game actions must use RNG components in a consistent way:

```rust
// src/game_engine/actions/dice_roll.rs
use bevy::prelude::*;
use crate::player::components::PlayerRng;
use rand::Rng;

/// System to handle dice roll actions
pub fn handle_dice_roll(
    mut commands: Commands,
    mut dice_roll_events: EventReader<DiceRollEvent>,
    mut players: Query<(&mut PlayerRng, &Player)>,
) {
    for event in dice_roll_events.read() {
        if let Ok((mut player_rng, player)) = players.get_mut(event.player_entity) {
            // Use the player's RNG to get a deterministic result
            let roll_result = player_rng.rng.gen_range(1..=event.sides);
            
            // Create the effect based on the roll result
            let effect_entity = commands.spawn(DiceRollEffect {
                player: event.player_entity,
                result: roll_result,
                sides: event.sides,
            }).id();
            
            // The effect is determined by the RNG, ensuring all clients get the same result
            // as long as they have the same RNG state and process events in the same order
        }
    }
}
```

### Ensuring Consistency

To maintain RNG consistency across clients:

1. The server is the authoritative source of RNG state
2. All random operations use player-specific RNGs or the global RNG, never `thread_rng()` or other non-deterministic sources
3. RNG state is synchronized periodically
4. Game actions that use randomness include a sequence ID to ensure they're processed in the same order on all clients
5. When a new player joins, they receive the current global RNG state as part of initialization

### Advanced: Network Partitioning

For scenarios where different subsets of players may need different random sequences (like shuffling a deck that only certain players should know the order of):

```rust
// src/game_engine/zones/library.rs
use bevy::prelude::*;
use crate::player::components::PlayerRng;

/// System to shuffle a player's library
pub fn shuffle_library(
    mut libraries: Query<(&mut Library, &Owner)>,
    mut players: Query<&mut PlayerRng>,
    shuffle_events: EventReader<ShuffleLibraryEvent>,
) {
    for event in shuffle_events.read() {
        if let Ok((mut library, owner)) = libraries.get_mut(event.library_entity) {
            // Get the owner's RNG
            if let Ok(mut player_rng) = players.get_mut(owner.entity) {
                // Use the player's RNG to shuffle the library deterministically
                library.shuffle_with_rng(&mut player_rng.rng);
                
                // This ensures that all clients who have access to this player's 
                // library will see the same shuffle result
            }
        }
    }
}
```

By following these patterns, your MTG Commander game will maintain consistent random results across all networked clients, ensuring fair gameplay regardless of network conditions.