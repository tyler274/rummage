# Integration with Core Rules

This document explains how the Commander format rules integrate with and extend the core Magic: The Gathering rules in Rummage.

## Table of Contents

1. [Overview](#overview)
2. [Extension Points](#extension-points)
3. [Commander-Specific Overrides](#commander-specific-overrides)
4. [Implementation Approach](#implementation-approach)
5. [Example: Commander Zone Implementation](#example-commander-zone-implementation)

## Overview

The Commander format builds upon the foundation of the core MTG rules, adding format-specific mechanics and modifications. In Rummage, we've designed the core rules to be format-agnostic, with well-defined extension points that allow format-specific plugins to modify or extend the base behavior.

This approach allows us to:
- Maintain a clean separation between core rules and format-specific rules
- Reuse common game logic across different formats
- Easily add support for new formats in the future
- Test format-specific rules independently from core rules

## Extension Points

The core rules provide several extension points that the Commander format leverages:

### Zone Extensions

The core rules define the standard MTG zones (library, hand, battlefield, graveyard, stack, and exile), while the Commander format adds:
- Enhanced Command Zone functionality
- Commander-specific zone transfer rules
- Zone visibility rules for multiplayer

### Game Setup Extensions

The core rules provide a basic game setup process, which Commander extends with:
- Starting life total of 40 (instead of 20)
- Commander placement in the Command Zone
- Color identity validation for deck construction
- Support for Partner Commanders

### Turn Structure Extensions

The core turn structure remains unchanged, but Commander adds:
- Multiplayer turn order management
- Commander-specific triggered abilities
- Political mechanics for multiplayer interaction

### State-Based Action Extensions

Commander adds several state-based actions:
- Commander damage tracking (21 damage from a single commander causes a loss)
- Commander zone transfer options when a commander would change zones
- Commander tax calculation

## Commander-Specific Overrides

In some cases, the Commander format needs to override core behavior:

### Life Total

```rust
// Core implementation
#[derive(Resource)]
struct GameSetupConfig {
    starting_life: i32,
    // Other configuration
}

// Commander override
fn commander_setup_system(mut config: ResMut<GameSetupConfig>) {
    config.starting_life = 40;
}
```

### Commander Zone Transfers

```rust
// Core implementation handles standard zone transfers
fn zone_change_system(/* ... */) {
    // Standard zone transfer logic
}

// Commander adds special handling for commanders
fn commander_zone_change_system(
    mut commands: Commands,
    mut zone_change_events: EventReader<ZoneChangeEvent>,
    commanders: Query<Entity, With<Commander>>,
    mut player_choices: EventWriter<PlayerChoiceEvent>,
) {
    for event in zone_change_events.iter() {
        if commanders.contains(event.entity) {
            // If a commander would go to graveyard/exile/hand/library
            if matches!(event.to, ZoneType::Graveyard | ZoneType::Exile | ZoneType::Hand | ZoneType::Library) {
                // Offer choice to move to command zone instead
                player_choices.send(PlayerChoiceEvent {
                    player: get_controller(event.entity),
                    choices: vec![
                        Choice::MoveToZone(event.entity, event.to),
                        Choice::MoveToCommandZone(event.entity),
                    ],
                    // Other event data
                });
            }
        }
    }
}
```

## Implementation Approach

Rummage implements the Commander format as a Bevy plugin that extends the core rules:

```rust
pub struct CommanderPlugin;

impl Plugin for CommanderPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register Commander-specific components
            .register_type::<Commander>()
            .register_type::<CommanderDamage>()
            .register_type::<CommanderTax>()
            
            // Add Commander-specific resources
            .init_resource::<CommanderConfig>()
            
            // Add Commander-specific systems
            .add_systems(Startup, commander_setup)
            .add_systems(
                Update,
                (
                    commander_zone_change_system,
                    commander_damage_system,
                    commander_tax_system,
                )
            )
            
            // Add Commander-specific events
            .add_event::<CommanderCastEvent>()
            .add_event::<CommanderDamageEvent>();
    }
}
```

This plugin approach allows the Commander format to be cleanly layered on top of the core rules without modifying them directly.

## Example: Commander Zone Implementation

The Command Zone is a key element of the Commander format. Here's how it's implemented in Rummage:

```rust
// Commander-specific components
#[derive(Component)]
struct Commander {
    player: Entity,
    times_cast: u32,
}

#[derive(Component)]
struct CommanderTax(u32);

// Command Zone entity creation
fn setup_command_zone(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player_entity in &players {
        // Create Command Zone entity for each player
        let command_zone = commands.spawn((
            ZoneType::Command,
            ZoneContents { entities: Vec::new() },
            BelongsToPlayer(player_entity),
            CommandZone,
        )).id();
        
        // Link player to their Command Zone
        commands.entity(player_entity).insert(OwnedZone {
            zone_type: ZoneType::Command,
            zone_entity: command_zone,
        });
    }
}

// Commander casting with tax
fn cast_commander_system(
    mut commands: Commands,
    mut cast_events: EventReader<CastSpellEvent>,
    mut commanders: Query<(Entity, &mut Commander)>,
    command_zones: Query<&ZoneContents, With<CommandZone>>,
) {
    for event in cast_events.iter() {
        if let Ok((commander_entity, mut commander)) = commanders.get_mut(event.card) {
            // Increase times cast counter
            commander.times_cast += 1;
            
            // Calculate and apply Commander tax
            let tax = (commander.times_cast - 1) * 2;
            commands.entity(commander_entity).insert(AdditionalCost(tax));
            
            // Other commander casting logic
        }
    }
}
```

This implementation demonstrates how Commander-specific mechanics are built on top of the core rules while maintaining a clean separation of concerns.

---

For more details on specific Commander mechanics, see the [Commander Format Overview](overview/index.md) and related sections. 