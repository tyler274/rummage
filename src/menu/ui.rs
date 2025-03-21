use crate::menu::{backgrounds::MenuBackground, components::MenuItem};
use bevy::prelude::*;

/// Resource to track previous window size
#[derive(Component, Default, Reflect, Debug)]
pub struct PreviousWindowSize {
    pub width: f32,
    pub height: f32,
}

/// Tracks visible menu items for diagnostics
#[derive(Resource, Default, Debug)]
pub struct MenuVisibilityState {
    pub item_count: usize,
    pub visible_count: usize,
}

/// Resource to control logging frequency for menu visibility
#[derive(Resource)]
pub struct MenuVisibilityLogState {
    pub last_item_count: usize,
    pub last_visible_items: usize,
    pub camera_states: std::collections::HashMap<Entity, Visibility>,
    pub last_update: std::time::Instant,
}

impl Default for MenuVisibilityLogState {
    fn default() -> Self {
        Self {
            last_item_count: 0,
            last_visible_items: 0,
            camera_states: std::collections::HashMap::new(),
            last_update: std::time::Instant::now(),
        }
    }
}

/// Creates the logo container for menu items
pub fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(300.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::vertical(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        Visibility::Visible,
        ZIndex::default(),
        Name::new("Logo Container"),
    )
}

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

/// Debug menu visibility and update visibility state
#[allow(dead_code)]
pub fn debug_menu_visibility(
    menu_cameras: Query<
        (Entity, &Visibility),
        (Changed<Visibility>, With<crate::menu::camera::MenuCamera>),
    >,
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
