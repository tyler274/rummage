# Bevy Replicon Integration

This guide explains how Rummage integrates with bevy_replicon to provide networked multiplayer functionality.

## Table of Contents

1. [Introduction to Replicon](#introduction-to-replicon)
2. [Replicon Architecture](#replicon-architecture)
3. [Replication Setup](#replication-setup)
4. [Replicated Components](#replicated-components)
5. [Server Authority](#server-authority)
6. [Client Prediction](#client-prediction)
7. [Deterministic RNG](#deterministic-rng)
8. [Network Events](#network-events)
9. [Optimizing Network Traffic](#optimizing-network-traffic)
10. [Testing Networked Gameplay](#testing-networked-gameplay)
11. [Troubleshooting](#troubleshooting)

## Introduction to Replicon

[Bevy Replicon](https://github.com/lifescapegame/bevy_replicon) is a networking library for Bevy that provides entity and component replication between server and clients. Rummage uses Replicon to enable multiplayer Magic: The Gathering games over a network.

Replicon follows a client-server model where:
- The server has authority over game state
- Clients receive updates from the server
- Client inputs are sent to the server
- The server processes inputs and updates the game state

## Replicon Architecture

The networking architecture in Rummage builds on Replicon's core features:

### Core Concepts

- **Replication**: The process of synchronizing entities and components from server to clients
- **Client Prediction**: Clients predict the outcome of their actions while waiting for server confirmation
- **Server Authority**: The server is the ultimate authority on game state
- **Rollback**: The ability to roll back and reapply actions when prediction is incorrect

### Plugin Integration

Rummage integrates Replicon through a dedicated plugin:

```rust
pub struct RummageNetworkPlugin;

impl Plugin for RummageNetworkPlugin {
    fn build(&self, app: &mut App) {
        // Add Replicon's server and client plugins based on configuration
        app.add_plugins(RepliconPlugins)
            // Add Rummage-specific network resources
            .init_resource::<NetworkConfig>()
            .init_resource::<ClientConnectionManager>()
            
            // Register replication types
            .register_type::<CardPosition>()
            .register_type::<PlayerLife>()
            .register_type::<HandContents>()
            // ... and more component types
            
            // Add network-specific systems
            .add_systems(PreUpdate, connect_to_server.run_if(resource_exists::<ClientConfig>()))
            .add_systems(Update, (
                handle_connection_events,
                process_player_actions,
                sync_game_state,
            ));
    }
}
```

## Replication Setup

Setting up replication in Rummage involves several key steps:

### Server Setup

```rust
fn setup_server(mut commands: Commands, config: Res<NetworkConfig>) {
    // Create the server
    let server = RenetServer::new(ConnectionConfig {
        protocol: ProtocolId::default(),
        server_channels_config: ServerChannelsConfig::default(),
        client_channels_config: ClientChannelsConfig::default(),
        authentication: ServerAuthentication::Unsecure,
    });
    
    // Add server components
    commands.insert_resource(server);
    commands.insert_resource(IsServer(true));
    
    // Initialize server game state
    commands.insert_resource(ServerGameState::default());
    
    info!("Server started on port {}", config.server_port);
}
```

### Client Setup

```rust
fn connect_to_client(mut commands: Commands, client_config: Res<ClientConfig>) {
    // Create the client
    let client = RenetClient::new(ConnectionConfig {
        protocol: ProtocolId::default(),
        server_channels_config: ServerChannelsConfig::default(),
        client_channels_config: ClientChannelsConfig::default(),
        authentication: ClientAuthentication::Unsecure,
    });
    
    // Add client components
    commands.insert_resource(client);
    commands.insert_resource(IsServer(false));
    
    info!("Client connecting to {}:{}", 
          client_config.server_address, 
          client_config.server_port);
}
```

## Replicated Components

Components that need network synchronization must be marked with Replicon's `Replicate` component:

```rust
// Card component that will be replicated
#[derive(Component, Serialize, Deserialize, Clone, Debug, Reflect)]
#[reflect(Component)] // Required for Replicon to reflect the component
pub struct Card {
    pub id: String,
    pub name: String,
    pub card_type: CardType,
    // Other card properties...
}

// In your setup code, mark entities for replication
fn setup_replication(mut commands: Commands) {
    // Spawn an entity with components that will be replicated
    commands.spawn((
        Card { 
            id: "some_card_id",
            name: "Card Name",
            card_type: CardType::Creature,
            // ...
        },
        // Mark this component for replication
        Replicate,
    ));
}
```

Some entities may have components that should only exist on the server:

```rust
// ServerOnly component is not replicated to clients
#[derive(Component)]
pub struct ServerOnly {
    pub secret_data: String,
}

// ServerReplication marks a type as server-only
commands.spawn((
    Card { /* ... */ },
    ServerOnly { secret_data: "hidden from clients" },
    Replicate,
    ServerReplication, // This entity is only replicated from server to clients
));
```

## Server Authority

The server maintains authority over game state through several mechanisms:

### Authoritative Systems

```rust
// This system only runs on the server
fn process_game_actions(
    mut commands: Commands,
    mut action_events: EventReader<GameActionEvent>,
    mut game_state: ResMut<GameState>,
    is_server: Res<IsServer>,
) {
    // Only run on the server
    if !is_server.0 {
        return;
    }
    
    for action in action_events.iter() {
        // Process the action authoritatively
        match action {
            GameActionEvent::PlayCard { player_id, card_id, target } => {
                // Server implementation of playing a card
                // This will automatically be replicated to clients
            },
            // Handle other actions...
        }
    }
}
```

### Action Validation

```rust
fn validate_card_play(
    action: &PlayCardAction,
    player_state: &PlayerState,
    game_state: &GameState,
) -> Result<(), ActionError> {
    // Validate the player has the card in hand
    if !player_state.hand.contains(&action.card_id) {
        return Err(ActionError::InvalidCard);
    }
    
    // Validate the player has enough mana
    let card = game_state.cards.get(&action.card_id)
        .ok_or(ActionError::CardNotFound)?;
        
    if !player_state.can_pay_mana_cost(&card.mana_cost) {
        return Err(ActionError::InsufficientMana);
    }
    
    // Additional validation logic...
    
    Ok(())
}
```

## Client Prediction

To provide a responsive feel, clients can predict the outcome of actions while waiting for server confirmation:

```rust
fn client_predict_card_play(
    mut commands: Commands,
    mut action_events: EventReader<PlayCardAction>,
    mut game_state: ResMut<ClientGameState>,
    is_server: Res<IsServer>,
) {
    // Only run on clients
    if is_server.0 {
        return;
    }
    
    for action in action_events.iter() {
        // Make a prediction about what will happen
        if let Some(card) = game_state.player.hand.remove(&action.card_id) {
            // Create a predicted entity for the card on the battlefield
            commands.spawn((
                card.clone(),
                PredictedEntity, // Mark as a prediction that might need correction
                BattlefieldCard { position: action.position },
                // ...
            ));
            
            // Update predicted game state
            game_state.prediction_applied = true;
            
            // Send the action to the server for confirmation
            // ...
        }
    }
}
```

When server confirmation is received, predictions are validated or corrected:

```rust
fn handle_server_confirmation(
    mut commands: Commands,
    mut confirmation_events: EventReader<ServerConfirmationEvent>,
    mut predicted_query: Query<(Entity, &PredictedEntity)>,
    mut game_state: ResMut<ClientGameState>,
) {
    for confirmation in confirmation_events.iter() {
        if confirmation.action_id == game_state.last_prediction_id {
            if confirmation.success {
                // Remove the prediction marker as it was correct
                for (entity, _) in predicted_query.iter() {
                    commands.entity(entity).remove::<PredictedEntity>();
                }
            } else {
                // Prediction was wrong, remove predicted entities
                for (entity, _) in predicted_query.iter() {
                    commands.entity(entity).despawn();
                }
                
                // Server will send the correct state via replication
                game_state.prediction_applied = false;
            }
        }
    }
}
```

## Deterministic RNG

For card games, deterministic random number generation is critical to ensure all clients and the server see the same outcome:

```rust
// Resource for synchronized RNG
#[derive(Resource)]
pub struct DeterministicRng {
    rng: StdRng,
    seed: u64,
    sequence: u64,
}

impl Default for DeterministicRng {
    fn default() -> Self {
        let seed = 12345; // In practice, use a seed from server
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
            sequence: 0,
        }
    }
}

impl DeterministicRng {
    // Get a random value and advance the sequence
    pub fn get_random(&mut self) -> u32 {
        self.sequence += 1;
        self.rng.next_u32()
    }
    
    // Reset to a specific sequence point
    pub fn reset_to_sequence(&mut self, sequence: u64) {
        // Reseed with original seed
        self.rng = StdRng::seed_from_u64(self.seed);
        
        // Fast-forward to the specified sequence
        for _ in 0..sequence {
            self.rng.next_u32();
        }
        
        self.sequence = sequence;
    }
}
```

Synchronizing RNG state:

```rust
// Server sends RNG state to clients
fn sync_rng_state(
    mut sync_events: EventWriter<RngSyncEvent>,
    rng: Res<DeterministicRng>,
    is_server: Res<IsServer>,
) {
    if is_server.0 && rng.is_changed() {
        sync_events.send(RngSyncEvent {
            seed: rng.seed,
            sequence: rng.sequence,
        });
    }
}

// Clients update their RNG state
fn apply_rng_sync(
    mut sync_events: EventReader<RngSyncEvent>,
    mut rng: ResMut<DeterministicRng>,
    is_server: Res<IsServer>,
) {
    if !is_server.0 {
        for event in sync_events.iter() {
            rng.seed = event.seed;
            rng.reset_to_sequence(event.sequence);
        }
    }
}
```

## Network Events

Rummage uses events to communicate between server and clients:

```rust
// Server to client events
#[derive(Event, Serialize, Deserialize)]
pub enum ServerToClientEvent {
    GameStateUpdate(GameStateSnapshot),
    PlayerJoined { player_id: u64, name: String },
    PlayerLeft { player_id: u64 },
    ChatMessage { player_id: u64, message: String },
    GameAction { action_id: u64, action: GameAction },
}

// Client to server events
#[derive(Event, Serialize, Deserialize)]
pub enum ClientToServerEvent {
    RequestAction { action_id: u64, action: GameAction },
    ChatMessage { message: String },
    Ready,
    Concede,
}
```

Handling these events:

```rust
fn handle_client_events(
    mut client_events: EventReader<ClientToServerEvent>,
    mut game_state: ResMut<GameState>,
    mut player_states: Query<&mut PlayerState>,
    is_server: Res<IsServer>,
) {
    if !is_server.0 {
        return;
    }
    
    for event in client_events.iter() {
        match event {
            ClientToServerEvent::RequestAction { action_id, action } => {
                // Process client action request
                // ...
            },
            ClientToServerEvent::ChatMessage { message } => {
                // Broadcast chat to all clients
                // ...
            },
            // Handle other events...
        }
    }
}
```

## Optimizing Network Traffic

Efficient network usage is important for a smooth multiplayer experience:

### Bandwidth Optimization

```rust
// Configure what components to replicate and how often
fn configure_replication(mut config: ResMut<ReplicationConfig>) {
    // High-priority components update every frame
    config.set_frequency::<PlayerLife>(UpdateFrequency::EveryFrame);
    
    // Medium-priority components update less frequently
    config.set_frequency::<CardPosition>(UpdateFrequency::Every(2));
    
    // Low-priority components update rarely
    config.set_frequency::<PlayerName>(UpdateFrequency::Every(60));
}
```

### Delta Compression

```rust
// Only send component changes, not full state
fn optimize_card_updates(
    mut card_query: Query<&mut Card, Changed<Card>>,
    mut replicon: ResMut<RepliconServer>,
) {
    for mut card in card_query.iter_mut() {
        // Replicon will only send the changes
        // No need to do anything special here as Replicon
        // automatically detects and sends only changed components
    }
}
```

## Testing Networked Gameplay

Testing multiplayer functionality is crucial:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_client_server_sync() {
        // Create a server app
        let mut server_app = App::new();
        server_app
            .add_plugins(MinimalPlugins)
            .add_plugin(RepliconServerPlugin)
            .add_plugin(RummageNetworkPlugin)
            .insert_resource(NetworkConfig {
                server_port: 42424,
                max_clients: 4,
            });
            
        // Create a client app
        let mut client_app = App::new();
        client_app
            .add_plugins(MinimalPlugins)
            .add_plugin(RepliconClientPlugin)
            .add_plugin(RummageNetworkPlugin)
            .insert_resource(ClientConfig {
                server_address: "127.0.0.1".to_string(),
                server_port: 42424,
            });
            
        // Simulate connection and game actions
        // ...
        
        // Verify client and server have synchronized state
        // ...
    }
}
```

## Troubleshooting

### Common Network Issues

#### Connection Failures

If clients can't connect to the server:

1. Verify the server is running and listening on the correct port
2. Check firewall settings
3. Ensure the client is using the correct server address and port
4. Check for network connectivity between the client and server

#### Desynchronization

If clients become desynchronized from the server:

1. Check for non-deterministic behavior in game logic
2. Ensure all RNG is using the deterministic RNG system
3. Verify replicated components have proper Change detection
4. Check for race conditions in event handling

#### High Latency

If game actions feel sluggish:

1. Optimize the frequency of component replication
2. Implement more client-side prediction
3. Consider delta compression for large state changes
4. Profile network traffic to identify bottlenecks

---

For complete examples of networked gameplay, see the [Multiplayer Samples](../gameplay/multiplayer_samples.md) section. 