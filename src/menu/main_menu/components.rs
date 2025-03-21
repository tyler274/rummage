use bevy::prelude::*;

/// Marker component for main menu items
#[derive(Component, Debug, Clone)]
pub struct MainMenuItem;

/// Marker component for the main menu container
#[derive(Component, Debug, Clone)]
pub struct MainMenuContainer;

/// Marker component for the main menu background
#[derive(Component, Debug, Clone)]
pub struct MainMenuBackground;

/// Marker component for main menu buttons
#[derive(Component, Debug, Clone)]
pub struct MainMenuButton;

/// Component to mark the main menu music entity
#[derive(Component)]
pub struct MainMenuMusic; 