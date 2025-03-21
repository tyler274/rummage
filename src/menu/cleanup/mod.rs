//! Module for handling cleanup of menu and game entities

use bevy::prelude::*;

mod decorative;
mod game;
mod main_menu;
mod pause_menu;

pub use decorative::*;
pub use game::*;
pub use main_menu::*;
pub use pause_menu::*;

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic;
