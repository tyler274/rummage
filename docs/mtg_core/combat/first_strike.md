# First Strike and Double Strike

## Overview

First Strike and Double Strike are two of the most important keyword abilities in Magic: The Gathering that modify how creatures deal combat damage. They create a special combat damage step that occurs before the regular combat damage step.

## First Strike

First Strike allows creatures to deal combat damage before creatures without First Strike.

### Rules

- Creatures with First Strike deal damage in the "first strike combat damage step" which happens before the regular combat damage step
- Creatures without First Strike or Double Strike don't deal or receive combat damage in this step
- If no creatures with First Strike or Double Strike are involved in combat, this step is skipped entirely
- First Strike doesn't provide any additional advantage in creature-to-creature combat outside of combat damage timing

### Implementation

```rust
#[derive(Component)]
pub struct FirstStrike;

pub fn handle_first_strike_damage(
    combat_state: Res<CombatState>,
    creatures: Query<(Entity, &Creature, &Attack, &BlockedBy, Option<&FirstStrike>, Option<&DoubleStrike>)>,
    mut health: Query<&mut Health>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    // Identify creatures with First Strike or Double Strike
    let first_strikers = creatures.iter()
        .filter(|(_, _, _, _, first_strike, double_strike)| 
            first_strike.is_some() || double_strike.is_some());
            
    // Process First Strike damage
    for (entity, creature, attack, blocked_by, _, _) in first_strikers {
        // Calculate and assign damage
        // ...
    }
}
```

## Double Strike

Double Strike combines the benefits of First Strike with the ability to deal damage again in the regular combat damage step.

### Rules

- Creatures with Double Strike deal combat damage in both the first strike combat damage step AND the regular combat damage step
- This effectively allows them to deal twice their power in damage during combat
- If a Double Strike creature is removed from combat after the first strike step (e.g., by being destroyed), it won't deal damage in the regular step
- Double Strike doesn't increase the amount of damage dealt in each individual step - it allows the creature to deal damage in both steps

### Implementation

```rust
#[derive(Component)]
pub struct DoubleStrike;

pub fn handle_regular_combat_damage(
    combat_state: Res<CombatState>,
    creatures: Query<(Entity, &Creature, &Attack, &BlockedBy, Option<&DoubleStrike>)>,
    mut health: Query<&mut Health>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    // Process Regular Combat Damage
    for (entity, creature, attack, blocked_by, _) in creatures.iter() {
        // All creatures without First Strike always deal damage here
        // Creatures with Double Strike also deal damage here
        // ...
    }
}
```

## Interactions with Other Abilities

First Strike and Double Strike interact with other combat abilities in specific ways:

- **Deathtouch**: A creature with both First Strike/Double Strike and Deathtouch can destroy blocking/blocked creatures before they deal damage
- **Lifelink**: A creature with both Double Strike and Lifelink will cause its controller to gain life twice
- **Trample**: A creature with both First Strike/Double Strike and Trample can deal excess damage to players in both damage steps

## Example Combat Scenarios

### First Strike vs. Non-First Strike

When a 2/2 creature with First Strike blocks or is blocked by a 2/2 creature without First Strike:
1. First Strike creature deals 2 damage in the first strike damage step
2. The other creature is destroyed before it can deal damage in the regular damage step

### Double Strike vs. Regular Creature

When a 2/2 creature with Double Strike blocks or is blocked by a 3/3 creature without First Strike:
1. Double Strike creature deals 2 damage in the first strike damage step
2. The 3/3 creature survives with 1 toughness remaining
3. Both creatures deal damage in the regular damage step (2 more from Double Strike, 3 from the regular creature)
4. Both creatures are destroyed

## Related Documentation

- [Combat Phases](combat_phases.md)
- [Combat Damage Calculation](damage_calculation.md) 