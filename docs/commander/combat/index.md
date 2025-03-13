# Commander-Specific Combat Mechanics

## Overview

This section covers the Commander-specific combat mechanics in the Rummage game engine. For core combat mechanics that apply to all formats, see the [MTG Core Combat](../../mtg_core/combat/index.md) documentation.

## Commander Combat Extensions

Commander extends the basic MTG combat system with these key mechanics:

1. **Commander Damage** - A player who has been dealt 21 or more combat damage by the same commander loses the game
2. **Multiplayer Combat** - Special considerations for attacking and blocking in a multiplayer environment
3. **Commander-specific Combat Abilities** - Handling abilities that interact with commander status

## Contents

- [Commander Damage](commander_damage.md) - Implementation of the 21-damage loss condition
- [Multiplayer Combat](multiplayer_combat.md) - Special rules for combat with multiple players
- [Combat Verification](combat_verification.md) - Validation of combat decisions in multiplayer scenarios
- [Combat Abilities](combat_abilities.md) - Implementation of commander-specific combat abilities

## Key Commander Combat Features

### Commander Damage Tracking

The system tracks commander damage separately from regular damage:

```rust
#[derive(Component)]
pub struct CommanderDamageTracker {
    // Maps commander entity -> damage dealt to player
    pub damage_taken: HashMap<Entity, u32>,
}
```

When a player takes 21 or more combat damage from the same commander, they lose the game regardless of their life total.

### Multiplayer Combat Dynamics

Commander's multiplayer format introduces unique combat dynamics:

- Players can attack any opponent, not just the player to their left/right
- Political considerations affect attack and block decisions
- Players can make deals regarding combat (though these aren't enforced by the game rules)

### Combat in Multiplayer Politics

Combat is central to the political dynamics of Commander:

- Attacks signal aggression and can lead to retaliation
- Defending other players can forge temporary alliances
- Commander damage creates an additional threat vector beyond life totals

## Related Systems

Commander combat interacts with several other systems:

- [Player Mechanics](../player_mechanics/index.md) - For life total and commander damage tracking
- [Game Mechanics](../game_mechanics/index.md) - For state-based actions that check commander damage thresholds
- [Special Rules](../special_rules/index.md) - For politics and multiplayer considerations

## Testing

The [tests](tests/) directory contains comprehensive test cases for validating commander-specific combat mechanics, with special focus on commander damage tracking and multiplayer scenarios.

---

Next: [Commander Damage](commander_damage.md) 