use crate::game_engine::PrioritySystem;
use crate::game_engine::state::GameState;
use bevy::prelude::*;
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
}

/// An item on the stack (spell or ability)
pub struct StackItem {
    /// The effect to resolve
    pub effect: Box<dyn Effect>,

    /// The controller of the effect
    pub controller: Entity,

    /// The targets of the effect
    pub targets: Vec<Entity>,
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
    pub fn push(&mut self, effect: Box<dyn Effect>) {
        let controller = effect.controller();
        let targets = effect.targets();

        self.items.push(StackItem {
            effect,
            controller,
            targets,
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

        // Resolve the effect
        item.effect.resolve(commands);

        self.resolving = false;

        // Return the controller of the resolved effect
        Some(item.controller)
    }

    /// Check if the targets are still valid for all items on the stack
    pub fn validate_targets(&self) -> bool {
        // In a full implementation, this would check if each target is still valid
        // For now, assume all targets are valid
        true
    }
}

impl Default for GameStack {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            resolving: false,
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
    // Other resources and queries needed for stack resolution
) {
    // Only resolve if all players have passed priority and the stack is not empty
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
