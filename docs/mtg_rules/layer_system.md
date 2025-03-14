# Layer System

## Overview

The Layer System is one of Magic: The Gathering's most complex rule mechanisms, designed to handle the application of continuous effects in a consistent, predictable order. This document explains how the Layer System is implemented in Rummage and how it resolves potentially conflicting continuous effects.

## The Seven Layers

According to the MTG Comprehensive Rules (section 613), continuous effects are applied in a specific order across seven distinct layers:

1. **Copy Effects** - Effects that modify how an object is copied
2. **Control-Changing Effects** - Effects that change control of an object
3. **Text-Changing Effects** - Effects that change an object's text
4. **Type-Changing Effects** - Effects that change an object's types, subtypes, or supertypes
5. **Color-Changing Effects** - Effects that change an object's colors
6. **Ability-Adding/Removing Effects** - Effects that add or remove abilities
7. **Power/Toughness-Changing Effects** - Effects that modify a creature's power and/or toughness

Within the Power/Toughness layer (layer 7), effects are applied in this sub-order:
   - 7a. Effects from characteristic-defining abilities
   - 7b. Effects that set power and/or toughness to a specific value
   - 7c. Effects that modify power and/or toughness (but don't set them)
   - 7d. Effects from counters on the creature
   - 7e. Effects that switch power and toughness

## Implementation in Rummage

In Rummage, the Layer System is implemented using a combination of components, resources, and systems:

```rust
/// Resource that manages continuous effects
#[derive(Resource)]
pub struct ContinuousEffectsSystem {
    pub effects: Vec<ContinuousEffect>,
    pub timestamp_counter: u64,
}

/// Representation of a continuous effect
#[derive(Clone, Debug)]
pub struct ContinuousEffect {
    pub id: Uuid,
    pub source: Entity,
    pub layer: Layer,
    pub effect_type: ContinuousEffectType,
    pub targets: EffectTargets,
    pub duration: EffectDuration,
    pub timestamp: u64,
    pub dependency_info: Option<DependencyInfo>,
}

/// Enumeration of the different layers
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
    CopyEffects = 1,
    ControlChangingEffects = 2,
    TextChangingEffects = 3,
    TypeChangingEffects = 4,
    ColorChangingEffects = 5,
    AbilityAddingEffects = 6,
    PowerToughnessCharDefining = 7,
    PowerToughnessSetValue = 8,
    PowerToughnessModify = 9,
    PowerToughnessCounters = 10,
    PowerToughnessSwitch = 11,
}
```

### Application Process

Continuous effects are applied through a multi-stage process:

```rust
pub fn apply_continuous_effects_system(
    mut commands: Commands,
    effects_system: Res<ContinuousEffectsSystem>,
    mut card_query: Query<(Entity, &mut Card, &mut ContinuousEffectsApplied)>,
    // Other query parameters...
) {
    // Group effects by layer
    let mut effects_by_layer: HashMap<Layer, Vec<&ContinuousEffect>> = HashMap::new();
    
    for effect in &effects_system.effects {
        if effect.is_active() {
            effects_by_layer.entry(effect.layer)
                .or_insert_with(Vec::new)
                .push(effect);
        }
    }
    
    // Sort each layer's effects by timestamp
    for effects in effects_by_layer.values_mut() {
        effects.sort_by_key(|effect| effect.timestamp);
    }
    
    // Apply effects layer by layer
    for layer in Layer::iter() {
        if let Some(effects) = effects_by_layer.get(&layer) {
            apply_layer_effects(layer, effects, &mut commands, &mut card_query);
        }
    }
}
```

### Dependency Handling

One of the most intricate parts of the Layer System is handling dependencies between effects. According to the rules, if applying one effect would change what another effect does or what it applies to, there's a dependency:

```rust
pub fn detect_dependencies(effects: &mut [ContinuousEffect]) {
    for i in 0..effects.len() {
        for j in 0..effects.len() {
            if i != j {
                if does_effect_depend_on(
                    &effects[i], 
                    &effects[j], 
                    // Other parameters to check dependency
                ) {
                    effects[i].dependency_info = Some(DependencyInfo {
                        depends_on: effects[j].id,
                    });
                }
            }
        }
    }
    
    // Reorder effects based on dependencies
    reorder_effects_by_dependencies(effects);
}
```

## Layer System Examples

### Example 1: Layers 7b and 7c Interaction

Consider these two effects:
1. Humility (Layer 7b): "All creatures lose all abilities and have base power and toughness 1/1."
2. Glorious Anthem (Layer 7c): "Creatures you control get +1/+1."

```rust
// First, in layer 7b, set all creatures to 1/1
for (entity, mut card, _) in creature_query.iter_mut() {
    if let CardDetails::Creature(ref mut creature) = card.details.details {
        // Apply Humility effect (Layer 7b - sets P/T)
        creature.power = 1;
        creature.toughness = 1;
    }
}

// Then, in layer 7c, apply the +1/+1 bonus to controlled creatures
for (entity, mut card, controller) in creature_query.iter_mut() {
    if controller.controller == anthem_controller {
        if let CardDetails::Creature(ref mut creature) = card.details.details {
            // Apply Glorious Anthem effect (Layer 7c - modifies P/T)
            creature.power += 1;
            creature.toughness += 1;
        }
    }
}
```

### Example 2: Dependency Between Layers

Consider these effects:
1. Conspiracy (Layer 4): "All creatures in your hand, library, graveyard, and battlefield are Elves."
2. Elvish Champion (Layer 6): "All Elves get +1/+1 and have forestwalk."

Here, the effect of Elvish Champion depends on what creatures are Elves, which is affected by Conspiracy:

```rust
// Detect the dependency
pub fn detect_type_dependent_abilities(effects: &mut [ContinuousEffect]) {
    for i in 0..effects.len() {
        for j in 0..effects.len() {
            if i != j {
                let effect_i = &effects[i];
                let effect_j = &effects[j];
                
                // If effect_i adds abilities to a specific type
                if let ContinuousEffectType::AddAbilities { type_requirement: Some(type_req), .. } = &effect_i.effect_type {
                    // And effect_j changes types
                    if let ContinuousEffectType::ChangeTypes { .. } = &effect_j.effect_type {
                        // Then effect_i depends on effect_j
                        effects[i].dependency_info = Some(DependencyInfo {
                            depends_on: effect_j.id,
                        });
                    }
                }
            }
        }
    }
}
```

## Timestamp Ordering

When multiple effects apply in the same layer and don't have dependencies, they're applied in timestamp order:

```rust
pub fn add_continuous_effect(
    mut effects_system: ResMut<ContinuousEffectsSystem>,
    effect_data: ContinuousEffectData,
) {
    let timestamp = effects_system.timestamp_counter;
    effects_system.timestamp_counter += 1;
    
    let effect = ContinuousEffect {
        id: Uuid::new_v4(),
        layer: effect_data.layer,
        effect_type: effect_data.effect_type,
        // Other fields...
        timestamp,
        dependency_info: None,
    };
    
    effects_system.effects.push(effect);
}
```

## Characteristic-Defining Abilities (CDAs)

Layer 7a handles characteristic-defining abilities, which are abilities that define a creature's power/toughness, like Tarmogoyf's "*/*+1 equal to the number of card types in all graveyards":

```rust
pub fn apply_characteristic_defining_abilities(
    mut card_query: Query<(Entity, &mut Card, &CardAbilities)>,
) {
    for (entity, mut card, abilities) in card_query.iter_mut() {
        for ability in &abilities.characteristic_defining {
            match ability {
                CharacteristicDefiningAbility::PowerToughness(calculator) => {
                    if let CardDetails::Creature(ref mut creature) = card.details.details {
                        let (power, toughness) = calculator(entity);
                        creature.power = power;
                        creature.toughness = toughness;
                    }
                },
                // Other CDA types...
            }
        }
    }
}
```

## Testing the Layer System

Testing the layer system is complex due to the numerous potential interactions. Rummage uses a combination of unit tests for each layer and integration tests for complex scenarios:

```rust
#[test]
fn test_layers_example_scenario() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, apply_continuous_effects_system)
       .init_resource::<ContinuousEffectsSystem>();
    
    // Create test cards and effects
    let humility = spawn_humility(&mut app.world);
    let anthem = spawn_anthem(&mut app.world);
    let test_creature = spawn_test_creature(&mut app.world, 2, 2);
    
    // Run systems to apply effects
    app.update();
    
    // Verify the combined effect: creature should be 2/2
    let (_, card, _) = app.world
        .query::<(Entity, &Card, &ContinuousEffectsApplied)>()
        .get(&app.world, test_creature)
        .unwrap();
    
    if let CardDetails::Creature(creature) = &card.details.details {
        assert_eq!(creature.power, 2);
        assert_eq!(creature.toughness, 2);
    } else {
        panic!("Expected a creature!");
    }
}
```

## Edge Cases and Advanced Interactions

The layer system must handle many edge cases:

### Self-Replacement Effects

When an effect modifies how itself works:

```rust
pub fn handle_self_replacement_effects(effects: &mut [ContinuousEffect]) {
    // Identify and mark self-replacement effects
    let mut self_replacing = Vec::new();
    
    for (idx, effect) in effects.iter().enumerate() {
        if is_self_replacing(effect) {
            self_replacing.push(idx);
        }
    }
    
    // Apply them in a special order
    // ...
}
```

### Copiable Values

Layer 1 deals with what values are copied when an effect copies an object:

```rust
pub fn determine_copiable_values(
    source: Entity,
    target: Entity,
    card_query: &Query<(Entity, &Card)>,
) -> Card {
    // Get the base card to copy
    let (_, source_card) = card_query.get(source).unwrap();
    
    // Create a copy with only the copiable values
    Card {
        name: source_card.name.clone(),
        cost: source_card.cost.clone(),
        type_info: source_card.type_info.clone(),
        details: source_card.details.clone(),
        rules_text: source_card.rules_text.clone(),
        keywords: KeywordAbilities::default(), // Keywords aren't copied in layer 1
    }
}
```

## Conclusion

The Layer System is one of the most complex parts of the Magic: The Gathering rules, but its step-by-step approach ensures that continuous effects are applied consistently. Rummage's implementation carefully follows these rules to ensure games play out correctly, even in the most complex scenarios.

---

Next: [Triggered Abilities](triggered_abilities.md) 