# Targeting

Targeting is a fundamental mechanic in Magic: The Gathering that determines which game objects (cards, players, etc.) will be affected by a spell or ability. This document outlines how targeting is implemented in Rummage.

## Core Concepts

### Target Types

The targeting system supports various target types:

- **Cards**: Cards in any zone
- **Players**: Players in the game
- **Tokens**: Token creatures on the battlefield
- **Spells**: Spells on the stack
- **Emblems**: Persistent effects in the command zone
- **Abilities**: Abilities on the stack

### Target Restrictions

Targets can be restricted based on:

- **Characteristics**: Card type, subtypes, color, power/toughness, etc.
- **Game State**: Tapped/untapped, controller, zone, etc.
- **Custom Conditions**: Any programmable condition

## Implementation

### Target Selection

Target selection is implemented using queries and predicates:

```rust
// Example of a targeting predicate for "target creature you control"
pub fn creature_you_control_predicate(
    entity: Entity,
    world: &World,
    source_player: Entity,
) -> bool {
    let card_type = world.get::<CardType>(entity);
    let controller = world.get::<Controller>(entity);
    let zone = world.get::<Zone>(entity);
    
    matches!(card_type, Ok(CardType::Creature)) &&
    matches!(controller, Ok(Controller(player)) if *player == source_player) &&
    matches!(zone, Ok(Zone::Battlefield))
}
```

### Target Validation

Targets are validated at multiple points:

1. **During casting/activation**: Initial target selection
2. **On resolution**: Confirming targets are still legal
3. **During targeting events**: When other effects change targeting

### Illegal Targets

The system handles illegal targets by:

- Preventing spells with illegal targets from being cast
- Countering spells with illegal targets on resolution
- Removing illegal targets from multi-target effects

## UI Integration

Targeting integrates with the UI system:

- **Visual indicators**: Highlighting valid targets
- **Targeting arrows**: Showing connections between source and targets
- **Invalid target feedback**: Indicating why a target is invalid

## Special Cases

### Protection

The system handles protection effects:
- Can't be targeted by spells/abilities with specific characteristics
- Can't be blocked by creatures with specific characteristics
- Can't be damaged by sources with specific characteristics
- Can't be enchanted/equipped by specific characteristics

### Hexproof and Shroud

- **Hexproof**: Can't be targeted by spells/abilities opponents control
- **Shroud**: Can't be targeted by any spells/abilities

### Changing Targets

The system supports effects that change targets:
- Redirect effects
- "Change the target" effects
- Copying with new targets

## Related Documentation

- [Effect Resolution](effect_resolution.md): How effects are applied once targets are selected
- [Complex Interactions](complex_interactions.md): Handling interactions between targeting effects
- [UI Targeting System](../../game_gui/interaction/targeting.md): UI implementation of targeting
- [MTG Targeting Rules](../../mtg_rules/targeting.md): Official MTG rules for targeting 