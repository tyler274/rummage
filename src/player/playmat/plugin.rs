//! Plugin definition for the player playmat module.

use bevy::prelude::*;

// Import resources and systems from the parent module's submodules
use super::{
    battlefield, hand,
    resources::{CurrentPhaseLayout, PlaymatDebugState, ZoneFocusState},
    systems::{
        adapt_zone_sizes, handle_zone_interactions, highlight_active_zones,
        update_phase_based_layout,
    },
};

/// System set to identify all playmat-related systems for proper ordering
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PlaymatSystemSet {
    /// Core playmat systems
    Core,
}

/// Plugin for player playmat functionality
pub struct PlayerPlaymatPlugin;

impl Plugin for PlayerPlaymatPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing PlayerPlaymatPlugin");
        app.init_resource::<ZoneFocusState>()
            .init_resource::<PlaymatDebugState>()
            .init_resource::<CurrentPhaseLayout>()
            .configure_sets(Update, PlaymatSystemSet::Core)
            // UI interaction systems - keep in Update for responsiveness
            .add_systems(
                Update,
                (
                    handle_zone_interactions,
                    // Systems from submodules need explicit path
                    hand::toggle_hand_expansion,
                    battlefield::toggle_battlefield_grouping,
                    battlefield::adjust_battlefield_zoom,
                )
                    .in_set(PlaymatSystemSet::Core),
            )
            // Layout and rendering systems - can be in Update but after UI interactions
            .add_systems(
                Update,
                (
                    highlight_active_zones,
                    adapt_zone_sizes,
                    update_phase_based_layout,
                    // Systems from submodules need explicit path
                    hand::arrange_cards_in_hand,
                    battlefield::organize_battlefield_cards,
                )
                    .in_set(PlaymatSystemSet::Core)
                    .after(handle_zone_interactions),
            );
        info!("PlayerPlaymatPlugin initialization complete");
    }
}
