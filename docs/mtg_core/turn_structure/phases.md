# Turn Phases

This document details the implementation of turn phases and steps in Rummage, explaining how the game progresses through the structured sequence of a Magic: The Gathering turn.

## Phase and Step Structure

A turn in Magic: The Gathering consists of five phases, some of which are divided into steps:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Beginning,
    PreCombatMain,
    Combat,
    PostCombatMain,
    Ending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Step {
    // Beginning Phase
    Untap,
    Upkeep,
    Draw,
    
    // Main Phase (no steps)
    Main,
    
    // Combat Phase
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    FirstStrikeDamage, // Only used when first/double strike is involved
    EndOfCombat,
    
    // Ending Phase
    End,
    Cleanup,
}
```

## Phase Transitions

The game progresses through phases and steps in a specific order. This is managed by a phase transition system:

```rust
pub fn phase_transition_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut phase_events: EventWriter<PhaseChangeEvent>,
    mut step_events: EventWriter<StepChangeEvent>,
    stack: Res<Stack>,
    priority: Res<PrioritySystem>,
) {
    // Only transition if priority is not active and stack is empty
    if priority.active || !stack.items.is_empty() {
        return;
    }
    
    // Current phase and step
    let current_phase = game_state.current_phase;
    let current_step = game_state.current_step;
    
    // Determine next phase and step
    let (next_phase, next_step) = get_next_phase_step(current_phase, current_step);
    
    // Update game state
    game_state.current_phase = next_phase;
    game_state.current_step = next_step;
    
    // Send events
    if current_phase != next_phase {
        phase_events.send(PhaseChangeEvent {
            previous: current_phase,
            current: next_phase,
        });
    }
    
    step_events.send(StepChangeEvent {
        previous: current_step,
        current: next_step,
    });
    
    // Execute phase/step entry actions
    execute_phase_entry_actions(next_phase, next_step, &mut commands, &game_state);
}
```

## Phase Progression

The progression from one phase/step to the next follows this pattern:

```rust
fn get_next_phase_step(current_phase: Phase, current_step: Step) -> (Phase, Step) {
    match (current_phase, current_step) {
        // Beginning Phase progression
        (Phase::Beginning, Step::Untap) => (Phase::Beginning, Step::Upkeep),
        (Phase::Beginning, Step::Upkeep) => (Phase::Beginning, Step::Draw),
        (Phase::Beginning, Step::Draw) => (Phase::PreCombatMain, Step::Main),
        
        // Pre-Combat Main Phase progression
        (Phase::PreCombatMain, Step::Main) => (Phase::Combat, Step::BeginningOfCombat),
        
        // Combat Phase progression
        (Phase::Combat, Step::BeginningOfCombat) => (Phase::Combat, Step::DeclareAttackers),
        (Phase::Combat, Step::DeclareAttackers) => (Phase::Combat, Step::DeclareBlockers),
        (Phase::Combat, Step::DeclareBlockers) => {
            // Check if first strike is needed
            if combat_has_first_strike() {
                (Phase::Combat, Step::FirstStrikeDamage)
            } else {
                (Phase::Combat, Step::CombatDamage)
            }
        },
        (Phase::Combat, Step::FirstStrikeDamage) => (Phase::Combat, Step::CombatDamage),
        (Phase::Combat, Step::CombatDamage) => (Phase::Combat, Step::EndOfCombat),
        (Phase::Combat, Step::EndOfCombat) => (Phase::PostCombatMain, Step::Main),
        
        // Post-Combat Main Phase progression
        (Phase::PostCombatMain, Step::Main) => (Phase::Ending, Step::End),
        
        // Ending Phase progression
        (Phase::Ending, Step::End) => (Phase::Ending, Step::Cleanup),
        (Phase::Ending, Step::Cleanup) => {
            // Move to next turn
            (Phase::Beginning, Step::Untap)
        },
        
        // Default case (should never happen)
        _ => (current_phase, current_step),
    }
}
```

## Phase Entry Actions

Each phase and step has specific actions that occur when entering:

```rust
fn execute_phase_entry_actions(
    phase: Phase,
    step: Step,
    commands: &mut Commands,
    game_state: &GameState,
) {
    match (phase, step) {
        // Beginning Phase actions
        (Phase::Beginning, Step::Untap) => {
            // Untap all permanents controlled by active player
            commands.add(untap_permanents_command(game_state.active_player));
            
            // Handle "at beginning of untap step" triggers
            commands.add(check_beginning_of_step_triggers_command(phase, step));
        },
        (Phase::Beginning, Step::Upkeep) => {
            // Handle "at beginning of upkeep" triggers
            commands.add(check_beginning_of_step_triggers_command(phase, step));
            
            // Grant priority to active player
            commands.add(grant_priority_command(game_state.active_player));
        },
        (Phase::Beginning, Step::Draw) => {
            // Active player draws a card (except on first turn)
            if game_state.turn_number > 1 {
                commands.add(draw_card_command(game_state.active_player, 1));
            }
            
            // Handle "at beginning of draw step" triggers
            commands.add(check_beginning_of_step_triggers_command(phase, step));
            
            // Grant priority to active player
            commands.add(grant_priority_command(game_state.active_player));
        },
        
        // Main Phase actions
        (Phase::PreCombatMain, Step::Main) | (Phase::PostCombatMain, Step::Main) => {
            // Reset land plays for turn if entering first main phase
            if phase == Phase::PreCombatMain && game_state.current_phase != Phase::PreCombatMain {
                commands.add(reset_land_plays_command());
            }
            
            // Grant priority to active player
            commands.add(grant_priority_command(game_state.active_player));
        },
        
        // Combat Phase actions
        (Phase::Combat, _) => {
            // Handle specific combat step actions
            match step {
                Step::BeginningOfCombat => {
                    // Handle "at beginning of combat" triggers
                    commands.add(check_beginning_of_step_triggers_command(phase, step));
                },
                Step::DeclareAttackers => {
                    // Active player declares attackers
                    commands.add(declare_attackers_command());
                },
                Step::DeclareBlockers => {
                    // Defending players declare blockers
                    commands.add(declare_blockers_command());
                },
                Step::FirstStrikeDamage => {
                    // Assign and deal first strike damage
                    commands.add(assign_first_strike_damage_command());
                    commands.add(deal_combat_damage_command(true));
                },
                Step::CombatDamage => {
                    // Assign and deal regular combat damage
                    commands.add(assign_combat_damage_command());
                    commands.add(deal_combat_damage_command(false));
                },
                _ => {}
            }
            
            // Grant priority to active player (for all combat steps)
            commands.add(grant_priority_command(game_state.active_player));
        },
        
        // Ending Phase actions
        (Phase::Ending, Step::End) => {
            // Handle "at beginning of end step" triggers
            commands.add(check_beginning_of_step_triggers_command(phase, step));
            
            // Grant priority to active player
            commands.add(grant_priority_command(game_state.active_player));
        },
        (Phase::Ending, Step::Cleanup) => {
            // Discard to hand size
            commands.add(discard_to_hand_size_command(game_state.active_player));
            
            // Remove "until end of turn" effects
            commands.add(remove_until_end_of_turn_effects_command());
            
            // Clear damage from permanents
            commands.add(clear_damage_command());
            
            // No priority is granted in cleanup step unless a trigger occurs
        },
    }
}
```

## Turn Advancement

When a turn ends, the game advances to the next player's turn:

```rust
fn advance_turn_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut turn_events: EventWriter<TurnChangeEvent>,
) {
    // Only advance turn when transitioning from cleanup to untap
    if game_state.current_phase == Phase::Ending && 
       game_state.current_step == Step::Cleanup &&
       !game_state.transitioning_to_next_turn {
        
        // Mark that we're transitioning to next turn
        game_state.transitioning_to_next_turn = true;
        
        // Get next player
        let next_player = get_next_player(game_state.active_player, &game_state);
        
        // Send turn change event
        turn_events.send(TurnChangeEvent {
            previous_player: game_state.active_player,
            next_player,
            turn_number: game_state.turn_number + 1,
        });
        
        // Update game state
        game_state.active_player = next_player;
        game_state.turn_number += 1;
        
        // Reset turn-based tracking
        commands.add(reset_turn_tracking_command());
    }
}
```

## Extra Turns

The system also supports extra turns, which are handled by modifying the turn order:

```rust
pub fn add_extra_turn(
    player: Entity,
    game_state: &mut GameState,
    extra_turn_events: &mut EventWriter<ExtraTurnEvent>,
) {
    // Add extra turn to the queue
    game_state.extra_turns.push(player);
    
    // Send event
    extra_turn_events.send(ExtraTurnEvent {
        player,
        source_turn: game_state.turn_number,
    });
}

fn determine_next_turn_player(game_state: &GameState) -> Entity {
    // Check if there are extra turns queued
    if !game_state.extra_turns.is_empty() {
        // Take the next extra turn
        return game_state.extra_turns[0];
    }
    
    // Otherwise, proceed to next player in turn order
    get_next_player(game_state.active_player, game_state)
}
```

## Implementation Status

The turn structure implementation currently:

- ✅ Handles all standard phases and steps
- ✅ Implements proper phase transitions
- ✅ Supports phase-specific actions
- ✅ Manages turn advancement
- ✅ Supports extra turns
- 🔄 Implementing special turn modifications (e.g., additional combat phases)
- 🔄 Supporting time counters and suspended cards

---

Next: [Combat System](../combat/index.md) 