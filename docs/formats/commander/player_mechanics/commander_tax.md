# Commander Tax

This document explains the implementation of the Commander Tax mechanic in the Rummage game engine.

## Overview

Commander Tax is a core rule of the Commander format that increases the cost to cast your commander from the command zone by {2} for each previous time you've cast it from the command zone during the game.

The rule ensures that:
1. Players cannot repeatedly abuse their commander's abilities by letting it die and recasting it
2. The game becomes progressively more challenging as players need to invest more mana into recasting their commander
3. Decks need to consider their commander's mana value when building their strategy

## Formula

The cost to cast a commander is calculated as:

```
Total Cost = Base Cost + (2 × Number of Previous Casts)
```

For example:
- First cast: Base cost
- Second cast: Base cost + {2}
- Third cast: Base cost + {4}
- Fourth cast: Base cost + {6}

## Implementation

### Commander Component

```rust
/// Component for commanders
#[derive(Component, Debug, Clone, Reflect)]
pub struct Commander {
    /// Entity ID of the player who owns this commander
    pub owner: Entity,
    /// Number of times this commander has been cast from the command zone
    pub cast_count: u32,
    /// Whether this commander is in the command zone
    pub in_command_zone: bool,
}

impl Default for Commander {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            cast_count: 0,
            in_command_zone: true,
        }
    }
}
```

### Commander Tax Calculation

```rust
/// Calculates the total cost to cast a commander
pub fn calculate_commander_cost(
    base_cost: Mana,
    commander: &Commander,
) -> Mana {
    // Apply commander tax: 2 generic mana for each previous cast
    let tax_amount = commander.cast_count * 2;
    
    // Create a new mana cost with additional generic mana
    let mut total_cost = base_cost.clone();
    total_cost.colorless += tax_amount;
    
    total_cost
}
```

### Casting System

```rust
/// System to handle commander casting
pub fn handle_commander_casting(
    mut commands: Commands,
    mut commanders: Query<(Entity, &mut Commander, &CardCost)>,
    mut cast_events: EventReader<CastCommanderEvent>,
    mut mana_events: EventWriter<ManaCostEvent>,
) {
    for event in cast_events.read() {
        if let Ok((entity, mut commander, cost)) = commanders.get_mut(event.commander) {
            // Calculate total cost with commander tax
            let total_cost = calculate_commander_cost(cost.mana.clone(), &commander);
            
            // Send event to check if player can pay the cost
            mana_events.send(ManaCostEvent {
                player: commander.owner,
                source: entity,
                cost: total_cost,
                on_paid: CastEffect::Commander { entity },
            });
            
            // Increment cast count for next time
            if event.successful {
                commander.cast_count += 1;
                commander.in_command_zone = false;
            }
        }
    }
}
```

## Commander Tax Tracking

The UI displays the current commander tax for each player's commander:

1. Current cast count is shown next to the commander in the command zone
2. The additional cost is displayed when hovering over the commander in the command zone
3. The total cost (base + tax) is shown when attempting to cast the commander

## Special Interactions

### Tax Reduction Effects

Some cards can reduce the commander tax:

```rust
/// Component for effects that reduce commander tax
#[derive(Component, Debug, Clone, Reflect)]
pub struct CommanderTaxReduction {
    /// Amount to reduce the tax by
    pub amount: u32,
    /// Whether this applies to all commanders or specific ones
    pub targets: CommanderTaxReductionTarget,
}

/// Different types of tax reduction targets
#[derive(Debug, Clone, Reflect)]
pub enum CommanderTaxReductionTarget {
    /// Applies to all of your commanders
    AllOwn,
    /// Applies to all commanders (including opponents')
    All,
    /// Applies to specific commanders
    Specific(Vec<Entity>),
}
```

### Partner Commanders

Partner commanders track their tax separately:

```rust
/// When checking if a player has a partner commander
pub fn handle_partner_commander_tax(
    partners: Query<(Entity, &Commander, &Partner)>,
    // ... other parameters
) {
    // Each partner tracks its own commander tax separately
    // ...
}
```

## Testing

### Test Cases

We test the commander tax mechanics with:

1. **Basic Tax Progression**: Verify tax increases correctly with each cast
2. **Tax After Zone Changes**: Ensure tax only increases when cast from command zone
3. **Tax Reduction Effects**: Test cards that reduce commander tax
4. **Partner Commander Interaction**: Verify partners track tax separately

### Example Test

```rust
#[test]
fn test_commander_tax_progression() {
    // Set up test environment
    let mut app = App::new();
    app.add_systems(Update, handle_commander_casting)
        .add_event::<CastCommanderEvent>()
        .add_event::<ManaCostEvent>();
    
    // Create a player and commander
    let player = app.world.spawn_empty().id();
    let base_cost = Mana::new(2, 1, 0, 0, 0, 0); // 2G
    
    let commander = app.world.spawn((
        Commander {
            owner: player,
            cast_count: 0,
            in_command_zone: true,
        },
        CardCost { mana: base_cost.clone() },
    )).id();
    
    // First cast (should be base cost)
    app.world.send_event(CastCommanderEvent {
        commander,
        successful: true,
    });
    app.update();
    
    // Check commander was updated
    let cmdr = app.world.get::<Commander>(commander).unwrap();
    assert_eq!(cmdr.cast_count, 1);
    
    // Second cast (should be base cost + {2})
    app.world.send_event(CastCommanderEvent {
        commander,
        successful: true,
    });
    app.update();
    
    // Verify tax increased
    let cmdr = app.world.get::<Commander>(commander).unwrap();
    assert_eq!(cmdr.cast_count, 2);
    
    // Verify correct cost calculation
    let cmdr = app.world.get::<Commander>(commander).unwrap();
    let total_cost = calculate_commander_cost(base_cost, cmdr);
    assert_eq!(total_cost.colorless, 6); // Base 2 + 4 tax
    assert_eq!(total_cost.green, 1);     // Colored cost unchanged
}
```

## Summary

The Commander Tax mechanic in Rummage is implemented as a dynamic cost increase system that:

1. Tracks cast count per commander
2. Applies the correct tax formula
3. Supports complex interactions like cost reduction effects
4. Handles partner commanders appropriately
5. Provides clear UI feedback on current tax amounts 