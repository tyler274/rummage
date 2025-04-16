use bevy::prelude::*;

use super::systems::{
    debug_menu_visibility, detect_ui_hierarchy_issues, ensure_menu_item_visibility,
    fix_changed_main_menu_visibility, fix_visibility_for_changed_items,
    force_main_menu_items_visibility, force_startup_visibility, update_menu_background,
};
use crate::menu::components::MenuVisibilityState;
use crate::menu::state::AppState;
use crate::menu::ui::update_menu_visibility_state;

/// Plugin for managing menu item visibility and UI hierarchy
#[derive(Default)]
pub struct MenuVisibilityPlugin;

impl Plugin for MenuVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuVisibilityState>()
            // Startup systems
            .add_systems(Startup, force_startup_visibility)
            // First update systems - detect and fix issues
            .add_systems(
                Update,
                (
                    detect_ui_hierarchy_issues,
                    update_menu_visibility_state,
                    debug_menu_visibility,
                )
                    .run_if(in_state(AppState::Menu)),
            )
            // Main update systems - handle visibility changes
            .add_systems(
                PostUpdate,
                (
                    ensure_menu_item_visibility,
                    fix_visibility_for_changed_items,
                    fix_changed_main_menu_visibility,
                    force_main_menu_items_visibility,
                )
                    .run_if(in_state(AppState::Menu)),
            )
            // Background update system runs last
            .add_systems(
                Last,
                update_menu_background.run_if(in_state(AppState::Menu)),
            );

        debug!("Visibility plugin registered");
    }
}
