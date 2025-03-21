use bevy::prelude::*;

/// Marker component for input blockers
#[derive(Component, Debug, Reflect)]
pub struct InputBlocker;

/// Resource to track input blocking state
#[derive(Resource, Default, Debug)]
pub struct InteractionBlockState {
    /// Whether interaction should be blocked
    pub should_block: bool,
}

/// A simple plugin for handling input blocking
#[derive(Default)]
pub struct InputBlockerPlugin;

impl Plugin for InputBlockerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InputBlocker>()
            .init_resource::<InteractionBlockState>();

        info!("InputBlocker plugin registered");
    }
}
