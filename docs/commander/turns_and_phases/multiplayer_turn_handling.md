# Multiplayer Turn Handling

## Overview

Commander's multiplayer nature introduces special considerations for turn management, particularly regarding turn order, player elimination, and simultaneous effects. This document outlines how the turn system handles these multiplayer-specific scenarios.

## Player Order Management

Turn order in Commander follows a clockwise rotation from a randomly determined starting player. The turn order is maintained in the `TurnManager`:

```rust
#[derive(Resource)]
pub struct TurnManager {
    // Other fields...
    pub player_order: Vec<Entity>,
}
```

At game initialization, the player order is randomized:

```rust
fn initialize_turn_order(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    let mut player_entities: Vec<Entity> = player_query.iter().collect();
    // Randomize the order
    player_entities.shuffle(&mut thread_rng());
    
    commands.insert_resource(TurnManager {
        player_order: player_entities,
        // Initialize other fields...
    });
}
```

## Player Elimination Handling

When a player is eliminated from the game, the turn system must adjust the turn order and potentially the active player and priority indicators:

```rust
fn handle_player_elimination(
    mut commands: Commands,
    mut elimination_events: EventReader<PlayerEliminationEvent>,
    mut turn_manager: ResMut<TurnManager>,
) {
    for event in elimination_events.read() {
        let eliminated_player = event.player;
        
        // Remove player from turn order
        if let Some(pos) = turn_manager.player_order.iter().position(|&p| p == eliminated_player) {
            turn_manager.player_order.remove(pos);
            
            // Adjust current active and priority indices if needed
            if pos <= turn_manager.active_player_index {
                turn_manager.active_player_index = 
                    (turn_manager.active_player_index - 1) % turn_manager.player_order.len();
            }
            
            if pos <= turn_manager.priority_player_index {
                turn_manager.priority_player_index = 
                    (turn_manager.priority_player_index - 1) % turn_manager.player_order.len();
            }
        }
    }
}
```

## Simultaneous Effects

In Commander, many effects can happen simultaneously across multiple players. The system handles these according to Magic's APNAP (Active Player, Non-Active Player) rule:

```rust
fn handle_simultaneous_effects(
    turn_manager: Res<TurnManager>,
    mut simultaneous_events: EventReader<SimultaneousEffectEvent>,
    mut commands: Commands,
) {
    let mut effect_queue = Vec::new();
    
    // Collect all simultaneous effects
    for event in simultaneous_events.read() {
        effect_queue.push((event.player, event.effect.clone()));
    }
    
    // Sort by APNAP order
    effect_queue.sort_by(|(player_a, _), (player_b, _)| {
        let active_player = turn_manager.player_order[turn_manager.active_player_index];
        
        if *player_a == active_player {
            return Ordering::Less;
        }
        if *player_b == active_player {
            return Ordering::Greater;
        }
        
        // For non-active players, compare positions in turn order
        let pos_a = turn_manager.player_order.iter().position(|&p| p == *player_a).unwrap();
        let pos_b = turn_manager.player_order.iter().position(|&p| p == *player_b).unwrap();
        pos_a.cmp(&pos_b)
    });
    
    // Process effects in sorted order
    for (player, effect) in effect_queue {
        // Process each effect...
    }
}
```

## Multi-Player Variants

The turn system supports different Commander variants:

- **Free-for-All**: Standard Commander with turn order passing clockwise
- **Two-Headed Giant**: Teams share turns and life totals 
- **Star**: Five players with win conditions based on eliminating opposing "colors"
- **Archenemy**: One player versus all others, with the Archenemy having access to scheme cards

Each variant may modify the `TurnManager` initialization and behavior:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVariant {
    FreeForAll,
    TwoHeadedGiant,
    Star,
    Archenemy,
    // Other variants
}

// The variant affects turn handling
pub fn initialize_turn_manager(
    variant: GameVariant,
    players: Vec<Entity>,
) -> TurnManager {
    match variant {
        GameVariant::TwoHeadedGiant => {
            // Team members share turns
            // ...
        },
        GameVariant::Archenemy => {
            // Archenemy goes first
            // ...
        },
        _ => {
            // Standard initialization
            // ...
        }
    }
}
```

## Multiplayer-Specific Considerations

The turn system accounts for multiplayer-specific mechanics:

- **Range of Influence**: Limited in some variants (like Star)
- **Shared Team Turns**: In variants like Two-Headed Giant
- **Table Politics**: Support for game actions like voting
- **Monarch**: Special designation that passes between players
- **Initiative**: Tracking which player has the initiative in Undercity dungeon scenarios 