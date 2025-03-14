# Commander-Specific Stack and Priority

## Overview

This section covers the Commander-specific aspects of the stack and priority system in Rummage. For core stack and priority mechanics that apply to all formats, see the [MTG Core Stack and Priority](../../mtg_core/stack/index.md) documentation.

## Commander Stack Extensions

The Commander format extends the basic MTG stack and priority system with these key features:

1. **Multiplayer Priority** - Modified priority passing for games with 3+ players
2. **Commander Casting** - Special handling for casting commanders from the command zone
3. **Political Interaction** - Support for verbal agreements and diplomacy during priority windows

## Contents

- [Stack Implementation](stack_implementation.md) - Commander-specific stack extensions
- [Priority Passing](priority_passing.md) - Priority in multiplayer Commander games
- [Special Timing Rules](special_timing.md) - Format-specific timing considerations

## Key Commander Stack Features

### Multiplayer Priority Flow

In multiplayer Commander games, priority passes in turn order starting with the active player:

```rust
pub fn get_next_priority_player(
    current_player: Entity,
    player_order: &Vec<Entity>,
) -> Entity {
    let current_index = player_order.iter().position(|&p| p == current_player).unwrap();
    let next_index = (current_index + 1) % player_order.len();
    player_order[next_index]
}
```

This system handles priority passing among all players in the game, ensuring each player has an opportunity to respond before a spell or ability resolves.

### Commander Casting from Command Zone

When a player casts their commander from the command zone, special rules apply:

- The commander tax increases the cost by {2} for each previous cast from the command zone
- The commander moves from the command zone to the stack
- All players have an opportunity to respond before it resolves

### Political Considerations

While not encoded in the rules, the Commander format involves political dynamics:

- Players may make deals about actions they'll take when they have priority
- Verbal agreements can influence who players target with spells or attacks
- Priority windows are important moments for diplomatic negotiation

## Command Zone Integration

The stack system integrates closely with the [Command Zone](../zones/command_zone.md) implementation to handle:

- Moving commanders between the command zone and the stack
- Tracking commander tax for casting costs
- Handling commanders with alternative casting methods

## Related Systems

Commander stack and priority interact with several other systems:

- [Game Zones](../zones/index.md) - Especially the Command Zone
- [Player Mechanics](../player_mechanics/index.md) - For managing player turns and actions
- [Special Rules](../special_rules/index.md) - For Commander-specific abilities

## Testing

Comprehensive tests verify correct handling of stack and priority in multiplayer Commander games, with special attention to:

- Correct priority order in multiplayer scenarios
- Commander tax application when casting from the command zone
- Interaction of simultaneous triggered abilities from multiple players

---

Next: [Stack Implementation](stack_implementation.md) 