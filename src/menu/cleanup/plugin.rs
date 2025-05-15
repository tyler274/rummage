use crate::menu::main_menu::pause_main_menu_music_on_settings_enter;
use crate::menu::state::GameMenuState;
use bevy::prelude::*;

/// Plugin for handling menu cleanup
pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(GameMenuState::MainMenu),
            super::main_menu::cleanup_main_menu,
        )
        .add_systems(
            OnEnter(GameMenuState::Settings),
            pause_main_menu_music_on_settings_enter,
        )
        .add_systems(
            OnExit(GameMenuState::PauseMenu),
            super::pause_menu::cleanup_pause_menu,
        )
        .add_systems(
            OnExit(GameMenuState::InGame),
            (super::game::cleanup_game, ApplyDeferred).chain(),
        );

        debug!("Cleanup plugin registered");
    }
}
