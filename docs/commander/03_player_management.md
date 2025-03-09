# Player Management

## Overview

The Player Management module handles all player-specific state and actions in Commander games. It's responsible for tracking player life totals, commander damage, resource management, and player-specific zones.

## Core Components

### Enhanced Player Component

```rust
#[derive(Component, Debug, Clone)]
pub struct CommanderPlayer {
    // Basic player information
    pub name: String,
    pub player_id: uuid::Uuid,
    
    // Life and game status
    pub life: i32,      // Starts at 40 in Commander
    pub has_lost: bool,
    pub win_condition: Option<WinCondition>,
    
    // Commander-specific tracking
    pub commander_entities: Vec<Entity>,  // Can have multiple with Partner
    pub commander_casts: HashMap<Entity, u32>,  // Tracks commander tax
    pub commander_in_command_zone: Vec<bool>,
    
    // Commander damage tracking (maps player entity to damage received)
    pub commander_damage_received: HashMap<Entity, u32>,  
    
    // Resource tracking
    pub mana_pool: ManaPool,
    pub lands_played_this_turn: u32,
    
    // Turn information
    pub has_drawn_for_turn: bool,
    
    // Special counters and markers
    pub is_monarch: bool,
    pub has_initiative: bool,
    pub experience_counters: u32,
    pub energy_counters: u32,
    
    // Multiplayer politics
    pub can_be_attacked: bool,  // For effects like Propaganda
    pub vote_modifiers: i32,    // For Council's Judgment and similar
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinCondition {
    LastPlayerStanding,
    InfiniteLife,
    LabManiac,      // Laboratory Maniac effect
    Approach,       // Approach of the Second Sun
    Poison,         // 10+ poison counters
    Coalition,      // Coalition Victory
    DoorToDoor,     // Door to Nothingness
    TreasureMaze,   // Maze's End
    Custom(String), // Custom win condition from other cards
}
```

### Commander Tax Tracker

```rust
pub fn calculate_commander_tax(player: &CommanderPlayer, commander_entity: Entity) -> u32 {
    // Each time a commander has been cast from the command zone, it costs 2 more
    let casts = player.commander_casts.get(&commander_entity).unwrap_or(&0);
    2 * casts
}
```

### Color Identity System

```rust
#[derive(Resource)]
pub struct ColorIdentityTracker {
    // Maps player to allowed colors in their deck based on Commander
    pub player_color_identities: HashMap<Entity, ColorIdentity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorIdentity {
    pub white: bool,
    pub blue: bool,
    pub black: bool,
    pub red: bool,
    pub green: bool,
}

impl ColorIdentity {
    // Check if a card is legal in this color identity
    pub fn can_include_card(&self, card: &Card) -> bool {
        // Card must not have colors outside commander's color identity
        if (card.cost.white > 0 && !self.white) ||
           (card.cost.blue > 0 && !self.blue) ||
           (card.cost.black > 0 && !self.black) ||
           (card.cost.red > 0 && !self.red) ||
           (card.cost.green > 0 && !self.green) {
            return false;
        }
        
        // Check rules text for mana symbols
        // (Simplified - full implementation needs to parse rules text)
        
        true
    }
}
```

## Key Systems

### Player Setup System

```rust
fn setup_commander_players(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player)>,
    mut game_state: ResMut<CommanderGameState>,
) {
    for (entity, mut player) in player_query.iter_mut() {
        // Initialize Commander player with starting life total
        let commander_player = CommanderPlayer {
            name: player.name.clone(),
            player_id: uuid::Uuid::new_v4(),
            life: game_state.starting_life as i32,
            has_lost: false,
            win_condition: None,
            commander_entities: Vec::new(),  // Will be filled later
            commander_casts: HashMap::new(),
            commander_in_command_zone: Vec::new(),
            commander_damage_received: HashMap::new(),
            mana_pool: ManaPool::default(),
            lands_played_this_turn: 0,
            has_drawn_for_turn: false,
            is_monarch: false,
            has_initiative: false,
            experience_counters: 0,
            energy_counters: 0,
            can_be_attacked: true,
            vote_modifiers: 0,
        };
        
        // Add the commander player component
        commands.entity(entity).insert(commander_player);
    }
}
```

### Commander Damage System

```rust
fn check_commander_damage(
    mut commands: Commands,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    mut game_events: EventWriter<GameEvent>,
) {
    for (entity, mut player) in players.iter_mut() {
        // Check if any commander has dealt 21+ damage
        for (source_player, damage) in player.commander_damage_received.iter() {
            if *damage >= 21 {
                player.has_lost = true;
                game_events.send(GameEvent::PlayerEliminated {
                    player: entity,
                    reason: EliminationReason::CommanderDamage(*source_player),
                });
                break;
            }
        }
    }
}
```

### Life Change System

```rust
fn process_life_changes(
    mut life_change_events: EventReader<LifeChangeEvent>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in life_change_events.read() {
        if let Ok((entity, mut player)) = players.get_mut(event.player) {
            // Update life total
            player.life += event.amount;
            
            // Check for death by life loss
            if player.life <= 0 {
                player.has_lost = true;
                game_events.send(GameEvent::PlayerEliminated {
                    player: entity,
                    reason: EliminationReason::LifeLoss,
                });
            }
        }
    }
}
```

### Commander Casting System

```rust
fn handle_commander_cast(
    mut commands: Commands,
    mut cast_events: EventReader<CommanderCastEvent>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
    mut zone_manager: ResMut<ZoneManager>,
) {
    for event in cast_events.read() {
        if let Ok((player_entity, mut player)) = players.get_mut(event.player) {
            // Update cast count for commander tax
            let commander_entity = event.commander;
            let cast_count = player.commander_casts.entry(commander_entity).or_insert(0);
            *cast_count += 1;
            
            // Update command zone status
            if let Some(index) = player.commander_entities.iter().position(|&e| e == commander_entity) {
                player.commander_in_command_zone[index] = false;
            }
            
            // Move commander from command zone to stack
            let command_zone = zone_manager.command_zones.get_mut(&player_entity).unwrap();
            if let Some(pos) = command_zone.iter().position(|&e| e == commander_entity) {
                command_zone.swap_remove(pos);
                zone_manager.stack.push(StackItem {
                    effect: Box::new(CastCommanderEffect {
                        commander: commander_entity,
                        owner: player_entity,
                    }),
                    controller: player_entity,
                    targets: vec![],
                });
            }
        }
    }
}
```

## Politics and Multiplayer Mechanics

```rust
#[derive(Event)]
pub struct MultipayerPoliticalEvent {
    pub event_type: PoliticalEventType,
    pub source_player: Entity,
    pub target_players: Vec<Entity>,
    pub card_source: Option<Entity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PoliticalEventType {
    StartVote { topic: String, options: Vec<String> },
    OfferDeal { deal_terms: String },
    AcceptDeal,
    RejectDeal,
    BecomeMonarch,
    GainInitiative,
}
```

## Integration Points

- **Game State Module**: Registers player entities and manages transitions
- **Command Zone Module**: Manages commander placement and movement
- **Combat System**: Processes combat damage including commander damage
- **Turn Structure**: Manages player turns and processes player actions

## Testing Strategy

1. **Unit Tests**:
   - Test commander damage calculation
   - Verify life total management
   - Test color identity enforcement

2. **Integration Tests**:
   - Test interaction between player state changes and game state
   - Verify commander movement between zones
   - Test multiplayer political mechanics

## Design Considerations

- **Scalability**: Support for up to 13 players requires efficient data structures
- **Separation of Concerns**: Clear division between player state and game logic
- **UI Integration**: Expose player state information for rendering player boards
- **Multiplayer Mechanics**: Handle political interactions and deals between players 