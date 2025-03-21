use bevy::prelude::*;

use crate::menu::state::GameMenuState;

use super::systems::{cleanup_star_of_david, setup_main_menu_star, setup_pause_star};

/// Plugin for Star of David functionality
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, app: &mut App) {
        app
            // Main menu star setup
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu_star)
            // Pause menu star setup
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_star)
            // Cleanup systems
            .add_systems(OnExit(GameMenuState::MainMenu), cleanup_star_of_david)
            .add_systems(OnExit(GameMenuState::PausedGame), cleanup_star_of_david);

        info!("Star of David plugin registered");
    }
}
