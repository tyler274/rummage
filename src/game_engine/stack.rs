use crate::game_engine::PrioritySystem;
use crate::game_engine::priority::{CounterReason, EffectCounteredEvent, ResolveStackItemEvent};
use crate::game_engine::state::GameState;
use bevy::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;

/// Trait for effects that can go on the stack
pub trait Effect: Debug + Send + Sync {
    /// Resolve the effect when it comes off the stack
    fn resolve(&self, commands: &mut Commands);

    /// Get the controller of this effect
    fn controller(&self) -> Entity;

    /// Get the targets of this effect
    fn targets(&self) -> Vec<Entity>;
}

/// Event fired when a stack item is resolved
#[derive(Event)]
pub struct StackItemResolvedEvent {
    /// The controller of the resolved effect
    pub controller: Entity,
}

/// The MTG stack system that manages spells and abilities
#[derive(Resource)]
pub struct GameStack {
    /// Items currently on the stack (first = bottom, last = top)
    pub items: Vec<StackItem>,

    /// Whether the stack is currently resolving an item
    pub resolving: bool,

    /// The current item being resolved, if any
    pub currently_resolving: Option<Entity>,

    /// Whether the stack contains a split-second effect (prevents further additions)
    pub contains_split_second: bool,

    /// Entities of items that cannot be countered
    pub uncounterable_items: HashSet<Entity>,
}

/// An item on the stack (spell or ability)
pub struct StackItem {
    /// The effect to resolve
    pub effect: Box<dyn Effect>,

    /// The controller of the effect
    pub controller: Entity,

    /// The targets of the effect
    pub targets: Vec<Entity>,

    /// Entity ID for this stack item
    pub entity: Entity,

    /// Whether this item has split-second
    pub has_split_second: bool,

    /// Whether this item can be countered
    pub can_be_countered: bool,
}

impl GameStack {
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get the number of items on the stack
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Push an item onto the stack
    pub fn push(
        &mut self,
        effect: Box<dyn Effect>,
        entity: Entity,
        has_split_second: bool,
        can_be_countered: bool,
    ) {
        let controller = effect.controller();
        let targets = effect.targets();

        // If this has split-second, set the flag
        if has_split_second {
            self.contains_split_second = true;
        }

        // Track uncounterable items
        if !can_be_countered {
            self.uncounterable_items.insert(entity);
        }

        self.items.push(StackItem {
            effect,
            controller,
            targets,
            entity,
            has_split_second,
            can_be_countered,
        });
    }

    /// Resolve the top item on the stack
    pub fn resolve_top(&mut self, commands: &mut Commands) -> Option<Entity> {
        if self.items.is_empty() {
            return None;
        }

        self.resolving = true;

        // Get the top item (last in the vec)
        let item = self.items.pop().unwrap();

        // Store the entity of the currently resolving item
        self.currently_resolving = Some(item.entity);

        // Resolve the effect
        item.effect.resolve(commands);

        // Update split-second status after resolution
        self.update_split_second_status();

        // Remove from uncounterable tracking if needed
        self.uncounterable_items.remove(&item.entity);

        self.resolving = false;
        self.currently_resolving = None;

        // Return the controller of the resolved effect
        Some(item.controller)
    }

    /// Update split-second status after a stack item resolves
    fn update_split_second_status(&mut self) {
        // Reset split-second flag then check if any remaining items have it
        self.contains_split_second = false;
        for item in &self.items {
            if item.has_split_second {
                self.contains_split_second = true;
                break;
            }
        }
    }

    /// Check if the targets are still valid for all items on the stack
    pub fn validate_targets(&self) -> bool {
        // This is a simplified implementation
        // A real implementation would check each target against game rules
        for item in &self.items {
            // Check if all targets still exist and are valid
            let targets = item.effect.targets();
            if targets.is_empty() {
                continue; // No targets to validate
            }

            // For now, just assume targets are valid if they were provided
            // In a real implementation, you would check:
            // - If target entities still exist in the game
            // - If targets are still valid according to targeting restrictions
            // - If targets have gained protection or other effects
        }
        true
    }

    /// Validate targets for a specific stack item
    pub fn validate_item_targets(&self, item_entity: Entity) -> bool {
        if let Some(item) = self.items.iter().find(|i| i.entity == item_entity) {
            // A real implementation would check each target against game rules
            let targets = item.effect.targets();
            if targets.is_empty() {
                return true; // No targets to validate
            }

            // For now, just assume targets are valid if they were provided
            // In a real implementation, you would check:
            // - If target entities still exist in the game
            // - If targets are still valid according to targeting restrictions
            // - If targets have gained protection or other effects
        }
        true
    }

    /// Check if a stack item can be countered
    pub fn can_be_countered(&self, target: Entity) -> bool {
        // Check if this is in the uncounterable set
        !self.uncounterable_items.contains(&target)
    }

    /// Find a stack item by entity
    pub fn find_item(&self, entity: Entity) -> Option<&StackItem> {
        self.items.iter().find(|item| item.entity == entity)
    }

    /// Find a stack item by entity and remove it
    pub fn remove_item(&mut self, entity: Entity) -> Option<StackItem> {
        if let Some(index) = self.items.iter().position(|item| item.entity == entity) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
}

impl Default for GameStack {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            resolving: false,
            currently_resolving: None,
            contains_split_second: false,
            uncounterable_items: HashSet::new(),
        }
    }
}

/// System to handle stack resolution
pub fn stack_resolution_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut priority: ResMut<PrioritySystem>,
    game_state: ResMut<GameState>,
    mut stack_resolution_events: EventWriter<StackItemResolvedEvent>,
    mut resolve_events: EventReader<ResolveStackItemEvent>,
    mut counter_events: EventWriter<EffectCounteredEvent>,
) {
    // Process specific resolution events first
    for event in resolve_events.read() {
        let item_entity = event.item;

        // Check if we can find this item on the stack
        if let Some(item_index) = stack
            .items
            .iter()
            .position(|item| item.entity == item_entity)
        {
            // Mark the stack as resolving
            stack.resolving = true;

            // Check if targets are still valid
            let targets_valid = stack.validate_item_targets(item_entity);

            // Get the item
            let item = stack.items.remove(item_index);

            if targets_valid {
                // Store entity in currently resolving
                stack.currently_resolving = Some(item.entity);

                // Resolve the effect
                let controller = item.controller;
                item.effect.resolve(&mut commands);

                // Send resolution event
                stack_resolution_events.send(StackItemResolvedEvent { controller });

                // Update split second status
                stack.update_split_second_status();

                // Clean up
                stack.uncounterable_items.remove(&item.entity);
            } else {
                // Targets not valid, counter the effect
                counter_events.send(EffectCounteredEvent {
                    item: item_entity,
                    reason: CounterReason::InvalidTargets,
                });
            }

            // Reset resolving state
            stack.resolving = false;
            stack.currently_resolving = None;

            // Reset priority after resolution
            let players: Vec<Entity> = game_state.turn_order.iter().copied().collect();
            priority.reset_after_stack_action(&players, game_state.active_player);

            // Update stack empty status
            priority.set_stack_empty(stack.is_empty());
        }
    }

    // Only proceed with automatic resolution if we didn't handle a specific event
    if resolve_events.is_empty() {
        // Handle priority-based resolution (this is the original logic)
        if priority.all_players_passed && !stack.is_empty() && !stack.resolving {
            // Resolve the top item on the stack
            if let Some(controller) = stack.resolve_top(&mut commands) {
                // Send a stack item resolved event
                stack_resolution_events.send(StackItemResolvedEvent { controller });

                // Reset priority after resolution
                let players: Vec<Entity> = game_state.turn_order.iter().copied().collect();
                priority.reset_after_stack_action(&players, game_state.active_player);

                // Update stack empty status
                priority.set_stack_empty(stack.is_empty());
            }
        }
    }
}
