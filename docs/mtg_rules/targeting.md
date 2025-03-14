# MTG Targeting Rules

This document outlines the rules for targeting in Magic: The Gathering as implemented in Rummage.

## Core Targeting Rules

According to the Magic: The Gathering Comprehensive Rules:

1. The term "target" is used to describe an object that a spell or ability will affect.
2. An object that requires targets is put on the stack with those targets already chosen.
3. Targets are always declared as part of casting a spell or activating an ability.
4. A target must be valid both when declared and when the spell or ability resolves.

## Valid Targets

A valid target must:

1. Meet any specific requirements of the targeting effect
2. Be in the appropriate zone (usually on the battlefield)
3. Not have hexproof or shroud (relative to the controller of the targeting effect)
4. Not have protection from the relevant quality of the targeting effect

## Implementation

In Rummage, targeting is implemented through several components:

```rust
pub enum TargetType {
    Player,
    Creature,
    Permanent,
    AnyTarget, // Player or creature
    // ...other target types
}

pub struct TargetRequirement {
    pub target_type: TargetType,
    pub count: TargetCount,
    pub additional_requirements: Vec<Box<dyn TargetFilter>>,
}
```

## Targeting Phases

The targeting process has several phases:

1. **Declaration**: The player declares targets for a spell or ability
2. **Validation**: The system validates that the targets are legal
3. **Resolution**: When the spell or ability resolves, targets are checked again
4. **Effect Application**: The effect is applied to valid targets

## Illegal Targets

If a target becomes illegal before a spell or ability resolves:

1. The spell or ability will still resolve
2. The effect will not be applied to the illegal target
3. If all targets are illegal, the spell or ability is countered by game rules

## UI Integration

The targeting rules integrate with the UI through:

- [Targeting System](../game_gui/interaction/targeting.md): UI implementation of targeting
- [Drag and Drop](../game_gui/interaction/drag_and_drop.md): Using drag interactions for targeting

## Examples

### Single Target Spell

```rust
// Lightning Bolt implementation
let lightning_bolt = Card::new("Lightning Bolt")
    .with_cost(Mana::new(0, 1, 0, 0, 0, 0))
    .with_target_requirement(TargetRequirement {
        target_type: TargetType::AnyTarget,
        count: TargetCount::Exactly(1),
        additional_requirements: vec![],
    })
    .with_effect(|game, targets| {
        for target in targets {
            game.deal_damage(target, 3, DamageType::Spell);
        }
    });
```

### Multiple Targets Spell

```rust
// Electrolyze implementation
let electrolyze = Card::new("Electrolyze")
    .with_cost(Mana::new(0, 1, 0, 1, 0, 0))
    .with_target_requirement(TargetRequirement {
        target_type: TargetType::AnyTarget,
        count: TargetCount::UpTo(2),
        additional_requirements: vec![],
    })
    .with_effect(|game, targets| {
        for target in targets {
            game.deal_damage(target, 1, DamageType::Spell);
        }
        game.draw_cards(game.active_player, 1);
    });
``` 