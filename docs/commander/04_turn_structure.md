# Turn Structure and Phases

## Overview

The Turn Structure module manages the flow of a Commander game, handling the sequence of phases and steps within each player's turn. It coordinates player transitions, priority passing, and phase-specific actions while accounting for the multiplayer nature of Commander.

## Core Turn Structure

### Phase and Step Definitions

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Beginning(BeginningStep),
    Precombat(PrecombatStep),
    Combat(CombatStep),
    Postcombat(PostcombatStep),
    Ending(EndingStep),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecombatStep {
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostcombatStep {
    Main,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndingStep {
    End,
    Cleanup,
}
```

### Turn Manager

```rust
#[derive(Resource)]
pub struct TurnManager {
    // Current turn information
    pub current_phase: Phase,
    pub active_player_index: usize,
    pub turn_number: u32,
    
    // Priority system
    pub priority_player_index: usize,
    pub all_players_passed: bool,
    pub stack_is_empty: bool,
    
    // Step tracking
    pub step_started_at: std::time::Instant,
    pub auto_pass_enabled: bool,
    pub auto_pass_delay: std::time::Duration,
    
    // Multiplayer tracking
    pub player_order: Vec<Entity>,
    pub extra_turns: VecDeque<(Entity, ExtraTurnSource)>,
    pub skipped_turns: HashSet<Entity>,
}

#[derive(Debug, Clone)]
pub struct ExtraTurnSource {
    pub source_card: Entity,
    pub extra_turn_type: ExtraTurnType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtraTurnType {
    Standard,
    CombatOnly,
    WithRestrictions(Vec<TurnRestriction>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnRestriction {
    NoUntap,
    NoUpkeep,
    NoDraw,
    NoMainPhase,
    NoCombat,
    MaxSpells(u32),
    // Other restrictions...
}
```

## Key Systems

### Phase Transition System

```rust
fn phase_transition_system(
    mut commands: Commands,
    time: Res<Time>,
    mut turn_manager: ResMut<TurnManager>,
    mut phase_events: EventWriter<PhaseTransitionEvent>,
    mut game_state: ResMut<CommanderGameState>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    let current_phase = turn_manager.current_phase;
    
    // Check if conditions are met to progress to next phase
    if turn_manager.all_players_passed && turn_manager.stack_is_empty {
        // Time to move to the next phase/step
        let next_phase = get_next_phase(current_phase);
        
        // Record phase transition
        phase_events.send(PhaseTransitionEvent {
            from: current_phase,
            to: next_phase,
            active_player: turn_manager.player_order[turn_manager.active_player_index],
        });
        
        // Update turn manager
        turn_manager.current_phase = next_phase;
        turn_manager.all_players_passed = false;
        
        // For phases that change active player
        if matches!(current_phase, Phase::Ending(EndingStep::Cleanup)) && 
           turn_manager.extra_turns.is_empty() {
            // Move to next player
            turn_manager.active_player_index = 
                (turn_manager.active_player_index + 1) % turn_manager.player_order.len();
            turn_manager.turn_number += 1;
        } else if !turn_manager.extra_turns.is_empty() && 
                  matches!(current_phase, Phase::Ending(EndingStep::Cleanup)) {
            // Handle extra turns
            let (player, _) = turn_manager.extra_turns.pop_front().unwrap();
            
            // Find player index
            if let Some(index) = turn_manager.player_order.iter().position(|&p| p == player) {
                turn_manager.active_player_index = index;
            }
            
            turn_manager.turn_number += 1;
        }
        
        // Prepare for first step of the new phase
        prepare_phase(
            &mut commands,
            &mut turn_manager,
            &mut game_state,
            &players,
        );
    }
}
```

### Phase Preparation System

```rust
fn prepare_phase(
    commands: &mut Commands,
    turn_manager: &mut TurnManager,
    game_state: &mut CommanderGameState,
    players: &Query<(Entity, &CommanderPlayer)>,
) {
    // Set priority to active player by default
    turn_manager.priority_player_index = turn_manager.active_player_index;
    
    // Record phase start time
    turn_manager.step_started_at = std::time::Instant::now();
    
    // Phase-specific setup
    match turn_manager.current_phase {
        Phase::Beginning(BeginningStep::Untap) => {
            // Untap step doesn't use priority, auto-progress
            turn_manager.all_players_passed = true;
            
            // Perform untap actions
            let active_player = turn_manager.player_order[turn_manager.active_player_index];
            commands.spawn(UntapStepEvent { player: active_player });
        },
        Phase::Beginning(BeginningStep::Upkeep) => {
            // Trigger "at beginning of upkeep" abilities
            let active_player = turn_manager.player_order[turn_manager.active_player_index];
            commands.spawn(UpkeepTriggerEvent { player: active_player });
        },
        Phase::Beginning(BeginningStep::Draw) => {
            // Handle the active player's draw
            let active_player = turn_manager.player_order[turn_manager.active_player_index];
            commands.spawn(DrawStepEvent { player: active_player });
        },
        Phase::Combat(CombatStep::Beginning) => {
            // Reset combat-related flags 
            game_state.creatures_attacking = HashSet::new();
            game_state.creatures_blocking = HashMap::new();
        },
        // Setup for other phases...
        _ => {}
    }
}
```

### Priority System

```rust
fn priority_system(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    stack: Res<GameStack>,
    mut priority_events: EventReader<PriorityEvent>,
    time: Res<Time>,
) {
    // Handle explicit priority passes
    for event in priority_events.read() {
        match event {
            PriorityEvent::Pass(player) => {
                // Skip to next player if current priority holder passes
                if *player == turn_manager.player_order[turn_manager.priority_player_index] {
                    advance_priority(&mut turn_manager);
                }
            },
            PriorityEvent::Take(player) => {
                // Action is being taken, reset the all_players_passed flag
                turn_manager.all_players_passed = false;
            },
        }
    }
    
    // Handle auto-pass if enabled and time threshold is met
    if turn_manager.auto_pass_enabled && 
       time.elapsed_seconds() - turn_manager.step_started_at.elapsed().as_secs_f32() > 
       turn_manager.auto_pass_delay.as_secs_f32() {
        
        advance_priority(&mut turn_manager);
    }
    
    // Check if the stack has changed
    turn_manager.stack_is_empty = stack.items.is_empty();
}

fn advance_priority(turn_manager: &mut TurnManager) {
    // Move to the next player in turn order
    let player_count = turn_manager.player_order.len();
    turn_manager.priority_player_index = (turn_manager.priority_player_index + 1) % player_count;
    
    // Check if we've gone full circle
    if turn_manager.priority_player_index == turn_manager.active_player_index {
        turn_manager.all_players_passed = true;
    }
}
```

### Special Turn Rules

Commander has special rules for the first turn(s) of the game:

```rust
fn initialize_first_turn(
    mut turn_manager: ResMut<TurnManager>,
    game_state: Res<CommanderGameState>,
) {
    // In Commander, the first player doesn't draw on their first turn
    if turn_manager.turn_number == 1 {
        // Set flag to skip first player's draw
        turn_manager.skip_first_draw = true;
    }
}

fn handle_draw_step(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    mut players: Query<(Entity, &mut CommanderPlayer)>,
) {
    let active_player = turn_manager.player_order[turn_manager.active_player_index];
    
    // Skip draw for first player on turn 1
    if turn_manager.turn_number == 1 && 
       turn_manager.active_player_index == 0 && 
       turn_manager.skip_first_draw {
        return;
    }
    
    // Otherwise process normal draw
    if let Ok((entity, mut player)) = players.get_mut(active_player) {
        player.has_drawn_for_turn = true;
        commands.spawn(DrawCardEvent {
            player: entity,
            amount: 1,
        });
    }
}
```

## Integration with Multiplayer Systems

For multiplayer Commander games, turn order and player elimination handling:

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

## Turn Modification Rules

For handling effects that modify turn structure (extra turns, skipped phases, etc.):

```rust
fn process_turn_modifications(
    mut turn_manager: ResMut<TurnManager>,
    mut turn_mod_events: EventReader<TurnModificationEvent>,
) {
    for event in turn_mod_events.read() {
        match event {
            TurnModificationEvent::ExtraTurn { player, source } => {
                // Add to extra turns queue
                turn_manager.extra_turns.push_back((player, source.clone()));
            },
            TurnModificationEvent::SkipPhase { player, phase } => {
                // Mark phase to be skipped
                if let Some(index) = turn_manager.player_order.iter().position(|&p| p == player) {
                    turn_manager.phase_skips.entry(index)
                       .or_insert_with(HashSet::new)
                       .insert(*phase);
                }
            },
            TurnModificationEvent::SkipStep { player, step } => {
                // Mark step to be skipped
                if let Some(index) = turn_manager.player_order.iter().position(|&p| p == player) {
                    turn_manager.step_skips.entry(index)
                       .or_insert_with(HashSet::new)
                       .insert(*step);
                }
            },
            // Other turn modifications...
        }
    }
}
```

## Integration Points

- **Game State Module**: Receives phase transitions and updates game state
- **Player Module**: Manages player-specific actions during their turn
- **Stack Module**: Coordinates with priority system for stack resolution
- **Combat Module**: Receives signals for combat phase transitions
- **UI Module**: Updates interface based on current phase

## Testing Strategy

1. **Unit Tests**:
   - Test phase transitions
   - Verify priority passing mechanics
   - Test turn order management
   
2. **Integration Tests**:
   - Test full turn cycles with multiple players
   - Verify state-based actions between phases
   - Test elimination handling in multiplayer games

## Performance Considerations

For games with many players:
- Optimize priority passing and player actions
- Use efficient data structures for tracking turn modifications
- Consider caching computed values for turn transitions 