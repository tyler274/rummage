# Commander Format

This section documents the implementation of the Commander (EDH) format in Rummage, a multiplayer format that emphasizes social gameplay, unique deck construction constraints, and strategic depth.

## Format Overview

Commander is a sanctioned Magic: The Gathering format with these defining characteristics:

| Feature | Description | Implementation |
|---------|-------------|----------------|
| **Deck Construction** | 100-card singleton decks (no duplicates except basic lands) | Deck validation systems |
| **Commander** | A legendary creature that leads your deck | Command zone and casting mechanics |
| **Color Identity** | Deck colors must match commander's color identity | Deck validation and color checking |
| **Life Total** | 40 starting life (vs. standard 20) | Modified game initialization |
| **Commander Damage** | 21 combat damage from a single commander causes loss | Per-commander damage tracking |
| **Multiplayer Focus** | Designed for 3-6 players | Turn ordering and multiplayer mechanics |

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

### Multiplayer Considerations

Commander's multiplayer nature introduces additional complexity:

- **Turn Order Management**: Turn cycles with priority passing
- **Global Effects Handling**: "Each opponent" effects in multiplayer
- **Politics System**: Agreements and deals between players
- **Table Talk**: In-game communication between players

## Module Structure

The Commander implementation is organized into these logical modules:

1. **[Core Integration](core_integration.md)** - How Commander extends MTG core rules
2. **[Player Mechanics](player_mechanics/index.md)** - Life totals, commander tax, color identity
3. **[Game Zones](zones/index.md)** - Command zone implementation, zone transfers
4. **[Turns and Phases](turns_and_phases/index.md)** - Multiplayer turn structure
5. **[Combat](combat/index.md)** - Commander damage tracking, multiplayer combat
6. **[Special Rules](special_rules/index.md)** - Partner commanders, commander ninjutsu

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

See the [Commander Testing Guide](game_mechanics/testing_guide.md) for detailed testing approaches.

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

## Next Steps

- [Core Integration](core_integration.md) - How Commander extends core MTG rules
- [Player Mechanics](player_mechanics/index.md) - Life, tax, and color identity
- [Game Zones](zones/index.md) - Command zone implementation
- [Combat](combat/index.md) - Commander damage tracking 