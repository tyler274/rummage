use super::systems::*;
use bevy::prelude::*;

/// Plugin for menu backgrounds
pub struct BackgroundsPlugin;

impl Plugin for BackgroundsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menu_background)
            .add_systems(Update, update_background);

        debug!("Backgrounds plugin registered");
    }
}
