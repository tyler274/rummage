# Priority and Stack System

## Overview

The Priority and Stack System manages the core timing and effect resolution in Magic: The Gathering. It handles player priority order, spell and ability resolution, and complex interaction management. This module is particularly important for handling multiplayer Commander games with up to 13 players, where complex stack interactions are common.

## Core Components

### Priority Resource

```rust
#[derive(Resource)]
pub struct PrioritySystem {
    // Current priority holder and tracking
    pub active_player: Entity,
    pub priority_player: Entity,
    pub has_priority_passed: HashMap<Entity, bool>,
    pub all_players_passed: bool,
    
    // Player order tracking
    pub player_order: Vec<Entity>,
    pub priority_index: usize,
    
    // Timing and state
    pub stack_is_empty: bool,
    pub current_phase: Phase,
    pub waiting_for_response: bool,
    pub response_timeout: Option<std::time::Instant>,
    
    // Multiplayer special cases
    pub simultaneous_decision_players: Vec<Entity>,
    pub decision_timeouts: HashMap<Entity, std::time::Duration>,
}
```

### Stack Resource

```rust
#[derive(Resource)]
pub struct GameStack {
    // Stack items in order of resolution (last in, first out)
    pub items: Vec<StackItem>,
    
    // Current state of the stack
    pub is_resolving: bool,
    pub currently_resolving: Option<Entity>,
    
    // Tracking for split-second and uncounterable effects
    pub contains_split_second: bool,
    pub uncounterable_items: HashSet<Entity>,
    
    // Tracking for triggers waiting to go on stack
    pub pending_triggers: Vec<TriggeredAbility>,
}

#[derive(Debug, Clone)]
pub struct StackItem {
    // Core information
    pub entity: Entity,  // The entity representing this stack item
    pub item_type: StackItemType,
    pub controller: Entity,
    pub source: Entity,      // Card or permanent that created this effect
    pub source_zone: Zone,   // Zone the source was in when activated/triggered
    
    // Targeting information
    pub targets: Vec<StackTarget>,
    pub requires_targets: bool,
    pub target_selection_complete: bool,

    // Cost information
    pub costs_paid: bool,
    pub additional_costs: Vec<AdditionalCost>,
    pub cost_reduction_effects: Vec<CostReduction>,
    
    // Resolution information
    pub can_be_countered: bool,
    pub split_second: bool,
    pub modified_by: Vec<Entity>,  // Other stack items that modified this one
    
    // Text and rules
    pub effect_text: String,
    pub oracle_id: Option<uuid::Uuid>,
}

#[derive(Debug, Clone)]
pub struct StackTarget {
    pub target_type: TargetType,
    pub entity: Entity,
    pub valid_on_resolution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackItemType {
    Spell(SpellType),
    Ability(AbilityType),
    SpecialAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbilityType {
    Activated,
    Triggered,
    Static,
    Mana,
    Loyalty,
    CommanderNinjutsu,
}

#[derive(Debug, Clone)]
pub struct TriggeredAbility {
    pub source: Entity,
    pub controller: Entity,
    pub trigger_condition: TriggerCondition,
    pub ability_text: String,
    pub created_at: std::time::Instant,
}
```

## Key Systems

### Priority Initialization System

```rust
fn initialize_priority_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    turn_manager: Res<TurnManager>,
    players: Query<Entity, With<CommanderPlayer>>,
) {
    // Set active player from turn manager
    priority.active_player = turn_manager.player_order[turn_manager.active_player_index];
    
    // Initialize priority with active player
    priority.priority_player = priority.active_player;
    priority.priority_index = turn_manager.active_player_index;
    
    // Reset pass tracking
    priority.has_priority_passed.clear();
    priority.all_players_passed = false;
    
    // Set up player order (same as turn order)
    priority.player_order = turn_manager.player_order.clone();
    
    // Update current phase
    priority.current_phase = turn_manager.current_phase;
    
    // Reset state
    priority.waiting_for_response = false;
    priority.response_timeout = None;
    priority.simultaneous_decision_players.clear();
    
    // Initialize pass tracking for all players
    for player in players.iter() {
        priority.has_priority_passed.insert(player, false);
    }
    
    // Set stack status
    priority.stack_is_empty = true;
}
```

### Priority Passing System

```rust
fn priority_passing_system(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut game_stack: ResMut<GameStack>,
    mut pass_events: EventReader<PassPriorityEvent>,
    mut phase_events: EventWriter<PhaseTransitionEvent>,
    players: Query<Entity, With<CommanderPlayer>>,
    time: Res<Time>,
) {
    // Check for priority passes
    for event in pass_events.read() {
        let player = event.player;
        
        // Verify player has priority
        if player == priority.priority_player {
            // Mark priority as passed
            priority.has_priority_passed.insert(player, true);
            
            // Move to next player in order
            advance_priority(&mut priority, &players);
        }
    }
    
    // Check for timeout-based automatic passing
    if let Some(timeout) = priority.response_timeout {
        if time.elapsed() > timeout {
            // Auto-pass for timed out player
            priority.has_priority_passed.insert(priority.priority_player, true);
            advance_priority(&mut priority, &players);
        }
    }
    
    // Update stack empty status
    priority.stack_is_empty = game_stack.items.is_empty();
    
    // Check if all players have passed
    if priority_round_complete(&priority) {
        // If stack is empty, move to next phase
        if priority.stack_is_empty {
            // All passed with empty stack = phase change
            commands.spawn(NextPhaseEvent);
        } else {
            // All passed with non-empty stack = resolve top of stack
            game_stack.is_resolving = true;
            let top_entity = game_stack.items.last().unwrap().entity;
            commands.spawn(ResolveStackItemEvent { item: top_entity });
        }
        
        // Reset priority passing tracker
        for player in players.iter() {
            priority.has_priority_passed.insert(player, false);
        }
        
        // If stack is being resolved, priority goes to active player after resolution
        if !priority.stack_is_empty {
            priority.priority_player = priority.active_player;
            priority.priority_index = priority.player_order
                .iter()
                .position(|&p| p == priority.active_player)
                .unwrap_or(0);
        }
    }
}

fn advance_priority(
    priority: &mut PrioritySystem,
    players: &Query<Entity, With<CommanderPlayer>>,
) {
    // Get the next player in turn order
    let player_count = priority.player_order.len();
    priority.priority_index = (priority.priority_index + 1) % player_count;
    priority.priority_player = priority.player_order[priority.priority_index];
    
    // Set up response timeout if enabled
    priority.response_timeout = Some(std::time::Instant::now() + 
                                   std::time::Duration::from_secs(30));  // Configurable
}

fn priority_round_complete(priority: &PrioritySystem) -> bool {
    // Check if all players have passed priority
    for (_, passed) in priority.has_priority_passed.iter() {
        if !passed {
            return false;
        }
    }
    true
}
```

### Stack Resolution System

```rust
fn stack_resolution_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut priority: ResMut<PrioritySystem>,
    mut resolve_events: EventReader<ResolveStackItemEvent>,
    cards: Query<&Card>,
    permanents: Query<&Permanent>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    if !stack.is_resolving {
        return;
    }
    
    // Handle stack item resolution
    for event in resolve_events.read() {
        let item_entity = event.item;
        
        // Find the stack item
        if let Some(pos) = stack.items.iter().position(|item| item.entity == item_entity) {
            // Get the item to resolve
            let item = stack.items.remove(pos);
            
            // Check if targets are still valid
            let targets_valid = validate_targets(&item, &cards, &permanents, &players);
            
            // Resolve the effect if targets are valid
            if targets_valid {
                match item.item_type {
                    StackItemType::Spell(spell_type) => {
                        resolve_spell(&item, spell_type, &mut commands);
                    },
                    StackItemType::Ability(ability_type) => {
                        resolve_ability(&item, ability_type, &mut commands);
                    },
                    StackItemType::SpecialAction => {
                        resolve_special_action(&item, &mut commands);
                    },
                }
            } else {
                // If targets invalid, spell/ability is countered on resolution
                commands.spawn(EffectCounteredEvent {
                    item: item_entity,
                    reason: CounterReason::InvalidTargets,
                });
            }
            
            // Update split second tracking
            update_split_second_status(&mut stack);
            
            // Finish resolution
            stack.is_resolving = false;
            stack.currently_resolving = None;
            
            // Check for triggered abilities from resolution
            process_pending_triggers(&mut stack, &mut commands);
            
            // Reset priority for next round
            priority.priority_player = priority.active_player;
            for (player, _) in players.iter() {
                priority.has_priority_passed.insert(player, false);
            }
        }
    }
}

fn validate_targets(
    item: &StackItem,
    cards: &Query<&Card>,
    permanents: &Query<&Permanent>,
    players: &Query<(Entity, &CommanderPlayer)>,
) -> bool {
    if !item.requires_targets || item.targets.is_empty() {
        return true;
    }
    
    // Check each target for validity
    for target in &item.targets {
        let valid = match target.target_type {
            TargetType::Player => players.get(target.entity).is_ok(),
            TargetType::Permanent => permanents.get(target.entity).is_ok(),
            TargetType::Card => cards.get(target.entity).is_ok(),
            TargetType::StackItem => true, // Already checked in targeting
            // Add more target types as needed
        };
        
        if !valid {
            return false;
        }
    }
    
    true
}

fn update_split_second_status(stack: &mut GameStack) {
    // Check if there's still a split second effect on the stack
    stack.contains_split_second = stack.items.iter()
        .any(|item| item.split_second);
}

fn process_pending_triggers(
    stack: &mut GameStack,
    commands: &mut Commands,
) {
    // Sort triggers by timestamp
    stack.pending_triggers.sort_by(|a, b| a.created_at.cmp(&b.created_at));
    
    // Add triggers to stack
    for trigger in stack.pending_triggers.drain(..) {
        // Create stack item for trigger
        let trigger_entity = commands.spawn_empty().id();
        
        let stack_item = StackItem {
            entity: trigger_entity,
            item_type: StackItemType::Ability(AbilityType::Triggered),
            controller: trigger.controller,
            source: trigger.source,
            source_zone: Zone::Battlefield, // Default, would need to be determined
            targets: Vec::new(),
            requires_targets: false, // Would be determined by ability
            target_selection_complete: false,
            costs_paid: true, // Triggered abilities don't have costs
            additional_costs: Vec::new(),
            cost_reduction_effects: Vec::new(),
            can_be_countered: true,
            split_second: false,
            modified_by: Vec::new(),
            effect_text: trigger.ability_text,
            oracle_id: None,
        };
        
        // Add to stack
        stack.items.push(stack_item);
        
        // Emit event that trigger was put on stack
        commands.spawn(TriggerAddedToStackEvent {
            trigger: trigger_entity,
            source: trigger.source,
            controller: trigger.controller,
        });
    }
}
```

### Casting and Activating System

```rust
fn handle_spell_casting(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut cast_events: EventReader<CastSpellEvent>,
    cards: Query<&Card>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    for event in cast_events.read() {
        let card_entity = event.card;
        let player = event.player;
        
        // Check if casting is legal
        if !can_cast_spell(card_entity, player, &stack, &cards, &players) {
            continue;
        }
        
        // Create stack item for spell
        let spell_entity = commands.spawn_empty().id();
        let card = cards.get(card_entity).unwrap();
        
        let spell_type = determine_spell_type(card);
        
        let stack_item = StackItem {
            entity: spell_entity,
            item_type: StackItemType::Spell(spell_type),
            controller: player,
            source: card_entity,
            source_zone: Zone::Hand, // This would come from the event in practice
            targets: Vec::new(),
            requires_targets: requires_targets(card),
            target_selection_complete: false,
            costs_paid: false,
            additional_costs: Vec::new(),
            cost_reduction_effects: Vec::new(),
            can_be_countered: true, // Default, would check card text
            split_second: has_split_second(card),
            modified_by: Vec::new(),
            effect_text: card.rules_text.clone(),
            oracle_id: None, // Would come from card
        };
        
        // Update split second status if needed
        if stack_item.split_second {
            stack.contains_split_second = true;
        }
        
        // Add to stack
        stack.items.push(stack_item);
        
        // Emit event for targeting phase
        if stack_item.requires_targets {
            commands.spawn(SelectTargetsEvent {
                stack_item: spell_entity,
                player,
            });
        } else {
            // No targets needed
            commands.spawn(PayCostsEvent {
                stack_item: spell_entity,
                player,
            });
        }
    }
}

fn handle_ability_activation(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut activate_events: EventReader<ActivateAbilityEvent>,
    permanents: Query<&Permanent>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    for event in activate_events.read() {
        let source_entity = event.source;
        let player = event.player;
        let ability_index = event.ability_index;
        
        // Check if activation is legal
        if !can_activate_ability(source_entity, ability_index, player, &stack, &permanents, &players) {
            continue;
        }
        
        // Get the permanent and ability
        let permanent = permanents.get(source_entity).unwrap();
        let ability = &permanent.abilities[ability_index];
        
        // Create stack item for ability
        let ability_entity = commands.spawn_empty().id();
        
        let ability_type = determine_ability_type(ability);
        
        let stack_item = StackItem {
            entity: ability_entity,
            item_type: StackItemType::Ability(ability_type),
            controller: player,
            source: source_entity,
            source_zone: Zone::Battlefield,
            targets: Vec::new(),
            requires_targets: ability.requires_targets,
            target_selection_complete: false,
            costs_paid: false,
            additional_costs: ability.additional_costs.clone(),
            cost_reduction_effects: Vec::new(),
            can_be_countered: !ability_type.is_mana_ability(), // Mana abilities don't use stack
            split_second: false, // Abilities don't have split second
            modified_by: Vec::new(),
            effect_text: ability.text.clone(),
            oracle_id: None,
        };
        
        // For mana abilities, resolve immediately without using the stack
        if matches!(ability_type, AbilityType::Mana) {
            resolve_ability(&stack_item, ability_type, &mut commands);
            continue;
        }
        
        // Add to stack
        stack.items.push(stack_item);
        
        // Emit event for targeting phase
        if stack_item.requires_targets {
            commands.spawn(SelectTargetsEvent {
                stack_item: ability_entity,
                player,
            });
        } else {
            // No targets needed
            commands.spawn(PayCostsEvent {
                stack_item: ability_entity,
                player,
            });
        }
    }
}
```

### Target Selection System

```rust
fn target_selection_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut target_events: EventReader<SelectTargetEvent>,
    mut selection_complete_events: EventReader<TargetSelectionCompleteEvent>,
) {
    // Handle individual target selections
    for event in target_events.read() {
        let stack_item_entity = event.stack_item;
        let target_entity = event.target;
        let target_type = event.target_type;
        
        // Find the stack item
        for item in stack.items.iter_mut() {
            if item.entity == stack_item_entity {
                // Add the target
                item.targets.push(StackTarget {
                    target_type,
                    entity: target_entity,
                    valid_on_resolution: true,
                });
                break;
            }
        }
    }
    
    // Handle completion of target selection
    for event in selection_complete_events.read() {
        let stack_item_entity = event.stack_item;
        
        // Find the stack item
        for item in stack.items.iter_mut() {
            if item.entity == stack_item_entity {
                // Mark targeting as complete
                item.target_selection_complete = true;
                
                // Move to cost payment
                commands.spawn(PayCostsEvent {
                    stack_item: stack_item_entity,
                    player: item.controller,
                });
                break;
            }
        }
    }
}
```

### Cost Payment System

```rust
fn cost_payment_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut pay_events: EventReader<PayCostsEvent>,
    mut payment_complete_events: EventReader<CostPaymentCompleteEvent>,
    players: Query<(Entity, &CommanderPlayer)>,
) {
    // Handle cost payment initiation
    for event in pay_events.read() {
        let stack_item_entity = event.stack_item;
        let player = event.player;
        
        // Find the stack item
        for item in stack.items.iter() {
            if item.entity == stack_item_entity {
                // Calculate total cost with additional costs and reductions
                // (This would be implemented based on cost types)
                
                // Request payment from player
                commands.spawn(RequestPaymentEvent {
                    stack_item: stack_item_entity,
                    player,
                    // Cost details would be included
                });
                break;
            }
        }
    }
    
    // Handle completion of cost payment
    for event in payment_complete_events.read() {
        let stack_item_entity = event.stack_item;
        let payment_successful = event.success;
        
        if payment_successful {
            // Find the stack item
            for item in stack.items.iter_mut() {
                if item.entity == stack_item_entity {
                    // Mark costs as paid
                    item.costs_paid = true;
                    
                    // Spell/ability successfully cast/activated
                    commands.spawn(StackItemAddedEvent {
                        item: stack_item_entity,
                        controller: item.controller,
                    });
                    break;
                }
            }
        } else {
            // Payment failed, remove from stack
            if let Some(pos) = stack.items.iter().position(|item| item.entity == stack_item_entity) {
                stack.items.remove(pos);
                
                // Notify of failed casting/activation
                commands.spawn(CastingFailedEvent {
                    item: stack_item_entity,
                    reason: "Cost payment failed".to_string(),
                });
            }
        }
    }
}
```

## Multiplayer Specific Systems

### Simultaneous Decision Handling

```rust
fn handle_simultaneous_decisions(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut decision_events: EventReader<SimultaneousDecisionEvent>,
    mut decision_responses: EventReader<PlayerDecisionResponse>,
    time: Res<Time>,
) {
    // Set up simultaneous decision requests
    for event in decision_events.read() {
        // Add players who need to make a decision simultaneously
        priority.simultaneous_decision_players = event.players.clone();
        
        // Set timeouts for each player
        for player in &event.players {
            priority.decision_timeouts.insert(
                *player, 
                std::time::Duration::from_secs(30) // Configurable
            );
        }
        
        // Pause normal priority
        priority.waiting_for_response = true;
    }
    
    // Handle decision responses
    for event in decision_responses.read() {
        let player = event.player;
        
        // Remove player from waiting list
        if let Some(pos) = priority.simultaneous_decision_players.iter().position(|&p| p == player) {
            priority.simultaneous_decision_players.remove(pos);
        }
        
        // Remove timeout
        priority.decision_timeouts.remove(&player);
    }
    
    // Handle timeouts
    let mut timed_out_players = Vec::new();
    for (player, timeout) in priority.decision_timeouts.iter() {
        if time.elapsed() > std::time::Instant::now() - *timeout {
            timed_out_players.push(*player);
        }
    }
    
    // Remove timed out players
    for player in timed_out_players {
        if let Some(pos) = priority.simultaneous_decision_players.iter().position(|&p| p == player) {
            priority.simultaneous_decision_players.remove(pos);
        }
        priority.decision_timeouts.remove(&player);
        
        // Send timeout event
        commands.spawn(PlayerDecisionTimeoutEvent { player });
    }
    
    // Check if all decisions are complete
    if priority.simultaneous_decision_players.is_empty() {
        // Resume normal priority
        priority.waiting_for_response = false;
        
        // Send event that all decisions are complete
        commands.spawn(AllDecisionsCompleteEvent);
    }
}
```

### Managing the Stack with Many Players

```rust
fn optimize_stack_management(stack: &mut GameStack) {
    // Group similar triggers when possible
    // For example, if multiple players have the same trigger from the same source
    // This would require deeper analysis of trigger conditions and effects
    
    // Optimize memory usage for very large stacks
    if stack.items.len() > 50 {
        // Implement more efficient storage or compression for large stacks
    }
}
```

## Integration Points

- **Game State Module**: Coordinates with priority for game flow control
- **Turn Structure**: Handles phase transitions based on priority
- **Command Zone**: Special Commander abilities interact with the stack
- **Player Module**: Players' actions and decisions affect the stack
- **UI System**: Visualizes the stack and priority holder

## Testing Strategy

1. **Unit Tests**:
   - Verify priority passing logic
   - Test stack resolution order
   - Validate targeting rules
   
2. **Integration Tests**:
   - Test complex stack interactions
   - Verify handling of split-second spells
   - Test multiplayer simultaneous decisions
   - Validate commander-specific stack interactions

## Performance Considerations

For Commander games with up to 13 players:

- Efficient priority tracking without O(n) lookups
- Batch processing of triggered abilities
- Caching of commonly accessed state data
- Optimized validation checks for large stacks
- Throttling UI updates during complex stack resolutions

## User Experience Improvements

- Clear visualization of the stack for multiplayer games
- Highlighting the current priority holder
- Timer indicators for priority and decisions
- Stack item grouping for readability with many players 