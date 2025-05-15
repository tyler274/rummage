use crate::menu::settings::state::SettingsMenuState;
use crate::menu::{
    backgrounds::MenuBackground,
    camera::MenuCamera,
    components::{MenuItem, MenuVisibilityState},
    state::GameMenuState,
    visibility::components::{MenuVisibilityLogState, PreviousWindowSize},
};
use bevy::prelude::*;
use bevy::render::view::InheritedVisibility;

// Type Aliases for complex queries
type MissingSizeBackgroundQueryVis<'w, 's> =
    Query<'w, 's, (Entity, &'static mut Node), (With<MenuBackground>, Without<PreviousWindowSize>)>;
type ChangedVisibilityItemGlobalZQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Visibility,
        &'static mut InheritedVisibility,
        &'static GlobalZIndex,
        &'static Name,
    ),
    (With<MenuItem>, Changed<Visibility>),
>;
type ChangedVisibilityItemZQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Visibility,
        &'static mut InheritedVisibility,
        &'static ZIndex,
        &'static Name,
    ),
    (With<MenuItem>, Changed<Visibility>, Without<GlobalZIndex>),
>;
type ChangedMainMenuVisibilityVisQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static mut Visibility,
        &'static mut InheritedVisibility,
        &'static Name,
    ),
    (With<MenuItem>, Changed<Visibility>),
>;

/// Update menu visibility state resource and ensure menu items are visible in appropriate states
pub fn ensure_menu_item_visibility(
    mut menu_items: Query<(&mut Visibility, &mut InheritedVisibility, &Name), With<MenuItem>>,
    state: Res<State<GameMenuState>>,
    settings_state: Res<State<SettingsMenuState>>,
    mut menu_state: ResMut<MenuVisibilityState>,
) {
    let should_be_visible = matches!(
        state.get(),
        GameMenuState::MainMenu | GameMenuState::PauseMenu | GameMenuState::Settings
    );

    let is_in_settings = *state.get() == GameMenuState::Settings
        && *settings_state.get() != SettingsMenuState::Disabled;

    let mut visible_count = 0;
    let total_items = menu_items.iter().count();

    for (mut visibility, mut inherited, name) in menu_items.iter_mut() {
        if should_be_visible {
            if *visibility != Visibility::Visible || *inherited != InheritedVisibility::VISIBLE {
                debug!(
                    "Setting menu item '{}' to Visible in state {:?}",
                    name,
                    state.get()
                );
                *visibility = Visibility::Visible;
                *inherited = InheritedVisibility::VISIBLE;
            }
            visible_count += 1;
        } else if *visibility == Visibility::Visible && !is_in_settings {
            debug!(
                "Setting menu item '{}' to Hidden in state {:?}",
                name,
                state.get()
            );
            *visibility = Visibility::Hidden;
        }
    }

    // Update menu state if counts changed
    if menu_state.visible_items != visible_count {
        menu_state.item_count = total_items;
        menu_state.visible_count = visible_count;
        menu_state.visible_items = visible_count;
        debug!(
            "Menu visibility update: {}/{} items visible",
            visible_count, total_items
        );
    }
}

/// System to update the menu background image size based on window dimensions
pub fn update_menu_background(
    windows: Query<&Window>,
    mut backgrounds: Query<(&mut Node, &mut PreviousWindowSize), With<MenuBackground>>,
    mut missing_size_backgrounds: MissingSizeBackgroundQueryVis,
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
    parents: Query<(Entity, Option<&Node>, Option<&ViewVisibility>)>,
    mut found_issues: Local<bool>,
) {
    // Only run this diagnostic once if issues are found
    if *found_issues {
        return;
    }

    // Check for menu items that have non-UI parent entities
    let mut issues = false;
    for (entity, parent, name, _) in menu_items.iter() {
        if let Ok((parent_entity, node, view_vis)) = parents.get(parent.get()) {
            if node.is_none() || view_vis.is_none() {
                issues = true;
                let name_str = name.map_or(String::from("unnamed"), |n| n.to_string());
                warn!(
                    "UI hierarchy issue: Node {:?} ({}) has parent {:?} without proper UI components",
                    entity, name_str, parent_entity
                );
            }
        }
    }

    // Set the flag if issues were found
    if issues {
        warn!("UI hierarchy issues detected - this may cause layout problems");
        *found_issues = true;
    }
}

/// Prints debug info about menu visibility state
pub fn debug_menu_visibility(
    menu_state: Res<MenuVisibilityState>,
    menu_cameras: Query<(Entity, &Visibility, &InheritedVisibility), With<MenuCamera>>,
    mut log_state: Local<MenuVisibilityLogState>,
) {
    // Only log if something changed
    if log_state.last_update.elapsed().as_secs_f32() >= 5.0 {
        // Log the state
        debug!(
            "Menu visibility: {}/{} items visible",
            menu_state.visible_count, menu_state.item_count
        );

        // Log camera visibility
        for (entity, visibility, inherited) in menu_cameras.iter() {
            debug!(
                "Menu camera {:?} visibility: {:?}, inherited: {:?}",
                entity, visibility, inherited
            );
        }

        log_state.last_update = std::time::Instant::now();
    }
}

/// Fix visibility for menu items when it changes
pub fn fix_visibility_for_changed_items(
    mut items_with_global_z: Query<
        (
            &mut Visibility,
            &mut InheritedVisibility,
            &GlobalZIndex,
            &Name,
        ),
        (With<MenuItem>, Without<ZIndex>),
    >,
    mut items_with_z: Query<
        (&mut Visibility, &mut InheritedVisibility, &ZIndex, &Name),
        (With<MenuItem>, Without<GlobalZIndex>),
    >,
) {
    let global_z_count = items_with_global_z.iter().count();
    let z_count = items_with_z.iter().count();

    if global_z_count > 0 || z_count > 0 {
        // debug!(
        //     "Checking visibility for {} menu items (GlobalZIndex: {}, ZIndex: {})",
        //     global_z_count + z_count,
        //     global_z_count,
        //     z_count
        // );

        // Process items with GlobalZIndex
        for (mut visibility, mut inherited, z_index, _name) in items_with_global_z.iter_mut() {
            if *visibility == Visibility::Hidden && z_index.0 > 0 {
                // debug!(
                //     "Forcing menu item '{}' with GlobalZIndex {} to be visible",
                //     name, z_index.0
                // );
                *visibility = Visibility::Visible;
                *inherited = InheritedVisibility::VISIBLE;
            }
        }

        // Process items with ZIndex
        for (mut visibility, mut inherited, z_index, _name) in items_with_z.iter_mut() {
            if *visibility == Visibility::Hidden && z_index.0 > 0 {
                // debug!(
                //     "Forcing menu item '{}' with ZIndex {} to be visible",
                //     name, z_index.0
                // );
                *visibility = Visibility::Visible;
                *inherited = InheritedVisibility::VISIBLE;
            }
        }
    }
}

/// Force visibility for menu items on startup
pub fn force_startup_visibility(
    mut menu_items: Query<
        (&mut Visibility, &mut InheritedVisibility, Option<&Name>),
        With<MenuItem>,
    >,
) {
    let item_count = menu_items.iter().count();
    debug!(
        "On startup, found {} menu items to force visible",
        item_count
    );

    for (mut visibility, mut inherited, name) in menu_items.iter_mut() {
        if *visibility != Visibility::Visible || *inherited != InheritedVisibility::VISIBLE {
            if let Some(name) = name {
                debug!("Forcing '{}' to be visible on startup", name);
            } else {
                debug!("Forcing unnamed menu item to be visible on startup");
            }
            *visibility = Visibility::Visible;
            *inherited = InheritedVisibility::VISIBLE;
        }
    }
}

/// Force visibility for hidden menu items in MainMenu state
pub fn force_main_menu_items_visibility(
    menu_items: Query<(Entity, &Visibility, &InheritedVisibility), With<MenuItem>>,
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
        .filter(|(_, visibility, inherited)| {
            **visibility != Visibility::Visible || **inherited != InheritedVisibility::VISIBLE
        })
        .count();

    if hidden_count > 0 {
        debug!(
            "Found {} hidden menu items out of {} total, forcing visibility",
            hidden_count, count
        );

        // Force visibility for any menu items that aren't visible
        for (entity, visibility, inherited) in menu_items.iter() {
            if *visibility != Visibility::Visible || *inherited != InheritedVisibility::VISIBLE {
                commands
                    .entity(entity)
                    .insert((Visibility::Visible, InheritedVisibility::VISIBLE));
            }
        }
    }
}

/// Fix visibility for changed menu items in the main menu
pub fn fix_changed_main_menu_visibility(mut menu_items: ChangedMainMenuVisibilityVisQuery) {
    let changed_count = menu_items.iter().count();
    if changed_count > 0 {
        debug!("Fixing visibility for {} changed menu items", changed_count);

        for (mut visibility, mut inherited, name) in menu_items.iter_mut() {
            if *visibility != Visibility::Visible || *inherited != InheritedVisibility::VISIBLE {
                debug!("Forcing menu item '{}' to be visible", name);
                *visibility = Visibility::Visible;
                *inherited = InheritedVisibility::VISIBLE;
            }
        }
    }
}
