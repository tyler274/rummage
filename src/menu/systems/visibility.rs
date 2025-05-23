use crate::menu::{
    backgrounds::MenuBackground,
    camera::MenuCamera,
    components::{MenuItem, MenuVisibilityState},
    settings::SettingsMenuState,
    state::GameMenuState,
    ui::{MenuVisibilityLogState, PreviousWindowSize},
};
use bevy::ecs::hierarchy::ChildOf;
use bevy::prelude::*;

/// Type alias for query accessing menu background nodes that are missing the PreviousWindowSize component.
type MissingSizeBackgroundQuery<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Node), (With<MenuBackground>, Without<PreviousWindowSize>)>;

/// Type alias for query accessing menu items whose visibility has changed.
type ChangedVisibilityItemQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Visibility,
        &'static GlobalZIndex,
        &'static Name,
    ),
    (With<MenuItem>, Changed<Visibility>),
>;

/// Type alias for query accessing menu items whose visibility has changed, specifically for the main menu fix.
type ChangedMainMenuVisibilityQuery<'w, 's> =
    Query<'w, 's, (&'static mut Visibility, &'static Name), (With<MenuItem>, Changed<Visibility>)>;

/// Update menu visibility state resource
pub fn update_menu_visibility_state(
    menu_items: Query<Entity, With<MenuItem>>,
    visible_items: Query<(Entity, &Visibility), With<MenuItem>>,
    mut menu_state: ResMut<MenuVisibilityState>,
) {
    // Gather counts
    let total_items = menu_items.iter().count();
    let visible_items_count = visible_items
        .iter()
        .filter(|(_, vis)| **vis == Visibility::Visible)
        .count();

    // Only update if there's a change
    if menu_state.visible_items != visible_items_count {
        menu_state.visible_items = visible_items_count;
        info!(
            "Menu visibility update: {}/{} items visible",
            visible_items_count, total_items
        );
    }
}

/// Prints debug info about menu visibility state
pub fn debug_menu_visibility(
    _menu_state: Res<MenuVisibilityState>,
    menu_items: Query<Entity, With<MenuItem>>,
    visible_items: Query<(Entity, &Visibility), With<MenuItem>>,
    mut log_state: Local<MenuVisibilityLogState>,
    menu_cameras: Query<(Entity, &Visibility), With<MenuCamera>>,
) {
    // Only log if something changed
    if log_state.last_update.elapsed().as_secs_f32() >= 5.0 {
        // Calculate visibility stats
        let total_items = menu_items.iter().count();
        let items_visible = visible_items
            .iter()
            .filter(|(_, vis)| **vis == Visibility::Visible)
            .count();

        // Log the state
        info!(
            "Menu visibility: {}/{} items visible",
            items_visible, total_items
        );

        // Log camera visibility
        for (entity, visibility) in menu_cameras.iter() {
            info!("Menu camera {:?} visibility: {:?}", entity, visibility);
        }

        log_state.last_update = std::time::Instant::now();
    }
}

/// System to update the menu background image size based on window dimensions
pub fn update_menu_background(
    windows: Query<&Window>,
    mut backgrounds: Query<(&mut Node, &mut PreviousWindowSize), With<MenuBackground>>,
    mut missing_size_backgrounds: MissingSizeBackgroundQuery,
    mut commands: Commands,
) {
    // Get the primary window
    if let Ok(window) = windows.single() {
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
    menu_items: Query<(Entity, &ChildOf, Option<&Name>, &Node), With<MenuItem>>,
    parents: Query<Entity, (Without<Node>, Without<ViewVisibility>)>,
    mut found_issues: Local<bool>,
) {
    // Only run this diagnostic once if issues are found
    if *found_issues {
        return;
    }

    // Check for menu items that have non-UI parent entities
    let mut issues = false;
    for (entity, parent_component, name, _) in menu_items.iter() {
        if parents.contains(parent_component.parent()) {
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
    game_state: Res<State<GameMenuState>>,
    settings_state: Res<State<SettingsMenuState>>,
) {
    // Determine if *any* menu UI should be visible based on game state
    let game_wants_menu_visible = matches!(
        game_state.get(),
        GameMenuState::MainMenu | GameMenuState::PauseMenu | GameMenuState::Settings
    );

    // Determine if the settings menu specifically is active (not Disabled)
    let settings_menu_active = *settings_state.get() != SettingsMenuState::Disabled;

    // Final visibility check: Menu is visible if the game state requires it,
    // AND if we are in the Settings game state, the settings menu state must also be active.
    let should_be_visible = game_wants_menu_visible
        && !(*game_state.get() == GameMenuState::Settings && !settings_menu_active);

    for (mut visibility, name) in menu_items.iter_mut() {
        if should_be_visible && *visibility != Visibility::Visible {
            info!(
                "Setting menu item '{}' to Visible based on states Game={:?}, Settings={:?}",
                name,
                game_state.get(),
                settings_state.get()
            );
            *visibility = Visibility::Visible;
        } else if !should_be_visible && *visibility == Visibility::Visible {
            info!(
                "Setting menu item '{}' to Hidden based on states Game={:?}, Settings={:?}",
                name,
                game_state.get(),
                settings_state.get()
            );
            *visibility = Visibility::Hidden;
        }
    }
}

/// Fix visibility for menu items when it changes
pub fn fix_visibility_for_changed_items(mut items: ChangedVisibilityItemQuery) {
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
pub fn fix_changed_main_menu_visibility(mut menu_items: ChangedMainMenuVisibilityQuery) {
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
