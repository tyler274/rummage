# Player Mechanics

This section covers player-specific mechanics in the Commander format implementation.

## Contents

- [Player Management](player_management.md) - Player data structures and basic player operations
- [Commander Damage](commander_damage.md) - Tracking and implementing the 21-damage rule
- [Multiplayer Politics](multiplayer_politics.md) - Voting, deals, and other social mechanics

The player mechanics section defines how players interact within the Commander format, including:

- Starting with 40 life (compared to 20 in standard Magic)
- Commander damage tracking (21+ combat damage from a single commander causes a loss)
- Turn order and priority management in multiplayer
- Special multiplayer interactions like voting, monarch status, and political deals
- Player elimination and handling of player-owned objects after elimination

These mechanics are essential for correctly implementing the multiplayer aspects of Commander, especially with games supporting between 2 and 13 players. 