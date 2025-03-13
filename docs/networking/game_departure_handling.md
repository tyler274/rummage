# Game Departure Handling

This document describes how the system handles players leaving an active Commander game, whether through quitting, being kicked, or disconnection. Handling player departures properly is crucial for maintaining game integrity and player experience.

## Table of Contents

1. [Overview](#overview)
2. [Departure Scenarios](#departure-scenarios)
3. [Game State Preservation](#game-state-preservation)
4. [UI Experience](#ui-experience)
5. [Reconnection Flow](#reconnection-flow)
6. [Implementation](#implementation)

## Overview

In a multiplayer Commander game, players may leave for various reasons, and the game must handle these departures gracefully. The system differentiates between different types of departures and provides appropriate mechanisms for each.

```
┌───────────────────┐
│                   │
│   Active Game     │◄───── New Player Joining (Spectator)
│                   │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  Player Departure │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┬────────────────┬───────────────────┐
│                   │                │                   │
│ Voluntary Quit    │ Kicked by Host │ Disconnection     │
│                   │                │                   │
└─────────┬─────────┴────────┬───────┴─────────┬─────────┘
          │                  │                 │
          ▼                  ▼                 ▼
┌───────────────────┐ ┌──────────────┐ ┌───────────────────┐
│ Return to Menu    │ │ Menu + Notif │ │ Reconnect Window  │
└───────────────────┘ └──────────────┘ └─────────┬─────────┘
                                                 │
                                                 ▼
                                       ┌───────────────────┐
                                       │  Success/Failure  │
                                       └───────────────────┘
```

## Departure Scenarios

### Voluntary Quitting

When a player intentionally quits a game:

1. Player selects "Quit Game" from the pause menu
2. A confirmation dialog appears
3. Upon confirmation, a quit message is sent to the server
4. The server processes the departure and notifies other players
5. Game state is updated to mark the player as departed
6. The quitting player returns to the main menu

```rust
/// System to handle quit game request
fn handle_quit_game(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    mut confirmation_dialog: ResMut<ConfirmationDialog>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Show confirmation dialog
            confirmation_dialog.show(
                "Quit Game?",
                "Are you sure you want to quit this game? Your progress will be lost.",
                ConfirmationAction::QuitGame,
            );
        }
    }
}

/// System to process quit confirmation
fn process_quit_confirmation(
    mut confirmation_events: EventReader<ConfirmationResponse>,
    mut game_connection: ResMut<GameConnection>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for event in confirmation_events.read() {
        if event.action == ConfirmationAction::QuitGame && event.confirmed {
            // Send quit notification to server
            game_connection.send_departure_notification(DepartureReason::Voluntary);
            
            // Return to main menu
            next_state.set(GameMenuState::MainMenu);
        }
    }
}
```

### Being Kicked

When a player is kicked by the host:

1. Host opens player menu and selects "Kick Player"
2. A confirmation dialog appears for the host
3. Upon confirmation, a kick message is sent to the server
4. The server validates the request and processes the kick
5. The kicked player receives a notification
6. The kicked player's UI transitions to the main menu with a message
7. Other players are notified that the player was kicked

```rust
/// Host-side kick request
fn handle_kick_player_request(
    mut interaction_query: Query<(&Interaction, &PlayerTarget), (Changed<Interaction>, With<KickPlayerButton>)>,
    mut confirmation_dialog: ResMut<ConfirmationDialog>,
) {
    for (interaction, target) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // Show confirmation dialog for host
            confirmation_dialog.show(
                "Kick Player?",
                &format!("Are you sure you want to kick {}?", target.name),
                ConfirmationAction::KickPlayer(target.player_id.clone()),
            );
        }
    }
}

/// System to handle being kicked from a game
fn handle_being_kicked_from_game(
    mut kick_events: EventReader<KickedFromGameEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for event in kick_events.read() {
        // Display kicked message
        commands.spawn(KickedGameNotificationUI {
            reason: event.reason.clone(),
        });
        
        // Return to main menu
        next_state.set(GameMenuState::MainMenu);
    }
}
```

### Disconnection

When a player disconnects unexpectedly:

1. The server detects a connection drop
2. The server keeps the player's game state for a reconnection window
3. Other players see the disconnected player's status change
4. If the player reconnects within the window, they rejoin seamlessly
5. If the reconnection window expires, the player is fully removed

```rust
/// System to handle disconnections in active games
fn handle_game_disconnection(
    mut disconnection_events: EventReader<PlayerDisconnectedGameEvent>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&mut PlayerComponent, &NetworkId)>,
) {
    for event in disconnection_events.read() {
        // Find and update player state
        for (mut player, network_id) in &mut player_query {
            if network_id.0 == event.player_id {
                player.connection_status = ConnectionStatus::Disconnected;
                player.reconnection_timer = Some(GAME_RECONNECTION_WINDOW);
                
                // Pause the player's turn if active
                if game_state.active_player == event.player_id {
                    game_state.active_turn_paused = true;
                    game_state.pause_reason = PauseReason::PlayerDisconnected;
                }
                
                break;
            }
        }
    }
}

/// System to handle reconnection windows
fn update_reconnection_timers(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<(&mut PlayerComponent, &NetworkId)>,
    mut commands: Commands,
) {
    for (mut player, network_id) in &mut player_query {
        if let Some(timer) = &mut player.reconnection_timer {
            *timer -= time.delta_seconds();
            
            if *timer <= 0.0 {
                // Reconnection window expired, remove player
                player.connection_status = ConnectionStatus::Left;
                player.reconnection_timer = None;
                
                // If it was their turn, pass to next player
                if game_state.active_player == network_id.0 {
                    commands.add(PassTurn);
                }
            }
        }
    }
}
```

## Game State Preservation

When a player leaves a Commander game, their game state is handled according to the format rules:

1. **Permanent Departure**: If a player quits, is kicked, or their reconnection window expires:
   - Their cards remain in play until they would naturally leave the battlefield
   - Their life total is set to 0 for Commander damage calculations
   - Their turns are skipped
   - They are no longer a valid target for spells/abilities that target players
   - Effects they controlled continue to function as normal until they expire

2. **Temporary Disconnection**: If a player disconnects but can reconnect:
   - Their game state is fully preserved
   - Their turn is paused if it was active
   - Other players can continue to play
   - The game resumes normally once they reconnect

```rust
/// Handle permanent player departure
fn handle_permanent_departure(
    mut commands: Commands,
    departure_event: Res<PlayerDepartureEvent>,
    mut game_state: ResMut<GameState>,
    player_query: Query<(Entity, &NetworkId, &PlayerComponent)>,
    card_query: Query<(Entity, &Owner, &Zone)>,
) {
    // Find the departed player's entity
    for (player_entity, network_id, player) in player_query.iter() {
        if network_id.0 == departure_event.player_id {
            // Mark player as departed in game state
            commands.entity(player_entity).insert(DepartedPlayer);
            
            // Set life total to 0 for commander damage calculations
            commands.entity(player_entity).insert(LifeTotal(0));
            
            // Handle active effects
            handle_departed_player_effects(commands, player_entity, &card_query);
            
            // If it was their turn, pass to next player
            if game_state.active_player == network_id.0 {
                commands.add(PassTurn);
            }
            
            break;
        }
    }
}
```

## UI Experience

The user interface handles departures with appropriate feedback:

### For the Departing Player

- **Voluntary Quit**: Simple transition to main menu
- **Kicked**: Notification explaining they were kicked before returning to main menu
- **Disconnection**: Reconnection attempts with progress indicators

### For Remaining Players

- **Player Quit**: Notification of player departure
- **Player Kicked**: Notification of player being kicked
- **Disconnection**: Status indicator showing disconnected state and reconnection attempt

```rust
/// UI component for disconnection status
#[derive(Component)]
pub struct DisconnectionUI {
    /// Disconnected player name
    player_name: String,
    /// Time remaining in reconnection window
    time_remaining: f32,
}

/// System to update disconnection UI
fn update_disconnection_ui(
    mut disconnection_query: Query<(&mut Text, &mut DisconnectionUI)>,
    time: Res<Time>,
) {
    for (mut text, mut disconnection) in &mut disconnection_query {
        // Update remaining time
        disconnection.time_remaining -= time.delta_seconds();
        
        // Update display text
        text.sections[0].value = format!(
            "{} disconnected. Reconnecting... ({}s)",
            disconnection.player_name,
            disconnection.time_remaining as u32
        );
        
        // Change text color based on time remaining
        if disconnection.time_remaining < 10.0 {
            text.sections[0].style.color = Color::RED;
        }
    }
}
```

## Reconnection Flow

The reconnection process involves several steps:

1. **Detection**: Client detects lost connection to the game server
2. **Retry**: Client attempts to reconnect automatically
3. **Authentication**: Upon reconnection, client provides game session token
4. **State Sync**: Server sends complete game state to the reconnected client
5. **Resumption**: Game continues with the reconnected player

```rust
/// Reconnection sequence
fn handle_reconnection(
    mut connection: ResMut<GameConnection>,
    mut game_state: ResMut<LocalGameState>,
    mut next_state: ResMut<NextState<ReconnectionState>>,
) {
    match *next_state.get() {
        ReconnectionState::Disconnected => {
            // Attempt to reconnect
            if connection.attempt_reconnect() {
                next_state.set(ReconnectionState::Connecting);
            }
        }
        ReconnectionState::Connecting => {
            // Check connection status
            if connection.is_connected() {
                next_state.set(ReconnectionState::Authenticating);
            } else if connection.attempts_exhausted() {
                next_state.set(ReconnectionState::Failed);
            }
        }
        ReconnectionState::Authenticating => {
            // Authenticate with game session
            if connection.authenticate_session() {
                next_state.set(ReconnectionState::SyncingState);
            }
        }
        ReconnectionState::SyncingState => {
            // Receive and process game state
            if game_state.is_synchronized() {
                next_state.set(ReconnectionState::Completed);
            }
        }
        ReconnectionState::Completed => {
            // Resume game
            next_state.set(ReconnectionState::None);
        }
        ReconnectionState::Failed => {
            // Return to main menu with error
            // ...
        }
        _ => {}
    }
}
```

## Implementation

### Message Types

```rust
/// Game departure notification
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameDepartureNotification {
    /// Player ID that is departing
    pub player_id: String,
    /// Reason for departure
    pub reason: DepartureReason,
    /// Timestamp of departure
    pub timestamp: f64,
}

/// Reasons for player departure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum DepartureReason {
    /// Player chose to leave
    Voluntary,
    /// Player was kicked by the host
    Kicked,
    /// Player disconnected unexpectedly
    Disconnected,
    /// Server closed the game
    ServerClosure,
    /// Game ended normally
    GameCompleted,
}

/// Reconnection request
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReconnectionRequest {
    /// Game ID to reconnect to
    pub game_id: String,
    /// Player ID reconnecting
    pub player_id: String,
    /// Authentication token
    pub auth_token: String,
    /// Last known game tick
    pub last_tick: u64,
}
```

### Systems

The implementation includes dedicated systems to handle the various player departure scenarios:

```rust
/// Register game departure systems
pub fn register_departure_systems(app: &mut App) {
    app
        .add_event::<PlayerDepartureEvent>()
        .add_event::<KickedFromGameEvent>()
        .add_event::<PlayerDisconnectedGameEvent>()
        .add_event::<ReconnectionEvent>()
        .add_systems(Update, (
            handle_quit_game,
            process_quit_confirmation,
            handle_kick_player_request,
            handle_being_kicked_from_game,
            handle_game_disconnection,
            update_reconnection_timers,
            handle_permanent_departure,
            update_disconnection_ui,
            handle_reconnection,
        ));
}
```

These systems work together to provide a seamless experience for players during game departures, ensuring the game remains playable and enjoyable for those who remain. 