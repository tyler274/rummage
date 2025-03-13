// Re-export everything from the original stack.rs file
// pub use crate::game_engine::stack::*;

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

    /// Add an item to the stack
    pub fn push(
        &mut self,
        effect: Box<dyn Effect>,
        entity: Entity,
        has_split_second: bool,
        can_be_countered: bool,
    ) {
        let controller = effect.controller();
        let targets = effect.targets();

        // Track if this item has split-second
        if has_split_second {
            self.contains_split_second = true;
        }

        // Track if this item can't be countered
        if !can_be_countered {
            self.uncounterable_items.insert(entity);
        }

        // Add the item to the stack (top of stack = end of vector)
        self.items.push(StackItem {
            effect,
            controller,
            targets,
            entity,
            has_split_second,
            can_be_countered,
        });

        info!("Added item to stack. Stack size: {}", self.items.len());
    }

    /// Resolve the top item on the stack
    pub fn resolve_top(&mut self, commands: &mut Commands) -> Option<Entity> {
        if self.items.is_empty() {
            return None;
        }

        // Set resolving flag
        self.resolving = true;

        // Get the top item
        let item = self.items.pop().unwrap();
        let controller = item.controller;

        // Store the entity of the item being resolved
        self.currently_resolving = Some(item.entity);

        // Clean up uncounterable tracking
        self.uncounterable_items.remove(&item.entity);

        // Update split-second status
        self.update_split_second_status();

        // Resolve the effect
        info!("Resolving stack item from {:?}", controller);
        item.effect.resolve(commands);

        // Reset flags
        self.resolving = false;
        self.currently_resolving = None;

        // Return the controller for priority
        Some(controller)
    }

    /// Update whether the stack contains a split-second effect
    fn update_split_second_status(&mut self) {
        // Check if any remaining item has split-second
        self.contains_split_second = self.items.iter().any(|item| item.has_split_second);
    }

    /// Check if all targets for all items on the stack are still valid
    pub fn validate_targets(&self) -> bool {
        // For each item on the stack, check if all its targets are still valid
        // This would involve more complex validation that includes checking if
        // targets are still in the expected zones, still match targeting requirements, etc.
        // For simplicity, we'll just return true for now
        true
    }

    /// Check if targets for a specific stack item are still valid
    pub fn validate_item_targets(&self, item_entity: Entity) -> bool {
        if let Some(_item) = self.find_item(item_entity) {
            // Validate all targets for this specific item
            // Would involve checking if targets are still in appropriate zones,
            // still match targeting restrictions, protection, hexproof, etc.
            true // Simplified for now
        } else {
            false
        }
    }

    /// Check if a stack item can be countered
    pub fn can_be_countered(&self, target: Entity) -> bool {
        !self.uncounterable_items.contains(&target)
    }

    /// Find a stack item by its entity ID
    pub fn find_item(&self, entity: Entity) -> Option<&StackItem> {
        self.items.iter().find(|item| item.entity == entity)
    }

    /// Remove an item from the stack by its entity ID
    pub fn remove_item(&mut self, entity: Entity) -> Option<StackItem> {
        if let Some(index) = self.items.iter().position(|item| item.entity == entity) {
            let item = self.items.remove(index);
            self.update_split_second_status();
            Some(item)
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

/// System that handles resolving items from the stack
pub fn stack_resolution_system(
    mut commands: Commands,
    mut stack: ResMut<GameStack>,
    mut priority: ResMut<PrioritySystem>,
    game_state: ResMut<GameState>,
    mut stack_resolution_events: EventWriter<StackItemResolvedEvent>,
    mut resolve_events: EventReader<ResolveStackItemEvent>,
    mut counter_events: EventWriter<EffectCounteredEvent>,
) {
    if !resolve_events.is_empty() {
        resolve_events.clear();

        // Check if we have any items on the stack to resolve
        if stack.is_empty() {
            return;
        }

        // Validate targets before resolution
        if !stack.validate_targets() {
            // If targets are invalid, counter the spell
            let entity = stack.items.last().unwrap().entity;
            let controller = stack.items.last().unwrap().controller;

            // Remove the item from the stack
            stack.remove_item(entity);

            // Emit a counter event with reason
            counter_events.send(EffectCounteredEvent {
                item: entity,
                reason: CounterReason::InvalidTargets,
            });

            // Get all players (simplified for now, using only active player)
            let players = vec![game_state.active_player];

            // Reset priority after stack action
            priority.reset_after_stack_action(&players, game_state.active_player);

            return;
        }

        // Resolve the top item
        if let Some(controller) = stack.resolve_top(&mut commands) {
            // Send an event so other systems know this stack item resolved
            stack_resolution_events.send(StackItemResolvedEvent { controller });

            // Get all players (simplified for now, using only active player)
            let players = vec![game_state.active_player];

            // Reset priority after stack action
            priority.reset_after_stack_action(&players, game_state.active_player);
        }
    }
}
