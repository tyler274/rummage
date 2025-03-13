# Game Zones

This section covers the implementation of game zones in the Commander format, with special focus on the Command Zone.

## Contents

- [Command Zone](command_zone.md) - Management of the Command Zone and commander movement
- [Zone Transitions](zone_transitions.md) - Special rules for commander movement between zones
- [Zone Management](zone_management.md) - General zone implementation for Commander games

The zones section defines how the unique zone mechanics of Commander are implemented, including:

- The Command Zone as a special game zone where commanders start the game
- Commander-specific zone transition rules (optional movement to command zone)
- Commander tax implementation (additional {2} cost for each previous cast from command zone)
- Tracking of commanders across all game zones
- Special handling for partner commanders and backgrounds

These zone implementations are critical for the proper functioning of the Commander format, as they enable many of the format's unique gameplay mechanics. 