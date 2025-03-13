# Core Types

This document outlines the fundamental types used throughout the Rummage game engine.

## Game State

The `GameState` struct is a central resource that tracks the overall state of the game.

```rust
pub struct GameState {
    pub active_player: Option<Entity>,
    pub priority_player: Option<Entity>,
    pub current_phase: Phase,
    pub turn_number: usize,
    // Additional fields...
}
```

## Phase

The `Phase` enum represents the different phases of a Magic: The Gathering turn.

```rust
pub enum Phase {
    Beginning(BeginningStep),
    MainPhaseOne,
    Combat(CombatStep),
    MainPhaseTwo,
    Ending(EndingStep),
}
```

Associated step enums:

```rust
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

pub enum CombatStep {
    BeginCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndCombat,
}

pub enum EndingStep {
    End,
    Cleanup,
}
```

## Game Actions

The `GameAction` enum represents all possible actions that can occur in the game. These include:

```rust
pub enum GameAction {
    CastSpell { source: Entity, target: Option<Entity> },
    ActivateAbility { source: Entity, ability_index: usize, targets: Vec<Entity> },
    PlayLand { card: Entity },
    AttackWithCreature { attacker: Entity, defender: Entity },
    BlockAttacker { blocker: Entity, attacker: Entity },
    PassPriority,
    MoveToNextPhase,
    DrawCard { player: Entity },
    TakeMulligan { player: Entity },
    // Additional actions...
}
```

## Zone Types

The `Zone` enum defines all the game zones in MTG:

```rust
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}
```

Each zone has associated components and resources for tracking cards within it.

## Stack

The stack is implemented through the `GameStack` resource:

```rust
pub struct GameStack {
    pub items: Vec<StackItem>,
}

pub struct StackItem {
    pub id: Entity,
    pub source: Entity,
    pub effect_type: EffectType,
    pub targets: Vec<Entity>,
    // Additional fields...
}

pub enum EffectType {
    Spell,
    TriggeredAbility,
    ActivatedAbility,
    // Additional types...
}
```

## Events

The game engine uses Bevy's event system extensively. Key events include:

### Turn Events
- `TurnStartEvent`
- `TurnEndEvent`
- `NextPhaseEvent`

### Combat Events
- `DeclareAttackersEvent`
- `DeclareBlockersEvent`
- `CombatDamageEvent`

### Stack Events
- `PassPriorityEvent`
- `ResolveStackItemEvent`
- `StackItemResolvedEvent`

### Zone Events
- `ZoneChangeEvent`
- `EntersBattlefieldEvent`

### Commander-Specific Events
- `CommanderZoneChoiceEvent`
- `PlayerEliminatedEvent`
- `CombatDamageEvent`

## Card Types

Cards are represented as entities with multiple components. Core card types include:

```rust
pub struct CardBase {
    pub name: String,
    pub card_type: Vec<CardType>,
    pub subtypes: Vec<String>,
    pub text: String,
    // Additional fields...
}

pub enum CardType {
    Land,
    Creature,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
}
```

Creature-specific components:

```rust
pub struct CreatureStats {
    pub power: i32,
    pub toughness: i32,
    pub damage_this_turn: i32,
    // Additional fields...
}
```

## Player State

Player state is tracked through multiple components:

```rust
pub struct Player {
    pub name: String,
    pub life_total: i32,
    pub is_active: bool,
    // Additional fields...
}

pub struct CommanderDamage {
    pub source_commander: Entity,
    pub damage: i32,
}

pub struct ManaPool {
    pub white: i32,
    pub blue: i32,
    pub black: i32,
    pub red: i32,
    pub green: i32,
    pub colorless: i32,
}
```

## Mana System

```rust
pub struct ManaCost {
    pub white: i32,
    pub blue: i32,
    pub black: i32,
    pub red: i32,
    pub green: i32,
    pub colorless: i32,
    pub generic: i32,
    // Special mana costs like X
}
``` 