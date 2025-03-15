use bevy::prelude::*;

use super::process_zone_changes;

/// Plugin that registers all card-related systems
pub struct CardSystemsPlugin;

impl Plugin for CardSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_zone_changes);
    }
}
