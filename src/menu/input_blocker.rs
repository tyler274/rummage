use bevy::prelude::*;

/// Component for blocking input to game elements when in menus
#[derive(Component)]
pub struct InputBlocker;

/// Resource to track whether interactions should be blocked
#[derive(Resource, Default)]
pub struct InteractionBlockState {
    pub should_block: bool,
}

/// Plugin to add input blocking functionality to the menu system
pub struct InputBlockerPlugin;

impl Plugin for InputBlockerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InteractionBlockState>()
            .add_systems(Update, update_interaction_block_state);
    }
}

/// System to determine if input/interactions should be blocked
fn update_interaction_block_state(
    input_blockers: Query<&InputBlocker>,
    mut block_state: ResMut<InteractionBlockState>,
) {
    // Block interactions if any InputBlocker exists
    let should_block = !input_blockers.is_empty();

    // Only log if the state changed
    if should_block != block_state.should_block {
        if should_block {
            debug!(
                "Input blocking activated - {} blockers present",
                input_blockers.iter().count()
            );
        } else {
            debug!("Input blocking deactivated");
        }
    }

    block_state.should_block = should_block;
}
