use crate::menu::{
    components::{MenuBackground, MenuCamera, MenuItem},
    state::GameMenuState,
    ui::{MenuVisibilityLogState, MenuVisibilityState, PreviousWindowSize},
};
use bevy::prelude::*;

/// Update menu visibility state resource
pub fn update_menu_visibility_state(
    menu_items: Query<&Visibility, With<MenuItem>>,
    mut menu_state: ResMut<MenuVisibilityState>,
) {
    let total_items = menu_items.iter().count();
    let visible_items = menu_items
        .iter()
        .filter(|visibility| matches!(visibility, Visibility::Visible))
        .count();

    // Only update if changed
    if menu_state.item_count != total_items || menu_state.visible_count != visible_items {
        menu_state.item_count = total_items;
        menu_state.visible_count = visible_items;
    }
}

/// Debug system to check visibility of menu elements
pub fn debug_menu_visibility(
    menu_cameras: Query<(Entity, &Visibility), (With<MenuCamera>, Changed<Visibility>)>,
    menu_items: Query<(Entity, &Visibility), (With<MenuItem>, Changed<Visibility>)>,
    mut log_state: ResMut<MenuVisibilityLogState>,
    menu_state: Res<MenuVisibilityState>,
) {
    // Count cameras and menu items with changed visibility
    let camera_count = menu_cameras.iter().count();
    let menu_item_count = menu_items.iter().count();

    // Only proceed if any component actually changed
    if camera_count == 0 && menu_item_count == 0 {
        return;
    }

    // Collect current camera states for changed cameras
    for (entity, visibility) in menu_cameras.iter() {
        log_state.camera_states.insert(entity, *visibility);
        debug!(
            "Menu camera {:?} visibility changed to: {:?}",
            entity, visibility
        );
    }

    // Count visible items
    if menu_item_count > 0 {
        // We need to query all items to get the total count when visibility changes
        let all_items = menu_state.item_count;
        let visible_items = menu_state.visible_count;

        // Only log if visibility actually changed
        if visible_items != log_state.last_visible_items {
            log_state.last_item_count = all_items;
            log_state.last_visible_items = visible_items;
            debug!("Total menu items: {}", all_items);
            debug!("Visible menu items: {}/{}", visible_items, all_items);
        }
    }
}

/// System to update the menu background image size based on window dimensions
pub fn update_menu_background(
    windows: Query<&Window>,
    mut backgrounds: Query<(&mut Node, &mut PreviousWindowSize), With<MenuBackground>>,
    mut missing_size_backgrounds: Query<
        (Entity, &mut Node),
        (With<MenuBackground>, Without<PreviousWindowSize>),
    >,
    mut commands: Commands,
) {
    // Get the primary window
    if let Ok(window) = windows.get_single() {
        let current_width = window.width();
        let current_height = window.height();

        // Get all background image nodes and update their size
        for (mut node, mut prev_size) in &mut backgrounds {
            // Check if window size has changed
            if prev_size.width != current_width || prev_size.height != current_height {
                // Update the UI node size to match the window size exactly
                node.width = Val::Px(current_width);
                node.height = Val::Px(current_height);

                // Update the previous size
                prev_size.width = current_width;
                prev_size.height = current_height;

                // Log window size changes at debug level
                debug!(
                    "Window size changed: {}x{}, updating menu background size",
                    current_width, current_height
                );
            }
        }

        // Add PreviousWindowSize component to any background nodes that don't have it
        for (entity, mut node) in missing_size_backgrounds.iter_mut() {
            // Update the node size
            node.width = Val::Px(current_width);
            node.height = Val::Px(current_height);

            // Add the PreviousWindowSize component
            commands.entity(entity).insert(PreviousWindowSize {
                width: current_width,
                height: current_height,
            });

            debug!(
                "Added PreviousWindowSize component to menu background. Window size: {}x{}",
                current_width, current_height
            );
        }
    }
}

/// System to detect and report UI hierarchy issues
pub fn detect_ui_hierarchy_issues(
    menu_items: Query<(Entity, &Parent, Option<&Name>, &Node), With<MenuItem>>,
    parents: Query<Entity, (Without<Node>, Without<ViewVisibility>)>,
    mut found_issues: Local<bool>,
) {
    // Only run this diagnostic once if issues are found
    if *found_issues {
        return;
    }

    // Check for menu items that have non-UI parent entities
    let mut issues = false;
    for (entity, parent, name, _) in menu_items.iter() {
        if parents.contains(parent.get()) {
            issues = true;
            let name_str = name.map_or(String::from("unnamed"), |n| n.to_string());
            warn!(
                "UI hierarchy issue: Node {:?} ({}) is in a non-UI entity hierarchy",
                entity, name_str
            );
        }
    }

    // Set the flag if issues were found
    if issues {
        warn!("UI hierarchy issues detected - this may cause layout problems");
        *found_issues = true;
    }
}

/// Ensure menu items are visible in appropriate states
pub fn ensure_menu_item_visibility(
    mut menu_items: Query<(&mut Visibility, &Name), With<MenuItem>>,
    state: Res<State<GameMenuState>>,
) {
    let should_be_visible = matches!(
        state.get(),
        GameMenuState::MainMenu | GameMenuState::PausedGame | GameMenuState::Settings
    );

    for (mut visibility, name) in menu_items.iter_mut() {
        if should_be_visible && *visibility != Visibility::Visible {
            info!(
                "Setting menu item '{}' to Visible in state {:?}",
                name,
                state.get()
            );
            *visibility = Visibility::Visible;
        } else if !should_be_visible && *visibility == Visibility::Visible {
            info!(
                "Setting menu item '{}' to Hidden in state {:?}",
                name,
                state.get()
            );
            *visibility = Visibility::Hidden;
        }
    }
}

/// Fix visibility for menu items when it changes
pub fn fix_visibility_for_changed_items(
    mut items: Query<
        (&mut Visibility, &GlobalZIndex, &Name),
        (With<MenuItem>, Changed<Visibility>),
    >,
) {
    let item_count = items.iter().count();
    if item_count > 0 {
        info!("Fixing visibility for {} changed menu items", item_count);

        for (mut visibility, z_index, name) in items.iter_mut() {
            if *visibility != Visibility::Visible && z_index.0 > 0 {
                info!("Forcing menu item '{}' to be visible", name);
                *visibility = Visibility::Visible;
            }
        }
    }
}

/// Force visibility for menu items on startup
pub fn force_startup_visibility(
    mut menu_items: Query<(&mut Visibility, Option<&Name>), With<MenuItem>>,
) {
    let item_count = menu_items.iter().count();
    info!(
        "On startup, found {} menu items to force visible",
        item_count
    );

    for (mut visibility, name) in menu_items.iter_mut() {
        if *visibility != Visibility::Visible {
            if let Some(name) = name {
                info!("Forcing '{}' to be visible on startup", name);
            } else {
                info!("Forcing unnamed menu item to be visible on startup");
            }
            *visibility = Visibility::Visible;
        }
    }
}

/// Force visibility for hidden menu items in MainMenu state
pub fn force_main_menu_items_visibility(
    menu_items: Query<(Entity, &Visibility), With<MenuItem>>,
    mut commands: Commands,
    game_state: Res<State<GameMenuState>>,
) {
    // Only run when in MainMenu state
    if *game_state.get() != GameMenuState::MainMenu {
        return;
    }

    let count = menu_items.iter().count();
    let hidden_count = menu_items
        .iter()
        .filter(|(_, visibility)| **visibility != Visibility::Visible)
        .count();

    if hidden_count > 0 {
        info!(
            "Found {} hidden menu items out of {} total, forcing visibility",
            hidden_count, count
        );

        // Force visibility for any menu items that aren't visible
        for (entity, visibility) in menu_items.iter() {
            if *visibility != Visibility::Visible {
                commands.entity(entity).insert(Visibility::Visible);
            }
        }
    }
}

/// Fix visibility for any menu items whose visibility has changed while in main menu
pub fn fix_changed_main_menu_visibility(
    mut menu_items: Query<(&mut Visibility, &Name), (With<MenuItem>, Changed<Visibility>)>,
) {
    // Only update items whose visibility has changed
    if !menu_items.is_empty() {
        info!(
            "Setting visibility for {} changed menu items",
            menu_items.iter().count()
        );

        for (mut visibility, name) in menu_items.iter_mut() {
            if *visibility != Visibility::Visible {
                info!("Setting menu item '{}' visibility to Visible", name);
                *visibility = Visibility::Visible;
            }
        }
    }
}
