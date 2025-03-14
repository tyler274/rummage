# Multiplayer Turns in Commander

This document explains how multiplayer turns are implemented in the Rummage game engine for the Commander format.

## Multiplayer Turn Structure

Commander is designed as a multiplayer format, typically played with 3-6 players. The multiplayer turn structure follows these rules:

1. Players take turns in clockwise order, starting from a randomly determined first player
2. Turn order remains fixed throughout the game unless modified by card effects
3. All standard turn phases occur during each player's turn
4. Players can act during other players' turns when they have priority
5. Some effects specifically reference "each player" or "each opponent" which have multiplayer implications

## Turn Order Implementation

```rust
/// Resource that tracks turn order
#[derive(Resource, Debug, Clone)]
pub struct TurnOrder {
    /// List of players in turn order
    pub players: Vec<Entity>,
    /// Current player index in the players list
    pub current_player_index: usize,
    /// Direction of turn progression (1 for clockwise, -1 for counter-clockwise)
    pub direction: i32,
}

impl TurnOrder {
    /// Creates a new turn order with the specified players
    pub fn new(players: Vec<Entity>) -> Self {
        Self {
            players,
            current_player_index: 0,
            direction: 1,
        }
    }
    
    /// Gets the current player
    pub fn current_player(&self) -> Entity {
        self.players[self.current_player_index]
    }
    
    /// Advances to the next player's turn
    pub fn advance(&mut self) {
        let len = self.players.len() as i32;
        self.current_player_index = (
            (self.current_player_index as i32 + self.direction).rem_euclid(len)
        ) as usize;
    }
    
    /// Reverses the turn order direction
    pub fn reverse(&mut self) {
        self.direction = -self.direction;
    }
    
    /// Shuffles the player order
    pub fn shuffle(&mut self, rng: &mut impl Rng) {
        self.players.shuffle(rng);
        // Maintain current player at index 0 after shuffle
        let current = self.current_player();
        let current_pos = self.players.iter().position(|&p| p == current).unwrap();
        self.players.swap(0, current_pos);
        self.current_player_index = 0;
    }
    
    /// Skips the next player's turn
    pub fn skip_next(&mut self) {
        self.advance();
    }
    
    /// Returns an iterator over all players starting from the current player
    pub fn iter_from_current(&self) -> impl Iterator<Item = Entity> + '_ {
        let len = self.players.len();
        (0..len).map(move |offset| {
            let index = (self.current_player_index + offset) % len;
            self.players[index]
        })
    }
    
    /// Returns an iterator over all opponents of the current player
    pub fn iter_opponents(&self) -> impl Iterator<Item = Entity> + '_ {
        let current = self.current_player();
        self.players.iter().copied().filter(move |&p| p != current)
    }
}
```

## Turn Advancement System

```rust
/// System that advances to the next player's turn
pub fn advance_turn(
    mut turn_order: ResMut<TurnOrder>,
    mut turn_phase: ResMut<TurnPhase>,
    mut turn_events: EventWriter<TurnEvent>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if game_state.current_state != GameStateType::EndOfTurn {
        return;
    }
    
    // Get current player before advancing
    let previous_player = turn_order.current_player();
    
    // Advance to next player
    turn_order.advance();
    let next_player = turn_order.current_player();
    
    // Reset turn phase to beginning of turn
    *turn_phase = TurnPhase::Beginning(BeginningPhase::Untap);
    
    // Update game state
    game_state.current_state = GameStateType::ActiveTurn;
    game_state.active_player = next_player;
    
    // Send events
    turn_events.send(TurnEvent::TurnEnded { player: previous_player });
    turn_events.send(TurnEvent::TurnBegan { player: next_player });
}
```

## Multiplayer-Specific Mechanics

### Turn-Based Effects

Some effects need to track when players take turns:

```rust
/// Component for effects that trigger at the beginning of a player's turn
#[derive(Component, Debug, Clone)]
pub struct BeginningOfTurnTrigger {
    /// Whether this triggers on the controller's turn only
    pub controller_only: bool,
    /// Whether this triggers on opponents' turns only
    pub opponents_only: bool,
    /// Function to execute when triggered
    pub effect: fn(Commands, Entity) -> (),
}

/// System that handles beginning of turn triggers
pub fn handle_beginning_of_turn_triggers(
    mut commands: Commands,
    turn_order: Res<TurnOrder>,
    turn_phase: Res<TurnPhase>,
    triggers: Query<(Entity, &BeginningOfTurnTrigger, &Owner)>,
) {
    // Only execute during the beginning of turn phase
    if !matches!(turn_phase.as_ref(), TurnPhase::Beginning(BeginningPhase::Upkeep)) {
        return;
    }
    
    let current_player = turn_order.current_player();
    
    for (entity, trigger, owner) in triggers.iter() {
        let is_owner_turn = owner.0 == current_player;
        
        // Check if trigger condition is met
        if (trigger.controller_only && is_owner_turn) ||
           (trigger.opponents_only && !is_owner_turn) ||
           (!trigger.controller_only && !trigger.opponents_only) {
            (trigger.effect)(commands, entity);
        }
    }
}
```

### Simultaneous Effects

In multiplayer games, effects that affect all players need special handling:

```rust
/// Handles effects that apply to all players simultaneously
pub fn handle_global_effects(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    global_effects: Query<(Entity, &GlobalEffect)>,
) {
    for (effect_entity, global_effect) in global_effects.iter() {
        match global_effect.target_type {
            GlobalTargetType::AllPlayers => {
                for player in players.iter() {
                    (global_effect.apply_effect)(commands, effect_entity, player);
                }
            },
            GlobalTargetType::AllOpponents => {
                let turn_order = commands.world().resource::<TurnOrder>();
                let current_player = turn_order.current_player();
                
                for player in players.iter() {
                    if player != current_player {
                        (global_effect.apply_effect)(commands, effect_entity, player);
                    }
                }
            },
            // Other global target types...
        }
    }
}
```

## Special Turn Interactions

### Extra Turns

Commander allows for extra turn effects:

```rust
#[derive(Resource, Debug)]
pub struct ExtraTurns {
    /// Queue of players taking extra turns
    pub queue: VecDeque<Entity>,
}

/// System that handles extra turn insertion
pub fn handle_extra_turns(
    mut turn_order: ResMut<TurnOrder>,
    mut extra_turns: ResMut<ExtraTurns>,
    mut turn_events: EventWriter<TurnEvent>,
) {
    if let Some(player) = extra_turns.queue.pop_front() {
        // Current player takes an extra turn
        let current = turn_order.current_player();
        if player == current {
            turn_events.send(TurnEvent::ExtraTurn { player });
        } else {
            // Another player takes the next turn
            // Temporarily modify turn order
            let current_index = turn_order.current_player_index;
            let player_index = turn_order.players.iter().position(|&p| p == player).unwrap();
            turn_order.current_player_index = player_index;
            turn_events.send(TurnEvent::ExtraTurn { player });
            
            // Store original order to restore after the extra turn
            commands.insert_resource(PendingTurnRestoration {
                restore_index: current_index,
            });
        }
    }
}
```

### Turn Modifications

Some cards can modify turn order or skip turns:

```rust
/// Event for turn order modifications
#[derive(Event, Debug, Clone)]
pub enum TurnOrderEvent {
    /// Reverses turn order direction
    Reverse,
    /// Skips the next player's turn
    SkipNext,
    /// Takes an extra turn after the current one
    ExtraTurn { player: Entity },
    /// Exchanges turns between players
    ExchangeTurn { player_a: Entity, player_b: Entity },
}

/// System that handles turn order modifications
pub fn handle_turn_modifications(
    mut turn_order: ResMut<TurnOrder>,
    mut extra_turns: ResMut<ExtraTurns>,
    mut turn_events: EventReader<TurnOrderEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in turn_events.read() {
        match event {
            TurnOrderEvent::Reverse => {
                turn_order.reverse();
                game_events.send(GameEvent::TurnOrderReversed);
            },
            TurnOrderEvent::SkipNext => {
                turn_order.skip_next();
                game_events.send(GameEvent::TurnSkipped);
            },
            TurnOrderEvent::ExtraTurn { player } => {
                extra_turns.queue.push_back(*player);
                game_events.send(GameEvent::ExtraTurnAdded { player: *player });
            },
            TurnOrderEvent::ExchangeTurn { player_a, player_b } => {
                let pos_a = turn_order.players.iter().position(|&p| p == *player_a).unwrap();
                let pos_b = turn_order.players.iter().position(|&p| p == *player_b).unwrap();
                turn_order.players.swap(pos_a, pos_b);
                game_events.send(GameEvent::TurnOrderModified);
            },
        }
    }
}
```

## UI Representation

Multiplayer turns are visually represented in the UI:

1. **Turn Order Display**: Shows all players in order with the current player highlighted
2. **Active Player Indicator**: Clearly highlights whose turn it is
3. **Turn Direction Indicator**: Shows the current direction of play (clockwise/counter-clockwise)
4. **Extra Turn Queue**: Displays any pending extra turns
5. **Phase Tracker**: Shows the current phase of the active player's turn

## Testing

### Example Test

```rust
#[test]
fn test_multiplayer_turn_order() {
    // Create test app with required systems
    let mut app = App::new();
    app.add_systems(Update, advance_turn)
        .add_event::<TurnEvent>()
        .init_resource::<GameState>();
    
    // Create four players for a typical 4-player game
    let player1 = app.world.spawn_empty().id();
    let player2 = app.world.spawn_empty().id();
    let player3 = app.world.spawn_empty().id();
    let player4 = app.world.spawn_empty().id();
    
    let players = vec![player1, player2, player3, player4];
    
    // Initialize turn order
    app.insert_resource(TurnOrder::new(players.clone()));
    app.insert_resource(TurnPhase::Ending(EndingPhase::End));
    
    // Set game state to end of turn to trigger advancement
    let mut game_state = GameState::default();
    game_state.current_state = GameStateType::EndOfTurn;
    app.insert_resource(game_state);
    
    // First turn should be player1
    let turn_order = app.world.resource::<TurnOrder>();
    assert_eq!(turn_order.current_player(), player1);
    
    // Advance turn
    app.update();
    
    // Should now be player2's turn
    let turn_order = app.world.resource::<TurnOrder>();
    assert_eq!(turn_order.current_player(), player2);
    
    // Test turn events
    let events = app.world.resource::<Events<TurnEvent>>();
    let mut reader = events.get_reader();
    
    let mut saw_end = false;
    let mut saw_begin = false;
    
    for event in reader.read(&events) {
        match event {
            TurnEvent::TurnEnded { player } => {
                assert_eq!(*player, player1);
                saw_end = true;
            },
            TurnEvent::TurnBegan { player } => {
                assert_eq!(*player, player2);
                saw_begin = true;
            },
            _ => {}
        }
    }
    
    assert!(saw_end, "Should have seen turn ended event");
    assert!(saw_begin, "Should have seen turn began event");
}

#[test]
fn test_reversed_turn_order() {
    // Similar test setup
    let mut app = App::new();
    // ...
    
    // Insert reversed turn order
    let mut turn_order = TurnOrder::new(players.clone());
    turn_order.reverse(); // Direction becomes -1
    app.insert_resource(turn_order);
    
    // Advance from player1
    app.update();
    
    // Should now be player4's turn (counter-clockwise)
    let turn_order = app.world.resource::<TurnOrder>();
    assert_eq!(turn_order.current_player(), player4);
}
```

## Summary

Multiplayer turns in Commander are implemented with a flexible system that:

1. Maintains proper turn order in multiplayer games
2. Supports direction changes (clockwise/counter-clockwise)
3. Handles extra turns and turn skipping
4. Processes effects that affect multiple players
5. Provides clear UI indicators for turn progression
6. Is thoroughly tested for all turn modification scenarios 