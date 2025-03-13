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
    pub game_over: bool,
    pub winner: Option<Entity>,
    
    // Commander-specific tracking
    pub color_identity_enforced: bool, // For deck validation (rule 903.5c)
    pub commander_tax_enabled: bool, // Track commander tax (rule 903.8)
    pub commander_zone_transitions_enabled: bool, // Special command zone transitions (rule 903.9)
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
    
    // Zone transition tracking
    pub zone_history: HashMap<Entity, Vec<ZoneTransition>>,
}

#[derive(Clone, Debug)]
pub struct ZoneTransition {
    pub card: Entity,
    pub from_zone: Zone,
    pub to_zone: Zone,
    pub timestamp: std::time::Instant,
    pub turn_number: u32,
    pub is_commander: bool,
}
```

### Action History

```rust
#[derive(Resource)]
pub struct GameActionHistory {
    pub actions: Vec<GameAction>,
    pub current_turn_actions: Vec<GameAction>,
    pub commander_casts: HashMap<Entity, Vec<CommanderCast>>, // Tracks commander casts for tax
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

#[derive(Clone, Debug)]
pub struct CommanderCast {
    pub commander: Entity,
    pub cast_count: u32,
    pub tax_paid: u32,
    pub turn_number: u32,
    pub timestamp: std::time::Instant,
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
    game_state.starting_life = 40; // Rule 903.7
    game_state.commander_damage_threshold = 21; // Rule 903.10a
    game_state.color_identity_enforced = true; // Rule 903.5c
    game_state.commander_tax_enabled = true; // Rule 903.8
    game_state.commander_zone_transitions_enabled = true; // Rule 903.9
    
    // Initialize command zones for each player (will be done in a separate system)
}
```

### State Update System

```rust
fn update_game_state(
    mut game_state: ResMut<CommanderGameState>,
    players: Query<(Entity, &Player, &CommanderPlayer)>,
    time: Res<Time>,
) {
    // Check for game-ending conditions
    let active_players = players.iter()
        .filter(|(_, _, commander_player)| commander_player.life > 0 && !commander_player.has_lost)
        .count();
    
    if active_players <= 1 {
        game_state.game_over = true;
        
        // Find the winner if there is one
        for (entity, _, commander_player) in players.iter() {
            if commander_player.life > 0 && !commander_player.has_lost {
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
            ActionType::CastCommander => {
                // Handle commander casting with tax calculation (rule 903.8)
                if let Some(commander_entity) = action.cards.first() {
                    let player_entity = action.player;
                    
                    // Get current cast count or default to 0
                    let commander_casts = history.commander_casts
                        .entry(player_entity)
                        .or_insert_with(Vec::new);
                    
                    let cast_count = commander_casts.iter()
                        .filter(|cast| cast.commander == *commander_entity)
                        .count() as u32;
                    
                    // Calculate and record tax
                    let tax = 2 * cast_count; // Each previous cast adds {2} to cost
                    
                    commander_casts.push(CommanderCast {
                        commander: *commander_entity,
                        cast_count: cast_count + 1,
                        tax_paid: tax,
                        turn_number: game_state.turn_number,
                        timestamp: std::time::Instant::now(),
                    });
                    
                    // Move the commander from command zone to stack
                    let command_zone = zone_manager.command_zones.entry(player_entity).or_default();
                    if let Some(idx) = command_zone.iter().position(|e| e == commander_entity) {
                        let commander = command_zone.remove(idx);
                        
                        // Add to stack (simplified, actual stack implementation in stack.rs)
                        zone_manager.stack.push(StackItem {
                            entity: *commander_entity,
                            item_type: StackItemType::Spell(SpellType::Commander),
                            controller: player_entity,
                            source: *commander_entity,
                            source_zone: Zone::CommandZone,
                            targets: Vec::new(),
                            // Additional stack item properties...
                        });
                        
                        // Record zone transition
                        zone_manager.zone_history.entry(*commander_entity)
                            .or_insert_with(Vec::new)
                            .push(ZoneTransition {
                                card: *commander_entity,
                                from_zone: Zone::CommandZone,
                                to_zone: Zone::Stack,
                                timestamp: std::time::Instant::now(),
                                turn_number: game_state.turn_number,
                                is_commander: true,
                            });
                    }
                }
            },
            ActionType::CommanderDamage => {
                // Handle commander damage (rule 903.10a)
                // Implementation details in combat system
            },
            ActionType::CommanderZoneTransition => {
                // Handle commander returning to command zone (rule 903.9)
                // Implementation details in commander zone system
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
    // Include all Commander-specific state
}

pub fn deserialize_game_state(
    json_str: &str,
) -> Result<(CommanderGameState, ZoneManager, GameActionHistory), serde_json::Error> {
    // Deserialize from JSON to reconstruct the game state
}
```

## Integration Points

- **Player Management**: Tracks player entities and their commanders
- **Turn Structure**: Updates phase information and manages phase transitions
- **Stack System**: Coordinates with ZoneManager for stack operations
- **Command Zone**: Special handling for Commander cards and zone transitions
- **Combat System**: Tracks and enforces commander damage rules
- **UI System**: Provides state information for rendering

## Testing Strategy

1. **Unit Tests**:
   - Verify commander tax calculation
   - Test commander zone transitions
   - Validate commander damage tracking
   
2. **Integration Tests**:
   - Test multi-player commander game state
   - Verify color identity enforcement
   - Simulate special commander scenarios
   - Test state-based actions for commanders

## Performance Considerations

For games with many players (up to 13):
- Efficient entity lookups with spatial partitioning
- Batch processing of state updates
- Lazy evaluation of derived state information
- Optimized zone transition tracking 