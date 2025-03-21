use super::systems::*;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Plugin for menu logo
pub struct LogoPlugin;

impl Plugin for LogoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu_star)
            .add_systems(OnEnter(GameMenuState::PausedGame), setup_pause_star)
            .add_systems(
                OnExit(GameMenuState::MainMenu),
                cleanup_star_of_david_thoroughly,
            )
            .add_systems(
                OnExit(GameMenuState::PausedGame),
                cleanup_star_of_david_thoroughly,
            );

        debug!("Logo plugin registered");
    }
}
