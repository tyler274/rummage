# Combat System

## Overview

The Combat system is a fundamental part of Magic: The Gathering, implemented in Rummage according to the comprehensive rules. This section documents the core combat mechanics that apply to all formats.

## Combat Phases

A combat phase consists of the following steps in order:

1. **Beginning of Combat Step** - The last chance for players to cast spells or activate abilities before attackers are declared
2. **Declare Attackers Step** - The active player chooses which creatures will attack and which opponents or planeswalkers they will attack
3. **Declare Blockers Step** - Each defending player chooses which creatures will block and which attacking creatures they will block
4. **Combat Damage Step** - Combat damage is assigned and dealt (with separate first strike and regular damage steps if needed)
5. **End of Combat Step** - The last chance for players to cast spells or activate abilities before the combat phase ends

## Core Combat Mechanics

### Attacking

- Only untapped creatures can be declared as attackers
- Creatures can't attack their controller
- Creatures can attack players or planeswalkers (unless modified by card effects)
- Creatures with Defender can't attack
- Creatures with Summoning Sickness (entered battlefield this turn) can't attack unless they have Haste

### Blocking

- Tapped creatures can't block
- Each blocking creature can only block one attacker (unless modified by card effects)
- Multiple creatures can block a single attacker
- Creatures can only block attackers that are attacking their controller or planeswalkers controlled by their controller

### Combat Damage

- Attacking creatures deal damage equal to their power
- Blocking creatures deal damage equal to their power
- If multiple creatures block a single attacker, the attacker's controller assigns the damage among them
- Combat damage is dealt simultaneously (unless first strike or double strike is involved)
- Combat damage to players causes life loss
- Combat damage to planeswalkers removes loyalty counters
- Combat damage to creatures causes damage marked on them

### Special Combat Abilities

- **First Strike**: Creatures with first strike deal combat damage before creatures without first strike
- **Double Strike**: Creatures with double strike deal damage in both the first strike and regular damage steps
- **Trample**: Excess combat damage can be assigned to the defending player or planeswalker
- **Vigilance**: Creatures with vigilance don't tap when attacking
- **Deathtouch**: Any amount of damage from a creature with deathtouch is enough to destroy the damaged creature
- **Lifelink**: Damage dealt by a creature with lifelink causes its controller to gain that much life

## Implementation Details

The combat system is implemented through a series of components and systems that manage the state of creatures in combat, process combat events, and enforce the rules of combat.

## Extensions for Formats

Different formats may extend or modify these core combat mechanics:

- **Commander Format**: Adds commander damage tracking
- **Two-Headed Giant**: Modifies how creatures can attack and block
- **Planechase**: May add special combat rules based on the current plane

For format-specific extensions like Commander's combat rules, see the respective format documentation.

---

Next: [Combat Phases](combat_phases.md) 