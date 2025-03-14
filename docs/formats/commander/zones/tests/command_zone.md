# Command Zone Tests

## Overview

This document outlines test cases for the Command Zone in the Commander format. These tests ensure correct implementation of all Commander-specific rules related to the Command Zone, commander casting, commander tax, and zone transitions.

## Test Case: Commander Casting

### Test: Initial Commander Cast from Command Zone

```rust
#[test]
fn test_initial_commander_cast() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (check_commander_cast, update_command_zone));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        Mana::default(),
        ActivePlayer,
    )).id();
    
    let commander = app.world.spawn((
        Card { 
            name: "Test Commander".to_string(),
            cost: ManaCost::from_string("{2}{G}{G}").unwrap(),
        },
        Commander { owner: player, cast_count: 0 },
        Zone::CommandZone,
    )).id();
    
    // Player has enough mana to cast
    let mut player_mana = app.world.get_mut::<Mana>(player).unwrap();
    player_mana.add(ManaType::Green, 2);
    player_mana.add(ManaType::Colorless, 2);
    
    // Attempt to cast commander
    app.world.send_event(CastSpellEvent {
        caster: player,
        card: commander,
    });
    app.update();
    
    // Verify commander moved to stack
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::Stack);
    
    // Verify no additional tax was applied (first cast)
    assert_eq!(app.world.get::<Commander>(commander).unwrap().cast_count, 1);
}
```

### Test: Recasting Commander with Commander Tax

```rust
#[test]
fn test_commander_tax() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (check_commander_cast, update_command_zone, apply_commander_tax));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        Mana::default(),
        ActivePlayer,
    )).id();
    
    let commander = app.world.spawn((
        Card { 
            name: "Test Commander".to_string(),
            cost: ManaCost::from_string("{2}{G}{G}").unwrap(),
        },
        Commander { owner: player, cast_count: 1 }, // Already cast once before
        Zone::CommandZone,
    )).id();
    
    // Player has enough mana to cast with tax
    let mut player_mana = app.world.get_mut::<Mana>(player).unwrap();
    player_mana.add(ManaType::Green, 2);
    player_mana.add(ManaType::Colorless, 4);  // 2 original + 2 tax
    
    // Attempt to cast commander
    app.world.send_event(CastSpellEvent {
        caster: player,
        card: commander,
    });
    app.update();
    
    // Verify commander moved to stack
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::Stack);
    
    // Verify cast count was incremented
    assert_eq!(app.world.get::<Commander>(commander).unwrap().cast_count, 2);
    
    // Verify mana was properly deducted including tax
    let player_mana = app.world.get::<Mana>(player).unwrap();
    assert_eq!(player_mana.get(ManaType::Green), 0);
    assert_eq!(player_mana.get(ManaType::Colorless), 0);
}
```

### Test: Insufficient Mana for Commander Tax

```rust
#[test]
fn test_insufficient_mana_for_commander_tax() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (check_commander_cast, update_command_zone, apply_commander_tax));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        Mana::default(),
        ActivePlayer,
    )).id();
    
    let commander = app.world.spawn((
        Card { 
            name: "Test Commander".to_string(),
            cost: ManaCost::from_string("{2}{G}{G}").unwrap(),
        },
        Commander { owner: player, cast_count: 2 }, // Already cast twice before
        Zone::CommandZone,
    )).id();
    
    // Player doesn't have enough mana for tax
    let mut player_mana = app.world.get_mut::<Mana>(player).unwrap();
    player_mana.add(ManaType::Green, 2);
    player_mana.add(ManaType::Colorless, 3);  // 2 original + 4 tax needed, only 3 available
    
    // Attempt to cast commander
    app.world.send_event(CastSpellEvent {
        caster: player,
        card: commander,
    });
    app.update();
    
    // Verify commander stayed in command zone
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::CommandZone);
    
    // Verify cast count was not incremented
    assert_eq!(app.world.get::<Commander>(commander).unwrap().cast_count, 2);
}
```

## Test Case: Zone Transitions

### Test: Commander Dies and Goes to Command Zone

```rust
#[test]
fn test_commander_death_to_command_zone() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_zone_transitions, commander_replacement_effects));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        CommanderZoneChoice { use_command_zone: true },
    )).id();
    
    let commander = app.world.spawn((
        Card { name: "Test Commander".to_string() },
        Commander { owner: player, cast_count: 1 },
        Zone::Battlefield,
    )).id();
    
    // Trigger death of commander
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Graveyard,
        cause: ZoneChangeCause::Death,
    });
    app.update();
    
    // Verify commander went to command zone instead of graveyard
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::CommandZone);
}
```

### Test: Commander Goes to Exile and Replaced to Command Zone

```rust
#[test]
fn test_commander_exile_to_command_zone() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_zone_transitions, commander_replacement_effects));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        CommanderZoneChoice { use_command_zone: true },
    )).id();
    
    let commander = app.world.spawn((
        Card { name: "Test Commander".to_string() },
        Commander { owner: player, cast_count: 1 },
        Zone::Battlefield,
    )).id();
    
    // Trigger exile of commander
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Exile,
        cause: ZoneChangeCause::Exile,
    });
    app.update();
    
    // Verify commander went to command zone instead of exile
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::CommandZone);
}
```

### Test: Commander Goes to Hand by Player Choice

```rust
#[test]
fn test_commander_to_hand_by_choice() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_zone_transitions, commander_replacement_effects));
       
    // Create player and their commander
    let player = app.world.spawn((
        Player {},
        CommanderZoneChoice { use_command_zone: false }, // Choose not to use command zone
    )).id();
    
    let commander = app.world.spawn((
        Card { name: "Test Commander".to_string() },
        Commander { owner: player, cast_count: 1 },
        Zone::Battlefield,
    )).id();
    
    // Trigger bounce of commander
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Hand,
        cause: ZoneChangeCause::ReturnToHand,
    });
    app.update();
    
    // Verify commander went to hand as chosen
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::Hand);
}
```

## Test Case: Partner Commanders

### Test: Two Partner Commanders in Command Zone

```rust
#[test]
fn test_partner_commanders() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (check_command_zone_validity, handle_partner_mechanics));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        CommanderList::default(),
    )).id();
    
    // Create two partner commanders
    let commander1 = app.world.spawn((
        Card { name: "Partner Commander 1".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        Zone::CommandZone,
    )).id();
    
    let commander2 = app.world.spawn((
        Card { name: "Partner Commander 2".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        Zone::CommandZone,
    )).id();
    
    // Add commanders to player's commander list
    let mut commander_list = app.world.get_mut::<CommanderList>(player).unwrap();
    commander_list.add(commander1);
    commander_list.add(commander2);
    app.update();
    
    // Verify player can have two commanders because they have partner
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(validation_errors.is_empty());
    
    // Verify both commanders are in the command zone
    assert_eq!(app.world.get::<Zone>(commander1).unwrap(), &Zone::CommandZone);
    assert_eq!(app.world.get::<Zone>(commander2).unwrap(), &Zone::CommandZone);
}
```

## Test Case: Command Zone Visibility

### Test: Command Zone Cards Visible to All Players

```rust
#[test]
fn test_command_zone_visibility() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, check_card_visibility);
       
    // Create multiple players
    let player1 = app.world.spawn(Player {}).id();
    let player2 = app.world.spawn(Player {}).id();
    let player3 = app.world.spawn(Player {}).id();
    
    // Create a commander in the command zone
    let commander = app.world.spawn((
        Card { name: "Test Commander".to_string() },
        Commander { owner: player1, cast_count: 0 },
        Zone::CommandZone,
        Visibility::default(),
    )).id();
    
    // Update visibility
    app.update();
    
    // Verify all players can see the commander in command zone
    let visibility = app.world.get::<Visibility>(commander).unwrap();
    assert!(visibility.is_visible_to(player1));
    assert!(visibility.is_visible_to(player2));
    assert!(visibility.is_visible_to(player3));
}
```

## Test Case: Commander Identity Restrictions

### Test: Card Color Identity Validation for Commander Deck

```rust
#[test]
fn test_color_identity_restrictions() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, validate_deck_color_identity);
       
    // Create player and commander
    let player = app.world.spawn(Player {}).id();
    
    let commander = app.world.spawn((
        Card { name: "Mono-Green Commander".to_string() },
        Commander { owner: player, cast_count: 0 },
        ColorIdentity { colors: vec![Color::Green] },
    )).id();
    
    // Create legal and illegal cards for the deck
    let legal_card = app.world.spawn((
        Card { name: "Green Card".to_string() },
        ColorIdentity { colors: vec![Color::Green] },
        InDeck { owner: player },
    )).id();
    
    let illegal_card = app.world.spawn((
        Card { name: "Blue Card".to_string() },
        ColorIdentity { colors: vec![Color::Blue] },
        InDeck { owner: player },
    )).id();
    
    // Create deck with the commander
    app.world.spawn((
        Deck { owner: player },
        CommanderDeck { commander: commander },
        Cards { entities: vec![legal_card, illegal_card] },
    ));
    
    // Validate deck
    app.update();
    
    // Verify validation errors for illegal card
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(!validation_errors.is_empty());
    assert!(validation_errors.contains_error_about(illegal_card, "color identity"));
}
```

These test cases provide comprehensive coverage of the Command Zone mechanics and ensure that all the format-specific rules are correctly implemented and enforced. 