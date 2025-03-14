# Commander Exile Zone

This section documents the implementation of the Exile Zone in the Commander format.

## Overview

The Exile Zone in Magic: The Gathering represents a special area where cards are removed from the game. In Commander, there are specific interactions between the Exile Zone and the Command Zone that merit special attention.

## Implementation Details

### Exile Zone Component

```rust
#[derive(Component)]
pub struct ExileZone {
    pub contents: Vec<Entity>,
}
```

### Commander-Specific Exile Interactions

When a commander would be exiled, the owner can choose to move it to the Command Zone instead. This is implemented through a player choice event:

```rust
fn handle_commander_exile(
    mut commands: Commands,
    mut exile_events: EventReader<ExileEvent>,
    commanders: Query<(Entity, &Commander)>,
    mut choice_events: EventWriter<PlayerChoiceEvent>,
) {
    for event in exile_events.iter() {
        if let Ok((entity, commander)) = commanders.get(event.card) {
            // Give player the choice to move to command zone instead
            choice_events.send(PlayerChoiceEvent {
                player: commander.owner,
                choices: vec![
                    PlayerChoice::MoveToExile,
                    PlayerChoice::MoveToCommandZone,
                ],
                context: ChoiceContext::CommanderExile { commander: entity },
            });
        }
    }
}
```

## Exile-Based Effects

Many Commander cards interact with the Exile Zone in specific ways:

1. **Temporary Exile**: Effects that exile cards until certain conditions are met
2. **Exile as Resource**: Effects that use exiled cards for special abilities
3. **Commander Reprocessing**: When commanders return from exile, they reset any temporary effects

## Testing

The Exile Zone implementation is tested with these scenarios:

```rust
#[test]
fn test_commander_exile_choice() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(CommanderPlugin);
    
    // Create test entities
    let player = app.world.spawn_empty().id();
    let commander = app.world.spawn((
        Commander { owner: player, cast_count: 0 },
        Card::default(),
    )).id();
    
    // Trigger exile event
    app.world.resource_mut::<Events<ExileEvent>>().send(ExileEvent {
        card: commander,
        source: None,
    });
    
    // Run systems
    app.update();
    
    // Verify player received choice
    let choices = app.world.resource::<Events<PlayerChoiceEvent>>()
        .get_reader()
        .iter(app.world.resource::<Events<PlayerChoiceEvent>>())
        .collect::<Vec<_>>();
    
    assert!(!choices.is_empty());
    // Additional assertions...
}
```

## Integration with Other Zones

The Exile Zone interacts with other zones in specific ways:

- **Command Zone**: Commander replacement effect
- **Battlefield**: "Until end of turn" exile effects
- **Graveyard**: "Exile from graveyard" effects common in Commander

## Next Steps

- [Command Zone](command_zone.md)
- [Zone Transitions](zone_transitions.md)

---

This page is part of the [Game Zones](index.md) documentation for the Commander format. 