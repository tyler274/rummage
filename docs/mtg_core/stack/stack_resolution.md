# Stack Resolution

This document details the implementation of the stack resolution process in Rummage, explaining how spells and abilities on the stack are processed and resolved.

## Resolution Overview

Stack resolution follows the Last-In-First-Out (LIFO) principle. When all players pass priority in succession without taking any actions, the top item on the stack resolves. This process requires careful handling of various card effects, targets, and game state changes.

## Resolution Process

### 1. Prepare for Resolution

The system begins by taking the top item off the stack and preparing it for resolution:

```rust
fn prepare_for_resolution(stack: &mut Stack, world: &mut World) -> Option<StackItem> {
    if stack.items.is_empty() {
        return None;
    }
    
    // Remove the top item from the stack
    let item = stack.items.pop().unwrap();
    
    // Store the currently resolving item for reference
    stack.currently_resolving = Some(item.clone());
    
    Some(item)
}
```

### 2. Check Targets

Before resolution, the system verifies that all targets are still legal:

```rust
fn verify_targets(item: &StackItem, world: &World) -> bool {
    let mut all_targets_legal = true;
    
    for target in &item.targets {
        if !is_legal_target(target, item, world) {
            all_targets_legal = false;
            break;
        }
    }
    
    all_targets_legal
}
```

If a spell or ability has no legal targets remaining, it is countered by game rules:

```rust
if !verify_targets(&item, world) {
    if item.item_type.is_spell() {
        // Counter the spell
        events.send(SpellCounteredEvent {
            spell_id: item.id,
            countered_by: CounterReason::IllegalTargets,
        });
        
        // Move spell card to graveyard
        zone_transitions.send(ZoneTransitionEvent {
            entity: item.source,
            from: Zone::Stack,
            to: Zone::Graveyard,
            cause: ZoneTransitionCause::Countered,
        });
    } else {
        // Counter the ability
        events.send(AbilityCounteredEvent {
            ability_id: item.id,
            countered_by: CounterReason::IllegalTargets,
        });
    }
    
    return;
}
```

### 3. Process Spell Resolution

For spell resolution, the system handles different card types appropriately:

```rust
fn resolve_spell(item: &StackItem, world: &mut World, events: &mut EventWriter<ZoneTransitionEvent>) {
    match item.item_type {
        StackItemType::Spell(CardType::Creature) |
        StackItemType::Spell(CardType::Artifact) |
        StackItemType::Spell(CardType::Enchantment) |
        StackItemType::Spell(CardType::Planeswalker) |
        StackItemType::Spell(CardType::Land) => {
            // Permanent spells - move to battlefield
            events.send(ZoneTransitionEvent {
                entity: item.source,
                from: Zone::Stack,
                to: Zone::Battlefield,
                cause: ZoneTransitionCause::Resolved,
            });
        },
        StackItemType::Spell(CardType::Instant) |
        StackItemType::Spell(CardType::Sorcery) => {
            // Non-permanent spells - move to graveyard
            events.send(ZoneTransitionEvent {
                entity: item.source,
                from: Zone::Stack,
                to: Zone::Graveyard,
                cause: ZoneTransitionCause::Resolved,
            });
        },
        _ => {} // Handle other card types
    }
}
```

### 4. Process Ability Resolution

For ability resolution, the system processes the effects without zone transitions:

```rust
fn resolve_ability(item: &StackItem, world: &mut World) {
    // Abilities don't change zones, only their effects are processed
    match item.item_type {
        StackItemType::Ability(AbilityType::Activated) |
        StackItemType::Ability(AbilityType::Triggered) => {
            // Process ability effects
        },
        _ => {} // Handle other ability types
    }
}
```

### 5. Apply Effects

The system processes each effect of the resolving item:

```rust
fn apply_effects(
    item: &StackItem, 
    world: &mut World,
    effect_events: &mut EventWriter<EffectEvent>
) {
    for effect in &item.effects {
        effect_events.send(EffectEvent {
            effect: effect.clone(),
            source: item.source,
            controller: item.controller,
            targets: item.targets.clone(),
        });
    }
}
```

### 6. Post-Resolution Cleanup

After resolution, the system clears the currently resolving item and checks for triggered abilities:

```rust
fn post_resolution_cleanup(stack: &mut Stack, world: &mut World) {
    // Clear the currently resolving item
    stack.currently_resolving = None;
    
    // Process pending triggers that occurred during resolution
    for trigger in stack.pending_triggers.drain(..) {
        world.spawn_trigger(trigger);
    }
    
    // Check state-based actions
    check_state_based_actions(world);
    
    // Return priority to active player
    set_priority(world.resource::<GameState>().active_player, world);
}
```

## Special Resolution Cases

### Countering Spells and Abilities

When a spell or ability is countered, it never resolves:

```rust
pub fn counter_spell_system(
    mut counter_events: EventReader<CounterSpellEvent>,
    mut stack: ResMut<Stack>,
    mut zone_transitions: EventWriter<ZoneTransitionEvent>,
) {
    for event in counter_events.iter() {
        // Find the spell on the stack
        if let Some(index) = stack.items.iter().position(|item| item.id == event.target) {
            // Remove it from the stack
            let item = stack.items.remove(index);
            
            // Move the card to graveyard
            zone_transitions.send(ZoneTransitionEvent {
                entity: item.source,
                from: Zone::Stack,
                to: Zone::Graveyard,
                cause: ZoneTransitionCause::Countered,
            });
        }
    }
}
```

### Split Second

When resolving items with Split Second, special handling prevents responses:

```rust
pub fn check_split_second(stack: &Stack) -> bool {
    stack.items.iter().any(|item| {
        if let StackItemType::Spell(card_type) = &item.item_type {
            // Check if any spell on the stack has split second
            let has_split_second = world
                .get::<Card>(item.source)
                .map(|card| card.has_ability(Ability::SplitSecond))
                .unwrap_or(false);
                
            return has_split_second;
        }
        false
    })
}
```

## Integration with Other Systems

The stack resolution process integrates with:

1. **Zone System**: Moving cards between zones after resolution
2. **Effect System**: Processing the effects of spells and abilities
3. **Targeting System**: Verifying target legality
4. **Trigger System**: Handling triggers that occur during resolution
5. **State-Based Action System**: Checking state-based actions after resolution

## Implementation Status

The stack resolution implementation currently:

- ✅ Handles basic spell and ability resolution
- ✅ Processes spell types correctly (permanent vs. non-permanent)
- ✅ Validates targets before resolution
- ✅ Implements spell countering
- ✅ Supports triggered abilities that trigger during resolution
- 🔄 Handling complex resolution effects (choices, conditions, etc.)
- 🔄 Implementing special cases like Split Second

---

Next: [Priority System](../turn_structure/priority.md) 