# Game UI Testing Guide

This document outlines testing strategies and methodologies for Rummage's game UI system, with a focus on ensuring stability, correctness, and usability across different scenarios.

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Unit Testing](#unit-testing)
3. [Integration Testing](#integration-testing)
4. [Visual Regression Testing](#visual-regression-testing)
5. [Performance Testing](#performance-testing)
6. [Accessibility Testing](#accessibility-testing)
7. [Automation Framework](#automation-framework)
8. [Test Case Organization](#test-case-organization)

## Testing Philosophy

The UI testing approach for Rummage follows these core principles:

1. **Comprehensive Coverage**: Test all UI components across different configurations
2. **Behavior-Driven**: Focus on testing functionality from a user perspective
3. **Automated Where Possible**: Leverage automation for regression testing
4. **Visual Correctness**: Ensure visual elements appear as designed
5. **Performance Aware**: Verify UI performs well under different conditions

## Unit Testing

Unit tests focus on testing individual UI components in isolation.

### Component Testing

Test each UI component separately to verify:

- Correct initialization
- Proper event handling
- State transitions
- Component lifecycle behaviors

Example test for a component:

```rust
#[test]
fn test_hand_zone_initialization() {
    // Create app with test plugins
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, ui_systems::update_hand_zone);
    
    // Setup test state
    let player_id = PlayerId(1);
    let test_cards = vec![
        Card::new("Test Card 1"),
        Card::new("Test Card 2"),
        Card::new("Test Card 3"),
    ];
    
    // Spawn hand zone with test cards
    app.world.spawn((
        HandZone { player_id, expanded: false },
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(200.0),
            ..default()
        },
    ));
    
    // Add cards to player's hand
    let hand = Hand { cards: test_cards, player_id };
    app.world.insert_resource(hand);
    
    // Run systems
    app.update();
    
    // Verify hand zone contains correct number of card entities
    let hand_entity = app.world.query_filtered::<Entity, With<HandZone>>().single(&app.world);
    let children = app.world.get::<Children>(hand_entity).unwrap();
    assert_eq!(children.len(), 3, "Hand zone should contain 3 card entities");
}
```

### Event Handling Tests

Test that UI components respond correctly to events:

```rust
#[test]
fn test_card_drag_event_handling() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_event::<CardDragStartEvent>()
       .add_event::<CardDragEndEvent>()
       .add_systems(Update, ui_systems::handle_card_drag_events);
    
    // Set up test entities
    let card_entity = app.world.spawn(Card::new("Test Card")).id();
    
    // Trigger drag start event
    app.world.send_event(CardDragStartEvent {
        card_entity,
        cursor_position: Vec2::new(100.0, 100.0),
    });
    
    // Run systems
    app.update();
    
    // Verify card is being dragged
    let dragging = app.world.get::<Dragging>(card_entity).unwrap();
    assert!(dragging.active, "Card should be in dragging state");
    
    // Trigger drag end event
    app.world.send_event(CardDragEndEvent {
        card_entity,
        cursor_position: Vec2::new(200.0, 200.0),
    });
    
    // Run systems
    app.update();
    
    // Verify card is no longer being dragged
    assert!(app.world.get::<Dragging>(card_entity).is_none(), "Dragging component should be removed");
}
```

### Layout Tests

Verify that UI layout components arrange children correctly:

```rust
#[test]
fn test_battlefield_layout() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, ui_systems::position_battlefield_cards);
    
    // Set up battlefield entity with cards
    let battlefield_entity = app.world.spawn((
        BattlefieldZone {
            player_id: PlayerId(1),
            organization: BattlefieldOrganization::ByType,
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
    )).id();
    
    // Add some test cards to the battlefield
    let card_entities = (0..5).map(|i| {
        let card_entity = app.world.spawn((
            Card::new(&format!("Test Card {}", i)),
            CardType::Creature,
            Transform::default(),
            GlobalTransform::default(),
            Parent(battlefield_entity),
        )).id();
        app.world.entity_mut(battlefield_entity).add_child(card_entity);
        card_entity
    }).collect::<Vec<_>>();
    
    // Run systems
    app.update();
    
    // Verify cards are positioned correctly
    for (i, card_entity) in card_entities.iter().enumerate() {
        let transform = app.world.get::<Transform>(*card_entity).unwrap();
        // Cards should be arranged in a row with spacing
        assert_approx_eq!(transform.translation.x, i as f32 * 120.0, 1.0);
    }
}
```

## Integration Testing

Integration tests verify that multiple UI components work together correctly.

### Playmat Integration Tests

Test that all zones in a player's playmat interact correctly:

```rust
#[test]
fn test_card_movement_between_zones() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_systems(Update, (
           ui_systems::handle_card_movement,
           ui_systems::update_zones,
       ));
    
    // Set up test player and playmat
    let player_id = PlayerId(1);
    setup_test_playmat(&mut app, player_id);
    
    // Get zone entities
    let hand_entity = app.world.query_filtered::<Entity, With<HandZone>>().single(&app.world);
    let battlefield_entity = app.world.query_filtered::<Entity, With<BattlefieldZone>>().single(&app.world);
    
    // Create test card in hand
    let card_entity = app.world.spawn((
        Card::new("Test Creature"),
        CardType::Creature,
        Transform::default(),
        GlobalTransform::default(),
        Parent(hand_entity),
        InZone::Hand,
    )).id();
    app.world.entity_mut(hand_entity).add_child(card_entity);
    
    // Simulate playing card from hand to battlefield
    app.world.send_event(PlayCardEvent {
        card_entity,
        source_zone: Zone::Hand,
        destination_zone: Zone::Battlefield,
        player_id,
    });
    
    // Run systems
    app.update();
    
    // Verify card moved to battlefield
    let card_parent = app.world.get::<Parent>(card_entity).unwrap();
    assert_eq!(card_parent.get(), battlefield_entity, "Card should be in battlefield");
    
    let in_zone = app.world.get::<InZone>(card_entity).unwrap();
    assert_eq!(*in_zone, InZone::Battlefield, "Card zone should be Battlefield");
}
```

### Table Integration Tests

Test the entire table layout with multiple players:

```rust
#[test]
fn test_four_player_table_layout() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_systems(Update, ui_systems::update_table_layout);
    
    // Set up game state with 4 players
    let mut game_state = GameState::default();
    game_state.player_count = 4;
    app.world.insert_resource(game_state);
    
    // Set up virtual table
    let table_entity = app.world.spawn((
        VirtualTable,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    )).id();
    
    // Trigger table setup
    app.world.send_event(SetupTableEvent { player_count: 4 });
    
    // Run systems
    app.update();
    
    // Verify table has correct structure for 4 players
    let children = app.world.get::<Children>(table_entity).unwrap();
    
    // Should have 5 children: 4 player playmats + shared area
    assert_eq!(children.len(), 5, "Table should have 5 main areas for 4 players");
    
    // Verify each player's playmat exists and has correct position
    let playmat_query = app.world.query_filtered::<(&PlayerPlaymat, &Node), With<Node>>();
    assert_eq!(playmat_query.iter(&app.world).count(), 4, "Should have 4 player playmats");
}
```

### UI Flow Tests

Test complete UI workflows from start to finish:

```rust
#[test]
fn test_cast_spell_targeting_ui_flow() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_systems(Update, (
           ui_systems::handle_card_selection,
           ui_systems::handle_targeting,
           ui_systems::update_stack_visualization,
       ));
    
    // Set up game with 2 players
    setup_test_game(&mut app, 2);
    
    // Get player hand and opponent battlefield
    let player_id = PlayerId(1);
    let opponent_id = PlayerId(2);
    let hand_entity = get_player_zone_entity(&app, player_id, ZoneType::Hand);
    let opp_battlefield_entity = get_player_zone_entity(&app, opponent_id, ZoneType::Battlefield);
    
    // Add test spell to player's hand
    let spell_entity = app.world.spawn((
        Card::new("Test Lightning Bolt"),
        CardType::Instant,
        RequiresTarget { valid_targets: TargetType::CreatureOrPlayer },
        Transform::default(),
        GlobalTransform::default(),
        Parent(hand_entity),
        InZone::Hand,
    )).id();
    app.world.entity_mut(hand_entity).add_child(spell_entity);
    
    // Add creature to opponent's battlefield
    let creature_entity = app.world.spawn((
        Card::new("Test Creature"),
        CardType::Creature,
        Transform::default(),
        GlobalTransform::default(),
        Parent(opp_battlefield_entity),
        InZone::Battlefield,
    )).id();
    app.world.entity_mut(opp_battlefield_entity).add_child(creature_entity);
    
    // Simulate spell cast
    app.world.send_event(CardSelectedEvent {
        card_entity: spell_entity,
        player_id,
    });
    app.update();
    
    // Verify targeting mode is active
    let ui_state = app.world.resource::<UiState>();
    assert_eq!(ui_state.mode, UiMode::Targeting, "UI should be in targeting mode");
    
    // Simulate target selection
    app.world.send_event(TargetSelectedEvent {
        source_entity: spell_entity,
        target_entity: creature_entity,
        player_id,
    });
    app.update();
    
    // Verify spell is on stack
    let stack_entity = app.world.query_filtered::<Entity, With<StackZone>>().single(&app.world);
    let stack_children = app.world.get::<Children>(stack_entity).unwrap();
    assert!(!stack_children.is_empty(), "Stack should contain the cast spell");
    
    // Verify targeting visualization
    let targeting = app.world.query_filtered::<&TargetingVisualization, With<TargetingVisualization>>().single(&app.world);
    assert_eq!(targeting.source, spell_entity);
    assert_eq!(targeting.target, creature_entity);
}
```

## Visual Regression Testing

Visual tests verify that UI components appear correctly.

### Screenshot Comparison Tests

Compare screenshots of UI elements against reference images:

```rust
#[test]
fn test_card_appearance() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(VisualTestPlugins)
       .add_systems(Update, ui_systems::render_card);
    
    // Set up test card
    let card = Card::new("Test Card");
    card.card_type = CardType::Creature;
    card.mana_cost = "2R".into();
    
    app.world.spawn((
        card,
        Transform::from_xyz(400.0, 300.0, 0.0),
        GlobalTransform::default(),
    ));
    
    // Render frame
    app.update();
    
    // Take screenshot
    let screenshot = take_screenshot(&app);
    
    // Compare with reference image
    let reference = load_reference_image("card_appearance.png");
    let difference = compare_images(&screenshot, &reference);
    
    assert!(difference < 0.01, "Card appearance doesn't match reference");
}
```

### Layout Verification Tests

Verify layout properties of UI elements:

```rust
#[test]
fn test_playmat_layout_proportions() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins);
    
    // Set up test playmat
    setup_test_playmat(&mut app, PlayerId(1));
    
    // Run systems
    app.update();
    
    // Query zone entities
    let hand_query = app.world.query_filtered::<&Node, With<HandZone>>();
    let battlefield_query = app.world.query_filtered::<&Node, With<BattlefieldZone>>();
    
    // Verify hand zone height
    let hand_node = hand_query.single(&app.world);
    match hand_node.height {
        Val::Percent(percent) => {
            assert!(percent > 15.0 && percent < 25.0, "Hand zone height should be ~20%");
        },
        _ => panic!("Hand height should be a percentage"),
    }
    
    // Verify battlefield zone height
    let battlefield_node = battlefield_query.single(&app.world);
    match battlefield_node.height {
        Val::Percent(percent) => {
            assert!(percent > 50.0 && percent < 70.0, "Battlefield height should be ~60%");
        },
        _ => panic!("Battlefield height should be a percentage"),
    }
}
```

## Performance Testing

Performance tests verify that the UI performs well under different conditions.

### Card Volume Tests

Test UI performance with large numbers of cards:

```rust
#[test]
fn test_battlefield_performance_with_many_cards() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(DiagnosticsPlugin);
    
    // Set up battlefield with many cards
    let battlefield_entity = app.world.spawn((
        BattlefieldZone {
            player_id: PlayerId(1),
            organization: BattlefieldOrganization::ByType,
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    )).id();
    
    // Add 100 test cards to the battlefield
    for i in 0..100 {
        let card_entity = app.world.spawn((
            Card::new(&format!("Test Card {}", i)),
            CardType::Creature,
            Transform::default(),
            GlobalTransform::default(),
        )).id();
        app.world.entity_mut(battlefield_entity).add_child(card_entity);
    }
    
    // Run performance test
    let mut frame_times = Vec::new();
    for _ in 0..100 {
        let start = std::time::Instant::now();
        app.update();
        frame_times.push(start.elapsed());
    }
    
    // Calculate average frame time
    let avg_frame_time = frame_times.iter().sum::<std::time::Duration>() / frame_times.len() as u32;
    
    // Average frame time should be under 16ms (60fps)
    assert!(avg_frame_time.as_millis() < 16, "UI is not performing well with many cards");
}
```

### Animation Performance Tests

Test performance of UI animations:

```rust
#[test]
fn test_card_draw_animation_performance() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_plugins(DiagnosticsPlugin);
    
    // Set up player zones
    setup_test_playmat(&mut app, PlayerId(1));
    
    // Get zone entities
    let library_entity = app.world.query_filtered::<Entity, With<LibraryZone>>().single(&app.world);
    let hand_entity = app.world.query_filtered::<Entity, With<HandZone>>().single(&app.world);
    
    // Set up library with cards
    for i in 0..50 {
        let card_entity = app.world.spawn((
            Card::new(&format!("Library Card {}", i)),
            Transform::default(),
            GlobalTransform::default(),
            Parent(library_entity),
            InZone::Library,
        )).id();
        app.world.entity_mut(library_entity).add_child(card_entity);
    }
    
    // Measure performance while drawing multiple cards
    let mut frame_times = Vec::new();
    for i in 0..7 {
        // Draw a card
        let card_entity = app.world.query_filtered::<Entity, (With<Card>, With<InZone>)>()
            .iter(&app.world)
            .next()
            .unwrap();
            
        app.world.send_event(DrawCardEvent {
            player_id: PlayerId(1),
            card_entity,
        });
        
        // Run update and measure frame time
        let start = std::time::Instant::now();
        app.update();
        frame_times.push(start.elapsed());
    }
    
    // Calculate average and maximum frame time
    let avg_frame_time = frame_times.iter().sum::<std::time::Duration>() / frame_times.len() as u32;
    let max_frame_time = frame_times.iter().max().unwrap();
    
    // Performance assertions
    assert!(avg_frame_time.as_millis() < 16, "Card draw animation average performance is too slow");
    assert!(max_frame_time.as_millis() < 32, "Card draw animation maximum frame time is too slow");
}
```

## Accessibility Testing

Tests to verify UI accessibility features.

### Color Blind Mode Tests

Test that UI is usable in color blind modes:

```rust
#[test]
fn test_color_blind_mode_card_distinction() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins);
    
    // Set accessibility settings to deuteranopia mode
    let mut settings = AccessibilitySettings::default();
    settings.color_blind_mode = ColorBlindMode::Deuteranopia;
    app.world.insert_resource(settings);
    
    // Set up cards of different colors
    let red_card = app.world.spawn((
        Card::new("Red Card"),
        CardColor::Red,
        Transform::from_xyz(100.0, 100.0, 0.0),
        GlobalTransform::default(),
    )).id();
    
    let green_card = app.world.spawn((
        Card::new("Green Card"),
        CardColor::Green,
        Transform::from_xyz(300.0, 100.0, 0.0),
        GlobalTransform::default(),
    )).id();
    
    // Run systems to update card appearance
    app.update();
    
    // Get card visual components
    let red_card_appearance = app.world.get::<CardAppearance>(red_card).unwrap();
    let green_card_appearance = app.world.get::<CardAppearance>(green_card).unwrap();
    
    // In deuteranopia mode, these colors should have distinct patterns or indicators
    // rather than relying solely on red/green color difference
    assert_ne!(
        red_card_appearance.pattern_type,
        green_card_appearance.pattern_type,
        "Cards should have distinct patterns in color blind mode"
    );
}
```

### Keyboard Navigation Tests

Test keyboard navigation through UI elements:

```rust
#[test]
fn test_keyboard_navigation() {
    // Create test app
    let mut app = App::new();
    app.add_plugins(GameUiTestPlugins)
       .add_systems(Update, ui_systems::handle_keyboard_input);
    
    // Set up test game
    setup_test_game(&mut app, 2);
    
    // Initialize UI focus state
    app.world.insert_resource(UiFocus {
        current_focus: None,
        navigation_mode: true,
    });
    
    // Simulate keyboard input to start navigation
    app.world.send_event(KeyboardInput {
        key: KeyCode::Tab,
        state: ButtonState::Pressed,
    });
    app.update();
    
    // Verify navigation mode is active and something is focused
    let ui_focus = app.world.resource::<UiFocus>();
    assert!(ui_focus.navigation_mode, "Keyboard navigation mode should be active");
    assert!(ui_focus.current_focus.is_some(), "An element should be focused");
    
    // Test navigation between elements
    let first_focus = ui_focus.current_focus;
    
    // Simulate arrow key press
    app.world.send_event(KeyboardInput {
        key: KeyCode::ArrowRight,
        state: ButtonState::Pressed,
    });
    app.update();
    
    // Verify focus changed
    let ui_focus = app.world.resource::<UiFocus>();
    assert_ne!(ui_focus.current_focus, first_focus, "Focus should have moved to a new element");
}
```

## Automation Framework

The testing framework uses several helper functions and utilities:

```rust
/// Set up a test playmat with all zones for a player
fn setup_test_playmat(app: &mut App, player_id: PlayerId) -> Entity {
    // Create player entity
    let player_entity = app.world.spawn((
        Player { id: player_id, name: format!("Test Player {}", player_id.0) },
    )).id();
    
    // Spawn playmat
    let playmat_entity = app.world.spawn((
        PlayerPlaymat { player_id },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).id();
    
    // Spawn hand zone
    let hand_entity = app.world.spawn((
        HandZone { player_id, expanded: false },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(20.0),
            ..default()
        },
        Parent(playmat_entity),
    )).id();
    app.world.entity_mut(playmat_entity).add_child(hand_entity);
    
    // Spawn battlefield zone
    let battlefield_entity = app.world.spawn((
        BattlefieldZone {
            player_id,
            organization: BattlefieldOrganization::ByType,
        },
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(60.0),
            ..default()
        },
        Parent(playmat_entity),
    )).id();
    app.world.entity_mut(playmat_entity).add_child(battlefield_entity);
    
    // Add other zones (library, graveyard, etc.)
    // ...
    
    playmat_entity
}

/// Set up a test game with the specified number of players
fn setup_test_game(app: &mut App, player_count: usize) {
    // Create game state
    let mut game_state = GameState::default();
    game_state.player_count = player_count;
    app.world.insert_resource(game_state);
    
    // Create table
    let table_entity = app.world.spawn((
        VirtualTable,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
    )).id();
    
    // Add playmats for each player
    for i in 0..player_count {
        let player_id = PlayerId(i as u32 + 1);
        let playmat_entity = setup_test_playmat(app, player_id);
        app.world.entity_mut(table_entity).add_child(playmat_entity);
    }
    
    // Add shared zones
    setup_shared_zones(app, table_entity);
}

/// Helper function to take screenshot in tests
fn take_screenshot(app: &App) -> Image {
    // Implementation would depend on rendering backend
    // This is a simplified example
    let mut image = Image::default();
    
    // Get main camera
    let camera_entity = app.world.query_filtered::<Entity, With<Camera>>().single(&app.world);
    
    // Render to texture
    // (Implementation details would depend on Bevy's rendering API)
    
    image
}
```

## Test Case Organization

Test cases are organized into test suites that focus on specific aspects of the UI:

1. **Layout Tests**: Verify correct positioning and sizing of UI elements
2. **Interaction Tests**: Verify user interactions work correctly
3. **Visual Tests**: Verify visual appearance matches expectations
4. **Performance Tests**: Verify UI performance under different conditions
5. **Accessibility Tests**: Verify accessibility features work correctly
6. **Integration Tests**: Verify different UI components work together

Each test suite is implemented as a separate module, with common helpers in a shared module:

```rust
mod layout_tests {
    use super::*;
    
    #[test]
    fn test_two_player_table_layout() { /* ... */ }
    
    #[test]
    fn test_four_player_table_layout() { /* ... */ }
    
    // More layout tests...
}

mod interaction_tests {
    use super::*;
    
    #[test]
    fn test_card_selection() { /* ... */ }
    
    #[test]
    fn test_drag_and_drop() { /* ... */ }
    
    // More interaction tests...
}

// More test modules...
```

When running tests, use tags to run specific categories:

```bash
cargo test ui::layout  # Run layout tests
cargo test ui::visual  # Run visual tests
cargo test ui::all     # Run all UI tests
``` 