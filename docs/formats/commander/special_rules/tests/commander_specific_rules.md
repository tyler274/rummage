# Commander-Specific Rules Tests

## Overview

This document outlines test cases for Commander-specific rules, including starting life totals, commander tax, color identity restrictions, singleton deck construction, and other format-specific mechanics that don't fit into other test categories.

## Test Case: Starting Life Total

### Test: 40 Life for Commander Format

```rust
#[test]
fn test_commander_starting_life() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, initialize_game);
       
    // Set game format
    app.insert_resource(GameFormat::Commander);
    
    // Create players
    let player1 = app.world.spawn(Player {}).id();
    let player2 = app.world.spawn(Player {}).id();
    let player3 = app.world.spawn(Player {}).id();
    let player4 = app.world.spawn(Player {}).id();
    
    // Initialize game
    app.world.send_event(InitializeGameEvent {
        players: vec![player1, player2, player3, player4],
    });
    app.update();
    
    // Verify all players have 40 life
    for player in [player1, player2, player3, player4].iter() {
        let health = app.world.get::<Health>(*player).unwrap();
        assert_eq!(health.current, 40);
        assert_eq!(health.maximum, 40);
    }
}
```

## Test Case: 100-Card Singleton Deck Validation

### Test: Singleton Rule (Only One Copy of Each Card)

```rust
#[test]
fn test_singleton_rule() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, validate_commander_deck);
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create a commander
    let commander = app.world.spawn((
        Card { name: "Commander".to_string() },
        Commander { owner: player, cast_count: 0 },
    )).id();
    
    // Create cards for the deck
    let card1 = app.world.spawn((
        Card { name: "Unique Card 1".to_string() },
        CardIdentity { oracle_id: "id1".to_string() },
    )).id();
    
    let card2 = app.world.spawn((
        Card { name: "Unique Card 2".to_string() },
        CardIdentity { oracle_id: "id2".to_string() },
    )).id();
    
    // Duplicate card (same oracle ID)
    let duplicate_card = app.world.spawn((
        Card { name: "Unique Card 1".to_string() },
        CardIdentity { oracle_id: "id1".to_string() },
    )).id();
    
    // Basic land (allowed multiple copies)
    let basic_land = app.world.spawn((
        Card { name: "Forest".to_string() },
        CardIdentity { oracle_id: "forest_id".to_string() },
        BasicLand,
    )).id();
    
    let basic_land2 = app.world.spawn((
        Card { name: "Forest".to_string() },
        CardIdentity { oracle_id: "forest_id".to_string() },
        BasicLand,
    )).id();
    
    // Create deck with the commander
    app.world.spawn((
        Deck { owner: player },
        CommanderDeck { commander },
        Cards { entities: vec![card1, card2, duplicate_card, basic_land, basic_land2] },
    ));
    
    // Validate deck
    app.update();
    
    // Verify validation errors for duplicate card
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(!validation_errors.is_empty());
    
    // Should have error about duplicate card
    let duplicate_error = validation_errors.errors.iter().any(|error| {
        error.card == duplicate_card && error.error_type == ValidationErrorType::DuplicateCard
    });
    assert!(duplicate_error);
    
    // Should NOT have error about basic lands
    let basic_land_error = validation_errors.errors.iter().any(|error| {
        error.card == basic_land || error.card == basic_land2
    });
    assert!(!basic_land_error);
}
```

### Test: Exactly 100 Cards Including Commander

```rust
#[test]
fn test_deck_size() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, validate_commander_deck);
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create a commander
    let commander = app.world.spawn((
        Card { name: "Commander".to_string() },
        Commander { owner: player, cast_count: 0 },
    )).id();
    
    // Create 98 cards for deck (which is invalid, should be 99 + commander)
    let mut cards = Vec::new();
    for i in 0..98 {
        let card = app.world.spawn((
            Card { name: format!("Card {}", i) },
            CardIdentity { oracle_id: format!("id{}", i) },
        )).id();
        cards.push(card);
    }
    
    // Create deck with the commander
    app.world.spawn((
        Deck { owner: player },
        CommanderDeck { commander },
        Cards { entities: cards.clone() },
    ));
    
    // Validate deck
    app.update();
    
    // Verify validation error for deck size
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(!validation_errors.is_empty());
    
    let size_error = validation_errors.errors.iter().any(|error| {
        error.error_type == ValidationErrorType::InvalidDeckSize
    });
    assert!(size_error);
    
    // Add one more card to make it 99 + commander = 100
    let last_card = app.world.spawn((
        Card { name: "Card 99".to_string() },
        CardIdentity { oracle_id: "id99".to_string() },
    )).id();
    cards.push(last_card);
    
    // Update deck
    app.world.query_mut::<&mut Cards>().for_each_mut(|mut deck_cards| {
        deck_cards.entities = cards.clone();
    });
    
    // Clear previous errors
    app.world.resource_mut::<ValidationErrors>().errors.clear();
    
    // Re-validate deck
    app.update();
    
    // Verify no validation errors for deck size
    let validation_errors = app.world.resource::<ValidationErrors>();
    let size_error = validation_errors.errors.iter().any(|error| {
        error.error_type == ValidationErrorType::InvalidDeckSize
    });
    assert!(!size_error);
}
```

## Test Case: Commander Tax Implementation

### Test: Tracking Commander Tax Across Zone Changes

```rust
#[test]
fn test_commander_tax_tracking() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (update_commander_tax, handle_zone_transitions, cast_from_command_zone));
       
    // Create player
    let player = app.world.spawn((
        Player {},
        Mana::default(),
    )).id();
    
    // Create commander
    let commander = app.world.spawn((
        Card { 
            name: "Test Commander".to_string(),
            cost: ManaCost::from_string("{3}{R}").unwrap(),
        },
        Commander { owner: player, cast_count: 0 },
        Zone::CommandZone,
    )).id();
    
    // Cast commander first time
    app.world.send_event(CastSpellEvent {
        caster: player,
        spell: commander,
    });
    app.update();
    
    // Verify commander moved to stack
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::Stack);
    
    // Verify cast count increased to 1
    assert_eq!(app.world.get::<Commander>(commander).unwrap().cast_count, 1);
    
    // Resolve spell (move to battlefield)
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Stack,
        to: Zone::Battlefield,
        cause: ZoneChangeCause::SpellResolution,
    });
    app.update();
    
    // Send to command zone
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::CommandZone,
        cause: ZoneChangeCause::CommanderReplacement,
    });
    app.update();
    
    // Cast commander second time
    app.world.send_event(CastSpellEvent {
        caster: player,
        spell: commander,
    });
    app.update();
    
    // Verify cast count increased to 2
    assert_eq!(app.world.get::<Commander>(commander).unwrap().cast_count, 2);
    
    // Verify cost now includes commander tax (2 additional mana)
    let required_mana = app.world.resource::<LastCastAttempt>().required_mana.clone();
    assert_eq!(required_mana.total_cmc(), 7); // 3R + 2 tax = 7
}
```

## Test Case: Partner Commanders

### Test: Two Partner Commanders

```rust
#[test]
fn test_partner_commanders() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, validate_commander_setup);
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create two partner commanders
    let commander1 = app.world.spawn((
        Card { name: "Partner Commander 1".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        ColorIdentity { colors: vec![Color::Red, Color::White] },
    )).id();
    
    let commander2 = app.world.spawn((
        Card { name: "Partner Commander 2".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        ColorIdentity { colors: vec![Color::Blue, Color::Black] },
    )).id();
    
    // Set commanders for player
    app.world.spawn((
        CommanderList { commanders: vec![commander1, commander2] },
        Owner { player },
    ));
    
    // Validate commander setup
    app.update();
    
    // Verify no validation errors
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(validation_errors.is_empty());
    
    // Verify combined color identity
    let color_identity = app.world.resource::<CombinedColorIdentity>();
    let player_identity = color_identity.get_colors(player);
    assert!(player_identity.contains(&Color::Red));
    assert!(player_identity.contains(&Color::White));
    assert!(player_identity.contains(&Color::Blue));
    assert!(player_identity.contains(&Color::Black));
}
```

### Test: Two Non-Partner Commanders (Invalid)

```rust
#[test]
fn test_invalid_multiple_commanders() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, validate_commander_setup);
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create two regular commanders without partner
    let commander1 = app.world.spawn((
        Card { name: "Commander 1".to_string() },
        Commander { owner: player, cast_count: 0 },
        ColorIdentity { colors: vec![Color::Green] },
    )).id();
    
    let commander2 = app.world.spawn((
        Card { name: "Commander 2".to_string() },
        Commander { owner: player, cast_count: 0 },
        ColorIdentity { colors: vec![Color::Black] },
    )).id();
    
    // Set commanders for player
    app.world.spawn((
        CommanderList { commanders: vec![commander1, commander2] },
        Owner { player },
    ));
    
    // Validate commander setup
    app.update();
    
    // Verify validation errors
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(!validation_errors.is_empty());
    
    // Should have error about too many commanders without partner
    let multi_commander_error = validation_errors.errors.iter().any(|error| {
        error.error_type == ValidationErrorType::MultipleCommandersWithoutPartner
    });
    assert!(multi_commander_error);
}
```

## Test Case: Commander Color Identity

### Test: Combined Color Identity with Partner Commanders

```rust
#[test]
fn test_color_identity_calculation() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (calculate_color_identity, validate_commander_deck));
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create partner commanders with different color identities
    let commander1 = app.world.spawn((
        Card { name: "Silas Renn".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        ColorIdentity { colors: vec![Color::Blue, Color::Black] },
    )).id();
    
    let commander2 = app.world.spawn((
        Card { name: "Akiri".to_string() },
        Commander { owner: player, cast_count: 0 },
        PartnerAbility,
        ColorIdentity { colors: vec![Color::Red, Color::White] },
    )).id();
    
    // Set commanders for player
    app.world.spawn((
        CommanderList { commanders: vec![commander1, commander2] },
        Owner { player },
    ));
    
    // Calculate combined color identity
    app.update();
    
    // Verify color identity includes all colors from both commanders
    let color_identity = app.world.resource::<CombinedColorIdentity>();
    let player_identity = color_identity.get_colors(player);
    
    assert!(player_identity.contains(&Color::Blue));
    assert!(player_identity.contains(&Color::Black));
    assert!(player_identity.contains(&Color::Red));
    assert!(player_identity.contains(&Color::White));
    assert!(!player_identity.contains(&Color::Green)); // Should not have green
    
    // Create cards with various color identities
    let valid_card = app.world.spawn((
        Card { name: "Esper Card".to_string() },
        ColorIdentity { colors: vec![Color::White, Color::Blue, Color::Black] },
    )).id();
    
    let invalid_card = app.world.spawn((
        Card { name: "Green Card".to_string() },
        ColorIdentity { colors: vec![Color::Green] },
    )).id();
    
    // Create deck with these cards
    app.world.spawn((
        Deck { owner: player },
        CommanderDeck { commander: commander1 }, // Just refers to one commander
        Cards { entities: vec![valid_card, invalid_card] },
    ));
    
    // Validate deck against color identity
    app.update();
    
    // Verify validation error for card outside color identity
    let validation_errors = app.world.resource::<ValidationErrors>();
    assert!(!validation_errors.is_empty());
    
    let color_identity_error = validation_errors.errors.iter().any(|error| {
        error.card == invalid_card && error.error_type == ValidationErrorType::ColorIdentityViolation
    });
    assert!(color_identity_error);
    
    // Should NOT have error for valid card
    let valid_card_error = validation_errors.errors.iter().any(|error| {
        error.card == valid_card
    });
    assert!(!valid_card_error);
}
```

### Test: Color Identity Includes Mana Symbols in Rules Text

```rust
#[test]
fn test_color_identity_from_rules_text() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, calculate_comprehensive_color_identity);
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create cards with various color identity components
    
    // Card with mana cost only
    let card1 = app.world.spawn((
        Card { 
            name: "Red Creature".to_string(),
            cost: ManaCost::from_string("{1}{R}").unwrap(),
        },
        RulesText { text: "This is a red creature.".to_string() },
    )).id();
    
    // Card with color indicator
    let card2 = app.world.spawn((
        Card { 
            name: "Blue Card".to_string(),
            cost: ManaCost::from_string("{1}").unwrap(),
        },
        ColorIndicator { color: Color::Blue },
    )).id();
    
    // Card with mana symbol in rules text
    let card3 = app.world.spawn((
        Card { 
            name: "Colorless Card".to_string(),
            cost: ManaCost::from_string("{1}").unwrap(),
        },
        RulesText { text: "{G}: This card gets +1/+1 until end of turn.".to_string() },
    )).id();
    
    // Card with hybrid mana in cost
    let card4 = app.world.spawn((
        Card { 
            name: "Hybrid Card".to_string(),
            cost: ManaCost::from_string("{1}{W/B}").unwrap(),
        },
    )).id();
    
    // Calculate comprehensive color identity
    app.update();
    
    // Verify correct color identities
    let c1 = app.world.get::<ColorIdentity>(card1).unwrap();
    assert_eq!(c1.colors.len(), 1);
    assert!(c1.colors.contains(&Color::Red));
    
    let c2 = app.world.get::<ColorIdentity>(card2).unwrap();
    assert_eq!(c2.colors.len(), 1);
    assert!(c2.colors.contains(&Color::Blue));
    
    let c3 = app.world.get::<ColorIdentity>(card3).unwrap();
    assert_eq!(c3.colors.len(), 1);
    assert!(c3.colors.contains(&Color::Green));
    
    let c4 = app.world.get::<ColorIdentity>(card4).unwrap();
    assert_eq!(c4.colors.len(), 2);
    assert!(c4.colors.contains(&Color::White));
    assert!(c4.colors.contains(&Color::Black));
}
```

## Test Case: Commander Damage

### Test: Commander Damage Win Condition

```rust
#[test]
fn test_commander_damage_win_condition() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (apply_combat_damage, check_commander_damage_win_condition));
       
    // Create players
    let player1 = app.world.spawn((
        Player { id: 1 },
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    let player2 = app.world.spawn((
        Player { id: 2 },
        Health { current: 40, maximum: 40 },
    )).id();
    
    // Create commander for player 2
    let commander = app.world.spawn((
        Card { name: "Commander".to_string() },
        Commander { owner: player2, cast_count: 0 },
        Creature { power: 7, toughness: 7 },
        Zone::Battlefield,
    )).id();
    
    // Apply commander damage 3 times (7 * 3 = 21 damage)
    for _ in 0..3 {
        app.world.send_event(CommanderDamageEvent {
            commander,
            target: player1,
            amount: 7,
        });
        app.update();
    }
    
    // Verify player received 21 commander damage
    let commander_damage = app.world.get::<CommanderDamage>(player1).unwrap();
    assert_eq!(commander_damage.get_damage(commander), 21);
    
    // Verify player was eliminated due to commander damage
    let player_status = app.world.get::<PlayerStatus>(player1).unwrap();
    assert_eq!(player_status.state, PlayerState::Eliminated);
    
    // Verify elimination reason is commander damage
    assert_eq!(player_status.elimination_reason, Some(EliminationReason::CommanderDamage));
}
```

### Test: Tracking Commander Damage Separately

```rust
#[test]
fn test_multiple_commander_damage_tracking() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, apply_combat_damage);
       
    // Create player and opponents
    let player = app.world.spawn((
        Player {},
        Health { current: 40, maximum: 40 },
        CommanderDamage::default(),
    )).id();
    
    let opponent1 = app.world.spawn(Player { id: 1 }).id();
    let opponent2 = app.world.spawn(Player { id: 2 }).id();
    let opponent3 = app.world.spawn(Player { id: 3 }).id();
    
    // Create commanders for each opponent
    let commander1 = app.world.spawn((
        Card { name: "Commander 1".to_string() },
        Commander { owner: opponent1, cast_count: 0 },
        Creature { power: 3, toughness: 3 },
    )).id();
    
    let commander2 = app.world.spawn((
        Card { name: "Commander 2".to_string() },
        Commander { owner: opponent2, cast_count: 0 },
        Creature { power: 5, toughness: 5 },
    )).id();
    
    let commander3 = app.world.spawn((
        Card { name: "Commander 3".to_string() },
        Commander { owner: opponent3, cast_count: 0 },
        Creature { power: 2, toughness: 2 },
    )).id();
    
    // Apply damage from each commander
    app.world.send_event(CommanderDamageEvent {
        commander: commander1,
        target: player,
        amount: 3,
    });
    
    app.world.send_event(CommanderDamageEvent {
        commander: commander2,
        target: player,
        amount: 5,
    });
    
    app.world.send_event(CommanderDamageEvent {
        commander: commander3,
        target: player,
        amount: 2,
    });
    app.update();
    
    // Verify damage was tracked separately for each commander
    let commander_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(commander_damage.get_damage(commander1), 3);
    assert_eq!(commander_damage.get_damage(commander2), 5);
    assert_eq!(commander_damage.get_damage(commander3), 2);
    
    // Verify total life loss is the sum of all commander damage
    let health = app.world.get::<Health>(player).unwrap();
    assert_eq!(health.current, 30); // 40 - (3+5+2)
    
    // Apply more damage from commander2 to reach lethal from a single commander
    app.world.send_event(CommanderDamageEvent {
        commander: commander2,
        target: player,
        amount: 16,
    });
    app.update();
    
    // Verify commander2 damage is now 21
    let commander_damage = app.world.get::<CommanderDamage>(player).unwrap();
    assert_eq!(commander_damage.get_damage(commander2), 21);
    
    // Verify player was eliminated
    let player_status = app.world.get::<PlayerStatus>(player).unwrap();
    assert_eq!(player_status.state, PlayerState::Eliminated);
}
```

## Test Case: Command Zone Mechanics

### Test: Commander Replacement Effects for Different Zones

```rust
#[test]
fn test_commander_replacement_effects() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_zone_transitions, commander_replacement_effects));
       
    // Create player with commander zone choice preferences
    let player = app.world.spawn((
        Player {},
        CommanderZoneChoice::default(), // Defaults to sending commanders to command zone
    )).id();
    
    // Create commander
    let commander = app.world.spawn((
        Card { name: "Commander".to_string() },
        Commander { owner: player, cast_count: 0 },
        Zone::Battlefield,
    )).id();
    
    // Test commander dying (would go to graveyard)
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Graveyard,
        cause: ZoneChangeCause::Death,
    });
    app.update();
    
    // Verify commander went to command zone instead
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::CommandZone);
    
    // Move commander back to battlefield for next test
    app.world.get_mut::<Zone>(commander).unwrap().0 = Zone::Battlefield.0;
    
    // Test commander being exiled
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Exile,
        cause: ZoneChangeCause::Exile,
    });
    app.update();
    
    // Verify commander went to command zone instead
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::CommandZone);
    
    // Test with player choosing to let commander go to graveyard
    app.world.get_mut::<CommanderZoneChoice>(player).unwrap().use_command_zone = false;
    
    // Move commander back to battlefield
    app.world.get_mut::<Zone>(commander).unwrap().0 = Zone::Battlefield.0;
    
    // Test commander dying with choice to go to graveyard
    app.world.send_event(ZoneChangeEvent {
        entity: commander,
        from: Zone::Battlefield,
        to: Zone::Graveyard,
        cause: ZoneChangeCause::Death,
    });
    app.update();
    
    // Verify commander went to graveyard as chosen
    assert_eq!(app.world.get::<Zone>(commander).unwrap(), &Zone::Graveyard);
}
```

These test cases provide comprehensive coverage for Commander-specific rules and mechanics that form the foundation of the format. 