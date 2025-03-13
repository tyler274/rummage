# Game State Management

## Overview

The Game State Management module is responsible for tracking and maintaining the complete state of a Commander game. It serves as the central source of truth for all game data and coordinates the interactions between other modules, ensuring rules compliance according to the Magic: The Gathering Comprehensive Rules section 903.

## Core Components

### GameState Resource

```rust
#[derive(Resource)]
pub struct CommanderGameState {
    // Game metadata
    pub game_id: uuid::Uuid,
    pub turn_number: u32,
    pub start_time: std::time::Instant,
    
    // Active player and turn state
    pub active_player_index: usize,
    pub player_order: Vec<Entity>,
    pub current_phase: Phase,
    pub priority_holder: Entity,
    
    // Game parameters
    pub starting_life: u32,  // 40 for Commander (rule 903.7)
    pub max_players: u8,     // Up to 13 supported
    pub commander_damage_threshold: u32,  // 21 (rule 903.10a)
    
    // Game state flags
    pub is_game_over: bool,
    pub winner: Option<Entity>,
    
    // Special Commander format states
    pub commander_cast_count: HashMap<Entity, u32>,  // For commander tax
    pub eliminated_players: Vec<Entity>,
}
```

### State Tracking Components

```rust
#[derive(Component)]
pub struct CommanderGameParticipant {
    pub player_index: usize,
    pub is_active: bool,
}

#[derive(Component)]
pub struct TurnOrderPosition {
    pub position: usize,
    pub is_skipping_turn: bool,
}

#[derive(Component, Default)]
pub struct GameStateFlags {
    pub has_played_land: bool,
    pub commanders_in_play: HashSet<Entity>,
    pub has_attacked_this_turn: bool,
}
```

## Key Systems

### Game State Initialization

```rust
pub fn initialize_commander_game(
    mut commands: Commands,
    mut game_state: ResMut<CommanderGameState>,
    players: Query<Entity, With<Player>>,
) {
    // Set up initial game state
    game_state.game_id = uuid::Uuid::new_v4();
    game_state.turn_number = 1;
    game_state.start_time = std::time::Instant::now();
    
    // Set up player order (randomized)
    let mut player_entities = players.iter().collect::<Vec<_>>();
    player_entities.shuffle(&mut rand::thread_rng());
    game_state.player_order = player_entities;
    
    // Set initial active player
    game_state.active_player_index = 0;
    game_state.priority_holder = player_entities[0];
    
    // Set starting phase
    game_state.current_phase = Phase::Beginning(BeginningPhaseStep::Untap);
    
    // Initialize Commander-specific parameters
    game_state.starting_life = 40;  // Commander rule 903.7
    game_state.commander_damage_threshold = 21;  // Commander rule 903.10a
    
    // Tag entities with player index
    for (idx, entity) in player_entities.iter().enumerate() {
        commands.entity(*entity).insert(CommanderGameParticipant {
            player_index: idx,
            is_active: idx == 0,
        });
        
        commands.entity(*entity).insert(TurnOrderPosition {
            position: idx,
            is_skipping_turn: false,
        });
        
        commands.entity(*entity).insert(GameStateFlags::default());
    }
}
```

### Game State Update System

```rust
pub fn update_game_state(
    mut game_state: ResMut<CommanderGameState>,
    mut players: Query<(Entity, &mut CommanderGameParticipant, &PlayerState)>,
    time: Res<Time>,
) {
    // Check for eliminated players
    for (entity, participant, state) in players.iter() {
        if state.life_total <= 0 && !game_state.eliminated_players.contains(&entity) {
            game_state.eliminated_players.push(entity);
        }
    }
    
    // Check for game over condition
    let remaining_players = players
        .iter()
        .filter(|(entity, _, _)| !game_state.eliminated_players.contains(&entity))
        .count();
    
    if remaining_players <= 1 {
        game_state.is_game_over = true;
        if remaining_players == 1 {
            let winner = players
                .iter()
                .find(|(entity, _, _)| !game_state.eliminated_players.contains(&entity))
                .map(|(entity, _, _)| entity);
            game_state.winner = winner;
        }
    }
}
```

## State Validation

The game state management module includes validation functions to ensure game state consistency:

```rust
pub fn validate_game_state(game_state: &CommanderGameState) -> Result<(), String> {
    // Validate player count
    if game_state.player_order.is_empty() {
        return Err("Game must have at least one player".to_string());
    }
    
    if game_state.player_order.len() > game_state.max_players as usize {
        return Err(format!("Game cannot have more than {} players", game_state.max_players));
    }
    
    // Validate active player
    if game_state.active_player_index >= game_state.player_order.len() {
        return Err("Active player index out of bounds".to_string());
    }
    
    // More validations...
    
    Ok(())
}
```

## Integration Points

The game state module integrates with other systems through:

1. **Resource access** - Other systems can read the game state
2. **Events** - Game state changes trigger events for other systems
3. **Commands** - Game state can be updated through commands

This module forms the foundation of the Commander game engine, providing the central state management needed to coordinate all other game systems. 