# Targeting System

The targeting system allows players to select targets for spells, abilities, and other effects.

## Core Concepts

The targeting system consists of:

- **Target Sources**: Cards or abilities that require targets
- **Target Validators**: Rules that determine valid targets
- **Target Selectors**: UI elements that allow players to select targets
- **Visual Feedback**: Effects that show targeting in progress

## Targeting Flow

1. Player initiates targeting (by playing a card or activating an ability)
2. System highlights valid targets based on game rules
3. Player selects target(s)
4. System validates the selection
5. Action completes with chosen targets

## Component Structure

```rust
pub struct TargetSource {
    pub targeting_active: bool,
    pub required_targets: usize,
    pub current_targets: Vec<Entity>,
    pub target_rules: TargetRules,
}

pub struct Targetable {
    pub can_be_targeted: bool,
    pub highlight_color: Color,
}
```

## Visual Effects

The targeting system provides several visual cues:

- Highlighting valid targets
- Animated arcs or arrows from source to target
- Pulsing effects on potential targets
- Confirmation animations when targets are selected

## Integration

The targeting system integrates with:

- [Card Selection](card_selection.md): Selected cards can be used as targets
- [Drag and Drop](drag_and_drop.md): Dragging can initiate targeting
- [Game Rules](../../mtg_rules/targeting.md): Rules determine valid targets

## Implementation Example

```rust
// Create a spell that requires targeting
commands.spawn((
    card_bundle,
    TargetSource {
        targeting_active: false,
        required_targets: 1,
        current_targets: Vec::new(),
        target_rules: TargetRules::CreaturesOnly,
    },
));

// Make a creature targetable
commands.spawn((
    creature_bundle,
    Targetable {
        can_be_targeted: true,
        highlight_color: Color::rgb(0.9, 0.3, 0.3),
    },
));
```

For more details on targeting rules, see the [Magic rules on targeting](../../mtg_rules/targeting.md). 