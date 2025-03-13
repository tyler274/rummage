# Multiplayer Lobby System

This document outlines the networking architecture for the Commander format multiplayer lobby system. The lobby system enables players to discover, join, and configure games before starting a match.

## Table of Contents

1. [System Overview](#system-overview)
2. [Lobby Discovery](#lobby-discovery)
3. [Lobby Information and Browsing](#lobby-information-and-browsing)
4. [Joining and Managing Lobbies](#joining-and-managing-lobbies)
5. [Chat System](#chat-system)
6. [Ready-Up Mechanism](#ready-up-mechanism)
7. [Deck and Commander Viewing](#deck-and-commander-viewing)
8. [Game Launch](#game-launch)
9. [Connection Protocol](#connection-protocol)
10. [Implementation Details](#implementation-details)

## System Overview

The lobby system serves as the pre-game matchmaking component of the multiplayer experience. It allows players to:

1. Browse available game lobbies
2. Create new lobbies with custom settings
3. Join existing lobbies
4. Chat with other players
5. View other players' decks and commanders
6. Ready up for the game
7. Launch into a Commander game once all players are ready

```rust
/// The main states of the lobby system
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LobbyState {
    /// Lobby browser showing available games
    Browser,
    /// Inside a specific lobby
    InLobby,
    /// Transitioning to game
    LaunchingGame,
}

/// Resource tracking the active lobby connection
#[derive(Resource, Default)]
pub struct LobbyConnection {
    /// ID of the connected lobby or None if browsing
    pub current_lobby_id: Option<String>,
    /// Whether the player is the host of the current lobby
    pub is_host: bool,
    /// The server address this lobby is hosted on
    pub server_address: Option<String>,
}
```

## Lobby Discovery

Players can discover lobbies through two primary methods:

1. **Server List**: Connect to a lobby server that hosts multiple lobbies
2. **Direct IP**: Connect directly to a specific lobby host

```rust
/// Methods for discovering lobbies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LobbyDiscoveryMethod {
    /// Connect to a lobby server
    ServerList(String), // Server address
    /// Connect directly to a host
    DirectIp(String),   // IP address and port
}

/// Server list request message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerListRequest {
    /// Client version
    pub version: String,
    /// Optional filter parameters
    pub filters: Option<LobbyFilters>,
}

/// Server list response message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerListResponse {
    /// List of available lobbies
    pub lobbies: Vec<LobbyInfo>,
    /// Total number of lobbies on the server
    pub total_lobbies: usize,
}
```

### Lobby Browser UI

The lobby browser presents a list of available lobbies with key information:

1. Lobby name
2. Host name
3. Current player count / maximum players
4. Commander format details (standard, cEDH, etc.)
5. Special restrictions or rules
6. Whether the lobby is password-protected

## Lobby Information and Browsing

Lobbies include various pieces of information for players to browse:

```rust
/// Information about a lobby visible in the browser
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyInfo {
    /// Unique identifier for the lobby
    pub id: String,
    /// Displayed name of the lobby
    pub name: String,
    /// Host player's name
    pub host_name: String,
    /// Current number of players
    pub player_count: usize,
    /// Maximum allowed players
    pub max_players: usize,
    /// Whether the lobby is password protected
    pub has_password: bool,
    /// Format information
    pub format: CommanderFormat,
    /// Game rules and restrictions
    pub restrictions: GameRestrictions,
    /// Brief description provided by the host
    pub description: Option<String>,
}

/// Commander format details
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CommanderFormat {
    /// Standard Commander rules
    Standard,
    /// Competitive EDH
    CEDH,
    /// Commander Variant (e.g., Brawl, Oathbreaker)
    Variant(String),
    /// Custom rule set
    Custom,
}

/// Game restrictions and rule modifications
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct GameRestrictions {
    /// Card pool restrictions (e.g., budget, no infinites)
    pub card_pool: Vec<String>,
    /// Deck construction rules
    pub deck_rules: Vec<String>,
    /// House rules for gameplay
    pub game_rules: Vec<String>,
}
```

## Joining and Managing Lobbies

When a player selects a lobby, they can view detailed information before joining:

```rust
/// Detailed lobby information shown when selected
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyDetails {
    /// Basic lobby info
    pub info: LobbyInfo,
    /// Detailed description
    pub full_description: String,
    /// Current players in the lobby
    pub players: Vec<LobbyPlayerInfo>,
    /// Expected game duration
    pub estimated_duration: Option<String>,
    /// Additional custom settings
    pub custom_settings: HashMap<String, String>,
}

/// Join request message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinLobbyRequest {
    /// Lobby to join
    pub lobby_id: String,
    /// Player name
    pub player_name: String,
    /// Password if required
    pub password: Option<String>,
}

/// Join response message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JoinLobbyResponse {
    /// Whether the join was successful
    pub success: bool,
    /// Reason for failure if unsuccessful
    pub failure_reason: Option<String>,
    /// Lobby details if successful
    pub lobby_details: Option<LobbyDetails>,
}
```

## Chat System

The lobby includes a chat system to allow players to communicate:

```rust
/// Chat message in a lobby
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    /// Message ID for tracking
    pub id: String,
    /// Sender of the message
    pub sender: String,
    /// Whether the message is from the system
    pub is_system: bool,
    /// Message content
    pub content: String,
    /// Timestamp
    pub timestamp: f64,
}

/// Chat message request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendChatRequest {
    /// Lobby ID
    pub lobby_id: String,
    /// Message content
    pub content: String,
}
```

The chat system should support:
- Player-to-player messages
- System announcements
- Emoji/reactions
- Message history

## Ready-Up Mechanism

Players need to "ready up" before a game can start:

```rust
/// Player state in the lobby
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PlayerLobbyState {
    /// Just joined, not ready
    Joined,
    /// Selecting deck
    SelectingDeck,
    /// Ready with deck selected
    Ready,
}

/// Ready status update
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReadyStatusUpdate {
    /// Player changing status
    pub player_name: String,
    /// New status
    pub status: PlayerLobbyState,
    /// Selected deck info if ready
    pub deck_info: Option<DeckInfo>,
}
```

The host can only start the game when all players are in the `Ready` state.

## Deck and Commander Viewing

Players can view each other's decks and commanders:

```rust
/// Basic deck information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeckInfo {
    /// Deck name
    pub name: String,
    /// Commander card
    pub commander: CommanderInfo,
    /// Partner commander if applicable
    pub partner: Option<CommanderInfo>,
    /// Deck color identity
    pub colors: Vec<String>,
    /// Card count
    pub card_count: usize,
    /// Average mana value
    pub avg_mana_value: f32,
    /// Deck power level (1-10)
    pub power_level: u8,
    /// Whether to share full decklist
    pub share_decklist: bool,
}

/// Commander card information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommanderInfo {
    /// Card name
    pub name: String,
    /// Mana cost
    pub mana_cost: String,
    /// Card type
    pub type_line: String,
    /// Rules text
    pub text: String,
    /// Power/toughness if applicable
    pub power_toughness: Option<String>,
    /// Card image URI
    pub image_uri: Option<String>,
}

/// Request to view a full decklist
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeckViewRequest {
    /// Player whose deck to view
    pub player_name: String,
}

/// Full decklist response
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeckViewResponse {
    /// Basic deck info
    pub info: DeckInfo,
    /// Full card list if shared
    pub card_list: Option<Vec<CardInfo>>,
    /// Reason if not shared
    pub not_shared_reason: Option<String>,
}
```

## Game Launch

When all players are ready, the host can launch the game:

```rust
/// Game launch request (host only)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchGameRequest {
    /// Lobby ID
    pub lobby_id: String,
}

/// Game launch notification to all players
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameLaunchNotification {
    /// Game connection details
    pub connection_details: GameConnectionDetails,
    /// Final player list
    pub players: Vec<LobbyPlayerInfo>,
    /// Game settings
    pub settings: GameSettings,
}

/// Connection details for the actual game
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameConnectionDetails {
    /// Game server address
    pub server_address: String,
    /// Game ID
    pub game_id: String,
    /// Connection token
    pub connection_token: String,
}
```

## Connection Protocol

The lobby system uses a reliable TCP connection for all communications:

1. Player connects to a lobby server
2. Authentication (if required)
3. Request lobby list
4. Select and join a lobby
5. Participate in lobby activities
6. Ready up when prepared
7. Transition to game when launched

### Connection States

```rust
/// Connection state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobbyConnectionState {
    /// Not connected
    Disconnected,
    /// Attempting to connect
    Connecting,
    /// Connected and authenticated
    Connected,
    /// Browsing available lobbies
    Browsing,
    /// Inside a specific lobby
    InLobby,
    /// Preparing to launch game
    PreLaunch,
    /// Transitioning to game
    Launching,
    /// Connection error
    Error,
}
```

## Implementation Details

The lobby system will be implemented using Bevy's ECS architecture with these key components:

### Lobby Browser Scene

```rust
pub fn setup_lobby_browser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lobbies: Res<AvailableLobbies>,
) {
    // Main container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            LobbyBrowserUI,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            }).with_children(|header| {
                // Title
                header.spawn(Text2d {
                    text: "Multiplayer Lobbies".into(),
                    // Add styling...
                });
                
                // Controls (Refresh, Create Lobby, etc.)
                header.spawn(Node {
                    width: Val::Auto,
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                }).with_children(|controls| {
                    // Refresh button
                    // Create lobby button
                    // Direct connect field
                });
            });
            
            // Lobby list
            parent.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::Scroll,
                ..default()
            }).with_children(|list| {
                // Render each lobby as a selectable item
                for lobby in &lobbies.list {
                    create_lobby_list_item(list, lobby, &asset_server);
                }
            });
            
            // Details panel
            parent.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                ..default()
            }).with_children(|details| {
                // Show selected lobby details
                // Join button
            });
        });
}

/// System to handle refreshing the lobby list
pub fn refresh_lobby_list(
    mut lobby_query: EventReader<RefreshLobbyListEvent>,
    mut connection: ResMut<LobbyConnection>,
    mut lobbies: ResMut<AvailableLobbies>,
) {
    for _ in lobby_query.read() {
        // Send request to server
        // Handle response and update lobbies.list
    }
}
```

### Lobby Detail Scene

```rust
pub fn setup_lobby_detail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lobby_details: Res<CurrentLobbyDetails>,
) {
    // Main container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            LobbyDetailUI,
        ))
        .with_children(|parent| {
            // Left panel (player list and details)
            parent.spawn(Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            }).with_children(|left_panel| {
                // Lobby info
                // Player list
                // Ready button
            });
            
            // Center panel (chat)
            parent.spawn(Node {
                width: Val::Percent(40.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            }).with_children(|center_panel| {
                // Chat history
                // Chat input
            });
            
            // Right panel (deck viewer)
            parent.spawn(Node {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            }).with_children(|right_panel| {
                // Selected deck info
                // Commander view
                // Deck stats
            });
        });
}

/// System to send chat messages
pub fn send_chat_message(
    mut chat_events: EventReader<SendChatEvent>,
    mut connection: ResMut<LobbyConnection>,
) {
    for event in chat_events.read() {
        // Format and send chat message to server
    }
}

/// System to update player ready status
pub fn update_ready_status(
    mut ready_events: EventReader<ReadyStatusEvent>,
    mut connection: ResMut<LobbyConnection>,
) {
    for event in ready_events.read() {
        // Update local state
        // Send ready status to server
    }
}
```

### Game Launch

```rust
pub fn launch_game(
    mut launch_events: EventReader<LaunchGameEvent>,
    mut connection: ResMut<LobbyConnection>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for event in launch_events.read() {
        // Handle game launch
        // Transition to Loading state
        next_state.set(GameMenuState::Loading);
    }
}
```

### Systems and Resources

The lobby system will use these key Bevy systems and resources:

```rust
/// Plugin to register all lobby-related systems
pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<LobbyState>()
            
            // Resources
            .init_resource::<LobbyConnection>()
            .init_resource::<AvailableLobbies>()
            .init_resource::<CurrentLobbyDetails>()
            .init_resource::<ChatHistory>()
            
            // Events
            .add_event::<RefreshLobbyListEvent>()
            .add_event::<JoinLobbyEvent>()
            .add_event::<LeaveLobbyEvent>()
            .add_event::<SendChatEvent>()
            .add_event::<ReadyStatusEvent>()
            .add_event::<ViewDeckEvent>()
            .add_event::<LaunchGameEvent>()
            
            // Systems
            .add_systems(OnEnter(LobbyState::Browser), setup_lobby_browser)
            .add_systems(OnExit(LobbyState::Browser), cleanup_lobby_browser)
            .add_systems(OnEnter(LobbyState::InLobby), setup_lobby_detail)
            .add_systems(OnExit(LobbyState::InLobby), cleanup_lobby_detail)
            
            .add_systems(Update, 
                (
                    handle_lobby_connections,
                    refresh_lobby_list,
                    process_lobby_messages,
                ).run_if(in_state(LobbyState::Browser))
            )
            
            .add_systems(Update, 
                (
                    process_lobby_messages,
                    send_chat_message,
                    update_ready_status,
                    handle_deck_viewing,
                    launch_game,
                ).run_if(in_state(LobbyState::InLobby))
            );
    }
}
```

## Integration with Main Menu

The multiplayer button in the main menu will trigger a transition to the lobby browser:

```rust
pub fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut lobby_state: ResMut<NextState<LobbyState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Multiplayer => {
                    // Transition to lobby browser
                    lobby_state.set(LobbyState::Browser);
                    // Note: We might need a new GameMenuState for multiplayer
                }
                // Other actions...
            }
        }
    }
}
```

This document provides a comprehensive overview of the multiplayer lobby system design, including the necessary components and systems to implement it within the Bevy ECS architecture. The implementation details can be expanded as needed during development. 