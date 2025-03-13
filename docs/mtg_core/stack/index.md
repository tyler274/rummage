# Stack and Priority System

## Overview

The stack is a fundamental mechanic in Magic: The Gathering, representing the order in which spells and abilities resolve. This section documents the core implementation of the stack and priority system in Rummage.

## The Stack

The stack is a last-in, first-out (LIFO) data structure that determines the order in which spells and abilities resolve during gameplay. 

### Stack Implementation

```rust
#[derive(Resource)]
pub struct Stack {
    // The stack itself - a list of stack items in order
    pub items: Vec<StackItem>,
    
    // The currently resolving stack item, if any
    pub currently_resolving: Option<StackItem>,
    
    // Temporary storage for abilities that trigger during resolution
    pub pending_triggers: Vec<TriggeredAbility>,
}

#[derive(Clone, Debug)]
pub struct StackItem {
    pub id: Entity,
    pub item_type: StackItemType,
    pub controller: Entity,
    pub source: Entity,
    pub targets: Vec<StackTarget>,
    pub effects: Vec<Effect>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StackItemType {
    Spell(CardType),
    Ability(AbilityType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbilityType {
    Activated,
    Triggered,
    Static, // Static abilities generally don't use the stack but are tracked here for targeting purposes
    Mana,   // Mana abilities don't use the stack but may be tracked here
}
```

## Stack Order and Resolution

The stack follows these core principles:

1. The active player gets priority first in each phase and step
2. When a player has priority, they can cast spells or activate abilities
3. Each spell or ability goes on top of the stack
4. After a spell or ability is put on the stack, all players get priority again
5. When all players pass priority in succession, the top item of the stack resolves
6. After resolution, the active player gets priority again

### Stack Resolution System

```rust
pub fn resolve_stack(
    mut commands: Commands,
    mut stack: ResMut<Stack>,
    mut game_state: ResMut<GameState>,
    mut zone_transitions: EventWriter<ZoneTransitionEvent>,
    mut effect_events: EventWriter<EffectEvent>,
) {
    // Check if there's something to resolve
    if stack.items.is_empty() {
        return;
    }
    
    // Get the top item
    let item = stack.items.pop().unwrap();
    stack.currently_resolving = Some(item.clone());
    
    // Process based on item type
    match item.item_type {
        StackItemType::Spell(card_type) => {
            // Process spell resolution
            // Move card to appropriate zone (battlefield for permanents, graveyard for others)
            // ...
        },
        StackItemType::Ability(ability_type) => {
            // Process ability resolution
            // ...
        }
    }
    
    // Process effects
    for effect in item.effects.iter() {
        effect_events.send(EffectEvent {
            effect: effect.clone(),
            source: item.source,
            controller: item.controller,
            targets: item.targets.clone(),
        });
    }
    
    // Clear currently resolving
    stack.currently_resolving = None;
    
    // Check for triggered abilities that happened during resolution
    // ...
}
```

## Priority System

The priority system determines which player can take actions at any given time. Priority passes in turn order, starting with the active player.

### Priority Implementation

```rust
#[derive(Resource)]
pub struct PrioritySystem {
    pub current_player: Entity,
    pub passed_players: HashSet<Entity>,
    pub active: bool,
}

#[derive(Event)]
pub struct PriorityEvent {
    pub player: Entity,
    pub action: PriorityAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityAction {
    Receive,
    Pass,
    TakeAction,
}
```

### Priority Passing System

```rust
pub fn handle_priority(
    mut commands: Commands,
    mut priority: ResMut<PrioritySystem>,
    mut priority_events: EventReader<PriorityEvent>,
    game_state: Res<GameState>,
    stack: Res<Stack>,
) {
    for event in priority_events.iter() {
        match event.action {
            PriorityAction::Pass => {
                // Record that this player passed
                priority.passed_players.insert(event.player);
                
                // Determine next player
                let next_player = get_next_player_in_turn_order(event.player, &game_state);
                
                // Check if all players have passed
                if priority.passed_players.len() == game_state.player_order.len() {
                    // Resolve top of stack or advance phase
                    if !stack.items.is_empty() {
                        // Resolve top of stack
                        commands.add(resolve_stack_command());
                    } else {
                        // Advance to next phase
                        commands.add(advance_phase_command());
                    }
                    
                    // Reset priority system
                    priority.passed_players.clear();
                    priority.current_player = game_state.active_player;
                } else {
                    // Pass priority to next player
                    priority.current_player = next_player;
                }
            },
            PriorityAction::TakeAction => {
                // Clear passed players since someone took an action
                priority.passed_players.clear();
                
                // After an action, active player gets priority
                priority.current_player = game_state.active_player;
            },
            // Other priority actions...
        }
    }
}
```

## Special Rules

### Mana Abilities

Mana abilities don't use the stack and resolve immediately, meaning they cannot be responded to. This is handled separately from the regular stack system.

### Split Second

Some cards have the "Split Second" ability, which means that while they're on the stack, players can't cast spells or activate abilities that aren't mana abilities.

### Interrupts and Special Timing

Certain effects can interrupt the normal flow of gameplay, such as:

- Replacement effects
- Prevention effects
- State-based actions
- Special actions that don't use the stack

## Format Extensions

Different formats may extend or modify the basic stack system:

- **Commander Format**: May have format-specific timing rules
- **Multiplayer Formats**: Must handle priority passing among multiple players

For format-specific stack and priority mechanics, see the respective format documentation.

---

Next: [Stack Resolution](stack_resolution.md) 