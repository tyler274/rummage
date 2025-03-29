use bevy::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

/// Resource to track previous window size
#[derive(Component, Default, Reflect, Debug)]
pub struct PreviousWindowSize {
    pub width: f32,
    pub height: f32,
}

/// Resource to control logging frequency for menu visibility
#[derive(Resource)]
pub struct MenuVisibilityLogState {
    pub last_item_count: usize,
    pub last_visible_items: usize,
    pub camera_states: HashMap<Entity, Visibility>,
    pub last_update: Instant,
}

impl Default for MenuVisibilityLogState {
    fn default() -> Self {
        Self {
            last_item_count: 0,
            last_visible_items: 0,
            camera_states: HashMap::new(),
            last_update: Instant::now(),
        }
    }
}
