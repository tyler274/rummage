# Commander-Specific Triggered Abilities

This document details the implementation of triggered abilities that are specific to or modified by the Commander format.

## Overview

Triggered abilities in Commander generally follow the same rules as in standard Magic: The Gathering. However, there are several Commander-specific triggers and interactions that require special implementation:

1. Triggers that reference the command zone
2. Commander-specific card triggers
3. Multiplayer-specific triggers
4. Commander death and zone change triggers

## Command Zone Triggers

Triggered abilities that interact with the command zone require special handling:

```rust
/// Component for abilities that trigger when a commander enters or leaves the command zone
#[derive(Component)]
pub struct CommandZoneTrigger {
    /// The event that triggers this ability
    pub trigger_event: CommandZoneEvent,
    /// The effect to apply
    pub effect: Box<dyn CommandZoneEffect>,
}

/// Events related to the command zone
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandZoneEvent {
    /// Triggers when a commander enters the command zone
    OnEnter,
    /// Triggers when a commander leaves the command zone
    OnLeave,
    /// Triggers when a commander is cast from the command zone
    OnCast,
}

/// System that handles command zone triggers
pub fn handle_command_zone_triggers(
    mut commands: Commands,
    mut zone_transition_events: EventReader<ZoneTransitionEvent>,
    mut cast_events: EventReader<CastFromCommandZoneEvent>,
    command_zone_triggers: Query<(Entity, &CommandZoneTrigger, &Controller)>,
    commanders: Query<(Entity, &Commander, &Owner)>,
) {
    // Handle zone transitions
    for event in zone_transition_events.read() {
        // Check if this is a commander
        if let Ok((commander_entity, _, owner)) = commanders.get(event.entity) {
            // Handle entering command zone
            if event.to == Zone::Command {
                trigger_command_zone_abilities(
                    CommandZoneEvent::OnEnter,
                    commander_entity,
                    owner.0,
                    &command_zone_triggers,
                    &mut commands,
                );
            }
            
            // Handle leaving command zone
            if event.from == Zone::Command {
                trigger_command_zone_abilities(
                    CommandZoneEvent::OnLeave,
                    commander_entity,
                    owner.0,
                    &command_zone_triggers,
                    &mut commands,
                );
            }
        }
    }
    
    // Handle cast events
    for event in cast_events.read() {
        trigger_command_zone_abilities(
            CommandZoneEvent::OnCast,
            event.commander,
            event.controller,
            &command_zone_triggers,
            &mut commands,
        );
    }
}

fn trigger_command_zone_abilities(
    event: CommandZoneEvent,
    commander: Entity,
    controller: Entity,
    triggers: &Query<(Entity, &CommandZoneTrigger, &Controller)>,
    commands: &mut Commands,
) {
    // Find all triggers that match this event
    for (trigger_entity, trigger, trigger_controller) in triggers.iter() {
        // Only trigger if the ability belongs to the controller of the commander
        if trigger_controller.0 == controller && trigger.trigger_event == event {
            // Create and resolve the triggered ability
            let ability_id = commands.spawn_empty().id();
            
            // Set up ability context
            let mut ctx = AbilityContext {
                ability: ability_id,
                source: trigger_entity,
                controller,
                targets: vec![commander],
                additional_data: HashMap::new(),
            };
            
            // Apply the effect
            trigger.effect.resolve(commands.world_mut(), &mut ctx);
        }
    }
}
```

## Commander Death Triggers

The Commander format has specific rules about commanders changing zones and death triggers:

```rust
/// System that handles commander death triggers
pub fn handle_commander_death_triggers(
    mut death_events: EventReader<DeathEvent>,
    mut commander_death_events: EventWriter<CommanderDeathEvent>,
    commanders: Query<&Commander>,
) {
    for event in death_events.read() {
        if commanders.contains(event.entity) {
            // Create a commander-specific death event
            commander_death_events.send(CommanderDeathEvent {
                commander: event.entity,
                controller: event.controller,
                cause: event.cause.clone(),
            });
        }
    }
}

/// Event for when a commander dies
#[derive(Event)]
pub struct CommanderDeathEvent {
    /// The commander that died
    pub commander: Entity,
    /// The controller of the commander
    pub controller: Entity,
    /// What caused the death
    pub cause: DeathCause,
}

/// System that handles the option to put commander in command zone instead of graveyard
pub fn handle_commander_zone_choice(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    mut zone_choice_events: EventWriter<CommanderZoneChoiceEvent>,
    commanders: Query<&Commander>,
    owners: Query<&Owner>,
) {
    for event in death_events.read() {
        if commanders.contains(event.entity) {
            if let Ok(owner) = owners.get(event.entity) {
                // Create a zone choice event for the commander's owner
                zone_choice_events.send(CommanderZoneChoiceEvent {
                    commander: event.entity,
                    owner: owner.0,
                    source_zone: Zone::Battlefield,
                    destination_options: vec![Zone::Graveyard, Zone::Command],
                });
            }
        }
    }
}
```

## Lieutenant Triggers

"Lieutenant" is a Commander-specific mechanic that gives bonuses when you control your commander:

```rust
/// Component for Lieutenant abilities
#[derive(Component)]
pub struct Lieutenant {
    /// The effect that happens when you control your commander
    pub effect: Box<dyn LieutenantEffect>,
}

/// System that checks for and applies Lieutenant effects
pub fn check_lieutenant_condition(
    mut commands: Commands,
    lieutenant_cards: Query<(Entity, &Lieutenant, &Controller)>,
    battlefield: Query<(Entity, &Controller), With<OnBattlefield>>,
    commanders: Query<(Entity, &Commander)>,
) {
    // For each card with a Lieutenant ability
    for (lieutenant_entity, lieutenant, controller) in lieutenant_cards.iter() {
        // Check if controller controls their commander on the battlefield
        let controls_commander = battlefield
            .iter()
            .filter(|(entity, card_controller)| {
                // Must be controlled by same player as Lieutenant card
                card_controller.0 == controller.0 &&
                // Must be a commander
                commanders.contains(*entity)
            })
            .count() > 0;
        
        // Apply or remove the Lieutenant effect based on condition
        if controls_commander {
            // Apply the effect if not already applied
            if !commands.entity(lieutenant_entity).contains::<ActiveLieutenantEffect>() {
                commands.entity(lieutenant_entity).insert(ActiveLieutenantEffect);
                lieutenant.effect.apply(commands.world_mut(), lieutenant_entity);
            }
        } else {
            // Remove the effect if it was applied
            if commands.entity(lieutenant_entity).contains::<ActiveLieutenantEffect>() {
                commands.entity(lieutenant_entity).remove::<ActiveLieutenantEffect>();
                lieutenant.effect.remove(commands.world_mut(), lieutenant_entity);
            }
        }
    }
}

/// Marker component for entities with active Lieutenant effects
#[derive(Component)]
pub struct ActiveLieutenantEffect;
```

## Partner Triggers

Partner commanders have unique triggered abilities:

```rust
/// Component for "Partner with" tutor ability
#[derive(Component)]
pub struct PartnerWithTutorAbility {
    /// The name of the partner card to search for
    pub partner_name: String,
}

/// System that handles the "Partner with" tutor trigger
pub fn handle_partner_tutor_trigger(
    mut commands: Commands,
    mut entered_battlefield_events: EventReader<EnteredBattlefieldEvent>,
    partner_tutors: Query<(Entity, &PartnerWithTutorAbility, &Controller)>,
    mut tutor_events: EventWriter<TutorEvent>,
) {
    for event in entered_battlefield_events.read() {
        if let Ok((entity, tutor, controller)) = partner_tutors.get(event.entity) {
            // Create a tutor event to find the partner
            tutor_events.send(TutorEvent {
                player: controller.0,
                card_name: tutor.partner_name.clone(),
                destination_zone: Zone::Hand,
                source: entity,
                optional: true,
            });
        }
    }
}
```

## Commander Damage Triggers

Commander damage has a specific trigger at 21 damage:

```rust
/// System that checks for lethal commander damage
pub fn check_commander_damage(
    mut commands: Commands,
    mut damage_events: EventReader<CommanderDamageEvent>,
    mut defeat_events: EventWriter<PlayerDefeatEvent>,
    commander_damage: Query<&CommanderDamageTracker>,
) {
    for event in damage_events.read() {
        // Get damage tracker for the damaged player
        if let Ok(damage_tracker) = commander_damage.get(event.damaged_player) {
            // Check if any commander has dealt 21+ damage
            for (commander, damage) in damage_tracker.damage.iter() {
                if *damage >= 21 {
                    // Player is defeated by commander damage
                    defeat_events.send(PlayerDefeatEvent {
                        player: event.damaged_player,
                        defeat_reason: DefeatReason::CommanderDamage {
                            commander: *commander,
                        },
                    });
                    
                    break;
                }
            }
        }
    }
}
```

## Multiplayer-Specific Triggers

Commander's multiplayer nature leads to unique triggered abilities:

```rust
/// Component for "attack trigger" abilities that scale with number of opponents attacked
#[derive(Component)]
pub struct AttackTriggeredAbility {
    /// The effect to apply
    pub effect: Box<dyn AttackEffect>,
    /// Multiplier for multi-opponent attacks
    pub scales_with_opponents: bool,
}

/// System that handles attack triggers in multiplayer
pub fn handle_attack_triggers(
    mut commands: Commands,
    mut attack_events: EventReader<AttackEvent>,
    attack_abilities: Query<(Entity, &AttackTriggeredAbility, &Controller)>,
) {
    // Group attacks by controller to count unique opponents
    let mut controller_attacks: HashMap<Entity, HashSet<Entity>> = HashMap::new();
    
    for event in attack_events.read() {
        if let Ok(attacker_controller) = controllers.get(event.attacker) {
            controller_attacks
                .entry(attacker_controller.0)
                .or_default()
                .insert(event.defender);
        }
    }
    
    // Trigger abilities based on attacks
    for (ability_entity, ability, controller) in attack_abilities.iter() {
        if let Some(attacked_opponents) = controller_attacks.get(&controller.0) {
            if !attacked_opponents.is_empty() {
                if ability.scales_with_opponents {
                    // Apply effect with scaling factor
                    let scale_factor = attacked_opponents.len() as i32;
                    ability.effect.apply_scaled(
                        commands.world_mut(), 
                        ability_entity, 
                        scale_factor
                    );
                } else {
                    // Apply effect normally
                    ability.effect.apply(commands.world_mut(), ability_entity);
                }
            }
        }
    }
}
```

## Testing Commander Triggers

Testing Commander-specific triggers requires special test fixtures:

```rust
#[test]
fn test_lieutenant_trigger() {
    let mut app = App::new();
    setup_test_game(&mut app);
    
    // Create a player
    let player = app.world.spawn(Player).id();
    
    // Create a commander in the command zone
    let commander = app.world.spawn((
        CardName("Test Commander".to_string()),
        Commander,
        Controller(player),
    )).id();
    
    // Create a card with Lieutenant ability
    let lieutenant = app.world.spawn((
        CardName("Test Lieutenant".to_string()),
        OnBattlefield,
        Controller(player),
        Lieutenant {
            effect: Box::new(TestLieutenantEffect),
        },
    )).id();
    
    // Run the lieutenant check system
    app.update();
    
    // Initially, commander not on battlefield, so no effect
    assert!(!app.world.entity(lieutenant).contains::<ActiveLieutenantEffect>());
    
    // Move commander to battlefield
    app.world.entity_mut(commander).insert(OnBattlefield);
    
    // Run the lieutenant check system again
    app.update();
    
    // Now the effect should be active
    assert!(app.world.entity(lieutenant).contains::<ActiveLieutenantEffect>());
    
    // Remove commander from battlefield
    app.world.entity_mut(commander).remove::<OnBattlefield>();
    
    // Run system again
    app.update();
    
    // Effect should be removed
    assert!(!app.world.entity(lieutenant).contains::<ActiveLieutenantEffect>());
}
```

## Integration with Core Triggered Abilities

Commander-specific triggered abilities integrate with the core triggered ability system:

```rust
pub fn register_commander_triggered_abilities(app: &mut App) {
    app
        .add_event::<CommanderDeathEvent>()
        .add_event::<CommanderZoneChoiceEvent>()
        .add_systems(Update, (
            handle_command_zone_triggers,
            handle_commander_death_triggers,
            handle_commander_zone_choice,
            check_lieutenant_condition,
            handle_partner_tutor_trigger,
            check_commander_damage,
            handle_attack_triggers,
        ).after(CoreTriggerSystems));
}
```

## Related Resources

- [Commander Death Triggers](../special_rules/commander_death.md)
- [Partner Commanders](../special_rules/partner_commanders.md)
- [Commander Damage](../combat/commander_damage.md)
- [Command Zone](../zones/command_zone.md)
- [Core Triggered Abilities Documentation](../../../mtg_rules/triggered_abilities.md) 