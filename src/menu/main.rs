use crate::menu::state::MenuState;
use crate::menu::systems::main_menu::{
    background::setup_menu_background, interactions::handle_main_menu_interactions,
    setup::setup_main_menu, states::MultiplayerState,
};
use bevy::prelude::*;

/// Plugin for handling the main menu functionality
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register multiplayer state
            .init_state::<MultiplayerState>()
            // Add main menu systems
            .add_systems(
                OnEnter(MenuState::MainMenu),
                (setup_main_menu, setup_menu_background),
            )
            .add_systems(
                Update,
                handle_main_menu_interactions.run_if(in_state(MenuState::MainMenu)),
            );

        info!("MainMenuPlugin initialized");
    }
}

/// Set up the main menu UI
#[allow(dead_code)]
pub fn setup_main_menu(_commands: Commands) {
    // Implementation for setting up main menu UI will go here
    // This is currently a placeholder function
}

/// Handle main menu actions
#[allow(dead_code)]
pub fn handle_main_menu_action() {
    // Implementation for main menu action handling will go here
    // This is currently a placeholder function
}
