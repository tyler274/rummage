# Combat Damage Calculation

## Overview

Combat damage calculation is a critical aspect of the Magic: The Gathering combat system. This document explains how damage is calculated, assigned, and applied during combat in Rummage's implementation.

## Damage Assignment

### Basic Rules

1. Each attacking or blocking creature assigns combat damage equal to its power
2. Attacking creatures that aren't blocked assign their damage to the player or planeswalker they're attacking
3. Blocking creatures assign their damage to the creature they're blocking
4. Blocked creatures assign their damage to the creatures blocking them

### Multiple Blockers

When multiple creatures block a single attacker:

1. The attacking player puts the blocking creatures in a damage assignment order
2. The attacker must assign at least lethal damage to each creature in the order before assigning damage to the next creature
3. "Lethal damage" is damage equal to the creature's toughness minus damage already marked on it

```rust
pub fn determine_damage_assignment_order(
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    blocker_groups: Query<(&Creature, &Health, &Blocking)>,
) {
    for (attacker_entity, blockers) in combat_state.blockers_per_attacker.iter() {
        if blockers.len() > 1 {
            // Request damage assignment order from attacking player
            // Store the order in the combat state
            // ...
        }
    }
}
```

## Damage Application

Once damage is assigned, it is applied simultaneously:

1. Damage to creatures is marked on them but doesn't immediately reduce toughness
2. Damage to players reduces their life total
3. Damage to planeswalkers removes loyalty counters
4. Special abilities like deathtouch and lifelink take effect at this time

```rust
pub fn apply_combat_damage(
    combat_state: Res<CombatState>,
    mut creatures: Query<(Entity, &Creature, &mut Health)>,
    mut players: Query<(Entity, &Player, &mut Life)>,
    mut planeswalkers: Query<(Entity, &Planeswalker, &mut Loyalty)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    // Calculate and apply damage from all sources
    // ...
    
    // For creatures
    for (target, damage_amount) in creature_damage.iter() {
        if let Ok((_, _, mut health)) = creatures.get_mut(*target) {
            health.marked_damage += *damage_amount;
            // Generate damage event
            damage_events.send(DamageEvent {
                source: source_entity,
                target: *target,
                amount: *damage_amount,
                is_combat_damage: true,
            });
        }
    }
    
    // For players and planeswalkers (similar logic)
    // ...
}
```

## Special Damage Considerations

### Deathtouch

Deathtouch modifies damage assignment rules:

- Any amount of damage from a source with deathtouch is considered lethal damage
- When assigning damage from an attacker with deathtouch to multiple blockers, only 1 damage needs to be assigned to each creature before moving to the next

### Trample

Trample allows excess damage to be assigned to the player or planeswalker:

- The attacker must still assign lethal damage to all blockers
- Any remaining damage can be assigned to the defending player or planeswalker
- If all blocking creatures are removed from combat, all damage is assigned to the player or planeswalker

```rust
pub fn handle_trample_damage(
    combat_state: Res<CombatState>,
    creatures: Query<(Entity, &Creature, &Attack, Option<&Trample>)>,
    blockers: Query<(Entity, &Creature, &Health, &Blocking)>,
    mut player_damage: ResMut<HashMap<Entity, u32>>,
) {
    for (attacker, creature, attack, trample) in creatures.iter() {
        if trample.is_some() {
            // Calculate how much damage is needed for blockers
            // Assign excess to the player or planeswalker
            // ...
        }
    }
}
```

### Protection and Prevention

Some effects modify how damage is dealt or received:

- Protection prevents damage from sources with specific characteristics
- Prevention effects can replace some or all damage that would be dealt
- These effects are applied during damage calculation, before damage is actually dealt

## Damage Resolution

After combat damage is dealt, the game checks for state-based actions:

1. Creatures with lethal damage are destroyed
2. Players with 0 or less life lose the game
3. Planeswalkers with 0 loyalty counters are put into their owner's graveyard

```rust
pub fn check_combat_damage_results(
    creatures: Query<(Entity, &Creature, &Health)>,
    players: Query<(Entity, &Player, &Life)>,
    planeswalkers: Query<(Entity, &Planeswalker, &Loyalty)>,
    mut destroy_events: EventWriter<DestroyPermanentEvent>,
    mut player_loss_events: EventWriter<PlayerLossEvent>,
) {
    // Check for destroyed creatures
    for (entity, _, health) in creatures.iter() {
        if health.marked_damage >= health.toughness {
            destroy_events.send(DestroyPermanentEvent {
                entity: entity,
                reason: DestructionReason::LethalDamage,
            });
        }
    }
    
    // Check for player losses and planeswalker destruction
    // ...
}
```

## Related Documentation

- [Combat Phases](combat_phases.md)
- [First Strike and Double Strike](first_strike.md) 