# Effect Resolution

Effect resolution is the process by which a card's effects are applied to the game state. This document outlines how Rummage implements this core game mechanic.

## Resolution Process

The resolution of an effect follows these steps:

1. **Validation**: Check if the effect can legally resolve
2. **Target Confirmation**: Verify targets are still legal
3. **Effect Application**: Apply the effect to the game state
4. **Triggered Abilities**: Check for abilities triggered by the effect
5. **State-Based Actions**: Check for state-based actions after resolution

## Implementation

Effect resolution is implemented using Bevy's entity-component-system architecture:

```rust
// Example of a system that resolves damage effects
fn resolve_damage_effects(
    mut commands: Commands,
    mut event_reader: EventReader<ResolveEffectEvent>,
    mut damage_effects: Query<(Entity, &DamageEffect, &EffectTargets)>,
    mut targets: Query<&mut Health>,
    mut damaged_event_writer: EventWriter<DamagedEvent>
) {
    for resolve_event in event_reader.read() {
        if let Ok((effect_entity, damage_effect, effect_targets)) = 
            damage_effects.get(resolve_event.effect_entity) {
            
            // Apply damage to each target
            for &target in &effect_targets.entities {
                if let Ok(mut health) = targets.get_mut(target) {
                    health.current -= damage_effect.amount;
                    
                    // Send a damage event for other systems to react to
                    damaged_event_writer.send(DamagedEvent {
                        entity: target,
                        amount: damage_effect.amount,
                        source: resolve_event.source_entity,
                    });
                }
            }
            
            // Clean up the resolved effect
            commands.entity(effect_entity).despawn();
        }
    }
}
```

## Effect Types

Different types of effects have specialized resolution procedures:

### One-Shot Effects

These effects happen once and are immediately complete:
- Damage dealing
- Card drawing
- Life gain/loss

### Continuous Effects

These effects modify the game state for a duration:
- Static abilities
- Enchantment effects
- "Until end of turn" effects

### Replacement Effects

These effects replace one event with another:
- Damage prevention
- Alternative costs
- "Instead" effects

## Effect Timing

Effects respect the timing rules of Magic:
- **Instant speed**: Can be played anytime priority is held
- **Sorcery speed**: Only during main phases of the controller's turn
- **Triggered**: When certain conditions are met
- **Static**: Continuous effects that always apply

## Error Handling

Effect resolution includes error handling for cases where:
- Targets become illegal
- Effect requirements can't be met
- Rules prevent the effect from resolving

## Related Documentation

- [Targeting](targeting.md): How targets are selected and validated
- [Complex Interactions](complex_interactions.md): Handling multiple interacting effects
- [Stack](../../mtg_core/stack/index.md): How effects wait for resolution
- [State-Based Actions](../../mtg_core/state_actions/index.md): Automatic game actions after effects resolve 