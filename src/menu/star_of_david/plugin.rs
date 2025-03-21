use bevy::prelude::*;

/// Plugin for Star of David components and functionality
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, _app: &mut App) {
        // This plugin now only registers components and provides helper functions
        // The actual spawning of Star of David is handled by the LogoPlugin
        info!("Star of David plugin registered - providing components only");
    }
}
