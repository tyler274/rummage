# Commander-Specific Combat Tests

## Overview

This document outlines test cases for Commander-specific combat scenarios, focusing on the unique rules and edge cases that arise in the Commander format. These tests are critical for ensuring our Commander engine correctly handles format-specific interactions during combat.

## Test Case: Commander Damage Tracking

### Test: Multiple Sources of Commander Damage

```rust
#[test]
fn test_multiple_commanders_damage_tracking() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, state_based_actions_system));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create commanders from different players
    let commander1 = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: opponent1 },
        Attacking { defending: player },
    )).id();
    
    let commander2 = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Commander { owner: opponent2 },
        Attacking { defending: player },
    )).id();
    
    let opponent1 = app.world.spawn(Player {}).id();
    let opponent2 = app.world.spawn(Player {}).id();
    
    // Process combat damage for first attack
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify commander damage was tracked separately
    let cmd_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage.get_damage(commander1), 5);
    assert_eq!(cmd_damage.get_damage(commander2), 3);
    assert_eq!(cmd_damage.total_damage(), 8);
    
    // Player's health should also be reduced
    let health = app.world.get::<Health>(player).unwrap();
    assert_eq!(health.current, 32); // 40 - 5 - 3 = 32
    
    // More damage from commander1 in a later combat
    app.world.entity_mut(commander1).insert(
        DamageEvent {
            source: commander1,
            target: player,
            amount: 5,
            is_combat_damage: true,
        }
    );
    
    // Process the damage
    app.update();
    
    // Check state-based actions after damage
    app.update();
    
    // Verify commander damage accumulated correctly
    let cmd_damage_after = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage_after.get_damage(commander1), 10);
    assert_eq!(cmd_damage_after.get_damage(commander2), 3);
    
    // Verify player still alive (no commander has dealt 21+ damage yet)
    assert!(app.world.entity(player).contains::<Health>());
    
    // Deal more damage from commander1 to reach lethal commander damage
    app.world.entity_mut(commander1).insert(
        DamageEvent {
            source: commander1,
            target: player,
            amount: 11,
            is_combat_damage: true,
        }
    );
    
    // Process the damage
    app.update();
    
    // Check state-based actions after damage
    app.update();
    
    // Verify player lost due to commander damage
    let cmd_damage_final = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage_final.get_damage(commander1), 21);
    assert!(app.world.entity(player).contains::<PlayerEliminated>());
}
```

### Test: Commander Damage from Copied Commander

```rust
#[test]
fn test_commander_damage_from_clone() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, clone_effects_system));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create commander
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: opponent },
        CreatureCard { controller: opponent },
    )).id();
    
    let opponent = app.world.spawn(Player {}).id();
    
    // Create clone of the commander
    let clone = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        CloneEffect { source: commander },
        CreatureCard { controller: opponent },
    )).id();
    
    // Make clone attack
    app.world.entity_mut(clone).insert(Attacking { defending: player });
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify damage from clone is NOT commander damage
    let cmd_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage.get_damage(commander), 0);
    assert_eq!(cmd_damage.get_damage(clone), 0);
    
    // Player's health should still be reduced
    let health = app.world.get::<Health>(player).unwrap();
    assert_eq!(health.current, 35); // 40 - 5 = 35
}
```

## Test Case: Commander Combat Abilities

### Test: Commander with Partner

```rust
#[test]
fn test_partner_commanders_in_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, state_based_actions_system));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create partner commanders
    let partner1 = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Commander { owner: opponent },
        Partner {},
        Attacking { defending: player },
    )).id();
    
    let partner2 = app.world.spawn((
        Creature { power: 2, toughness: 2 },
        Commander { owner: opponent },
        Partner {},
        Attacking { defending: player },
    )).id();
    
    let opponent = app.world.spawn(Player {}).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify commander damage was tracked separately for each partner
    let cmd_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage.get_damage(partner1), 3);
    assert_eq!(cmd_damage.get_damage(partner2), 2);
    
    // Player needs to take 21 damage from a single partner to lose
    // Simulate multiple combats to test this
    for _ in 0..6 {
        app.world.entity_mut(partner1).insert(
            DamageEvent {
                source: partner1,
                target: player,
                amount: 3,
                is_combat_damage: true,
            }
        );
        app.update();
    }
    
    // Check state-based actions
    app.update();
    
    // Verify player lost due to commander damage from one partner
    let cmd_damage_final = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage_final.get_damage(partner1), 21);
    assert!(app.world.entity(player).contains::<PlayerEliminated>());
}
```

### Test: Commander with Combat Damage Triggers

```rust
#[test]
fn test_commander_combat_damage_triggers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, trigger_system));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create commander with combat damage trigger
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: opponent },
        Attacking { defending: player },
        CombatDamageTrigger { 
            effect: TriggerEffect::DrawCards { 
                player: opponent,
                count: 1,
            }
        },
    )).id();
    
    let opponent = app.world.spawn((
        Player {},
        Library { cards: vec![card1, card2, card3] },
        Hand { cards: vec![] },
    )).id();
    
    let card1 = app.world.spawn(Card {}).id();
    let card2 = app.world.spawn(Card {}).id();
    let card3 = app.world.spawn(Card {}).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Process triggers
    app.update();
    
    // Verify commander damage was tracked
    let cmd_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(cmd_damage.get_damage(commander), 5);
    
    // Verify combat damage trigger resolved
    let hand = app.world.get::<Hand>(opponent).unwrap();
    assert_eq!(hand.cards.len(), 1);
}
```

## Test Case: Multiplayer Combat Scenarios

### Test: Attacking Multiple Players with Commander

```rust
#[test]
fn test_attacking_multiple_players_with_commander() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_attackers_system, combat_damage_system));
       
    // Create players
    let player1 = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    let active_player = app.world.spawn((
        Player {},
        ActivePlayer {},
    )).id();
    
    // Create commander with vigilance (can attack multiple players)
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: active_player },
        CreatureCard { controller: active_player },
        Vigilance {},
    )).id();
    
    // Create additional attacker
    let other_attacker = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        CreatureCard { controller: active_player },
    )).id();
    
    // Declare attackers for different players
    app.world.resource_mut::<AttackDeclaration>().declare_attacker(commander, player1);
    app.world.resource_mut::<AttackDeclaration>().declare_attacker(other_attacker, player2);
    
    // Process declare attackers
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareAttackers);
    app.update();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify commander damage was tracked for player1 only
    let cmd_damage1 = app.world.get::<CommanderDamage>(player1).unwrap();
    assert_eq!(cmd_damage1.get_damage(commander), 5);
    
    let cmd_damage2 = app.world.get::<CommanderDamage>(player2).unwrap();
    assert_eq!(cmd_damage2.get_damage(commander), 0);
    
    // Verify health reduction for both players
    let health1 = app.world.get::<Health>(player1).unwrap();
    assert_eq!(health1.current, 35); // 40 - 5 = 35
    
    let health2 = app.world.get::<Health>(player2).unwrap();
    assert_eq!(health2.current, 37); // 40 - 3 = 37
}
```

### Test: Goad Effect on Commander

```rust
#[test]
fn test_goaded_commander() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (declare_attackers_system, attack_restrictions_system));
       
    // Create players
    let player1 = app.world.spawn(Player {}).id();
    let player2 = app.world.spawn(Player {}).id();
    let player3 = app.world.spawn(Player {}).id();
    
    // Create commander that's been goaded by player2
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: player1 },
        CreatureCard { controller: player1 },
        Goad { goaded_by: player2 },
    )).id();
    
    // Set up attack validation
    app.world.resource_mut::<TurnManager>().active_player = player1;
    app.world.resource_mut::<CombatSystem>().validate_attack = |attacker, defender, world| {
        // Basic validation - can't attack yourself
        let creature_card = world.get::<CreatureCard>(attacker).unwrap();
        if creature_card.controller == defender {
            return Err(AttackError::IllegalTarget);
        }
        
        // Check goad restrictions
        if let Some(goad) = world.get::<Goad>(attacker) {
            if goad.goaded_by != defender && defender != player3 {
                return Err(AttackError::MustAttackGoader);
            }
        }
        
        Ok(())
    };
    
    // Try to attack player3 (allowed because not the goader)
    assert!(app.world.resource::<CombatSystem>()
        .validate_attack(commander, player3, &app.world)
        .is_ok());
    
    // Try to attack player2 (allowed because this is the goader)
    assert!(app.world.resource::<CombatSystem>()
        .validate_attack(commander, player2, &app.world)
        .is_ok());
    
    // Try to attack self (not allowed)
    assert!(app.world.resource::<CombatSystem>()
        .validate_attack(commander, player1, &app.world)
        .is_err());
    
    // Commander must attack if able
    app.world.resource_mut::<AttackRequirements>().creatures_that_must_attack.push(commander);
    
    // Process attack requirements
    app.update();
    
    // Verify commander is forced to attack
    assert!(app.world.get::<MustAttack>(commander).is_some());
}
```

## Test Case: Commander Zone Interactions

### Test: Combat Damage Sending Commander to Command Zone

```rust
#[test]
fn test_commander_to_command_zone_from_combat() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, state_based_actions_system, zone_change_system));
       
    // Create commander
    let commander = app.world.spawn((
        Creature { power: 3, toughness: 3 },
        Commander { owner: player },
        CreatureCard { controller: player },
        Health { current: 3, maximum: 3 },
    )).id();
    
    let player = app.world.spawn((
        Player {},
        CommandZone { commanders: vec![] },
    )).id();
    
    // Create attacking creature
    let attacker = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Attacking { defending: player },
    )).id();
    
    // Make commander block
    app.world.entity_mut(commander).insert(
        Blocking { blocked_attackers: vec![attacker] }
    );
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Check state-based actions - commander should die from damage
    app.update();
    
    // Player should be prompted to send commander to command zone
    // We'll simulate them choosing to do so
    app.world.spawn(
        ZoneChangeRequest {
            entity: commander,
            from: Zone::Battlefield,
            to: Zone::CommandZone,
            reason: ZoneChangeReason::Death,
        }
    );
    
    // Process zone change
    app.update();
    
    // Verify commander is in command zone
    assert!(!app.world.entity(commander).contains::<Health>());
    assert!(!app.world.entity(commander).contains::<Blocking>());
    
    let command_zone = app.world.get::<CommandZone>(player).unwrap();
    assert!(command_zone.commanders.contains(&commander));
}
```

### Test: Combat with Commander from Command Zone

```rust
#[test]
fn test_cast_commander_from_command_zone() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (cast_from_command_zone_system, declare_attackers_system));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Mana { 
            white: 5, blue: 5, black: 5, red: 5, green: 5, colorless: 5,
        },
        CommandZone { commanders: vec![commander] },
    )).id();
    
    // Create commander in command zone
    let commander = app.world.spawn((
        Commander { owner: player },
        CreatureCard { controller: player },
        ManaCost {
            white: 0, blue: 0, black: 0, red: 0, green: 0, colorless: 4,
            additional_cost: 0, // No command tax yet
        },
        InZone { zone: Zone::CommandZone },
    )).id();
    
    // Cast commander from command zone
    app.world.spawn(
        CastRequest {
            card: commander,
            controller: player,
            from_zone: Zone::CommandZone,
        }
    );
    
    // Process casting
    app.update();
    
    // Commander should now be on battlefield
    assert!(app.world.entity(commander).contains::<Creature>());
    assert_eq!(app.world.get::<InZone>(commander).unwrap().zone, Zone::Battlefield);
    
    // Verify mana was spent
    let mana_after = app.world.get::<Mana>(player).unwrap();
    assert_eq!(mana_after.colorless, 1); // 5 - 4 = 1
    
    // Declare commander as attacker
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::DeclareAttackers);
    app.world.resource_mut::<TurnManager>().active_player = player;
    
    app.world.resource_mut::<AttackDeclaration>().declare_attacker(commander, Entity::PLACEHOLDER);
    
    // Process declare attackers
    app.update();
    
    // Verify commander is attacking
    assert!(app.world.entity(commander).contains::<Attacking>());
}
```

## Test Case: Commander Damage Edge Cases

### Test: Commander Damage After Death and Recast

```rust
#[test]
fn test_commander_damage_after_recast() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, zone_change_system, cast_from_command_zone_system));
       
    // Create player
    let target_player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create commander owner
    let commander_owner = app.world.spawn((
        Player {},
        Mana { 
            white: 10, blue: 10, black: 10, red: 10, green: 10, colorless: 10,
        },
        CommandZone { commanders: vec![] },
    )).id();
    
    // Create commander
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: commander_owner },
        CreatureCard { controller: commander_owner },
        InZone { zone: Zone::Battlefield },
        ManaCost {
            white: 0, blue: 0, black: 0, red: 0, green: 0, colorless: 5,
            additional_cost: 0,
        },
    )).id();
    
    // Deal commander damage
    app.world.entity_mut(commander).insert(Attacking { defending: target_player });
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify initial commander damage
    let cmd_damage1 = app.world.get::<CommanderDamage>(target_player).unwrap();
    assert_eq!(cmd_damage1.get_damage(commander), 5);
    
    // Commander dies
    app.world.spawn(
        ZoneChangeRequest {
            entity: commander,
            from: Zone::Battlefield,
            to: Zone::CommandZone,
            reason: ZoneChangeReason::Death,
        }
    );
    
    // Process zone change
    app.update();
    
    // Update command zone
    app.world.entity_mut(commander_owner).insert(
        CommandZone { commanders: vec![commander] }
    );
    
    // Update commander in command zone
    app.world.entity_mut(commander).insert(
        InZone { zone: Zone::CommandZone }
    );
    
    // Cast commander from command zone (now with command tax)
    app.world.entity_mut(commander).insert(
        ManaCost {
            white: 0, blue: 0, black: 0, red: 0, green: 0, colorless: 5,
            additional_cost: 2, // Command tax
        }
    );
    
    app.world.spawn(
        CastRequest {
            card: commander,
            controller: commander_owner,
            from_zone: Zone::CommandZone,
        }
    );
    
    // Process casting
    app.update();
    
    // Commander should be back on battlefield
    assert_eq!(app.world.get::<InZone>(commander).unwrap().zone, Zone::Battlefield);
    
    // Deal more commander damage
    app.world.entity_mut(commander).insert(Attacking { defending: target_player });
    app.update();
    
    // Verify commander damage accumulates even after recasting
    let cmd_damage2 = app.world.get::<CommanderDamage>(target_player).unwrap();
    assert_eq!(cmd_damage2.get_damage(commander), 10); // 5 + 5 = 10
}
```

### Test: Commander Damage Through Redirect Effects

```rust
#[test]
fn test_commander_damage_with_redirect() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (combat_damage_system, replacement_effects_system));
       
    // Create players
    let player1 = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
        DamageReplacementEffect { 
            effect_type: ReplacementType::Redirect { 
                percentage: 100,
                target: TargetType::Player(player3),
                round_up: true,
            }
        },
    )).id();
    
    let player3 = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    // Create commander
    let commander = app.world.spawn((
        Creature { power: 5, toughness: 5 },
        Commander { owner: player1 },
        CreatureCard { controller: player1 },
        Attacking { defending: player2 },
    )).id();
    
    // Process combat damage
    app.world.resource_mut::<TurnManager>().current_phase = Phase::Combat(CombatStep::CombatDamage);
    app.update();
    
    // Verify damage was redirected
    let health2 = app.world.get::<Health>(player2).unwrap();
    assert_eq!(health2.current, 40); // No damage taken
    
    let health3 = app.world.get::<Health>(player3).unwrap();
    assert_eq!(health3.current, 35); // 40 - 5 = 35
    
    // Verify commander damage is tracked for the final recipient
    let cmd_damage3 = app.world.get::<CommanderDamage>(player3).unwrap();
    assert_eq!(cmd_damage3.get_damage(commander), 5);
    
    // Original target should not have commander damage
    let cmd_damage2 = app.world.get::<CommanderDamage>(player2).unwrap();
    assert_eq!(cmd_damage2.get_damage(commander), 0);
}
```

## Performance Considerations for Commander Combat Tests

1. **Efficient Commander Damage Tracking**: Optimize how commander damage is tracked and accumulated.

2. **Zone Change Optimizations**: Efficiently handle commander zone changes during and after combat.

3. **Multiplayer Combat Performance**: Ensure combat involving multiple players remains performant.

## Test Coverage Checklist

- [x] Multiple sources of commander damage tracking
- [x] Commander damage from clones/copies
- [x] Partner commanders in combat
- [x] Combat damage triggers from commanders
- [x] Attacking multiple players with a commander
- [x] Goaded commanders 
- [x] Commander dying in combat and going to command zone
- [x] Casting commander from command zone for combat
- [x] Commander damage persistence after recasting
- [x] Commander damage with redirection effects

## Additional Commander-Specific Edge Cases

1. Multiple copies of the same commander (from Clone effects)
2. Commanders with alternate combat damage effects (infect, wither)
3. Face-down commanders (via Morph or similar effects)
4. Commander damage with replacement effects (damage doubling)
5. "Voltron" strategies with heavily equipped/enchanted commanders 