# Multiplayer Lobby System Documentation

This document serves as an index for all documentation related to Rummage's multiplayer lobby system for Commander format games. These documents cover the user interface, networking architecture, and implementation details for creating a robust and enjoyable multiplayer experience.

## Overview and Architecture

- [Lobby UI System Overview](lobby_ui_overview.md) - High-level overview of the lobby UI architecture and flow
- [Lobby Networking](lobby_networking.md) - Networking architecture for the lobby system
- [Lobby Backend](lobby_backend.md) - Server-side implementation details

## User Interface Components

- [Lobby Browser UI](lobby_browser_ui.md) - UI for browsing available game lobbies
- [Lobby Detail UI](lobby_detail_ui.md) - UI for the specific lobby view, player management, and ready-up mechanics
- [Lobby Chat UI](lobby_chat_ui.md) - Chat system implementation for lobby communication
- [Lobby Deck Viewer](lobby_deck_viewer.md) - Deck and commander viewing UI

## Gameplay and Departure Handling

- [Game Departure Handling](game_departure_handling.md) - How the system handles players leaving games

## Implementation Guides

- [Lobby Implementation Guide](../development/lobby_implementation.md) - Step-by-step guide for implementing lobby features
- [Lobby Testing Guide](../testing/lobby_testing.md) - Testing strategies for the lobby system

## Feature Highlights

### Server List and Direct Connect

The lobby system supports two connection methods:
1. **Server List**: Browse a list of available lobby servers
2. **Direct Connect**: Connect directly to a specific host's IP

### Lobby Browsing

The browser screen provides a comprehensive view of available lobbies with:
- Lobby name and host information
- Player count and maximum players
- Game format details
- Password protection indicator
- Filtering and sorting options

### In-Lobby Features

Once in a lobby, players can:
- Chat with other players
- View player information
- Select a deck and indicate readiness
- View other players' commander cards and deck information
- Manage privacy settings for their decks

### Host Controls

Lobby hosts have additional capabilities:
- Kick players
- Set lobby rules and restrictions
- Configure game settings
- Launch the game when all players are ready

### Privacy and Deck Sharing

The system implements flexible privacy settings:
- Share just commander information
- Share basic deck statistics
- Share full decklist
- Customizable sharing options

### Robust Departure Handling

The system gracefully handles various departure scenarios:
- Voluntary quits
- Being kicked by host
- Temporary disconnections with reconnection window
- Preservation of game state for departed players

## Implementation Details

The lobby system is built using:
- **Bevy ECS**: Component-based architecture for UI and game state
- **Bevy Replicon**: Networking and state replication
- **WebRTC**: For efficient and reliable UDP-based communication
- **Delta Compression**: For efficient state updates

## Getting Started

To get started with implementing or extending the lobby system, we recommend:

1. Review the [Lobby UI System Overview](lobby_ui_overview.md) for a high-level understanding
2. Examine the [Lobby Networking](lobby_networking.md) document to understand the communication architecture
3. Follow the step-by-step [Lobby Implementation Guide](../development/lobby_implementation.md)

## Future Enhancements

Planned enhancements for the lobby system include:
- Voice chat integration
- Advanced deck analysis tools
- Tournament mode
- Spectator support
- Enhanced moderation tools 