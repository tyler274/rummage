# Subgames and Game Restarting

## Subgames

In Magic: The Gathering, a "subgame" is a complete game of Magic played within another game, most notably created by the card Shahrazad. Subgames have their own distinct game state, separate from the main game.

### Shahrazad Implementation

Shahrazad is a sorcery that instructs players to leave the current game temporarily and play a subgame with their libraries as their decks. When the subgame concludes, the main game resumes, and the loser of the subgame loses half their life, rounded up.

Key implementation details:
- The subgame creates a completely new game state with its own zones, life totals, etc.
- Cards used in the subgame come from the players' libraries in the main game
- Cards that leave the subgame (exile, graveyard, etc.) remain outside the subgame
- When the subgame ends, all cards still in the subgame return to their owners' libraries in the main game
- The main game's state is suspended but preserved entirely during the subgame

### Technical Implementation

Our implementation of subgames utilizes a stack-based approach:
1. When a subgame is created, we push the current game state onto a stack
2. A new game state is initialized for the subgame with appropriate starting conditions
3. The subgame runs as a complete game with its own turn structure and rules
4. When the subgame concludes, we pop the previous game state from the stack and resume
5. We apply any results from the subgame to the main game (like life loss)

Subgames can be nested, meaning a Shahrazad could be cast within a subgame created by another Shahrazad, creating sub-subgames.

## Game Restarting

Some cards, like Karn Liberated, have the ability to restart the game. This differs from subgames in that the current game ends completely and a new game begins with modified starting conditions.

### Karn Liberated Implementation

Karn Liberated's ultimate ability (-14 loyalty) restarts the game, with all cards in exile that were exiled with Karn put into their owners' hands in the new game.

Key implementation details:
- The current game ends immediately
- A new game begins with players at their starting life totals
- Players draw new starting hands
- Cards exiled with Karn's ability are returned to their owners' hands
- All other cards return to their starting zones (library, command zone, etc.)

### Technical Implementation

Our restart implementation:
1. Tracks cards exiled with specific abilities like Karn's
2. When a restart ability is triggered, saves references to the tracked exile cards
3. Cleans up all game resources from the current game
4. Initializes a new game with standard starting conditions
5. Modifies the initial state to include returned exiled cards in their owners' hands

## Differences Between Subgames and Restarting

| Feature | Subgames | Game Restarting |
|---------|----------|-----------------|
| Original game state | Preserved | Ended completely |
| Players' life | Unchanged in main game | Reset to starting amount |
| Cards from old game | Only library cards | All cards return to starting zones |
| Continuity | Returns to main game when done | Old game ended, only some cards carried over |
| Implementation | Stack-based state management | Complete reinitialization |

## Integration with Game Engine

Both features leverage our snapshot system for state management:
- Subgames use the snapshot system to save and restore the main game state
- Game restarting uses snapshots to track exiled cards before reinitializing

See the [Snapshot System documentation](../core_systems/snapshot/index.md) for more details on how state is managed for these complex mechanics. 