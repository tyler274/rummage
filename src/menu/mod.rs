mod cleanup;
mod components;
mod logo;
mod main_menu;
mod pause_menu;
mod simple_menu;
mod state;
mod styles;
mod systems;

pub use cleanup::*;
pub use components::*;
pub use main_menu::*;
pub use pause_menu::*;
pub use simple_menu::*;
pub use state::*;
pub use styles::*;
pub use systems::*;

use bevy::prelude::*;

/// Plugin for menu management
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(logo::StarOfDavidPlugin)
            .init_resource::<state::StateTransitionContext>()
            .init_state::<state::GameMenuState>()
            .add_systems(
                OnEnter(state::GameMenuState::MainMenu),
                simple_menu::setup_simple_menu,
            )
            .add_systems(
                OnEnter(state::GameMenuState::PauseMenu),
                pause_menu::setup_pause_menu,
            )
            .add_systems(
                Update,
                (
                    simple_menu::simple_menu_action,
                    pause_menu::pause_menu_action,
                    pause_menu::handle_pause_input,
                ),
            )
            .add_systems(
                OnExit(state::GameMenuState::MainMenu),
                cleanup::cleanup_main_menu,
            )
            .add_systems(
                OnExit(state::GameMenuState::PauseMenu),
                cleanup::cleanup_pause_menu,
            )
            .add_systems(OnExit(state::GameMenuState::InGame), cleanup::cleanup_game);
    }
}
