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

## Turn Structure Edge Cases in Commander

### Extra Turns and Turn Order Manipulation

Commander games often involve cards that grant extra turns or modify turn order. Here's how the system handles these edge cases:

```rust
fn handle_extra_turn_effects(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut extra_turn_events: EventReader<ExtraTurnEvent>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    // Process extra turn events
    for event in extra_turn_events.read() {
        // Insert the extra turn into the turn queue
        let extra_turn = TurnQueueEntry {
            player: event.player,
            source: TurnSource::ExtraTurn(event.source),
            skipped_phases: event.skipped_phases.clone(),
            extra_phases: event.extra_phases.clone(),
            modified_rules: event.modified_rules.clone(),
        };
        
        match event.insertion_method {
            ExtraTurnInsertion::Immediately => {
                // Insert immediately after the current turn
                let position = turn_manager.current_turn_index + 1;
                turn_manager.turn_queue.insert(position, extra_turn);
            },
            ExtraTurnInsertion::AfterFullCycle => {
                // Insert after all players have had a turn
                let player_count = turn_manager.player_order.len();
                let position = turn_manager.current_turn_index + player_count;
                
                // Ensure we don't go past the end of the queue
                let target_position = position.min(turn_manager.turn_queue.len());
                turn_manager.turn_queue.insert(target_position, extra_turn);
            },
            ExtraTurnInsertion::End => {
                // Add to the end of the queue
                turn_manager.turn_queue.push(extra_turn);
            },
        }
        
        // Notify that turn order has changed
        commands.spawn(TurnOrderChangedEvent {
            reason: TurnOrderChangeReason::ExtraTurn(event.source),
            new_queue: turn_manager.turn_queue.clone(),
        });
    }
}
```

### Phase and Step Skipping

Cards like Time Stop can end the turn immediately or skip phases:

```rust
fn handle_phase_skip_effects(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut phase_skip_events: EventReader<PhaseSkipEvent>,
) {
    for event in phase_skip_events.read() {
        match event.skip_type {
            PhaseSkipType::EndTurn => {
                // End the turn immediately
                turn_manager.phases_remaining.clear();
                turn_manager.current_phase = TurnPhase::EndingPhase;
                turn_manager.current_step = Some(TurnStep::EndStep);
                turn_manager.phase_complete = true;
                
                // Clear the stack and pending effects
                commands.spawn(EndTurnImmediatelyEvent {
                    source: event.source,
                });
            },
            PhaseSkipType::SkipPhase(phase) => {
                // Remove the specified phase from the remaining phases
                turn_manager.phases_remaining.retain(|p| p != &phase);
                
                // If we're currently in the phase to skip, end it immediately
                if turn_manager.current_phase == phase {
                    turn_manager.phase_complete = true;
                }
            },
            PhaseSkipType::SkipToPhase(phase) => {
                // Skip to a specific phase
                let current_phase_index = turn_manager.phases_remaining
                    .iter()
                    .position(|p| p == &turn_manager.current_phase);
                
                let target_phase_index = turn_manager.phases_remaining
                    .iter()
                    .position(|p| p == &phase);
                
                if let (Some(current), Some(target)) = (current_phase_index, target_phase_index) {
                    if target > current {
                        // Remove all phases between current and target
                        turn_manager.phases_remaining = turn_manager.phases_remaining
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i <= current || *i >= target)
                            .map(|(_, p)| p.clone())
                            .collect();
                            
                        // End current phase
                        turn_manager.phase_complete = true;
                    }
                }
            },
            PhaseSkipType::ExtraPhase(phase) => {
                // Insert an extra phase after the current one
                if let Some(current_index) = turn_manager.phases_remaining
                    .iter()
                    .position(|p| p == &turn_manager.current_phase) {
                    
                    turn_manager.phases_remaining.insert(current_index + 1, phase);
                }
            },
        }
    }
}
```

### Handling Modified Turn Rules

Some effects modify how turns work rather than skipping phases:

```rust
fn apply_turn_modifications(
    mut turn_manager: ResMut<TurnManager>,
    active_modifications: Query<&TurnRuleModification>,
) {
    // Reset to default rules
    turn_manager.max_lands_per_turn = 1;
    turn_manager.skip_untap = false;
    turn_manager.skip_draw = false;
    turn_manager.skip_combat = false;
    turn_manager.additional_combat_phases = 0;
    turn_manager.mana_empties_between_phases = true;
    
    // Apply all active modifications
    for modification in active_modifications.iter() {
        match modification.modification_type {
            TurnRuleModificationType::MaxLandsPerTurn(count) => {
                turn_manager.max_lands_per_turn = count;
            },
            TurnRuleModificationType::SkipUntap(player) => {
                if turn_manager.active_player == player {
                    turn_manager.skip_untap = true;
                }
            },
            TurnRuleModificationType::SkipDraw(player) => {
                if turn_manager.active_player == player {
                    turn_manager.skip_draw = true;
                }
            },
            TurnRuleModificationType::SkipCombat(player) => {
                if turn_manager.active_player == player {
                    turn_manager.skip_combat = true;
                }
            },
            TurnRuleModificationType::AdditionalCombatPhases(count) => {
                turn_manager.additional_combat_phases = count;
            },
            TurnRuleModificationType::ManaDoesNotEmpty => {
                turn_manager.mana_empties_between_phases = false;
            },
            // Other modifications...
        }
    }
}
```

### Controlled Player Turns

In Commander, one player can sometimes control another player's turn:

```rust
fn handle_player_control_effects(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut control_events: EventReader<PlayerControlEvent>,
    mut end_control_events: EventReader<EndPlayerControlEvent>,
) {
    // Process control events
    for event in control_events.read() {
        turn_manager.controlled_turns.insert(
            event.controlled_player,
            TurnControl {
                controller: event.controlling_player,
                source: event.source,
                restrictions: event.restrictions.clone(),
            }
        );
    }
    
    // Process end control events
    for event in end_control_events.read() {
        turn_manager.controlled_turns.remove(&event.controlled_player);
    }
    
    // Check if the current active player is controlled
    let active_player = turn_manager.active_player;
    if let Some(control) = turn_manager.controlled_turns.get(&active_player) {
        // Set up control for this turn
        commands.spawn(ActivePlayerControlledEvent {
            controlled_player: active_player,
            controlling_player: control.controller,
            restrictions: control.restrictions.clone(),
        });
    }
}
```

### Time Walk Effects vs. Normal Turn Cycle

Commander games can have both extra turns and modified turn cycles:

```rust
fn reconcile_turn_modifications(
    mut turn_manager: ResMut<TurnManager>,
    time_walk_effects: Query<&TimeWalkEffect>,
    turn_cycle_effects: Query<&TurnCycleModification>,
) {
    // First apply any global turn cycle modifications
    let mut turn_cycle_modified = false;
    for effect in turn_cycle_effects.iter() {
        match effect.modification_type {
            TurnCycleModificationType::ReverseOrder => {
                turn_manager.reversed_turn_order = true;
                turn_cycle_modified = true;
            },
            TurnCycleModificationType::SkipPlayers(ref players) => {
                // Mark players to skip in the turn order
                for &player in players {
                    turn_manager.skipped_players.insert(player);
                }
                turn_cycle_modified = true;
            },
            TurnCycleModificationType::EveryoneDrawsEveryTurn => {
                turn_manager.everyone_draws = true;
                turn_cycle_modified = true;
            },
            // Other modifications...
        }
    }
    
    // Then check for extra turns/time walks
    let has_time_walks = !time_walk_effects.is_empty();
    
    // Special case: when both are active, the rule is that
    // time walk effects override turn cycle modifications temporarily
    if has_time_walks && turn_cycle_modified {
        for effect in time_walk_effects.iter() {
            if effect.override_turn_cycle {
                // This time walk effect temporarily disables turn cycle modifications
                turn_manager.turn_cycle_override_active = true;
                break;
            }
        }
    }
}
```

### Edge Case: Simultaneous Extra Turn Effects

When multiple players would take extra turns simultaneously:

```rust
fn resolve_simultaneous_extra_turns(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut extra_turn_collisions: EventReader<ExtraTurnCollisionEvent>,
) {
    for event in extra_turn_collisions.read() {
        let colliding_turns = &event.colliding_turns;
        
        // Default rule: APNAP order (Active Player, Non-Active Player)
        // Sort by distance from active player in turn order
        let mut sorted_turns = colliding_turns.clone();
        sorted_turns.sort_by(|a, b| {
            let active_index = turn_manager.player_order
                .iter()
                .position(|p| *p == turn_manager.active_player)
                .unwrap_or(0);
                
            let a_index = turn_manager.player_order
                .iter()
                .position(|p| *p == a.player)
                .unwrap_or(0);
                
            let b_index = turn_manager.player_order
                .iter()
                .position(|p| *p == b.player)
                .unwrap_or(0);
                
            // Calculate distance in turn order
            let a_distance = (a_index + turn_manager.player_order.len() - active_index) 
                             % turn_manager.player_order.len();
            let b_distance = (b_index + turn_manager.player_order.len() - active_index) 
                             % turn_manager.player_order.len();
                
            a_distance.cmp(&b_distance)
        });
        
        // Replace the colliding turns with the sorted order
        let position = turn_manager.turn_queue
            .iter()
            .position(|t| colliding_turns.contains(t))
            .unwrap_or(0);
            
        // Remove old entries
        turn_manager.turn_queue.retain(|t| !colliding_turns.contains(t));
        
        // Insert sorted entries
        for (i, turn) in sorted_turns.into_iter().enumerate() {
            turn_manager.turn_queue.insert(position + i, turn);
        }
        
        // Notify about the resolution
        commands.spawn(ExtraTurnCollisionResolvedEvent {
            original_event: event.clone(),
            resolution_method: ExtraTurnResolutionMethod::APNAPOrder,
        });
    }
}
```

### Edge Case: Turn Cycles during Multiplayer Elimination

When a player is eliminated during a multiplayer game, the turn order must be maintained:

```rust
fn handle_elimination_turn_adjustments(
    mut turn_manager: ResMut<TurnManager>,
    mut player_eliminated_events: EventReader<PlayerEliminatedEvent>,
) {
    for event in player_eliminated_events.read() {
        let eliminated_player = event.player;
        
        // Remove from player order
        if let Some(pos) = turn_manager.player_order.iter().position(|&p| p == eliminated_player) {
            turn_manager.player_order.remove(pos);
            
            // Adjust indices if needed
            if pos <= turn_manager.active_player_index {
                turn_manager.active_player_index = turn_manager.active_player_index.saturating_sub(1);
            }
            
            if pos <= turn_manager.priority_player_index {
                turn_manager.priority_player_index = turn_manager.priority_player_index.saturating_sub(1);
            }
        }
        
        // Update future turns
        turn_manager.turn_queue.retain(|turn| turn.player != eliminated_player);
        
        // Handle cases where eliminated player had the next turn
        if turn_manager.next_player() == eliminated_player {
            // Advance to the next player
            advance_to_next_valid_player(&mut turn_manager);
        }
    }
}

fn advance_to_next_valid_player(turn_manager: &mut TurnManager) {
    // Find the next non-eliminated, non-skipped player
    while turn_manager.skipped_players.contains(&turn_manager.next_player()) {
        // Move to the next player in order
        turn_manager.active_player_index = 
            (turn_manager.active_player_index + 1) % turn_manager.player_order.len();
    }
}
```

### Split Second During Turn Transitions

Handle the edge case of Split Second effects during turn transitions:

```rust
fn handle_split_second_during_turn_transition(
    turn_manager: Res<TurnManager>,
    split_second_effects: Query<Entity, With<SplitSecondEffect>>,
    mut end_step_events: EventReader<EndStepEvent>,
    mut commands: Commands,
) {
    // Check if there's a split second effect active during turn transition
    let split_second_active = !split_second_effects.is_empty();
    
    if split_second_active {
        for event in end_step_events.read() {
            // Special handling for split second during turn transitions
            // Players can't respond to the turn ending or beginning
            commands.spawn(SplitSecondTurnTransitionEvent {
                from_player: turn_manager.active_player,
                to_player: turn_manager.next_player(),
                allows_responses: false,
            });
        }
    }
}
```

### Teferi's Protection and Similar Phase-Out Effects

Handle phasing out during turn transitions:

```rust
fn handle_phased_out_player(
    mut turn_manager: ResMut<TurnManager>,
    mut phase_out_events: EventReader<PlayerPhasedOutEvent>,
    mut phase_in_events: EventReader<PlayerPhasedInEvent>,
    phased_out_players: Query<Entity, With<PhasedOutStatus>>,
) {
    // Track players phasing out
    for event in phase_out_events.read() {
        turn_manager.phased_out_players.insert(event.player);
    }
    
    // Track players phasing in
    for event in phase_in_events.read() {
        turn_manager.phased_out_players.remove(&event.player);
    }
    
    // Special handling for phased out active player
    if turn_manager.phased_out_players.contains(&turn_manager.active_player) {
        // Player is phased out but still takes their turn
        // They just can't be affected by anything
        turn_manager.active_player_phased_out = true;
    } else {
        turn_manager.active_player_phased_out = false;
    }
}
```

### Commander Specific: Command Zone Interactions on Turn Start

```rust
fn handle_command_zone_turn_start(
    mut commands: Commands,
    turn_manager: Res<TurnManager>,
    command_zone: Res<CommandZone>,
    active_player_query: Query<Entity, (With<CommanderPlayer>, With<ActivePlayer>)>,
) {
    // If a player's commander is in the command zone at the start of their turn,
    // trigger any "at the beginning of your turn, if your commander is in the command zone" effects
    
    if let Ok(active_player) = active_player_query.get_single() {
        // Get player's commanders in command zone
        let commanders_in_command_zone = command_zone
            .get_commanders_for_player(active_player)
            .iter()
            .filter(|&&commander| command_zone.contains(commander))
            .copied()
            .collect::<Vec<_>>();
            
        if !commanders_in_command_zone.is_empty() {
            commands.spawn(CommanderInCommandZoneTurnStartEvent {
                player: active_player,
                commanders: commanders_in_command_zone,
            });
        }
    }
}
```

## Testing Turn Structure Edge Cases

Complex turn structure scenarios require thorough testing:

```rust
#[test]
fn test_turn_structure_edge_cases() {
    let mut app = App::new();
    app.add_plugins(CommanderTurnTestPlugin);
    
    // Test cases for complex turn structure scenarios
    
    // 1. Multiple extra turns from different sources
    // 2. Player elimination during extra turn resolution
    // 3. Split second effects during turn transitions
    // 4. Reversed turn order with extra turn effects
    // 5. Turn control combined with turn modifications
    // 6. Phase skipping combined with extra phase effects
    // 7. Multiple simultaneous end-the-turn effects
    // 8. Player phasing out during their turn
    // 9. Player concession during extra turn sequence
    // 10. Complex APNAP ordering with multiple turn-affecting spells
    
    // Test execution...
}
``` 