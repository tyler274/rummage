use bevy::audio::AudioSink;
use bevy::prelude::*;

use crate::{
    // TODO: menu::state::MenuState,
    menu::state::GameMenuState,
};

use super::systems::{
    background::update_background, interactions::handle_main_menu_interactions,
    setup::setup_main_menu,
};

#[derive(Resource, Default)]
pub struct MultiplayerState {
    // Field removed as it was unused
}

/// Plugin for main menu functionality
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<MultiplayerState>()
            // Register systems
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu_adapter)
            .add_systems(
                Update,
                (
                    // REMOVED: check_main_menu_setup.run_if(in_state(GameMenuState::MainMenu)),
                    handle_main_menu_interactions.run_if(in_state(GameMenuState::MainMenu)),
                    update_background.run_if(in_state(GameMenuState::MainMenu)),
                ),
            );

        info!("Main menu plugin registered");
    }
}

/// Adapter to call setup_main_menu with all the necessary parameters including all_cameras
pub fn setup_main_menu_adapter(
    commands: Commands,
    asset_server: Res<AssetServer>,
    menu_cameras: Query<Entity, With<crate::camera::components::MenuCamera>>,
    existing_roots: Query<Entity, With<crate::menu::components::MenuRoot>>,
    all_cameras: Query<&Camera>,
    save_exists: ResMut<crate::menu::save_load::SaveExists>,
    music_sinks: Query<&AudioSink, With<crate::menu::main_menu::components::MainMenuMusic>>,
) {
    setup_main_menu(
        commands,
        asset_server,
        menu_cameras,
        existing_roots,
        all_cameras,
        save_exists,
        music_sinks,
    );
}

// REMOVED: System to check if main menu needs to be set up
/*
pub fn check_main_menu_setup(
// ... existing code ...
}
*/
