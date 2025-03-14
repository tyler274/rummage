# Combat

This document provides an overview of the combat rules in Magic: The Gathering. For the detailed implementation in Rummage, please see [Combat System](../mtg_core/combat/index.md).

## Combat Phase Overview

The combat phase consists of five steps:

1. **Beginning of Combat Step**
2. **Declare Attackers Step**
3. **Declare Blockers Step**
4. **Combat Damage Step**
5. **End of Combat Step**

## Combat Flow

The flow of combat follows these steps:

### Beginning of Combat Step

- The active player receives priority
- Players can cast instants and activate abilities
- No combat actions yet occur

### Declare Attackers Step

- The active player declares which of their creatures are attacking and what player or planeswalker each is attacking
- Tapped creatures and creatures with summoning sickness (that don't have haste) cannot attack
- Once attackers are declared, triggered abilities trigger and the active player receives priority

### Declare Blockers Step

- The defending player(s) declare which of their untapped creatures are blocking and which attacker each is blocking
- A single creature can block only one attacker unless it has the ability to block multiple attackers
- Multiple creatures can block a single attacker
- After blockers are declared, the active player assigns the combat damage order for each attacking creature that's blocked by multiple creatures
- Triggered abilities trigger and the active player receives priority

### Combat Damage Step

- Combat damage is assigned and dealt simultaneously by all attacking and blocking creatures
- Creatures with first strike or double strike deal damage in a separate combat damage step before creatures without first strike
- Creatures with trample can assign excess damage to the player or planeswalker they're attacking
- After damage is dealt, triggered abilities trigger and the active player receives priority

### End of Combat Step

- Final opportunity to use "until end of combat" effects
- Triggered abilities trigger and the active player receives priority

## Special Combat Rules

### First Strike and Double Strike

- Creatures with first strike deal combat damage before creatures without first strike
- Creatures with double strike deal combat damage twice, once during the first strike damage step and once during the regular damage step
- If any creature has first strike or double strike, there are two combat damage steps

### Trample

- If all creatures blocking an attacking creature with trample are assigned lethal damage, excess damage can be assigned to the player or planeswalker that creature is attacking

### Evasion Abilities

- **Flying**: Can only be blocked by creatures with flying or reach
- **Fear**: Can only be blocked by artifact creatures and/or black creatures
- **Intimidate**: Can only be blocked by artifact creatures and/or creatures that share a color with it
- **Menace**: Can't be blocked except by two or more creatures
- **Shadow**: Can only block or be blocked by creatures with shadow

### Combat Keywords

- **Vigilance**: Creature doesn't tap when attacking
- **Deathtouch**: Any amount of damage dealt by a creature with deathtouch is considered lethal
- **Lifelink**: Damage dealt by a creature with lifelink causes its controller to gain that much life
- **Indestructible**: Creature can't be destroyed by damage or "destroy" effects

## Related Documentation

For the detailed implementation of combat in Rummage, including code examples and integration with other systems, see:

- [Combat System](../mtg_core/combat/index.md): Core implementation details
- [Combat Phases](../mtg_core/combat/combat_phases.md): Detailed phase implementation
- [First Strike and Double Strike](../mtg_core/combat/first_strike.md): First/double strike implementation
- [Combat Damage Calculation](../mtg_core/combat/damage_calculation.md): How damage is calculated and applied
- [Turn Structure](../mtg_core/turn_structure/index.md): How combat fits into the turn
- [Stack](../mtg_core/stack/index.md): How combat interacts with the stack 