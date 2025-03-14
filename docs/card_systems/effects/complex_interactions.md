# Complex Interactions

Magic: The Gathering is known for its intricate rule system and complex card interactions. This document outlines how Rummage handles these complex card interactions to ensure correct game behavior.

## Layering System

MTG uses a layering system to determine how continuous effects are applied. Rummage implements this system following the comprehensive rules:

### Layer Order

1. **Copy effects**: Effects that make an object a copy of another object
2. **Control-changing effects**: Effects that change control of an object
3. **Text-changing effects**: Effects that change the text of an object
4. **Type-changing effects**: Effects that change the types of an object
5. **Color-changing effects**: Effects that change the colors of an object
6. **Ability-adding/removing effects**: Effects that add or remove abilities
7. **Power/toughness-changing effects**: Effects that modify power and toughness

### Implementation

The layering system is implemented using a multi-pass approach:

```rust
// Example of layered effect application
fn apply_continuous_effects(
    mut card_entities: Query<(Entity, &mut CardState)>,
    continuous_effects: Query<&ContinuousEffect>,
    timestamps: Query<&Timestamp>,
) {
    // Collect all effects
    let mut effects_by_layer = [Vec::new(); 7];
    for (entity, effect) in continuous_effects.iter() {
        effects_by_layer[effect.layer as usize].push((entity, effect));
    }
    
    // Sort effects within each layer by timestamp
    for layer_effects in &mut effects_by_layer {
        layer_effects.sort_by_key(|(entity, _)| timestamps.get(*entity).unwrap().0);
    }
    
    // Apply effects layer by layer
    for (layer_idx, layer_effects) in effects_by_layer.iter().enumerate() {
        for (_, effect) in layer_effects {
            apply_effect_in_layer(layer_idx, effect, &mut card_entities);
        }
    }
}
```

## Dependency Chains

Some effects depend on the outcome of other effects. The system handles these dependencies by:

1. Building a dependency graph of effects
2. Topologically sorting the graph
3. Applying effects in the correct order

## State-Based Action Loops

Some complex interactions can create loops of state-based actions. The system:

1. Detects potential loops
2. Applies a maximum iteration limit
3. Resolves terminal states correctly

## Replacement Effects

Replacement effects modify how events occur. They're handled by:

1. Tracking the original event
2. Applying applicable replacement effects
3. Determining the order when multiple effects apply
4. Generating the modified event

## Triggered Ability Resolution

When multiple abilities trigger simultaneously:

1. APNAP order is used (Active Player, Non-Active Player)
2. The controller of each ability chooses the order for their triggers
3. Abilities are placed on the stack in that order

## Example: Layers in Action

Here's an example of how the layers system handles a complex interaction:

```
Initial card: Grizzly Bears (2/2 green creature - Bear)
Effect 1: Darksteel Mutation (becomes 0/1 artifact creature with no abilities)
Effect 2: +1/+1 counter
Effect 3: Painter's Servant (becomes blue)
```

The system processes this as:

1. Layer 4 (Type): Now an artifact creature
2. Layer 5 (Color): Now a blue artifact creature
3. Layer 6 (Abilities): Now has no abilities
4. Layer 7 (P/T): Base P/T becomes 0/1, then +1/+1 counter makes it 1/2

Final result: 1/2 blue artifact creature with no abilities.

## Related Documentation

- [Effect Resolution](effect_resolution.md): How individual effects are resolved
- [Targeting](targeting.md): How targeting works with complex effects
- [State-Based Actions](../../mtg_core/state_actions/index.md): Automatic game rules that interact with effects
- [Stack](../../mtg_core/stack/index.md): How effects are ordered for resolution 