# Game State Management

## Overview

The Game State Management module is responsible for tracking and maintaining the complete state of a Commander game. It serves as the central source of truth for all game data and coordinates the interactions between other modules.

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
    pub starting_life: u32,  // Typically 40 for Commander
    pub max_players: u8,     // Up to 13 supported
    pub commander_damage_threshold: u32,  // Typically 21
    
    // Game state flags
    pub game_over: bool,
    pub winner: Option<Entity>,
}
```

### Zone Management

```rust
#[derive(Resource)]
pub struct ZoneManager {
    // Maps player entities to their zones
    pub libraries: HashMap<Entity, Vec<Entity>>,
    pub hands: HashMap<Entity, Vec<Entity>>,
    pub graveyards: HashMap<Entity, Vec<Entity>>,
    pub exiles: HashMap<Entity, Vec<Entity>>,
    pub battlefields: HashMap<Entity, Vec<Entity>>,
    
    // Special Commander zones
    pub command_zones: HashMap<Entity, Vec<Entity>>,
    
    // Shared zones
    pub stack: Vec<StackItem>,
}
```

### Action History

```rust
#[derive(Resource)]
pub struct GameActionHistory {
    pub actions: Vec<GameAction>,
    pub current_turn_actions: Vec<GameAction>,
}

#[derive(Clone, Debug)]
pub struct GameAction {
    pub action_type: ActionType,
    pub player: Entity,
    pub timestamp: std::time::Instant,
    pub targets: Vec<Entity>,
    pub cards: Vec<Entity>,
    pub metadata: HashMap<String, String>,
}
```

## Key Systems

### State Initialization System

```rust
fn initialize_commander_game(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut game_state: ResMut<CommanderGameState>,
) {
    // Set up initial game state
    game_state.game_id = uuid::Uuid::new_v4();
    game_state.turn_number = 1;
    game_state.start_time = std::time::Instant::now();
    
    // Randomize player order
    let mut player_entities: Vec<Entity> = player_query.iter().collect();
    player_entities.shuffle(&mut rand::thread_rng());
    game_state.player_order = player_entities;
    
    // Set initial player as active
    game_state.active_player_index = 0;
    game_state.priority_holder = game_state.player_order[0];
    
    // Initialize starting phase
    game_state.current_phase = Phase::Beginning(BeginningStep::Untap);
    
    // Additional setup for Commander format
    // ...
}
```

### State Update System

```rust
fn update_game_state(
    mut game_state: ResMut<CommanderGameState>,
    players: Query<&Player>,
    time: Res<Time>,
) {
    // Check for game-ending conditions
    let active_players = players.iter()
        .filter(|p| p.life > 0 && !p.has_lost)
        .count();
    
    if active_players <= 1 {
        game_state.game_over = true;
        
        // Find the winner if there is one
        for (entity, player) in players.iter_entities() {
            if player.life > 0 && !player.has_lost {
                game_state.winner = Some(entity);
                break;
            }
        }
    }
    
    // Update other state variables as needed
}
```

## Action Dispatch System

The Game State module includes an event-based action dispatch system that other modules can use to modify the game state:

```rust
#[derive(Event)]
pub struct GameActionEvent {
    pub action: GameAction,
}

fn process_game_actions(
    mut commands: Commands,
    mut action_events: EventReader<GameActionEvent>,
    mut game_state: ResMut<CommanderGameState>,
    mut zone_manager: ResMut<ZoneManager>,
    mut history: ResMut<GameActionHistory>,
) {
    for event in action_events.read() {
        let action = &event.action;
        
        // Record the action in history
        history.actions.push(action.clone());
        history.current_turn_actions.push(action.clone());
        
        // Process the action based on its type
        match action.action_type {
            ActionType::DrawCard => {
                // Handle card drawing logic
            },
            ActionType::PlayLand => {
                // Handle land playing logic
            },
            // Handle other action types
            // ...
        }
    }
}
```

## State Persistence

For saving and loading game states:

```rust
pub fn serialize_game_state(
    game_state: &CommanderGameState,
    zone_manager: &ZoneManager,
    history: &GameActionHistory,
) -> Result<String, serde_json::Error> {
    // Serialize the complete game state to JSON
}

pub fn deserialize_game_state(
    json_str: &str,
) -> Result<(CommanderGameState, ZoneManager, GameActionHistory), serde_json::Error> {
    // Deserialize from JSON to reconstruct the game state
}
```

## Integration Points

- **Player Management**: Tracks player entities and their states
- **Turn Structure**: Updates phase information and manages phase transitions
- **Stack System**: Coordinates with ZoneManager for stack operations
- **Command Zone**: Special handling for Commander cards and zone transitions
- **UI System**: Provides state information for rendering

## Testing Strategy

1. **Unit Tests**:
   - Verify state transitions
   - Test game-ending condition detection
   - Validate persistence functionality
   
2. **Integration Tests**:
   - Test multi-player state management
   - Verify state consistency across complex operations
   - Simulate full game scenarios

## Performance Considerations

For games with many players (up to 13):
- Efficient entity lookups with spatial partitioning
- Batch processing of state updates
- Lazy evaluation of derived state information 