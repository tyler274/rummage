# Deck Database

The deck database in Rummage is a system for managing player decks, including creation, storage, validation, and manipulation during gameplay. This section documents the deck database architecture and implementation.

## Overview

The deck database provides the following functionality:

- **Deck Creation**: Building and configuring new decks
- **Storage**: Persistent storage of deck data
- **Validation**: Checking decks against format rules
- **Runtime Manipulation**: In-game deck operations like drawing and shuffling
- **Registry**: Managing a collection of predefined and custom decks

## Architecture

The deck database is implemented using a combination of:

- **Disk Storage**: JSON-based storage for deck persistence
- **In-Memory Representation**: Runtime deck entities and components
- **Deck Registry**: A global registry for decks available to all players
- **Player-Specific Decks**: Per-player deck instances

## Core Components

The deck database consists of these key components:

- **Deck Structure**: The core Deck type and its related components
- **Deck Builder**: A builder pattern for constructing decks
- **Deck Registry**: A resource for registering and retrieving decks
- **Deck Validation**: Format-specific validation systems
- **Deck Persistence**: Saving and loading deck configurations

## Integration with Game Systems

The deck database integrates with other game systems:

- **Card Database**: Drawing cards from the core card database
- **Player Systems**: Assigning decks to players
- **Game Rules**: Format-specific deck constraints
- **UI Systems**: Deck building and viewing interfaces

## Format Support

The deck database supports multiple deck formats:

- **Commander/EDH**: 100-card singleton decks with a commander
- **Standard**: 60-card minimum with 4-copy maximum per card
- **Modern/Legacy/Vintage**: Format-specific banned and restricted lists
- **Limited**: 40-card minimum built from a limited card pool
- **Custom**: User-defined formats with custom rules

## Related Documentation

- [Deck Structure](deck_structure.md): Core data structures for decks
- [Deck Builder](deck_builder.md): Building and validating decks
- [Deck Registry](deck_registry.md): Managing multiple decks
- [Format Validation](format_validation.md): Format-specific deck constraints
- [Card Integration](../database/index.md): Integration with the card database 