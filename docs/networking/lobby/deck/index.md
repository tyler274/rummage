# Lobby Deck System

This section covers the deck management functionality in the game lobby, allowing players to select, view, and validate decks before starting a game.

## Overview

The lobby deck system provides interfaces for players to interact with their deck collection before a game begins, ensuring that selected decks meet format requirements.

## Components

### [Viewer](viewer.md)

Details of the deck viewer interface, including:
- Deck list display
- Card preview functionality
- Deck statistics and analysis
- Format legality checking
- Commander and color identity validation
- Deck sharing options

## Features

- Deck selection from player's collection
- Format validation for Commander rules
- Deck statistics and composition analysis
- Last-minute deck adjustments
- Deck sharing with other players
- Deck import/export functionality

## Integration

The deck system integrates with:
- [Card Database](../../../card_systems/database/index.md) for card information
- [Deck Database](../../../card_systems/deck_database/index.md) for persistent storage
- [Lobby UI](../ui/index.md) for display integration
- [Lobby Backend](../backend/index.md) for deck validation and game setup 