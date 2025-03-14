# Commander Ninjutsu

This document details the implementation of the Commander Ninjutsu mechanic in the Commander format.

## Overview

Commander Ninjutsu is a Commander-specific variant of the Ninjutsu mechanic, introduced on the card "Yuriko, the Tiger's Shadow" from Commander 2018. It allows a commander with this ability to be put onto the battlefield from the command zone, bypassing commander tax.

## Mechanic Definition

Commander Ninjutsu {Cost} — {Cost}, Return an unblocked attacker you control to hand: Put this card from the command zone onto the battlefield tapped and attacking.

Key differences from regular Ninjutsu:
1. It can be activated from the command zone (instead of only from hand)
2. It bypasses the need to cast the commander, avoiding commander tax
3. It allows the commander to enter the battlefield directly attacking

## Rules Implementation

### Core Commander Ninjutsu Rules

```rust
/// Component for cards with Commander Ninjutsu
#[derive(Component, Clone, Debug)]
pub struct CommanderNinjutsu {
    /// Mana cost to activate the ability
    pub cost: ManaCost,
}

/// Event triggered when a Commander Ninjutsu ability is activated
#[derive(Event)]
pub struct CommanderNinjutsuEvent {
    /// The commander with ninjutsu being put onto the battlefield
    pub commander: Entity,
    /// The creature being returned to hand
    pub returned_creature: Entity,
    /// The player being attacked
    pub defending_player: Entity,
    /// The player activating the ability
    pub controller: Entity,
}

/// System that handles Commander Ninjutsu activation
pub fn handle_commander_ninjutsu(
    mut commands: Commands,
    mut ninjutsu_events: EventReader<CommanderNinjutsuEvent>,
    mut zone_transitions: EventWriter<ZoneTransitionEvent>,
    mut attacking_status: EventWriter<SetAttackingEvent>,
    command_zone: Query<&CommandZone>,
) {
    for event in ninjutsu_events.read() {
        // Verify commander is in command zone
        if !is_in_command_zone(event.commander, &command_zone) {
            continue;
        }
        
        // 1. Return the unblocked attacker to hand
        zone_transitions.send(ZoneTransitionEvent {
            entity: event.returned_creature,
            from: Zone::Battlefield,
            to: Zone::Hand,
            cause: TransitionCause::Ability {
                source: event.commander,
                ability_name: "Commander Ninjutsu".to_string(),
            },
        });
        
        // 2. Put commander onto battlefield
        zone_transitions.send(ZoneTransitionEvent {
            entity: event.commander,
            from: Zone::Command,
            to: Zone::Battlefield,
            cause: TransitionCause::Ability {
                source: event.commander,
                ability_name: "Commander Ninjutsu".to_string(),
            },
        });
        
        // 3. Set commander as tapped and attacking
        commands.entity(event.commander).insert(Tapped);
        
        attacking_status.send(SetAttackingEvent {
            attacker: event.commander,
            defending_player: event.defending_player,
        });
    }
}
```

### Activation Requirements

Commander Ninjutsu can only be activated during specific game states:

```rust
/// System to determine when Commander Ninjutsu can be activated
pub fn can_activate_commander_ninjutsu(
    commander: Entity,
    controller: Entity,
    game_state: &GameState,
    command_zone: &Query<&CommandZone>,
    ninjutsu_abilities: &Query<&CommanderNinjutsu>,
    attacking_creatures: &Query<(Entity, &AttackingStatus, &Controller)>,
) -> bool {
    // 1. Must be in the combat phase, after blockers are declared
    if !matches!(game_state.current_phase, 
                Phase::Combat(CombatStep::DeclareBlockers | 
                               CombatStep::CombatDamage)) {
        return false;
    }
    
    // 2. Commander must be in command zone
    if !is_in_command_zone(commander, command_zone) {
        return false;
    }
    
    // 3. Must have Commander Ninjutsu ability
    if !ninjutsu_abilities.contains(commander) {
        return false;
    }
    
    // 4. Must have an unblocked attacker to return to hand
    let has_unblocked_attacker = attacking_creatures
        .iter()
        .any(|(entity, status, creature_controller)| {
            creature_controller.0 == controller && 
            status.is_unblocked()
        });
    
    has_unblocked_attacker
}
```

### Mana Cost and Commander Tax

The Commander Ninjutsu ability itself has a fixed cost that does not increase when the commander is put into the command zone:

```rust
/// Function to get the mana cost for activating Commander Ninjutsu
pub fn get_commander_ninjutsu_cost(
    commander: Entity,
    ninjutsu_abilities: &Query<&CommanderNinjutsu>,
) -> ManaCost {
    // Commander Ninjutsu cost is fixed and doesn't include commander tax
    if let Ok(ninjutsu) = ninjutsu_abilities.get(commander) {
        ninjutsu.cost.clone()
    } else {
        ManaCost::default() // Fallback, shouldn't happen
    }
}
```

However, the normal casting cost of the commander still increases each time the commander is put into the command zone from anywhere:

```rust
/// System that tracks commander movement to command zone to apply tax
pub fn track_commander_tax(
    mut tax_events: EventWriter<IncrementCommanderTaxEvent>,
    mut zone_transition_events: EventReader<ZoneTransitionEvent>,
    commander_query: Query<&Commander>,
) {
    for event in zone_transition_events.read() {
        if event.to == Zone::Command && commander_query.contains(event.entity) {
            // Increment commander tax, even for commanders with Ninjutsu
            tax_events.send(IncrementCommanderTaxEvent {
                commander: event.entity,
            });
        }
    }
}
```

## User Interface Considerations

The UI needs special handling for Commander Ninjutsu:

1. Ability must be presented as an option during combat after blockers are declared
2. UI must show which unblocked attackers can be returned to hand
3. Cost display should show the fixed Ninjutsu cost (not affected by commander tax)

```rust
/// Function to get available Commander Ninjutsu actions
pub fn get_commander_ninjutsu_actions(
    player: Entity,
    game_state: &GameState,
    commanders: &Query<(Entity, &Commander, &CommanderNinjutsu, &Owner)>,
    unblocked_attackers: &Query<(Entity, &AttackingStatus, &Controller)>,
) -> Vec<CommanderNinjutsuAction> {
    let mut actions = Vec::new();
    
    // Only check during appropriate combat steps
    if !matches!(game_state.current_phase, 
                Phase::Combat(CombatStep::DeclareBlockers | 
                              CombatStep::CombatDamage)) {
        return actions;
    }
    
    // Get all unblocked attackers controlled by player
    let available_attackers: Vec<Entity> = unblocked_attackers
        .iter()
        .filter(|(_, status, controller)| {
            controller.0 == player && status.is_unblocked()
        })
        .map(|(entity, _, _)| entity)
        .collect();
    
    if available_attackers.is_empty() {
        return actions;
    }
    
    // Find commanders with ninjutsu in command zone
    for (commander, _, ninjutsu, owner) in commanders.iter() {
        if owner.0 == player && is_in_command_zone(commander, &game_state.zones) {
            // For each available attacker, create a potential action
            for attacker in &available_attackers {
                actions.push(CommanderNinjutsuAction {
                    commander,
                    unblocked_attacker: *attacker,
                    defending_player: get_defending_player(*attacker, &game_state),
                    cost: ninjutsu.cost.clone(),
                });
            }
        }
    }
    
    actions
}
```

## Notable Cards with Commander Ninjutsu

Currently, only one card has Commander Ninjutsu:

- **Yuriko, the Tiger's Shadow** (Commander 2018)
  - Commander Ninjutsu {1}{U}{B}
  - Whenever Yuriko deals combat damage to a player, reveal the top card of your library and put it into your hand. You lose life equal to that card's mana value.
  - Partner With: None
  - Color Identity: Blue-Black

Implementation of Yuriko:

```rust
pub fn create_yuriko() -> impl Bundle {
    (
        CardName("Yuriko, the Tiger's Shadow".to_string()),
        CardType::Creature,
        CreatureType(vec!["Human".to_string(), "Ninja".to_string()]),
        Commander,
        Power(1),
        Toughness(3),
        ManaCost::parse("{1}{U}{B}"),
        ColorIdentity::parse("{U}{B}"),
        CommanderNinjutsu {
            cost: ManaCost::parse("{1}{U}{B}"),
        },
        CombatDamageTriggeredAbility {
            trigger_condition: CombatDamageCondition::DamageToPlayer,
            ability: Box::new(YurikoRevealEffect),
        },
    )
}
```

## Testing Commander Ninjutsu

```rust
#[test]
fn test_commander_ninjutsu_activation() {
    let mut app = App::new();
    app.add_systems(Startup, setup_combat_test);
    app.add_systems(Update, (
        handle_commander_ninjutsu,
        track_commander_tax,
    ));
    
    // Create test entities
    let player = app.world.spawn_empty().id();
    let opponent = app.world.spawn_empty().id();
    
    // Create Yuriko commander in command zone
    let yuriko = app.world.spawn((
        CardName("Yuriko, the Tiger's Shadow".to_string()),
        Commander,
        CommanderNinjutsu { cost: ManaCost::parse("{1}{U}{B}") },
        Owner(player),
    )).id();
    
    // Add to command zone
    app.world.resource_mut::<Zones>().command.insert(yuriko);
    
    // Create an unblocked attacker
    let attacker = app.world.spawn((
        CardName("Ninja of the Deep Hours".to_string()),
        Controller(player),
        AttackingStatus {
            attacking: true,
            blocked: false,
            defending_player: Some(opponent),
        },
    )).id();
    
    // Set game state to post-blockers
    app.world.resource_mut::<GameState>().current_phase = Phase::Combat(CombatStep::DeclareBlockers);
    
    // Activate commander ninjutsu
    app.world.send_event(CommanderNinjutsuEvent {
        commander: yuriko,
        returned_creature: attacker,
        defending_player: opponent,
        controller: player,
    });
    
    app.update();
    
    // Verify Yuriko is now on battlefield and attacking
    let zones = app.world.resource::<Zones>();
    assert!(!zones.command.contains(&yuriko));
    assert!(zones.battlefield.contains(&yuriko));
    
    let attacking_status = app.world.get::<AttackingStatus>(yuriko).unwrap();
    assert!(attacking_status.attacking);
    assert_eq!(attacking_status.defending_player, Some(opponent));
    
    // Verify the attacker was returned to hand
    assert!(zones.hand.contains(&attacker));
}
```

## Related Documentation

- [Command Zone](../zones/command_zone.md): How commanders move to and from the command zone
- [Commander Tax](../player_mechanics/commander_tax.md): How tax is applied to commanders
- [Combat](../combat/index.md): Combat phase implementation where Ninjutsu is activated 