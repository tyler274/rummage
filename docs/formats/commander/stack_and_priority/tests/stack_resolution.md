# Stack Resolution Tests

## Overview

This document outlines test cases for the stack system in Commander format. These tests ensure that the stack operates correctly, spells and abilities resolve in the right order, and that priority passing works as expected in this multiplayer format.

## Test Case: Basic Stack Resolution

### Test: Last In, First Out Resolution Order

```rust
#[test]
fn test_lifo_resolution() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_stack_resolution, update_game_state));
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create spells for the stack
    let spell1 = app.world.spawn((
        Card { name: "First Spell".to_string() },
        Spell,
        StackPosition(0),
    )).id();
    
    let spell2 = app.world.spawn((
        Card { name: "Second Spell".to_string() },
        Spell,
        StackPosition(1),
    )).id();
    
    let spell3 = app.world.spawn((
        Card { name: "Third Spell".to_string() },
        Spell,
        StackPosition(2),
    )).id();
    
    // Set up stack with spells in order
    app.insert_resource(Stack {
        items: vec![spell1, spell2, spell3],
    });
    
    // Track resolution order
    app.insert_resource(ResolutionOrder { items: Vec::new() });
    
    // All players pass priority
    app.insert_resource(PrioritySystem {
        has_priority: player,
        stack_is_empty: false,
        all_players_passed: true,
    });
    
    // Resolve stack
    app.update();
    
    // Verify resolution happened in LIFO order
    let resolution_order = app.world.resource::<ResolutionOrder>();
    assert_eq!(resolution_order.items, vec![spell3, spell2, spell1]);
}
```

## Test Case: Interrupting Stack Resolution

### Test: Adding to the Stack During Resolution

```rust
#[test]
fn test_stack_interruption() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_stack_resolution, handle_triggered_abilities));
       
    // Create player
    let player = app.world.spawn(Player {}).id();
    
    // Create an ability that will trigger when a spell resolves
    let permanent = app.world.spawn((
        Card { name: "Ability Source".to_string() },
        Permanent,
        TriggeredAbility {
            trigger: Trigger::SpellResolution,
            ability: AbilityType::CreateEffect,
        },
        Controller { player },
    )).id();
    
    // Create spells for the stack
    let spell1 = app.world.spawn((
        Card { name: "First Spell".to_string() },
        Spell,
        StackPosition(0),
    )).id();
    
    let spell2 = app.world.spawn((
        Card { name: "Second Spell".to_string() },
        Spell,
        StackPosition(1),
    )).id();
    
    // Set up stack with spells
    app.insert_resource(Stack {
        items: vec![spell1, spell2],
    });
    
    // All players pass priority
    app.insert_resource(PrioritySystem {
        has_priority: player,
        stack_is_empty: false,
        all_players_passed: true,
    });
    
    // Partially resolve stack (just the top spell)
    app.update();
    
    // Verify the triggered ability was put on the stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 2); // Original spell1 + new triggered ability
    
    // Check that the new ability is at the top of the stack
    let top_item = stack.items.last().unwrap();
    assert!(app.world.get::<TriggeredAbility>(*top_item).is_some());
}
```

## Test Case: Priority Passing in Multiplayer

### Test: Full Round of Priority After Spell Cast

```rust
#[test]
fn test_multiplayer_priority() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_priority, handle_spell_cast));
       
    // Create multiple players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    let player3 = app.world.spawn(Player { id: 3 }).id();
    let player4 = app.world.spawn(Player { id: 4 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2, player3, player4],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::PreCombatMain,
        active_player: player1,
    });
    
    // Create a spell
    let spell = app.world.spawn((
        Card { name: "Test Spell".to_string() },
        Spell,
        Owner { player: player1 },
    )).id();
    
    // Player 1 has priority initially
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: true,
        all_players_passed: false,
    });
    
    // Player 1 casts a spell
    app.world.send_event(SpellCastEvent {
        caster: player1,
        spell: spell,
    });
    app.update();
    
    // Verify spell is on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    assert_eq!(stack.items[0], spell);
    
    // Verify priority returned to active player
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player1);
    
    // Player 1 passes priority
    app.world.send_event(PriorityPassedEvent { player: player1 });
    app.update();
    
    // Priority passes to player 2
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player2);
    
    // Player 2 has opportunity to respond
    let response_spell = app.world.spawn((
        Card { name: "Response Spell".to_string() },
        Spell,
        Owner { player: player2 },
    )).id();
    
    // Player 2 casts a response
    app.world.send_event(SpellCastEvent {
        caster: player2,
        spell: response_spell,
    });
    app.update();
    
    // Verify both spells are on stack with response on top
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 2);
    assert_eq!(stack.items[1], response_spell);
    
    // Verify priority returned to player 2
    let priority = app.world.resource::<PrioritySystem>();
    assert_eq!(priority.has_priority, player2);
    
    // All players need to pass again for anything to resolve
    app.world.send_event(PriorityPassedEvent { player: player2 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player3 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player4 });
    app.update();
    app.world.send_event(PriorityPassedEvent { player: player1 });
    app.update();
    
    // Top spell should resolve
    app.update();
    
    // Verify response spell resolved
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    assert_eq!(stack.items[0], spell);
}
```

## Test Case: Split Second and Interrupts

### Test: Split Second Prevents Further Responses

```rust
#[test]
fn test_split_second() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_priority, handle_spell_cast, validate_spell_cast));
       
    // Create players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    
    // Set up turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::PreCombatMain,
        active_player: player1,
    });
    
    // Create a split second spell
    let split_second_spell = app.world.spawn((
        Card { name: "Split Second Spell".to_string() },
        Spell,
        SplitSecond,
        Owner { player: player1 },
    )).id();
    
    // Player 1 has priority initially
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: true,
        all_players_passed: false,
    });
    
    // Player 1 casts split second spell
    app.world.send_event(SpellCastEvent {
        caster: player1,
        spell: split_second_spell,
    });
    app.update();
    
    // Verify split second spell is on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    assert_eq!(stack.items[0], split_second_spell);
    
    // Create a response spell
    let response_spell = app.world.spawn((
        Card { name: "Response Spell".to_string() },
        Spell,
        Owner { player: player2 },
    )).id();
    
    // Priority passes to player 2
    app.world.resource_mut::<PrioritySystem>().has_priority = player2;
    
    // Player 2 attempts to cast a response
    app.world.send_event(SpellCastEvent {
        caster: player2,
        spell: response_spell,
    });
    app.update();
    
    // Verify response spell was prevented from being cast
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1); // Still only the split second spell
    
    // Only mana abilities and special actions can be taken
    let mana_ability = app.world.spawn((
        Card { name: "Mana Source".to_string() },
        ManaAbility,
        Owner { player: player2 },
    )).id();
    
    // Player 2 activates a mana ability
    app.world.send_event(ActivateManaAbilityEvent {
        player: player2,
        ability: mana_ability,
    });
    app.update();
    
    // Verify mana ability was allowed
    assert!(app.world.resource::<Events<ManaProducedEvent>>().is_empty() == false);
}
```

## Test Case: Counterspells and Responses

### Test: Counterspell on the Stack

```rust
#[test]
fn test_counterspell() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_stack_resolution, handle_spell_cast, handle_counterspell));
       
    // Create player
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    
    // Create a target spell
    let target_spell = app.world.spawn((
        Card { name: "Target Spell".to_string() },
        Spell,
        Owner { player: player1 },
    )).id();
    
    // Create a counterspell
    let counterspell = app.world.spawn((
        Card { name: "Counterspell".to_string() },
        Spell,
        CounterspellEffect,
        Owner { player: player2 },
    )).id();
    
    // Put target spell on stack
    app.insert_resource(Stack {
        items: vec![target_spell],
    });
    
    // Player 2 casts counterspell targeting the spell
    app.world.send_event(SpellCastEvent {
        caster: player2,
        spell: counterspell,
    });
    
    // Set up targets for counterspell
    app.world.spawn((
        Target {
            source: counterspell,
            targets: vec![TargetInfo {
                entity: target_spell,
                target_type: TargetType::Spell,
            }],
        },
    ));
    app.update();
    
    // Verify both spells on stack with counterspell on top
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 2);
    assert_eq!(stack.items[1], counterspell);
    
    // All players pass priority
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: false,
        all_players_passed: true,
    });
    
    // Resolve counterspell
    app.update();
    
    // Verify target spell was countered and removed from stack
    let stack = app.world.resource::<Stack>();
    assert!(stack.items.is_empty());
    
    // Verify target spell moved to graveyard
    assert_eq!(app.world.get::<Zone>(target_spell).unwrap(), &Zone::Graveyard);
}
```

### Test: Uncounterable Spell

```rust
#[test]
fn test_uncounterable_spell() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_stack_resolution, handle_spell_cast, handle_counterspell));
       
    // Create player
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    
    // Create an uncounterable spell
    let uncounterable_spell = app.world.spawn((
        Card { name: "Uncounterable Spell".to_string() },
        Spell,
        CannotBeCountered,
        Owner { player: player1 },
    )).id();
    
    // Create a counterspell
    let counterspell = app.world.spawn((
        Card { name: "Counterspell".to_string() },
        Spell,
        CounterspellEffect,
        Owner { player: player2 },
    )).id();
    
    // Put uncounterable spell on stack
    app.insert_resource(Stack {
        items: vec![uncounterable_spell],
    });
    
    // Player 2 casts counterspell targeting the spell
    app.world.send_event(SpellCastEvent {
        caster: player2,
        spell: counterspell,
    });
    
    // Set up targets for counterspell
    app.world.spawn((
        Target {
            source: counterspell,
            targets: vec![TargetInfo {
                entity: uncounterable_spell,
                target_type: TargetType::Spell,
            }],
        },
    ));
    app.update();
    
    // Verify both spells on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 2);
    
    // All players pass priority
    app.insert_resource(PrioritySystem {
        has_priority: player1,
        stack_is_empty: false,
        all_players_passed: true,
    });
    
    // Resolve counterspell
    app.update();
    
    // Verify uncounterable spell remains on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    assert_eq!(stack.items[0], uncounterable_spell);
    
    // Verify counterspell went to graveyard
    assert_eq!(app.world.get::<Zone>(counterspell).unwrap(), &Zone::Graveyard);
}
```

## Test Case: Triggered Abilities and The Stack

### Test: Multiple Triggered Abilities Order

```rust
#[test]
fn test_multiple_triggers() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_triggered_abilities, order_triggered_abilities));
       
    // Create players
    let player1 = app.world.spawn(Player { id: 1 }).id();
    let player2 = app.world.spawn(Player { id: 2 }).id();
    
    // Turn order
    app.insert_resource(TurnOrder {
        players: vec![player1, player2],
        current_index: 0,
    });
    
    app.insert_resource(TurnManager {
        current_phase: Phase::Beginning(BeginningPhaseStep::Upkeep),
        active_player: player1,
    });
    
    // Create permanents with upkeep triggers for both players
    let permanent1 = app.world.spawn((
        Card { name: "Permanent 1".to_string() },
        Permanent,
        TriggeredAbility {
            trigger: Trigger::Upkeep(TriggerController::ActivePlayer),
            ability: AbilityType::DrawCard,
        },
        Controller { player: player1 },
    )).id();
    
    let permanent2 = app.world.spawn((
        Card { name: "Permanent 2".to_string() },
        Permanent,
        TriggeredAbility {
            trigger: Trigger::Upkeep(TriggerController::ActivePlayer),
            ability: AbilityType::GainLife(1),
        },
        Controller { player: player1 },
    )).id();
    
    let permanent3 = app.world.spawn((
        Card { name: "Permanent 3".to_string() },
        Permanent,
        TriggeredAbility {
            trigger: Trigger::Upkeep(TriggerController::ActivePlayer),
            ability: AbilityType::LoseLife(1),
        },
        Controller { player: player2 },
    )).id();
    
    // Trigger abilities
    app.world.send_event(PhaseChangedEvent {
        new_phase: Phase::Beginning(BeginningPhaseStep::Upkeep),
    });
    app.update();
    
    // Verify triggers were put on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 3);
    
    // Verify triggers from active player go on stack in APNAP order
    // (Active Player, Non-Active Player)
    let stack_items = &stack.items;
    
    // Active player's abilities should be first (in controller's choice order)
    let first_two_controllers: Vec<Entity> = stack_items.iter()
        .take(2)
        .map(|e| app.world.get::<Controller>(*e).unwrap().player)
        .collect();
    
    // The first two should belong to player1 (active player)
    assert_eq!(first_two_controllers, vec![player1, player1]);
    
    // The last ability should belong to player2
    let last_controller = app.world.get::<Controller>(stack_items[2]).unwrap().player;
    assert_eq!(last_controller, player2);
}
```

## Test Case: Stack and State-Based Actions

### Test: State-Based Actions Between Stack Resolutions

```rust
#[test]
fn test_state_based_actions_and_stack() {
    // Test setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_systems(Update, (handle_stack_resolution, handle_state_based_actions));
       
    // Create player and creature
    let player = app.world.spawn(Player {}).id();
    
    let creature = app.world.spawn((
        Card { name: "Creature".to_string() },
        Creature { power: 3, toughness: 3 },
        Health { current: 3, maximum: 3 },
        Zone::Battlefield,
        Owner { player },
    )).id();
    
    // Create damage spell
    let damage_spell = app.world.spawn((
        Card { name: "Lightning Bolt".to_string() },
        Spell,
        DamageEffect { amount: 3 },
        Owner { player },
    )).id();
    
    // Create spell target
    app.world.spawn((
        Target {
            source: damage_spell,
            targets: vec![TargetInfo {
                entity: creature,
                target_type: TargetType::Creature,
            }],
        },
    ));
    
    // Create heal spell that will be cast in response
    let heal_spell = app.world.spawn((
        Card { name: "Healing Touch".to_string() },
        Spell,
        HealEffect { amount: 3 },
        Owner { player },
    )).id();
    
    // Create heal target
    app.world.spawn((
        Target {
            source: heal_spell,
            targets: vec![TargetInfo {
                entity: creature,
                target_type: TargetType::Creature,
            }],
        },
    ));
    
    // Put spells on stack in order (heal on top, will resolve first)
    app.insert_resource(Stack {
        items: vec![damage_spell, heal_spell],
    });
    
    // Setup priority for resolution
    app.insert_resource(PrioritySystem {
        has_priority: player,
        stack_is_empty: false,
        all_players_passed: true,
    });
    
    // Resolve heal spell
    app.update();
    
    // Verify creature healed to full
    let health = app.world.get::<Health>(creature).unwrap();
    assert_eq!(health.current, 3);
    
    // Still one spell on stack
    let stack = app.world.resource::<Stack>();
    assert_eq!(stack.items.len(), 1);
    
    // Resolve damage spell
    app.update();
    
    // Verify creature took damage
    let health = app.world.get::<Health>(creature).unwrap();
    assert_eq!(health.current, 0);
    
    // Verify stack is empty
    let stack = app.world.resource::<Stack>();
    assert!(stack.items.is_empty());
    
    // Apply state-based actions check
    app.update();
    
    // Verify creature died due to state-based actions
    assert_eq!(app.world.get::<Zone>(creature).unwrap(), &Zone::Graveyard);
}
```

These test cases ensure the stack resolution system functions correctly, handling priority passing, triggered abilities, counterspells, and state-based actions in accordance with Commander's rules. 