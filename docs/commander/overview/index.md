# Commander Overview

This section provides a high-level overview of the Commander format implementation in our game engine.

## Contents

- [Format Rules](format_rules.md) - Core rules specific to the Commander format
- [Architecture](architecture.md) - Overview of the implementation architecture
- [Implementation Approach](implementation.md) - General approach to implementing the Commander format

The Commander format is a multiplayer variant of Magic: The Gathering with special rules including:
- Each player begins the game with a designated legendary creature as their "commander"
- Players start with 40 life
- A player can cast their commander from the command zone for its mana cost, plus an additional 2 mana for each previous time it's been cast
- If a commander would be put into a library, hand, graveyard or exile, its owner may choose to move it to the command zone
- A player who has been dealt 21 or more combat damage by the same commander loses the game

This documentation outlines our approach to implementing these rules in a digital format. 