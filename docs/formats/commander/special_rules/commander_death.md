# Commander Death Triggers

This document details the implementation of Commander death triggers and related mechanics in the Commander format.

## Overview

In Commander, when a commander changes zones from the battlefield to any zone other than the command zone, the commander's owner has the option to move it to the command zone instead. This rule creates a special interaction with "dies" triggers, as the commander technically doesn't die (go to the graveyard) if moved to the command zone.

## Rules Evolution

The Commander rules regarding death triggers have evolved over time:

1. **Pre-2020 Rule**: Commanders changing zones would create a replacement effect, preventing them from ever entering the graveyard.
2. **Post-2020 Rule**: Commanders now briefly touch the destination zone before being moved to the command zone as a state-based action, enabling death triggers to work.

## Current Rule Implementation

Under the current rules, when a commander would leave the battlefield:

1. The commander actually moves to the destination zone (e.g., graveyard)
2. This movement triggers any applicable abilities (e.g., "when this creature dies")
3. The next time state-based actions are checked, the commander's owner may choose to move it to the command zone

This creates a brief window where the commander exists in the destination zone, allowing "dies" and other zone-change triggers to function normally.

## Rules Implementation

### Step 1: Zone Transition Tracking

```rust
/// System to handle initial zone transitions of commanders
pub fn track_commander_zone_transitions(
    mut zone_events: EventReader<ZoneTransitionEvent>,
    mut pending_commander_moves: ResMut<PendingCommanderMoves>,
    commander_query: Query<&Commander>,
) {
    for event in zone_events.read() {
        // Only care about commanders moving from battlefield
        if event.from == Zone::Battlefield && 
           event.to != Zone::Command && 
           commander_query.contains(event.entity) {
            
            // Record that this commander might move to command zone
            pending_commander_moves.commanders.insert(
                event.entity,
                CommanderMoveInfo {
                    current_zone: event.to,
                    last_transition_time: Instant::now(),
                }
            );
        }
    }
}
```

### Step 2: State-Based Action for Command Zone Option

```rust
/// State-based action system that offers command zone option
pub fn commander_zone_choice_sba(
    mut commands: Commands,
    mut pending_moves: ResMut<PendingCommanderMoves>,
    mut zone_transitions: EventWriter<ZoneTransitionEvent>,
    mut player_choices: EventWriter<PlayerChoiceEvent>,
    commanders: Query<(Entity, &Owner), With<Commander>>,
    zones: Res<Zones>,
) {
    // Review all pending commander moves during state-based action check
    for (commander, move_info) in pending_moves.commanders.iter() {
        if let Ok((entity, owner)) = commanders.get(*commander) {
            // Offer choice to move to command zone
            player_choices.send(PlayerChoiceEvent {
                player: owner.0,
                choice_type: ChoiceType::CommandZoneOption {
                    commander: entity,
                    current_zone: move_info.current_zone,
                },
                timeout: Duration::from_secs(30),
            });
        }
    }
}

/// Response handler for command zone choice
pub fn handle_command_zone_choice(
    mut commands: Commands,
    mut choice_responses: EventReader<PlayerChoiceResponse>,
    mut zone_transitions: EventWriter<ZoneTransitionEvent>,
    mut pending_moves: ResMut<PendingCommanderMoves>,
) {
    for response in choice_responses.read() {
        if let ChoiceType::CommandZoneOption { commander, current_zone } = &response.choice_type {
            if response.choice == "command_zone" {
                // Player chose to move commander to command zone
                zone_transitions.send(ZoneTransitionEvent {
                    entity: *commander,
                    from: *current_zone,
                    to: Zone::Command,
                    cause: TransitionCause::CommanderRule,
                });
            }
            
            // Remove from pending moves either way
            pending_moves.commanders.remove(commander);
        }
    }
}
```

### Step 3: Death Trigger Processing

The death triggers themselves work normally since the commander actually enters the graveyard:

```rust
/// System that processes death triggers
pub fn process_death_triggers(
    mut commands: Commands,
    mut zone_events: EventReader<ZoneTransitionEvent>,
    mut triggered_abilities: EventWriter<TriggeredAbilityEvent>,
    death_triggers: Query<(Entity, &DiesTriggeredAbility, &Owner)>,
) {
    for event in zone_events.read() {
        // Check for dies triggers (battlefield to graveyard)
        if event.from == Zone::Battlefield && event.to == Zone::Graveyard {
            if let Ok((entity, ability, owner)) = death_triggers.get(event.entity) {
                // Trigger the ability
                triggered_abilities.send(TriggeredAbilityEvent {
                    source: entity,
                    ability_id: ability.id,
                    controller: owner.0,
                    trigger_cause: TriggerCause::ZoneChange {
                        entity,
                        from: event.from,
                        to: event.to,
                    },
                });
            }
        }
    }
}
```

## Timing and Implementation Considerations

The timing of the commander movement is important:

1. The death trigger must be processed before the commander moves to the command zone
2. The state-based action check that offers the command zone option must happen after death triggers are put on the stack
3. The commander stays in the graveyard (or other zone) until the command zone choice is made

```rust
// System ordering for proper death trigger handling
fn build_systems(app: &mut App) {
    app.add_systems(Update, (
        track_commander_zone_transitions,
        process_death_triggers,
        apply_state_based_actions,
        commander_zone_choice_sba
    ).chain());
}
```

## Special Interactions

Several cards interact with commander death and zone changes in unique ways:

### 1. "Dies" Triggers on Commanders

Commanders with "When this creature dies" triggers will function normally:

```rust
pub fn create_elenda_the_dusk_rose() -> impl Bundle {
    (
        CardName("Elenda, the Dusk Rose".to_string()),
        // Other card components...
        Commander,
        DiesTriggeredAbility {
            id: AbilityId::new(),
            effect: Box::new(ElendaDeathEffect),
        },
    )
}
```

### 2. Commanders with Recursion Abilities

Some commanders have abilities that can return them from the graveyard:

```rust
pub fn create_gisa_and_geralf() -> impl Bundle {
    (
        CardName("Gisa and Geralf".to_string()),
        // Other card components...
        Commander,
        // Ability to cast zombie cards from graveyard
        ActivatedAbility {
            id: AbilityId::new(),
            cost: AbilityCost::None,
            effect: Box::new(CastZombieFromGraveyardEffect),
            timing_restriction: TimingRestriction::YourTurn,
            zone_restriction: ZoneRestriction::OnBattlefield,
        },
    )
}
```

### 3. Graveyard Replacement Effects

Effects that replace going to the graveyard (like Rest in Peace) interact with commanders:

```rust
pub fn handle_rest_in_peace_effect(
    mut zone_events: EventReader<ZoneTransitionEvent>,
    mut modified_zone_events: EventWriter<ModifiedZoneTransitionEvent>,
    rest_in_peace_effects: Query<&ReplacementEffect>,
) {
    for event in zone_events.read() {
        if event.to == Zone::Graveyard {
            // Check if Rest in Peace or similar effect is active
            if has_active_graveyard_replacement(&rest_in_peace_effects) {
                // This will affect commander death triggers
                modified_zone_events.send(ModifiedZoneTransitionEvent {
                    original: event.clone(),
                    modified_to: Zone::Exile,
                    cause: TransitionCause::ReplacementEffect {
                        effect_name: "Rest in Peace".to_string(),
                    },
                });
            }
        }
    }
}
```

## Commander Death and State-Based Actions

The rules for commander death interact with multiple state-based actions:

```rust
pub fn apply_state_based_actions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    // ... other queries
) {
    // Only check when a player would receive priority
    if !should_check_sba(&game_state) {
        return;
    }
    
    // Check various state-based actions
    // ...
    
    // Process pending commander moves
    commander_zone_choice_sba(/* ... */);
    
    // Mark that we've checked SBAs
    game_state.last_sba_check = Instant::now();
}
```

## User Interface Considerations

The UI for commander death requires special handling:

1. Prompt for command zone choice must be clear and timely
2. Visual indication of commanders in non-command zones is needed
3. Death triggers should be shown clearly when applicable

## Testing Commander Death Triggers

```rust
#[test]
fn test_commander_death_triggers() {
    let mut app = App::new();
    app.add_systems(Startup, setup_test);
    app.add_systems(Update, (
        track_commander_zone_transitions,
        process_death_triggers,
        commander_zone_choice_sba,
        handle_command_zone_choice,
    ));
    
    // Create test entities
    let player = app.world.spawn_empty().id();
    
    // Create a commander with a death trigger
    let commander = app.world.spawn((
        CardName("Test Commander".to_string()),
        Commander,
        Owner(player),
        DiesTriggeredAbility {
            id: AbilityId::new(),
            effect: Box::new(TestDeathEffect),
        },
    )).id();
    
    // Move commander from battlefield to graveyard
    app.world.send_event(ZoneTransitionEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Graveyard,
        cause: TransitionCause::Destroy,
    });
    
    app.update();
    
    // Verify death trigger happened
    let triggered = app.world.resource::<TestState>().death_trigger_happened;
    assert!(triggered, "Death trigger should have happened");
    
    // Choose to move to command zone
    app.world.send_event(PlayerChoiceResponse {
        player: player,
        choice_type: ChoiceType::CommandZoneOption {
            commander: commander,
            current_zone: Zone::Graveyard,
        },
        choice: "command_zone".to_string(),
    });
    
    app.update();
    
    // Verify commander is now in command zone
    let zones = app.world.resource::<Zones>();
    assert!(zones.command.contains(&commander));
    assert!(!zones.graveyard.contains(&commander));
}
```

## Related Documentation

- [Command Zone](../zones/command_zone.md): How the command zone works
- [Commander Tax](../player_mechanics/commander_tax.md): How commander tax is applied
- [State-Based Actions](../game_mechanics/state_based_actions.md): How state-based actions interact with commander rules 