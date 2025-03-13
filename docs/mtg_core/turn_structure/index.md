# Turn Structure

This section documents the core implementation of Magic: The Gathering's turn structure and phase system in Rummage.

## Overview

The turn structure in Magic: The Gathering follows a specific sequence of phases and steps, during which players can take actions at defined points. This structured progression is essential for maintaining game order and ensuring rules consistency.

## Turn Phases and Steps

A turn in MTG consists of the following phases and steps, in order:

1. **Beginning Phase**
   - Untap Step - Active player untaps their permanents
   - Upkeep Step - Triggers "at the beginning of your upkeep" abilities
   - Draw Step - Active player draws a card

2. **First Main Phase (Precombat Main Phase)**
   - Player may play lands, cast spells, and activate abilities

3. **Combat Phase**
   - Beginning of Combat Step - Last chance for effects before attackers
   - Declare Attackers Step - Active player declares attackers
   - Declare Blockers Step - Defending players declare blockers
   - Combat Damage Step - Combat damage is assigned and dealt
   - End of Combat Step - Last chance for effects before combat ends

4. **Second Main Phase (Postcombat Main Phase)**
   - Player may play lands (if they haven't during this turn), cast spells, and activate abilities

5. **Ending Phase**
   - End Step - Triggers "at the beginning of your end step" abilities
   - Cleanup Step - Discard to hand size, damage wears off

## Implementation Details

### Phase Tracking

```rust
#[derive(Resource, Debug, Clone, PartialEq, Eq)]
pub enum Phase {
    Beginning(BeginningPhaseStep),
    Main(MainPhaseType),
    Combat(CombatStep),
    Ending(EndingPhaseStep),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeginningPhaseStep {
    Untap,
    Upkeep,
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainPhaseType {
    Precombat,
    Postcombat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatStep {
    Beginning,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndingPhaseStep {
    End,
    Cleanup,
}
```

### Phase Progression System

The phase progression system handles the advancement through turn phases:

```rust
pub fn advance_phase(
    mut game_state: ResMut<GameState>,
    mut phase_events: EventWriter<PhaseChangeEvent>,
) {
    let current_phase = game_state.current_phase.clone();
    let next_phase = determine_next_phase(&current_phase);
    
    game_state.current_phase = next_phase.clone();
    
    phase_events.send(PhaseChangeEvent {
        previous_phase: current_phase,
        new_phase: next_phase,
    });
}

fn determine_next_phase(current_phase: &Phase) -> Phase {
    match current_phase {
        Phase::Beginning(step) => match step {
            BeginningPhaseStep::Untap => Phase::Beginning(BeginningPhaseStep::Upkeep),
            BeginningPhaseStep::Upkeep => Phase::Beginning(BeginningPhaseStep::Draw),
            BeginningPhaseStep::Draw => Phase::Main(MainPhaseType::Precombat),
        },
        Phase::Main(MainPhaseType::Precombat) => Phase::Combat(CombatStep::Beginning),
        Phase::Combat(step) => match step {
            CombatStep::Beginning => Phase::Combat(CombatStep::DeclareAttackers),
            CombatStep::DeclareAttackers => Phase::Combat(CombatStep::DeclareBlockers),
            CombatStep::DeclareBlockers => Phase::Combat(CombatStep::CombatDamage),
            CombatStep::CombatDamage => Phase::Combat(CombatStep::End),
            CombatStep::End => Phase::Main(MainPhaseType::Postcombat),
        },
        Phase::Main(MainPhaseType::Postcombat) => Phase::Ending(EndingPhaseStep::End),
        Phase::Ending(step) => match step {
            EndingPhaseStep::End => Phase::Ending(EndingPhaseStep::Cleanup),
            EndingPhaseStep::Cleanup => Phase::Beginning(BeginningPhaseStep::Untap),
        },
    }
}
```

### Priority System

During most phases and steps, players receive priority to take actions:

```rust
pub fn handle_priority(
    game_state: Res<GameState>,
    mut stack: ResMut<Stack>,
    mut priority_events: EventWriter<PriorityEvent>,
) {
    // Skip priority in certain steps
    if !should_players_get_priority(&game_state.current_phase) {
        return;
    }
    
    // Determine priority holder
    let active_player = game_state.active_player;
    let next_player = determine_priority_player(active_player, &game_state.player_order);
    
    priority_events.send(PriorityEvent {
        player: next_player,
        phase: game_state.current_phase.clone(),
    });
}

fn should_players_get_priority(phase: &Phase) -> bool {
    match phase {
        Phase::Beginning(BeginningPhaseStep::Untap) => false,
        Phase::Ending(EndingPhaseStep::Cleanup) => false,
        _ => true,
    }
}
```

## Extension Points

The turn structure system is designed to be extensible for different formats:

1. **Format-Specific Phases** - Formats can add custom phases or steps
2. **Turn Modification** - Systems for extra turns, skipped turns, or additional phases
3. **Priority Rules** - Customizable priority passing for different formats

## Integration with Other Systems

The turn structure integrates with these other game systems:

- **State-Based Actions** - Checked whenever a player would receive priority
- **Stack Resolution** - Occurs during priority passes when no player adds to the stack
- **Triggered Abilities** - Put on the stack at appropriate times based on the current phase
- **Mana System** - Mana pools empty at phase boundaries

## Format-Specific Considerations

Different Magic formats may implement turn structure variations:

- **Multiplayer Formats** - Handle turn order for multiple players
- **Special Formats** - May modify turn structure (e.g., Two-Headed Giant)

For Commander-specific turn structure implementation, see [Commander Turn Structure](../../formats/commander/turns_and_phases/index.md).

---

Next: [Priority System](priority.md) 