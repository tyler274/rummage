/// Mana system for Magic: The Gathering.
///
/// This module provides functionality for:
/// - Mana cost representation and parsing
/// - Mana pool management
/// - Mana payment validation
/// - Color identity calculations
///
mod color;
mod cost;
mod pool;
pub mod render;
pub mod symbols;

pub use color::*;
pub use cost::*;
pub use pool::*;
pub use symbols::*;

use bevy::prelude::*;

/// Plugin for registering mana-related systems
#[derive(Default)]
pub struct ManaPlugin;

impl Plugin for ManaPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ReflectableColor>()
            .register_type::<Mana>()
            .register_type::<ManaPool>();
    }
}
