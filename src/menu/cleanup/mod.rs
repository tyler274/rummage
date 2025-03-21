//! Module for handling cleanup of menu and game entities

use bevy::prelude::*;

mod decorative;
mod game;
mod main_menu;
mod pause_menu;

// These modules are used internally but not exported
// to avoid unused import warnings

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic;
