# Combat

This section covers the implementation of the combat system in the Commander format.

## Contents

- [Combat System](combat_system.md) - Core combat mechanics implementation
- [Combat Phases](combat_phases.md) - Overview of the combat phase structure

### Combat Steps
- [Beginning of Combat](beginning_of_combat.md) - Initialization of combat and "beginning of combat" triggers
- [Declare Attackers](declare_attackers.md) - Attack declaration, restrictions, and requirements 
- [Declare Blockers](declare_blockers.md) - Block declaration, restrictions, and requirements
- [First Strike Damage](first_strike_damage.md) - Special damage step for first strike and double strike
- [Combat Damage](combat_damage.md) - Damage assignment, ordering, and resolution
- [End of Combat](end_of_combat.md) - Combat cleanup and "end of combat" triggers

### Combat Mechanics
- [Commander Damage](commander_damage.md) - Tracking and implementation of commander damage
- [Multiplayer Combat](multiplayer_combat.md) - Special rules for combat with multiple players
- [Combat Verification](combat_verification.md) - Validation of combat decisions and actions
- [Combat Abilities](combat_abilities.md) - Implementation of combat-related abilities

## Combat in Commander

The combat section defines how combat works in Commander games:

- Standard combat phases (beginning of combat, declare attackers, declare blockers, combat damage, end of combat)
- Commander damage tracking (21+ combat damage from a single commander causes a loss)
- Multiplayer attack declaration (attacking different players in the same combat)
- Special handling for commander-specific combat abilities
- Handling combat triggers in multiplayer scenarios

Combat in Commander is particularly complex due to the multiplayer nature of the format and the special rule regarding commander damage, which adds an additional loss condition to the game. The implementation must be robust, handling all edge cases while maintaining good performance and compatibility with the rest of the game engine.

## Related Systems

Combat interacts with several other systems in the Commander implementation:

- [Random Mechanics](../game_mechanics/random_mechanics.md) - For combat-related coin flips and dice rolls
- [Stack and Priority](../stack_and_priority/index.md) - For combat triggers and responses
- [State-Based Actions](../game_mechanics/state_based_actions.md) - For checking commander damage thresholds

## Testing

The [tests](tests/) directory contains comprehensive test cases for validating correct combat implementation, including commander damage tracking, multiplayer combat scenarios, and complex ability interactions. 