# Extra Turns and Turn Modifications

## Overview

Commander implements various turn modification effects from Magic: The Gathering, including extra turns, skipped turns, and modified turn structures. These effects are particularly impactful in a multiplayer environment.

## Extra Turns

Extra turns are implemented through a queue system in the `TurnManager`:

```rust
#[derive(Resource)]
pub struct TurnManager {
    // Other fields...
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
```

When a player would gain an extra turn:

1. The extra turn is added to the queue
2. When the current turn ends, the system checks the queue before advancing to the next player's turn
3. If there are extra turns queued, the player specified in the queue takes their extra turn
4. After all extra turns are complete, normal turn order resumes

## Turn Restrictions

Certain cards can impose restrictions on turns, implemented through the `TurnRestriction` enum:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnRestriction {
    NoUntap,
    NoUpkeep,
    NoDraw,
    NoMainPhase,
    NoCombat,
    MaxSpells(u32),
    NoActivatedAbilities,
    MustAttack,
    CantAttack,
    NoArtifactsEnchantmentsCreatures,
    LoseAtEndStep,
}
```

These restrictions can be associated with specific turns, including extra turns.

## Skipped Turns

The system also tracks turns that should be skipped:

```rust
pub fn process_turn_transition(
    mut turn_manager: ResMut<TurnManager>,
    // Other parameters...
) {
    // Find the next active player
    let mut next_player_index = (turn_manager.active_player_index + 1) % turn_manager.player_order.len();
    let next_player = turn_manager.player_order[next_player_index];
    
    // Check for skipped turns
    if turn_manager.skipped_turns.contains(&next_player) {
        turn_manager.skipped_turns.remove(&next_player);
        // Skip to the player after the one who would skip
        next_player_index = (next_player_index + 1) % turn_manager.player_order.len();
    }
    
    // Check for extra turns first
    if !turn_manager.extra_turns.is_empty() {
        let (extra_turn_player, source) = turn_manager.extra_turns.pop_front().unwrap();
        // Process the extra turn...
    } else {
        // Set the next player as active
        turn_manager.active_player_index = next_player_index;
    }
    
    // Reset for new turn
    turn_manager.current_phase = Phase::Beginning(BeginningStep::Untap);
    turn_manager.turn_number += 1;
}
```

## Additional Phase Modifications

The system supports other phase modifications:

- Additional combat phases
- Phase repetition
- Phase reordering (in specific cases)
- Phase duration modifications

## Multiplayer Impact

Extra turns and turn modifications have significant impact in Commander:

- Political considerations when taking extra turns
- More targets for "target player skips their next turn" effects
- Greater variance in game length due to turn manipulation
- House rules regarding excessive turn manipulation

## Implementation Details

The turn modification system integrates with the event system to ensure proper triggers occur:

```rust
pub fn extra_turn_system(
    mut commands: Commands,
    mut turn_manager: ResMut<TurnManager>,
    mut extra_turn_events: EventReader<ExtraTurnEvent>,
) {
    for event in extra_turn_events.iter() {
        turn_manager.extra_turns.push_back((event.player, event.source.clone()));
    }
}
```

Cards that grant or modify turns dispatch appropriate events that are handled by these systems. 