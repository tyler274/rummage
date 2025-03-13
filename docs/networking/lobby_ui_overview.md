# Multiplayer Lobby UI System Overview

This document provides a high-level overview of the multiplayer lobby user interface system for Rummage's Commander format game. For more detailed information on specific components, please see the related documentation files.

## Table of Contents

1. [UI Architecture Overview](#ui-architecture-overview)
2. [UI Flow](#ui-flow)
3. [Screen Components](#screen-components)
4. [Integration with Game States](#integration-with-game-states)
5. [Related Documentation](#related-documentation)

## UI Architecture Overview

The multiplayer lobby system uses Bevy's UI components, with a focus on clean, responsive design that works well across different screen sizes. The UI is built using these key Bevy components:

- `Node` for layout containers
- `Button` for interactive elements
- `Text2d` for text display
- `Image` for graphics and icons

The UI follows a component-based architecture where different parts of the interface are organized hierarchically and can be created, updated, or removed independently.

## UI Flow

The user flow through the multiplayer lobby system follows this sequence:

1. **Main Menu**: Player clicks the "Multiplayer" button in the main menu
2. **Server Connection**: Player selects a server or enters a direct IP address
3. **Lobby Browser**: Player views a list of available game lobbies
4. **Lobby Detail**: Player joins a lobby and prepares for the game
5. **Game Launch**: When all players are ready, the host launches the game

Each of these screens has its own set of UI components and systems to handle user interaction.

```
┌─────────────┐
│  Main Menu  │
└──────┬──────┘
       │
       ▼
┌─────────────────┐
│ Server Connection│
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Lobby Browser  │◄────┐
└────────┬────────┘     │
         │              │
         ▼              │
┌─────────────────┐     │
│  Lobby Detail   │─────┘
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   Game Launch   │
└─────────────────┘
```

## Screen Components

Each screen in the lobby system consists of multiple UI components:

### 1. Server Connection Screen

- Server list with ping indicators
- Direct IP connection field
- Connect button
- Back button

### 2. Lobby Browser Screen

- Lobby list with game information
- Filter and sort controls
- Create lobby button
- Refresh button
- Lobby details panel
- Join lobby button

### 3. Lobby Detail Screen

- Lobby information header
- Player list with status indicators
- Chat panel
- Deck selection controls
- Ready up button
- Deck viewing section

### 4. Game Launch Transition

- Loading screen
- Connection status indicators
- Game initialization progress

## Integration with Game States

The lobby UI system integrates with Bevy's state management system:

```rust
/// Game states extended to include multiplayer states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameMenuState {
    /// Initial state, showing the main menu
    #[default]
    MainMenu,
    /// Multiplayer lobby browser
    MultiplayerBrowser,
    /// Inside a specific lobby
    MultiplayerLobby,
    /// Transitional state for loading game assets
    Loading,
    /// Active gameplay state
    InGame,
    /// Game is paused, showing pause menu
    PausedGame,
}
```

State transitions are handled by systems that respond to user interactions and network events:

```rust
fn handle_multiplayer_button(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed && *action == MenuButtonAction::Multiplayer {
            next_state.set(GameMenuState::MultiplayerBrowser);
        }
    }
}
```

## Related Documentation

For more detailed information about specific aspects of the lobby UI system, please refer to these documents:

- [Lobby Browser UI](lobby_browser_ui.md) - Details of the lobby browsing interface
- [Lobby Detail UI](lobby_detail_ui.md) - Details of the specific lobby view
- [Lobby Chat UI](lobby_chat_ui.md) - Chat system implementation
- [Lobby Deck Viewer](lobby_deck_viewer.md) - Deck and commander viewing UI
- [Lobby Networking](lobby_networking.md) - Network architecture for the lobby system
- [Lobby Backend](lobby_backend.md) - Server-side implementation details 