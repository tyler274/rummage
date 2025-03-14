# Commander Format

This section documents the implementation of the Commander (EDH) format in Rummage, a multiplayer format that emphasizes social gameplay, unique deck construction constraints, and strategic depth.

## Format Overview

Commander (formerly known as Elder Dragon Highlander or EDH) is a sanctioned Magic: The Gathering format with these defining characteristics:

| Feature | Description | Implementation |
|---------|-------------|----------------|
| **Deck Construction** | 100-card singleton decks (no duplicates except basic lands) | Deck validation systems |
| **Commander** | A legendary creature that leads your deck | Command zone and casting mechanics |
| **Color Identity** | Deck colors must match commander's color identity | Deck validation and color checking |
| **Life Total** | 40 starting life (vs. standard 20) | Modified game initialization |
| **Commander Damage** | 21 combat damage from a single commander causes loss | Per-commander damage tracking |
| **Multiplayer Focus** | Designed for 3-6 players | Turn ordering and multiplayer mechanics |

## Documentation Structure

The documentation is organized into the following sections:

- [Overview](overview/index.md) - High-level overview of the Commander format and implementation approach
- [Game Mechanics](game_mechanics/index.md) - Core game state and mechanics implementation
  - Game State Management
  - State-Based Actions
  - Random Mechanics (coin flips, dice rolls)
- [Player Mechanics](player_mechanics/index.md) - Player-specific rules and interactions
  - Life Total Management
  - Commander Tax
  - Color Identity
- [Game Zones](zones/index.md) - Implementation of game zones, especially the Command Zone
  - Command Zone
  - Zone Transfers
  - Zone-specific Rules
- [Turns and Phases](turns_and_phases/index.md) - Turn structure and phase management
  - Turn Order
  - Phase Management
  - Multiplayer Considerations
- [Stack and Priority](stack_and_priority/index.md) - Stack implementation and priority system
  - Priority Passing
  - Stack Resolution
  - Special Timing Rules
- [Combat](combat/index.md) - Combat mechanics including commander damage
  - Combat Phases
  - Commander Damage Tracking
  - Multiplayer Combat
- [Special Rules](special_rules/index.md) - Format-specific rules and unique mechanics
  - Partner Commanders
  - Commander Death Triggers
  - Commander-specific Abilities
- [Core Integration](core_integration.md) - How Commander extends MTG core rules

## Key Mechanics Implementation

### Command Zone

The Command Zone serves as the foundation of the format:

```rust
// Command Zone implementation
#[derive(Component)]
struct CommandZone {
    owner: Entity,
    contents: Vec<Entity>,
}

// Commander component
#[derive(Component)]
struct Commander {
    owner: Entity,
    cast_count: u32,
}
```

Key implementations:
- Commanders start in the Command Zone
- Zone transfer options when commanders change zones
- Commander Tax calculation (`2` additional mana per previous cast)

### Commander Damage Tracking

```rust
// Tracking damage from each commander
#[derive(Component)]
struct CommanderDamageTracker {
    // Maps commander entities to damage received
    damage_taken: HashMap<Entity, u32>,
}

// System that checks for commander damage loss condition
fn check_commander_damage_loss(
    tracker: Query<(Entity, &CommanderDamageTracker, &Player)>,
    mut game_events: EventWriter<GameEvent>,
) {
    for (entity, tracker, player) in &tracker {
        for (_, damage) in tracker.damage_taken.iter() {
            if *damage >= 21 {
                game_events.send(GameEvent::PlayerLost {
                    player: entity,
                    reason: LossReason::CommanderDamage,
                });
                break;
            }
        }
    }
}
```

## Key Commander Rules

The following key Commander rules are implemented in our engine:

| Rule | Description | Implementation Status |
|------|-------------|----------------------|
| Singleton | Only one copy of each card allowed (except basic lands) | ✅ |
| Commander | Legendary creature in command zone | ✅ |
| Color Identity | Cards must match commander's color identity | ✅ |
| Command Zone | Special zone for commanders | ✅ |
| Commander Tax | Additional {2} cost each time cast from command zone | ✅ |
| Commander Damage | 21 combat damage from a single commander | ✅ |
| Starting Life | 40 life points | ✅ |
| Commander Replacement | Optional replacement to command zone | ✅ |
| Partner Commanders | Special commanders that can be paired | 🔄 |
| Commander Ninjutsu | Special ability for certain commanders | ⚠️ |
| Commander-specific Cards | Cards that reference the command zone or commanders | 🔄 |

## Technical Implementation

The Commander format is implemented as a Bevy plugin that extends the core MTG rules:

```rust
pub struct CommanderPlugin;

impl Plugin for CommanderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Commander components
            .register_type::<Commander>()
            .register_type::<CommanderDamage>()
            .register_type::<ColorIdentity>()
            
            // Commander resources
            .init_resource::<CommanderConfig>()
            
            // Commander systems
            .add_systems(Startup, commander_game_setup)
            .add_systems(
                PreUpdate,
                (check_commander_zone_transfers, validate_color_identity)
            )
            .add_systems(
                Update,
                (track_commander_damage, apply_commander_tax)
            );
    }
}
```

## Testing Strategy

Commander testing focuses on these key areas:

1. **Rule Compliance**: Verifying all Commander-specific rules
2. **Integration Testing**: Testing interaction with core MTG systems
3. **Multiplayer Scenarios**: Validating complex multiplayer situations
4. **Edge Cases**: Partner commanders, commander ninjutsu, and other special mechanics

Each section includes detailed test cases to validate the correct implementation of Commander rules. Our testing approach ensures:

1. Full coverage of Commander-specific rules
2. Edge case handling for unique interactions
3. Performance validation for multiplayer scenarios
4. Verification of correct rule application in complex board states

For detailed testing approaches, see the [Commander Testing Guide](game_mechanics/testing_guide.md).

## Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| Command Zone | ✅ Implemented | Complete zone mechanics |
| Commander Casting | ✅ Implemented | With tax calculation |
| Zone Transfers | ✅ Implemented | With player choice |
| Commander Damage | ✅ Implemented | With per-commander tracking |
| Color Identity | ✅ Implemented | Deck validation |
| Partner Commanders | 🔄 In Progress | Basic functionality working |
| Multiplayer Politics | ⚠️ Planned | Design in progress |

---

For more information on the official Commander rules, refer to the [Commander Format Rules](https://mtgcommander.net/index.php/rules/). 