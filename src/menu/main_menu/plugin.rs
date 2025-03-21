use bevy::prelude::*;

use crate::menu::{
    components::{MenuVisibilityState, NeedsMainMenuSetup},
    state::GameMenuState,
};

use super::systems::{
    background::update_background,
    interactions::handle_main_menu_interactions,
    setup::setup_main_menu,
};

#[derive(Resource, Default)]
pub struct MultiplayerState {
    pub is_multiplayer: bool,
}

/// Plugin for main menu functionality
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<MultiplayerState>()
            // Register systems
            .add_systems(OnEnter(GameMenuState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (
                    check_main_menu_setup.run_if(in_state(GameMenuState::MainMenu)),
                    handle_main_menu_interactions.run_if(in_state(GameMenuState::MainMenu)),
                    update_background.run_if(in_state(GameMenuState::MainMenu)),
                ),
            );

        info!("Main menu plugin registered");
    }
}

/// System to check if main menu needs to be set up
pub fn check_main_menu_setup(
    mut commands: Commands,
    menu_setup: Res<NeedsMainMenuSetup>,
    visibility: Res<MenuVisibilityState>,
) {
    // If the menu needs to be set up and it's supposed to be visible
    if menu_setup.0 && visibility.visible_items > 0 {
        // Trigger the setup
        info!("Main menu needs setup, dispatching setup event");
        commands.insert_resource(NeedsMainMenuSetup(false));
    }
} 