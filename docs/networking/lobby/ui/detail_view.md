# Multiplayer Lobby Detail UI

This document outlines the user interface for the lobby detail screen in the multiplayer system. This screen is displayed after a player joins a specific lobby and is where players prepare for the game, interact with each other, and manage their participation.

## Table of Contents

1. [UI Layout](#ui-layout)
2. [Components](#components)
3. [Player Management](#player-management)
4. [Player Actions](#player-actions)
5. [Host Controls](#host-controls)
6. [Handling Player Departures](#handling-player-departures)
7. [Implementation](#implementation)

## UI Layout

The lobby detail screen is divided into three main panels:

1. **Left Panel**: Lobby information and player list
2. **Center Panel**: Chat system
3. **Right Panel**: Deck viewer

```
┌───────────────────────────────────────────────────────┐
│ Lobby Name                  Host: PlayerName          │
├───────────────┬───────────────────┬───────────────────┤
│               │                   │                   │
│ PLAYERS       │                   │ DECK VIEWER       │
│               │                   │                   │
│ ○ Player1     │                   │ ┌─────────────┐   │
│   (Host)      │                   │ │             │   │
│               │                   │ │ Commander   │   │
│ ○ Player2     │     CHAT          │ │ Card        │   │
│   [Ready]     │                   │ │             │   │
│               │                   │ └─────────────┘   │
│ ○ Player3     │                   │                   │
│   [Selecting] │                   │ Deck Statistics   │
│               │                   │                   │
│ ○ Player4     │                   │                   │
│               │                   │                   │
├───────────────┴───────────────────┴───────────────────┤
│ [Leave Lobby]    [Select Deck]       [Ready Up]       │
└───────────────────────────────────────────────────────┘
```

## Components

### Lobby Header

The header displays essential information about the lobby:

- Lobby name
- Host name
- Game format (Standard Commander, cEDH, etc.)
- Player count (current/maximum)
- Password protection indicator

### Player List

The player list shows all players in the lobby with their current status:

- Player name
- Ready status
- Selected deck (if ready)
- Host indicator
- Color identity of selected commander (if ready)

### Ready Controls

Players can indicate their readiness through:

- Deck selection button
- Ready/Unready toggle
- Current status indicator

## Player Management

The lobby implements a robust player management system to handle various player actions and states.

### Player States

Players can be in various states while in the lobby:

```rust
/// Player state in the lobby
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub enum PlayerLobbyState {
    /// Just joined, not ready
    Joined,
    /// Selecting a deck
    SelectingDeck,
    /// Ready with deck selected
    Ready,
    /// Temporarily disconnected (can reconnect)
    Disconnected,
    /// In the process of joining
    Joining,
}
```

## Player Actions

Players can perform several actions in the lobby:

### Selecting a Deck

Players need to select a deck before they can ready up:

```rust
fn handle_deck_selection(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeckSelectButton>)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<DeckSelectionState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Transition to deck selection screen
            next_state.set(DeckSelectionState::Selecting);
        }
    }
}
```

### Ready Status

Players indicate they are ready to play:

```rust
fn handle_ready_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReadyButton>)>,
    mut player_query: Query<&mut PlayerLobbyState, With<LocalPlayer>>,
    mut lobby_connection: ResMut<LobbyConnection>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Toggle ready status
            if let Ok(mut state) = player_query.get_single_mut() {
                match *state {
                    PlayerLobbyState::Ready => {
                        *state = PlayerLobbyState::Joined;
                        lobby_connection.send_status_update(PlayerLobbyState::Joined);
                    }
                    _ => {
                        if lobby_connection.has_deck_selected() {
                            *state = PlayerLobbyState::Ready;
                            lobby_connection.send_status_update(PlayerLobbyState::Ready);
                        }
                    }
                }
            }
        }
    }
}
```

### Leaving a Lobby

Players can voluntarily leave a lobby:

```rust
fn handle_leave_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LeaveButton>)>,
    mut lobby_connection: ResMut<LobbyConnection>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Send leave notification to server
            lobby_connection.leave_lobby();
            
            // Return to lobby browser
            next_state.set(GameMenuState::MultiplayerBrowser);
        }
    }
}
```

## Host Controls

The host player has additional controls for managing the lobby:

### Start Game

The host can start the game when all players are ready:

```rust
fn handle_start_game_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut lobby_connection: ResMut<LobbyConnection>,
    player_query: Query<&PlayerLobbyState>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Check if all players are ready
            let all_ready = player_query
                .iter()
                .all(|state| *state == PlayerLobbyState::Ready);
                
            if all_ready {
                lobby_connection.start_game();
            }
        }
    }
}
```

### Kick Player

The host can remove players from the lobby:

```rust
fn handle_kick_button(
    mut interaction_query: Query<(&Interaction, &KickButtonTarget), (Changed<Interaction>, With<KickButton>)>,
    mut lobby_connection: ResMut<LobbyConnection>,
) {
    for (interaction, target) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Send kick request to server
            lobby_connection.kick_player(target.player_id.clone());
        }
    }
}
```

## Handling Player Departures

The system handles various scenarios for player departures from the lobby:

### Voluntary Departure

When a player chooses to leave a lobby:

1. The player initiates departure through the Leave button
2. A leave message is sent to the server
3. The server broadcasts the player's departure to all other players
4. The player's UI transitions to the lobby browser
5. Other players receive a notification of the departure

```rust
/// Message sent when a player leaves a lobby
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeaveLobbyMessage {
    /// ID of the lobby being left
    pub lobby_id: String,
    /// Reason for leaving
    pub reason: LeaveLobbyReason,
}

/// Reasons a player might leave a lobby
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LeaveLobbyReason {
    /// Player chose to leave
    Voluntary,
    /// Player was kicked by the host
    Kicked,
    /// Player disconnected unexpectedly
    Disconnected,
    /// Player was idle for too long
    Timeout,
}
```

### Being Kicked

When a host kicks a player:

1. The host selects a player and clicks the Kick button
2. A kick message is sent to the server
3. The server validates the request (ensures sender is host)
4. The server sends a departure notification to the kicked player
5. The kicked player's UI transitions to the lobby browser with a message
6. Other players receive a notification that the player was kicked

```rust
/// System to handle being kicked from a lobby
fn handle_being_kicked(
    mut kick_events: EventReader<KickedFromLobbyEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for event in kick_events.read() {
        // Display kicked message
        commands.spawn(KickedNotificationUI {
            reason: event.reason.clone(),
        });
        
        // Return to lobby browser
        next_state.set(GameMenuState::MultiplayerBrowser);
    }
}
```

### Disconnections

The system also handles unexpected disconnections:

1. When a player disconnects, the server detects the dropped connection
2. The server broadcasts a player disconnection to all remaining players
3. The server keeps the player's slot reserved for a period of time
4. If the player reconnects within the time window, they rejoin seamlessly
5. If the reconnection window expires, the slot is freed and other players are notified

```rust
/// System to handle player disconnections
fn handle_player_disconnection(
    mut disconnection_events: EventReader<PlayerDisconnectedEvent>,
    mut player_query: Query<(&mut PlayerLobbyState, &PlayerName)>,
    mut lobby_state: ResMut<LobbyState>,
) {
    for event in disconnection_events.read() {
        // Update the disconnected player's state
        for (mut state, name) in &mut player_query {
            if name.0 == event.player_name {
                *state = PlayerLobbyState::Disconnected;
                
                // Start reconnection timer
                lobby_state.disconnection_timers.insert(
                    event.player_name.clone(),
                    ReconnectionTimer {
                        time_remaining: RECONNECTION_WINDOW,
                        player_id: event.player_id.clone(),
                    }
                );
                
                break;
            }
        }
    }
}
```

### Host Migration

If the host leaves or disconnects, the system can migrate host privileges to another player:

1. When the host leaves, the server selects the next player (typically by join time)
2. The server broadcasts a host migration message to all players
3. The new host receives additional UI controls
4. All players are notified of the host change

```rust
/// Message for host migration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HostMigrationMessage {
    /// ID of the new host
    pub new_host_id: String,
    /// Name of the new host
    pub new_host_name: String,
}

/// System to handle host migration
fn handle_host_migration(
    mut migration_events: EventReader<HostMigrationEvent>,
    mut player_query: Query<(&mut PlayerLobbyState, &PlayerName, &mut PlayerUI)>,
    mut lobby_state: ResMut<LobbyState>,
) {
    for event in migration_events.read() {
        // Update host status
        lobby_state.host_id = event.new_host_id.clone();
        
        // Update UI to reflect new host
        for (state, name, mut ui) in &mut player_query {
            if name.0 == event.new_host_name {
                ui.show_host_controls();
            }
        }
    }
}
```

## Implementation

The lobby detail UI is implemented using Bevy's UI system:

```rust
/// Set up the lobby detail screen
pub fn setup_lobby_detail(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lobby_info: Res<CurrentLobbyInfo>,
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
            LobbyDetailUI,
        ))
        .with_children(|parent| {
            // Header
            setup_lobby_header(parent, &asset_server, &lobby_info);
            
            // Main content area
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(85.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|content| {
                    // Left panel (player list)
                    setup_player_list_panel(content, &asset_server, &lobby_info);
                    
                    // Center panel (chat)
                    setup_chat_panel(content, &asset_server);
                    
                    // Right panel (deck viewer)
                    setup_deck_viewer_panel(content, &asset_server);
                });
                
            // Footer with action buttons
            setup_action_buttons(parent, &asset_server, &lobby_info);
        });
}
```

This UI adapts based on the player's role (host or regular player) and provides appropriate controls for each state. 