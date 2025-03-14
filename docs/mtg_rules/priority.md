# Priority System

## Overview

The priority system is a fundamental mechanism in Magic: The Gathering that determines when players can cast spells and activate abilities. This document explains how priority is implemented in Rummage and how it controls the flow of gameplay.

## Priority Basics

In Magic: The Gathering, priority is the right to take an action such as casting a spell, activating an ability, or making a special game action. The priority system follows these key principles:

1. The active player receives priority at the beginning of each step and phase, except for the untap step and most cleanup steps
2. A player with priority may take an action or pass priority
3. When all players pass priority in succession without taking an action, the top object on the stack resolves or, if the stack is empty, the current step or phase ends
4. Whenever an object is put on the stack, all players pass priority, and the stack resolves, priority is given to the active player

## Implementation in Rummage

In Rummage, the priority system is implemented using a combination of resources and systems:

```rust
/// Resource tracking priority state
#[derive(Resource)]
pub struct PrioritySystem {
    /// Current player with priority
    pub current_priority: Option<Entity>,
    /// Players who have passed priority without taking an action
    pub passed_players: HashSet<Entity>,
    /// Whether the game is currently waiting for priority actions
    pub waiting_for_priority: bool,
    /// The last player to take an action
    pub last_action_player: Option<Entity>,
}

/// Component marking the active player
#[derive(Component)]
pub struct ActivePlayer;
```

### Priority Assignment

Priority is assigned according to turn order, starting with the active player:

```rust
pub fn assign_priority_system(
    mut priority_system: ResMut<PrioritySystem>,
    turn_system: Res<TurnSystem>,
    game_state: Res<GameState>,
    active_player_query: Query<Entity, With<ActivePlayer>>,
) {
    // Only assign priority if not already waiting for someone
    if priority_system.waiting_for_priority {
        return;
    }

    // When a phase or step begins, active player gets priority
    if game_state.pending_phase_change {
        let active_player = active_player_query.get_single().unwrap();
        priority_system.current_priority = Some(active_player);
        priority_system.passed_players.clear();
        priority_system.waiting_for_priority = true;
    }

    // After stack resolution, active player gets priority
    if game_state.stack_just_resolved {
        let active_player = active_player_query.get_single().unwrap();
        priority_system.current_priority = Some(active_player);
        priority_system.passed_players.clear();
        priority_system.waiting_for_priority = true;
    }
}
```

### Passing Priority

When a player passes priority, it's passed to the next player in turn order:

```rust
pub fn handle_pass_priority(
    mut priority_system: ResMut<PrioritySystem>,
    mut pass_priority_events: EventReader<PassPriorityEvent>,
    players: Query<(Entity, &Player)>,
    turn_system: Res<TurnSystem>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in pass_priority_events.iter() {
        // Only process if this player currently has priority
        if let Some(current_priority) = priority_system.current_priority {
            if current_priority == event.player {
                // Add player to passed list
                priority_system.passed_players.insert(current_priority);
                
                // Find next player in turn order
                let next_player = get_next_player_in_turn_order(
                    current_priority, 
                    &players, 
                    &turn_system
                );
                
                // If next player has already passed, check for all players passing
                if priority_system.passed_players.contains(&next_player) || 
                   priority_system.passed_players.len() == players.iter().count() {
                    // All players have passed priority
                    game_events.send(GameEvent::AllPlayersPassed);
                    priority_system.waiting_for_priority = false;
                    priority_system.current_priority = None;
                } else {
                    // Pass to next player
                    priority_system.current_priority = Some(next_player);
                    game_events.send(GameEvent::PriorityPassed {
                        from: current_priority,
                        to: next_player,
                    });
                }
            }
        }
    }
}
```

### Priority After Actions

After a player takes an action, they receive priority again:

```rust
pub fn handle_player_action(
    mut priority_system: ResMut<PrioritySystem>,
    mut player_action_events: EventReader<PlayerActionEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in player_action_events.iter() {
        // Record player taking action
        priority_system.last_action_player = Some(event.player);
        
        // Clear passed players list since an action was taken
        priority_system.passed_players.clear();
        
        // Return priority to the player who took action
        priority_system.current_priority = Some(event.player);
        priority_system.waiting_for_priority = true;
        
        game_events.send(GameEvent::PriorityAssigned {
            player: event.player,
            reason: PriorityReason::AfterAction,
        });
    }
}
```

### Stack Resolution

When all players pass priority, either the top of the stack resolves or the current phase/step ends:

```rust
pub fn handle_all_players_passed(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    mut game_state: ResMut<GameState>,
    mut priority_system: ResMut<PrioritySystem>,
    active_player_query: Query<Entity, With<ActivePlayer>>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Process what happens when all players pass priority
    if !stack.items.is_empty() {
        // Resolve top item of stack
        let top_item = stack.items.pop().unwrap();
        
        game_events.send(GameEvent::StackItemResolved {
            item_id: top_item.id,
        });
        
        // Mark for priority reassignment after resolution
        game_state.stack_just_resolved = true;
    } else {
        // Empty stack, end current phase/step
        game_state.proceed_to_next_phase_or_step();
        
        game_events.send(GameEvent::PhaseStepEnded {
            phase: game_state.current_phase.clone(),
            step: game_state.current_step.clone(),
        });
    }
}
```

## Special Priority Rules

### Special Timing Rules

Some game actions use special timing rules that modify how priority works:

```rust
pub fn handle_special_timing(
    mut commands: Commands,
    mut priority_system: ResMut<PrioritySystem>,
    mut special_action_events: EventReader<SpecialActionEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    for event in special_action_events.iter() {
        match event.action_type {
            SpecialActionType::PlayLand => {
                // Playing a land doesn't use the stack and doesn't pass priority
                // but does count as taking an action for priority purposes
                priority_system.last_action_player = Some(event.player);
                priority_system.passed_players.clear();
                priority_system.current_priority = Some(event.player);
                
                game_events.send(GameEvent::PriorityRetained {
                    player: event.player,
                    reason: "Played a land",
                });
            },
            SpecialActionType::ManaAbility => {
                // Mana abilities don't use the stack and don't change priority
                game_events.send(GameEvent::ManaAbilityResolved {
                    player: event.player,
                    source: event.source,
                });
            },
            // Other special timing rules...
        }
    }
}
```

### State-Based Actions

State-based actions are checked before a player would receive priority:

```rust
pub fn check_state_based_actions(
    mut commands: Commands,
    mut priority_system: ResMut<PrioritySystem>,
    mut game_state: ResMut<GameState>,
    // Other query parameters...
) {
    // Only check when a player would receive priority
    if !priority_system.waiting_for_priority && 
       game_state.stack_just_resolved {
        // Process all state-based actions
        let sba_performed = perform_state_based_actions(
            &mut commands,
            // Other parameters...
        );
        
        // If any state-based actions were performed, check again
        // before assigning priority
        if sba_performed {
            game_state.sba_check_needed = true;
        } else {
            game_state.sba_check_needed = false;
            game_state.stack_just_resolved = false;
        }
    }
}
```

### Turn-Based Actions

Turn-based actions happen automatically at specific points regardless of priority:

```rust
pub fn handle_turn_based_actions(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut turn_action_events: EventWriter<TurnBasedActionEvent>,
) {
    match game_state.current_phase {
        Phase::Beginning if game_state.current_step == Step::Draw => {
            // Draw step: Active player draws a card
            turn_action_events.send(TurnBasedActionEvent::ActivePlayerDraws);
        },
        Phase::Combat if game_state.current_step == Step::EndOfCombat => {
            // End of combat: Remove all creatures from combat
            turn_action_events.send(TurnBasedActionEvent::RemoveFromCombat);
        },
        // Other turn-based actions...
        _ => {}
    }
}
```

## UI Integration

The priority system is visually represented to players:

```rust
pub fn update_priority_ui(
    priority_system: Res<PrioritySystem>,
    stack: Res<Stack>,
    players: Query<(Entity, &Player, &PlayerName)>,
    mut ui_state: ResMut<UiState>,
) {
    if let Some(current_priority) = priority_system.current_priority {
        // Highlight player with priority
        if let Ok((_, _, name)) = players.get(current_priority) {
            ui_state.priority_indicator = Some(PriorityIndicator {
                player: current_priority,
                name: name.0.clone(),
                has_passed: false,
            });
        }
    } else {
        ui_state.priority_indicator = None;
    }
    
    // Update UI for players who have passed
    for entity in &priority_system.passed_players {
        if let Ok((_, _, name)) = players.get(*entity) {
            ui_state.passed_priority_indicators.push(PriorityIndicator {
                player: *entity,
                name: name.0.clone(),
                has_passed: true,
            });
        }
    }
    
    // Update stack indicator
    ui_state.stack_size = stack.items.len();
}
```

## Multiplayer Priority

In multiplayer games, priority follows turn order (APNAP - Active Player, Non-Active Player):

```rust
pub fn get_next_player_in_turn_order(
    current_player: Entity,
    players: &Query<(Entity, &Player)>,
    turn_system: &Res<TurnSystem>,
) -> Entity {
    let turn_order = &turn_system.player_order;
    let current_index = turn_order.iter()
        .position(|&p| p == current_player)
        .unwrap_or(0);
    
    // Get next player, wrapping around to the beginning
    let next_index = (current_index + 1) % turn_order.len();
    turn_order[next_index]
}
```

## Testing Priority Flow

The priority system is tested through a variety of scenarios:

```rust
#[test]
fn test_priority_basic_flow() {
    // Setup test environment
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
       .add_system(assign_priority_system)
       .add_system(handle_pass_priority)
       .add_system(handle_player_action)
       .add_system(handle_all_players_passed)
       // Other test setup...
       
    // Create test players
    let player1 = spawn_test_player(&mut app.world, "Player 1");
    let player2 = spawn_test_player(&mut app.world, "Player 2");
    
    // Set active player
    app.world.entity_mut(player1).insert(ActivePlayer);
    
    // Begin test phase
    let mut game_state = app.world.resource_mut::<GameState>();
    game_state.pending_phase_change = true;
    
    // Run systems
    app.update();
    
    // Verify active player got priority
    let priority_system = app.world.resource::<PrioritySystem>();
    assert_eq!(priority_system.current_priority, Some(player1));
    
    // Simulate player1 passing priority
    app.world.resource_mut::<Events<PassPriorityEvent>>()
        .send(PassPriorityEvent { player: player1 });
    
    // Run systems
    app.update();
    
    // Verify priority passed to player2
    let priority_system = app.world.resource::<PrioritySystem>();
    assert_eq!(priority_system.current_priority, Some(player2));
    
    // More test assertions...
}
```

## Edge Cases

The priority system handles several edge cases:

### Split Second

The "split second" keyword prevents players from casting spells or activating non-mana abilities while a spell with split second is on the stack:

```rust
pub fn handle_split_second(
    stack: Res<Stack>,
    mut priority_system: ResMut<PrioritySystem>,
    mut player_action_events: EventReader<PlayerActionEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    // Check if a split second spell is on the stack
    let split_second_active = stack.items.iter().any(|item| {
        if let StackItemType::Spell { abilities, .. } = &item.item_type {
            abilities.contains(&Ability::SplitSecond)
        } else {
            false
        }
    });
    
    if split_second_active {
        // Only allow certain actions while split second is active
        for event in player_action_events.iter() {
            match event.action_type {
                ActionType::ActivateManaAbility(_) => {
                    // Mana abilities are allowed
                    // Process normally
                },
                ActionType::TriggerSpecialAction(SpecialActionType::MorphFaceDown) => {
                    // Special actions like turning a face-down creature face up are allowed
                    // Process normally
                },
                _ => {
                    // All other actions are denied
                    game_events.send(GameEvent::ActionDenied {
                        player: event.player,
                        action_type: event.action_type.clone(),
                        reason: "Split second prevents this action",
                    });
                    continue;
                }
            }
        }
    }
}
```

### No Action Possible

If a player cannot take any action, they must pass priority:

```rust
pub fn handle_no_action_possible(
    mut priority_system: ResMut<PrioritySystem>,
    players: Query<(Entity, &Player, &Hand)>,
    mut pass_priority_events: EventWriter<PassPriorityEvent>,
    mut game_events: EventWriter<GameEvent>,
) {
    if let Some(current_priority) = priority_system.current_priority {
        if let Ok((entity, player, hand)) = players.get(current_priority) {
            // Check if player can take any action
            if hand.cards.is_empty() && !player_has_playable_permanents(entity) {
                // Player has no possible actions, auto-pass priority
                pass_priority_events.send(PassPriorityEvent { player: entity });
                
                game_events.send(GameEvent::AutoPassPriority {
                    player: entity,
                    reason: "No possible actions",
                });
            }
        }
    }
}
```

## Conclusion

The priority system is a fundamental aspect of Magic: The Gathering that controls the flow of game actions. Rummage's implementation carefully follows the official rules, ensuring that players can take actions in the correct order and that game state advances properly.

---

Next: [Stack](stack.md) 