# Subgames and Game Restarting Implementation

This document outlines the technical implementation of subgames (like those created by Shahrazad) and game restarting mechanics (like Karn Liberated's ultimate ability) within our engine.

## Component Design

The implementation uses several key components and resources to manage subgames and game restarting:

```rust
// Tracks whether we're currently in a subgame
#[derive(Resource)]
pub struct SubgameState {
    /// Stack of game states, with the main game at the bottom
    pub game_stack: Vec<GameSnapshot>,
    /// Current subgame depth (0 = main game)
    pub depth: usize,
}

// Marks cards that were exiled by Karn Liberated
#[derive(Component)]
pub struct ExiledWithKarn;

// Component that can restart the game
#[derive(Component)]
pub struct GameRestarter {
    pub condition: GameRestarterCondition,
}

// Enum defining what triggers game restart
pub enum GameRestarterCondition {
    KarnUltimate,
    Custom(Box<dyn Fn(&World) -> bool + Send + Sync + 'static>),
}
```

## Subgame Implementation

Subgames are implemented using a stack-based approach that leverages our snapshot system:

### Starting a Subgame

When a card like Shahrazad resolves:

1. The current game state is captured via the snapshot system
2. The snapshot is pushed onto the `game_stack` in `SubgameState`
3. The depth counter is incremented
4. A new game state is initialized with the appropriate starting conditions
5. Players' libraries from the main game become their decks in the subgame

```rust
fn start_subgame(
    mut commands: Commands,
    mut subgame_state: ResMut<SubgameState>,
    snapshot_system: Res<SnapshotSystem>,
    // Other dependencies...
) {
    // Take snapshot of current game
    let current_game = snapshot_system.capture_game_state();
    
    // Push to stack and increment depth
    subgame_state.game_stack.push(current_game);
    subgame_state.depth += 1;
    
    // Initialize new game state for subgame...
    // Transfer libraries from parent game to new subgame...
}
```

### Ending a Subgame

When a subgame concludes:

1. The result of the subgame is determined (winner/loser)
2. The most recent snapshot is popped from the `game_stack`
3. The depth counter is decremented
4. The main game state is restored from the snapshot
5. Any effects from the subgame are applied to the main game (loser loses half life)

```rust
fn end_subgame(
    mut commands: Commands,
    mut subgame_state: ResMut<SubgameState>,
    snapshot_system: Res<SnapshotSystem>,
    game_result: Res<SubgameResult>,
    // Other dependencies...
) {
    // Pop game state and decrement depth
    let parent_game = subgame_state.game_stack.pop().unwrap();
    subgame_state.depth -= 1;
    
    // Restore parent game state
    snapshot_system.restore_game_state(parent_game);
    
    // Apply subgame results to parent game
    // (loser loses half their life, rounded up)...
}
```

## Game Restarting Implementation

Game restarting is a complete reinitialization of the game state with some information carried over:

### Tracking Exiled Cards

Cards exiled with abilities like Karn Liberated's are marked with special components:

```rust
fn track_karn_exile(
    mut commands: Commands,
    karn_query: Query<Entity, With<KarnLiberated>>,
    exile_events: EventReader<CardExiledEvent>,
) {
    for event in exile_events.read() {
        if event.source_ability == AbilityType::KarnExile {
            commands.entity(event.card_entity).insert(ExiledWithKarn);
        }
    }
}
```

### Restarting the Game

When a game restart ability triggers:

1. Cards exiled with Karn are identified and stored in a temporary resource
2. All existing game resources are cleaned up
3. A new game is initialized with standard starting conditions
4. The exiled cards are placed in their owners' hands in the new game

```rust
fn restart_game(
    mut commands: Commands,
    restart_events: EventReader<GameRestartEvent>,
    exiled_cards: Query<(Entity, &Owner), With<ExiledWithKarn>>,
    // Other dependencies...
) {
    if restart_events.is_empty() {
        return;
    }
    
    // Store information about exiled cards
    let cards_to_return = exiled_cards
        .iter()
        .map(|(entity, owner)| (entity, owner.0))
        .collect::<Vec<_>>();
    
    // Clean up existing game state...
    
    // Initialize new game...
    
    // Return exiled cards to their owners' hands
    for (card_entity, owner_id) in cards_to_return {
        // Move card to owner's hand in the new game
        // ...
    }
}
```

## Interacting with Game Systems

Both subgames and game restarting interact with several core systems:

### Snapshot System Integration

The [Snapshot System](../../core_systems/snapshot/index.md) is crucial for both features:
- Subgames use it to preserve and restore game states
- Game restarting uses it to track information that needs to persist through restarts

### Turn System Integration

The turn system needs special handling for subgames:
- Subgames have their own turn structure independent of the main game
- When returning from a subgame, the turn structure of the main game must be properly restored

### Zone Management

Both features require special zone handling:
- Subgames need to track cards that leave the subgame
- Game restarting needs to move cards to their proper starting zones

## Error Handling and Edge Cases

The implementation includes handling for various edge cases:

- Nested subgames (subgames within subgames)
- Game restarts during a subgame
- Subgame creation during a game restart
- Corrupted state recovery
- Performance considerations for deep subgame nesting

See the [MTG Rules documentation](../../mtg_rules/subgames.md) for more information on the rules governing these mechanics. 