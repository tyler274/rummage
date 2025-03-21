use bevy::prelude::*;

use super::{
    components::MenuVisibilityState,
    systems::{
        debug_menu_visibility, detect_ui_hierarchy_issues, ensure_menu_item_visibility,
        fix_changed_main_menu_visibility, fix_visibility_for_changed_items,
        force_main_menu_items_visibility, force_startup_visibility, update_menu_background,
        update_menu_visibility_state,
    },
};

/// Plugin for managing menu item visibility and UI hierarchy
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuVisibilityState>()
            .add_systems(Startup, force_startup_visibility)
            .add_systems(
                Update,
                (
                    update_menu_visibility_state,
                    update_menu_background,
                    detect_ui_hierarchy_issues,
                    ensure_menu_item_visibility,
                    fix_visibility_for_changed_items,
                    force_main_menu_items_visibility,
                    fix_changed_main_menu_visibility,
                    debug_menu_visibility,
                ),
            );

        debug!("Visibility plugin registered");
    }
}
