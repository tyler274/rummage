# Phase Management

## Phase and Step Definitions

The Commander implementation uses a hierarchical structure of phases and steps that closely follows the official Magic rules:

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

## Phase Transitions

Phase transitions are managed by the turn system and occur when:
1. All players have passed priority with an empty stack
2. Certain turn-based actions are completed
3. Special effects cause phase skipping or addition

## Phase-Specific Actions

Each phase and step has specific turn-based actions that occur at its beginning:

### Beginning Phase

#### Untap Step
- The active player untaps all permanents they control
- No players receive priority during this step
- "At the beginning of your untap step" triggered abilities trigger

#### Upkeep Step
- "At the beginning of your upkeep" triggered abilities trigger
- Players receive priority

#### Draw Step
- Active player draws a card
- "At the beginning of your draw step" triggered abilities trigger
- Players receive priority

### Main Phases
- Active player may play land (once per turn)
- Players may cast spells and activate abilities
- No automatic actions occur

### Combat Phase
- Detailed in the [Combat System](../combat/index.md) documentation

### Ending Phase

#### End Step
- "At the beginning of your end step" triggered abilities trigger
- Players receive priority

#### Cleanup Step
- Active player discards down to maximum hand size
- Damage is removed from permanents
- "Until end of turn" effects end
- Normally, no player receives priority during this step
  - If any state-based actions are performed or triggered abilities trigger, players do receive priority

## Special Phase Handling

The phase system accounts for special cases:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnRestriction {
    NoUntap,
    NoUpkeep,
    NoDraw,
    NoMainPhase,
    NoCombat,
    MaxSpells(u32),
    // Other restrictions
}
```

These phase restrictions can be applied through card effects and are managed by the `TurnManager`. 