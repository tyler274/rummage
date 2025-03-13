# Multiplayer Lobby Backend Implementation

This document focuses on the server-side implementation details of the lobby system for Rummage's multiplayer Commander games. It covers the networking architecture, server infrastructure, and data flow between clients and servers.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Lobby Server Implementation](#lobby-server-implementation)
3. [Game Server Implementation](#game-server-implementation)
4. [Connection and Protocol](#connection-and-protocol)
5. [Data Persistence](#data-persistence)
6. [Security Considerations](#security-considerations)
7. [Testing and Validation](#testing-and-validation)
8. [Deployment Considerations](#deployment-considerations)

## Architecture Overview

The multiplayer system uses a dual-server architecture:

1. **Lobby Server**: Manages lobbies, matchmaking, and initial connections
2. **Game Server**: Handles the actual Commander gameplay after a match starts

```
                         ┌────────────────┐
                         │  Lobby Server  │
                         └───────┬────────┘
                                 │
                 ┌───────────────┼───────────────┐
                 │               │               │
         ┌───────┴─────┐ ┌───────┴─────┐ ┌───────┴─────┐
         │ Game Server │ │ Game Server │ │ Game Server │
         └─────────────┘ └─────────────┘ └─────────────┘
```

### Components

1. **Lobby Manager**: Tracks active lobbies and their states
2. **Session Manager**: Handles player authentication and persistence
3. **Game Instance Factory**: Creates and configures new game instances
4. **Message Broker**: Routes communications between components
5. **Persistence Layer**: Stores lobby and game data

## Lobby Server Implementation

The Lobby Server is responsible for:

1. Managing the list of available lobbies
2. Handling player authentication
3. Processing lobby creation, updates, and deletions
4. Facilitating player chat and interactions in lobbies
5. Initiating game sessions when a match starts

```rust
/// Main lobby server resource
#[derive(Resource)]
pub struct LobbyServer {
    /// Active lobbies indexed by ID
    pub lobbies: HashMap<String, LobbyData>,
    /// Connected clients
    pub clients: HashMap<ClientId, LobbyClientInfo>,
    /// Available game servers
    pub game_servers: Vec<GameServerInfo>,
}

/// Data structure for tracking a lobby
#[derive(Clone, Debug)]
pub struct LobbyData {
    /// Lobby information visible to players
    pub info: LobbyInfo,
    /// Detailed lobby settings
    pub settings: LobbySettings,
    /// Players in the lobby
    pub players: HashMap<String, LobbyPlayer>,
    /// Chat history
    pub chat_history: Vec<ChatMessage>,
    /// Creation timestamp
    pub created_at: f64,
    /// Last activity timestamp
    pub last_activity: f64,
}

/// Systems to handle lobby server operations
pub fn handle_lobby_connections(
    mut server: ResMut<RepliconServer>,
    mut lobby_server: ResMut<LobbyServer>,
    mut connection_events: EventReader<ServerEvent>,
    time: Res<Time>,
) {
    for event in connection_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                // Add client to connected clients
                lobby_server.clients.insert(*client_id, LobbyClientInfo {
                    client_id: *client_id,
                    state: LobbyClientState::Connected,
                    username: None,
                    lobby_id: None,
                    connected_at: time.elapsed_seconds(),
                    last_activity: time.elapsed_seconds(),
                });
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                // Handle client disconnection
                if let Some(client_info) = lobby_server.clients.remove(client_id) {
                    // If client was in a lobby, remove them
                    if let Some(lobby_id) = client_info.lobby_id {
                        if let Some(lobby) = lobby_server.lobbies.get_mut(&lobby_id) {
                            // Remove player from lobby
                            if let Some(username) = &client_info.username {
                                lobby.players.remove(username);
                                
                                // Add system message about player leaving
                                lobby.chat_history.push(ChatMessage {
                                    id: generate_uuid(),
                                    sender: "System".to_string(),
                                    is_system: true,
                                    content: format!("{} has left the lobby", username),
                                    timestamp: time.elapsed_seconds(),
                                });
                                
                                // Notify other players in the lobby
                                notify_lobby_update(&mut server, &lobby_id, &lobby_server);
                                
                                // If lobby is empty or host left, handle lobby cleanup
                                handle_potential_lobby_cleanup(&mut lobby_server, &lobby_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Process incoming lobby actions
pub fn process_lobby_requests(
    mut server: ResMut<RepliconServer>,
    mut lobby_server: ResMut<LobbyServer>,
    mut lobby_requests: EventReader<FromClient<LobbyRequest>>,
    time: Res<Time>,
) {
    for FromClient { client_id, event } in lobby_requests.read() {
        match event {
            LobbyRequest::ListLobbies(request) => {
                // Handle lobby list request
                let filtered_lobbies = filter_lobbies(&lobby_server.lobbies, &request.filters);
                let response = ServerListResponse {
                    lobbies: filtered_lobbies,
                    total_lobbies: lobby_server.lobbies.len(),
                };
                
                // Send response to client
                server.send_message(*client_id, response);
            }
            LobbyRequest::CreateLobby(request) => {
                // Handle lobby creation request
                let lobby_id = generate_uuid();
                let client_info = lobby_server.clients.get_mut(client_id).unwrap();
                
                // Create new lobby
                let lobby = LobbyData {
                    info: LobbyInfo {
                        id: lobby_id.clone(),
                        name: request.name.clone(),
                        host_name: client_info.username.clone().unwrap_or_default(),
                        player_count: 1,
                        max_players: request.max_players,
                        has_password: request.password.is_some(),
                        format: request.format.clone(),
                        restrictions: request.restrictions.clone(),
                        description: request.description.clone(),
                    },
                    settings: request.settings.clone(),
                    players: HashMap::new(),
                    chat_history: Vec::new(),
                    created_at: time.elapsed_seconds(),
                    last_activity: time.elapsed_seconds(),
                };
                
                // Add host to the lobby
                let player = LobbyPlayer {
                    name: client_info.username.clone().unwrap_or_default(),
                    is_host: true,
                    status: PlayerLobbyState::Joined,
                    deck_info: None,
                };
                
                let mut players = HashMap::new();
                players.insert(player.name.clone(), player);
                
                lobby.players = players;
                
                // Add lobby to server
                lobby_server.lobbies.insert(lobby_id.clone(), lobby);
                
                // Update client info
                client_info.lobby_id = Some(lobby_id.clone());
                client_info.state = LobbyClientState::InLobby;
                
                // Send response to client
                server.send_message(*client_id, CreateLobbyResponse {
                    success: true,
                    lobby_id: Some(lobby_id),
                    error: None,
                });
            }
            LobbyRequest::JoinLobby(request) => {
                // Handle join lobby request
                if let Some(lobby) = lobby_server.lobbies.get_mut(&request.lobby_id) {
                    // Check if lobby is joinable
                    if lobby.players.len() >= lobby.info.max_players {
                        // Lobby is full
                        server.send_message(*client_id, JoinLobbyResponse {
                            success: false,
                            failure_reason: Some("Lobby is full".to_string()),
                            lobby_details: None,
                        });
                        continue;
                    }
                    
                    // Check password if required
                    if lobby.info.has_password {
                        // Verify password
                    }
                    
                    // Add player to lobby
                    let client_info = lobby_server.clients.get_mut(client_id).unwrap();
                    let player_name = client_info.username.clone().unwrap_or_default();
                    
                    let player = LobbyPlayer {
                        name: player_name.clone(),
                        is_host: false,
                        status: PlayerLobbyState::Joined,
                        deck_info: None,
                    };
                    
                    lobby.players.insert(player_name, player);
                    lobby.info.player_count = lobby.players.len();
                    lobby.last_activity = time.elapsed_seconds();
                    
                    // Update client info
                    client_info.lobby_id = Some(request.lobby_id.clone());
                    client_info.state = LobbyClientState::InLobby;
                    
                    // Add system message about player joining
                    lobby.chat_history.push(ChatMessage {
                        id: generate_uuid(),
                        sender: "System".to_string(),
                        is_system: true,
                        content: format!("{} has joined the lobby", player_name),
                        timestamp: time.elapsed_seconds(),
                    });
                    
                    // Notify all players in the lobby
                    notify_lobby_update(&mut server, &request.lobby_id, &lobby_server);
                    
                    // Send response to joining client
                    server.send_message(*client_id, JoinLobbyResponse {
                        success: true,
                        failure_reason: None,
                        lobby_details: Some(convert_to_lobby_details(lobby)),
                    });
                } else {
                    // Lobby not found
                    server.send_message(*client_id, JoinLobbyResponse {
                        success: false,
                        failure_reason: Some("Lobby not found".to_string()),
                        lobby_details: None,
                    });
                }
            }
            LobbyRequest::SendChat(request) => {
                // Handle chat message
                process_chat_message(client_id, request, &mut server, &mut lobby_server, &time);
            }
            LobbyRequest::UpdateStatus(request) => {
                // Handle player status update
                process_status_update(client_id, request, &mut server, &mut lobby_server, &time);
            }
            LobbyRequest::ViewDeck(request) => {
                // Handle deck view request
                process_deck_view_request(client_id, request, &mut server, &mut lobby_server);
            }
            LobbyRequest::LaunchGame(request) => {
                // Handle game launch request
                process_game_launch(client_id, request, &mut server, &mut lobby_server);
            }
        }
    }
}
```

## Game Server Implementation

The Game Server handles the actual Commander gameplay after a match is initiated:

```rust
/// Main game server resource
#[derive(Resource)]
pub struct GameServer {
    /// Current game state
    pub game_state: Option<GameState>,
    /// Connected players
    pub players: HashMap<ClientId, Entity>,
    /// Whether this server is accepting new connections
    pub accepting_connections: bool,
    /// Game configuration
    pub config: GameConfig,
}

/// Start a new game instance
pub fn start_game_instance(
    mut commands: Commands,
    mut server: ResMut<RepliconServer>,
    game_launch: Res<GameLaunchInfo>,
) {
    // Initialize game state
    let game_state = initialize_game_state(&game_launch);
    commands.insert_resource(game_state.clone());
    
    // Configure server to accept connections from players
    commands.insert_resource(GameServer {
        game_state: Some(game_state),
        players: HashMap::new(),
        accepting_connections: true,
        config: game_launch.settings.clone(),
    });
    
    // Start server on specified port
    server.start_listening(game_launch.port);
    
    // Set up turn tracking
    commands.insert_resource(TurnManager::new());
    
    // Set up zones for cards
    commands.insert_resource(ZoneManager::new());
    
    // Notify lobby server that this game instance is ready
    notify_lobby_server_game_ready(game_launch.lobby_id.clone(), game_launch.port);
}

/// Transfer players from lobby to game
pub fn transfer_players_to_game(
    mut server: ResMut<RepliconServer>,
    game_server: Res<GameServer>,
    lobby_connection: Res<LobbyServerConnection>,
    game_launch: Res<GameLaunchInfo>,
) {
    // Notify all players in the lobby that the game is ready
    let connection_details = GameConnectionDetails {
        server_address: get_server_address(),
        game_id: game_launch.game_id.clone(),
        connection_token: generate_connection_tokens(&game_launch.players),
    };
    
    let notification = GameLaunchNotification {
        connection_details,
        players: game_launch.players.clone(),
        settings: game_launch.settings.clone(),
    };
    
    // Send notification to lobby server to distribute to players
    lobby_connection.send_message(LobbyServerMessage::GameReady(notification));
}
```

## Connection and Protocol

The networking protocol uses WebRTC for UDP-based communication with reliability layers:

```rust
/// Connection protocol constants
pub mod protocol {
    /// Protocol version
    pub const PROTOCOL_VERSION: &str = "1.0.0";
    /// Maximum message size in bytes
    pub const MAX_MESSAGE_SIZE: usize = 1024 * 64;
    /// Heartbeat interval in seconds
    pub const HEARTBEAT_INTERVAL: f32 = 1.0;
    /// Connection timeout in seconds
    pub const CONNECTION_TIMEOUT: f32 = 5.0;
}

/// Initialize the networking protocol
pub fn init_networking(app: &mut App) {
    app
        .add_plugins(RepliconPlugin)
        .add_systems(Startup, setup_network_config)
        .add_systems(PreUpdate, (
            process_connection_events,
            handle_protocol_messages,
        ))
        .add_systems(Update, (
            send_heartbeats,
            check_timeouts,
        ));
}

/// Set up network configuration
fn setup_network_config(mut commands: Commands) {
    commands.insert_resource(RepliconConfig {
        max_message_size: protocol::MAX_MESSAGE_SIZE,
        max_message_channel_count: 3,
        ..default()
    });
}

/// Channel types for different message priorities
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum NetworkChannel {
    /// Reliable ordered channel for important messages
    Reliable,
    /// Unreliable channel for position updates
    Unreliable,
    /// Ordered but can drop messages for less critical state
    ReliableUnordered,
}
```

### Network Optimization

To handle the complexity of Commander games, we implement several optimization techniques:

1. **Delta Compression**: Only send changes to game state
2. **Interest Management**: Only sync relevant parts of the game state to each client
3. **Batched Updates**: Collect multiple updates and send them together
4. **Prioritized Synchronization**: Critical game events take priority over visual updates

```rust
/// Delta encoder for game state changes
pub struct DeltaEncoder {
    /// Previous game state hash
    previous_hash: u64,
    /// Component change tracking
    component_changes: HashMap<Entity, Vec<ComponentChange>>,
}

impl DeltaEncoder {
    /// Encode delta changes between states
    pub fn encode_delta(&mut self, current_state: &GameState) -> DeltaPacket {
        // Calculate changes since previous state
        // ...
        
        DeltaPacket {
            base_hash: self.previous_hash,
            entities_added: vec![],
            entities_removed: vec![],
            component_changes: self.component_changes.clone(),
        }
    }
}
```

## Data Persistence

The lobby server persists certain data to provide continuity:

1. **User Accounts**: Player profiles and authentication
2. **Lobby Templates**: Saved lobby configurations
3. **Match History**: Record of played games
4. **Deck Statistics**: Anonymized deck performance data

```rust
/// Data persistence manager
pub struct PersistenceManager {
    /// Database connection
    db_connection: DbConnection,
    /// Cache for frequently accessed data
    cache: LruCache<String, CachedData>,
}

impl PersistenceManager {
    /// Save lobby to database
    pub async fn save_lobby(&self, lobby: &LobbyData) -> Result<(), DbError> {
        // Serialize and store lobby data
        // ...
        Ok(())
    }
    
    /// Load lobby from database
    pub async fn load_lobby(&self, lobby_id: &str) -> Result<Option<LobbyData>, DbError> {
        // Retrieve and deserialize lobby data
        // ...
        Ok(None)
    }
    
    /// Record match results
    pub async fn record_match_result(&self, results: &MatchResults) -> Result<(), DbError> {
        // Store match results for history and statistics
        // ...
        Ok(())
    }
}
```

## Security Considerations

The multiplayer system implements several security measures:

1. **Authentication**: Verify player identities
2. **Authorization**: Control access to lobbies and games
3. **Input Validation**: Sanitize all player input
4. **Rate Limiting**: Prevent spam and DoS attacks
5. **Encryption**: Secure sensitive communications

```rust
/// Security module for the lobby system
pub mod security {
    /// Validate player input
    pub fn validate_player_input(input: &str) -> bool {
        // Check for malicious content
        // ...
        true
    }
    
    /// Rate limit tracker
    pub struct RateLimiter {
        /// Action counts per client
        client_actions: HashMap<ClientId, Vec<(f64, ActionType)>>,
    }
    
    impl RateLimiter {
        /// Check if action is allowed
        pub fn check_rate_limit(&mut self, client_id: ClientId, action: ActionType, time: f64) -> bool {
            // Implement rate limiting logic
            // ...
            true
        }
    }
    
    /// Encrypt sensitive data
    pub fn encrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
        // Encryption implementation
        // ...
        Vec::new()
    }
}
```

## Testing and Validation

The lobby system includes comprehensive testing:

1. **Unit Tests**: Verify individual component behavior
2. **Integration Tests**: Test component interactions
3. **Load Tests**: Ensure system can handle many concurrent lobbies
4. **Latency Simulation**: Test under various network conditions
5. **Security Tests**: Verify system resilience against attacks

```rust
/// Test suite for the lobby system
#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test lobby creation and joining
    #[test]
    fn test_lobby_lifecycle() {
        // Set up test environment
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(LobbyPlugin);
        
        // Create a lobby
        // ...
        
        // Join the lobby
        // ...
        
        // Verify state
        // ...
    }
    
    /// Test chat functionality
    #[test]
    fn test_chat_system() {
        // Set up test environment
        // ...
        
        // Send chat messages
        // ...
        
        // Verify delivery
        // ...
    }
    
    /// Test game launch process
    #[test]
    fn test_game_launch() {
        // Set up test environment
        // ...
        
        // Ready up players
        // ...
        
        // Launch game
        // ...
        
        // Verify transition
        // ...
    }
}
```

## Deployment Considerations

The lobby system supports various deployment scenarios:

1. **Self-Hosted**: Players can host their own servers
2. **Dedicated Servers**: Centralized infrastructure
3. **Hybrid Model**: Official servers plus community hosting
4. **Cloud Deployment**: Scalable containers for peak times

```rust
/// Deployment configuration
pub struct DeploymentConfig {
    /// Server discovery method
    pub discovery: ServerDiscoveryMethod,
    /// Server capacity
    pub capacity: ServerCapacity,
    /// Geographic region
    pub region: String,
    /// Auto-scaling settings
    pub scaling: Option<ScalingConfig>,
}

/// Server capacity configuration
pub struct ServerCapacity {
    /// Maximum concurrent lobbies
    pub max_lobbies: usize,
    /// Maximum concurrent games
    pub max_games: usize,
    /// Maximum players per server
    pub max_players: usize,
}

/// Auto-scaling configuration
pub struct ScalingConfig {
    /// Minimum number of instances
    pub min_instances: usize,
    /// Maximum number of instances
    pub max_instances: usize,
    /// Scale up threshold (% utilization)
    pub scale_up_threshold: f32,
    /// Scale down threshold (% utilization)
    pub scale_down_threshold: f32,
}
```

This document provides a comprehensive overview of the server-side implementation for the multiplayer lobby system, including the necessary architectural components and security considerations. 