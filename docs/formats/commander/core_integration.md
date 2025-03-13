# Commander Format: Core Rules Integration

This document explains how the Commander format extends and modifies the core Magic: The Gathering rules in Rummage.

## Architecture Overview

Commander builds upon the format-agnostic core MTG rules through a layered architecture:

```
┌───────────────────────────────────────────┐
│             Commander Format              │
│  ┌─────────────┐  ┌────────────────────┐  │
│  │  Commander  │  │ Commander-Specific │  │
│  │  Components │  │      Systems       │  │
│  └─────────────┘  └────────────────────┘  │
├───────────────────────────────────────────┤
│            Extension Points               │
│  ┌─────────────┐  ┌────────────────────┐  │
│  │   Plugin    │  │     Override       │  │
│  │ Registration│  │    Mechanisms      │  │
│  └─────────────┘  └────────────────────┘  │
├───────────────────────────────────────────┤
│              Core MTG Rules               │
│  ┌─────────────┐  ┌────────────────────┐  │
│  │    Base     │  │       Base         │  │
│  │  Components │  │      Systems       │  │
│  └─────────────┘  └────────────────────┘  │
└───────────────────────────────────────────┘
```

This approach provides:
- **Clean Separation**: Format-specific code doesn't contaminate core rules
- **Reusability**: Core rules can support multiple formats
- **Testability**: Format mechanics can be tested in isolation
- **Extensibility**: New formats can be added without modifying core code

## Core Extension Points

Commander leverages these primary extension mechanisms:

### 1. Zone Management Extensions

The core rules provide standard zones (library, hand, battlefield, etc.) which Commander extends with:

```rust
// Core system handles standard zones
fn core_zone_setup(mut commands: Commands) {
    // Create standard zones for each player
    // ...
}

// Commander plugin adds Command Zone support
fn commander_zone_setup(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in &players {
        // Create Command Zone for this player
        commands.spawn((
            ZoneType::Command,
            ZoneContents::default(),
            BelongsToPlayer(player),
            CommandZone,
            Name::new("Command Zone"),
        ));
    }
}
```

### 2. Game Setup Extensions

Commander modifies initial game parameters:

```rust
// Commander plugin modifies core game setup
fn commander_game_setup(
    mut game_config: ResMut<GameConfig>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    // Set Commander-specific configuration
    game_config.starting_life = 40;
    game_config.mulligan_rule = MulliganRule::CommanderFreeFirst;
    
    // Add Commander-specific components to players
    for player in &players {
        commands.entity(player).insert(CommanderDamageTracker::default());
    }
}
```

### 3. Rules System Extensions

Commander adds new rule checks and state-based actions:

```rust
// Core state-based action system
fn core_state_based_actions(/* ... */) {
    // Check creature death, player life, etc.
    // ...
}

// Commander adds additional state-based actions
fn commander_state_based_actions(
    players: Query<(Entity, &CommanderDamageTracker)>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Check commander damage thresholds
    for (player, damage_tracker) in &players {
        for (&commander, &damage) in damage_tracker.damage_taken.iter() {
            if damage >= 21 {
                game_events.send(GameEvent::PlayerLost {
                    player,
                    reason: LossReason::CommanderDamage(commander),
                });
            }
        }
    }
}
```

## Commander-Specific Overrides

In certain cases, Commander needs to override default core behavior:

### Zone Transfer Overrides

When a commander would change zones, Commander rules allow for special handling:

```rust
// Commander zone transfer override
fn commander_zone_transfer_system(
    mut zone_events: EventReader<ZoneChangeEvent>,
    mut player_choices: EventWriter<PlayerChoiceEvent>,
    commanders: Query<Entity, With<Commander>>,
) {
    for event in zone_events.iter() {
        // If this is a commander...
        if commanders.contains(event.entity) {
            // And it's moving to graveyard/exile/hand/library...
            if matches!(event.to, 
                ZoneType::Graveyard | ZoneType::Exile | 
                ZoneType::Hand | ZoneType::Library
            ) {
                // Give the controller a choice
                player_choices.send(PlayerChoiceEvent {
                    player: get_controller(event.entity),
                    choice_type: ChoiceType::CommanderZoneChange {
                        commander: event.entity,
                        default_zone: event.to,
                        command_zone: ZoneType::Command,
                    },
                    timeout: Duration::from_secs(30),
                });
            }
        }
    }
}
```

### Commander Tax Implementation

The Commander format adds tax to casting commanders from the Command Zone:

```rust
// Commander casting cost modification
fn apply_commander_tax(
    mut cast_events: EventReader<PrepareTocastEvent>,
    mut cast_costs: Query<&mut ManaCost>,
    commanders: Query<&Commander>,
    zones: Query<(&ZoneContents, &ZoneType)>,
) {
    for event in cast_events.iter() {
        // Check if this is a commander being cast
        if let Ok(commander) = commanders.get(event.card) {
            // Check if it's being cast from Command Zone
            if is_in_command_zone(event.card, &zones) {
                // Apply commander tax
                if let Ok(mut cost) = cast_costs.get_mut(event.card) {
                    let tax_amount = commander.cast_count * 2;
                    cost.add_generic(tax_amount);
                }
            }
        }
    }
}
```

## Plugin Implementation

The Commander format is implemented as a Bevy plugin:

```rust
pub struct CommanderPlugin;

impl Plugin for CommanderPlugin {
    fn build(&self, app: &mut App) {
        // Register Commander-specific components
        app.register_type::<Commander>()
           .register_type::<CommanderDamageTracker>()
           .register_type::<CommanderTax>();
        
        // Add Commander-specific resources
        app.init_resource::<CommanderConfig>();
        
        // Add Commander setup systems
        app.add_systems(Startup, (
            commander_game_setup,
            commander_zone_setup,
        ));
        
        // Add Commander-specific gameplay systems
        app.add_systems(PreUpdate, (
            commander_zone_transfer_system
                .after(core_zone_change_system),
        ));
        
        // Add Commander rule enforcement systems
        app.add_systems(Update, (
            apply_commander_tax,
            track_commander_damage,
            commander_state_based_actions
                .after(core_state_based_actions),
        ));
        
        // Add Commander-specific events
        app.add_event::<CommanderCastEvent>()
           .add_event::<CommanderDamageEvent>();
    }
}
```

## Example: Commander Zone Integration

The Command Zone is central to the Commander format. Here's its complete implementation:

```rust
// Commander-specific components
#[derive(Component, Reflect)]
pub struct Commander {
    pub owner: Entity,
    pub cast_count: u32,
}

#[derive(Component, Reflect)]
pub struct CommandZone;

// Command Zone setup
fn commander_zone_setup(
    mut commands: Commands, 
    players: Query<Entity, With<Player>>,
    mut decks: Query<(&mut DeckList, &BelongsToPlayer)>,
) {
    // Create Command Zone for each player
    for player in &players {
        // Create Command Zone entity
        let command_zone = commands.spawn((
            ZoneType::Command,
            ZoneContents::default(),
            BelongsToPlayer(player),
            CommandZone,
            Name::new("Command Zone"),
        )).id();
        
        // Find player's deck and locate commander
        if let Some((mut deck_list, _)) = decks
            .iter_mut()
            .find(|(_, belongs_to)| belongs_to.0 == player) 
        {
            // Extract commander(s) from deck
            if let Some(commander_card) = deck_list.extract_commander() {
                // Spawn commander entity
                let commander_entity = commands.spawn((
                    Card::from_definition(commander_card),
                    Commander { 
                        owner: player,
                        cast_count: 0,
                    },
                    InZone(ZoneType::Command),
                )).id();
                
                // Add commander to command zone
                commands.entity(command_zone)
                    .update_component(|mut contents: Mut<ZoneContents>| {
                        contents.entities.push(commander_entity);
                    });
            }
        }
    }
}
```

## Testing Integration

Commander integration testing focuses on verifying that format-specific rules correctly interact with core systems:

```rust
#[test]
fn test_commander_zone_transfer() {
    // Setup test environment with both core and commander plugins
    let mut app = App::new();
    app.add_plugins(CoreRulesPlugin)
       .add_plugins(CommanderPlugin);
    
    // Setup test commander and zones
    let (player, commander) = setup_test_commander(&mut app);
    
    // Test that commander can move to command zone instead of graveyard
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: ZoneType::Battlefield,
        to: ZoneType::Graveyard,
    });
    
    // Handle player choice to move to command zone
    resolve_commander_zone_choice(&mut app, player, commander, ZoneType::Command);
    
    // Verify commander is in command zone
    let zone = get_entity_zone(&app, commander);
    assert_eq!(zone, ZoneType::Command, "Commander should be in Command Zone");
}
```

## Conclusion

The Commander format implementation builds upon the core MTG rules through a clean plugin architecture that:

1. **Extends** base functionality with Commander-specific features
2. **Overrides** certain behaviors to match Commander rules
3. **Adds** new components and systems unique to Commander

This approach maintains the integrity of the core rules while enabling the unique gameplay experience of the Commander format.

For detailed information on specific Commander mechanics, see:
- [Commander Damage](combat/commander_damage.md)
- [Color Identity](player_mechanics/color_identity.md)
- [Command Zone](zones/command_zone.md) 