use bevy::prelude::*;

/// Marker component for menu-related entities to facilitate cleanup
#[derive(Component)]
pub struct MenuItem;

/// Marker component for menu-related camera
#[derive(Component)]
pub struct MenuCamera;

/// Marker component for the game camera
#[allow(dead_code)]
#[derive(Component)]
pub struct GameCamera;

/// Marker component for menu item decorative elements
#[derive(Component)]
pub struct MenuDecorativeElement;

/// Marker for menu root node
#[derive(Component)]
pub struct MenuRoot;

/// Marker for menu background
#[derive(Component)]
pub struct MenuBackground;

/// Button actions for different menu states
#[derive(Component, Clone, Debug)]
pub enum MenuButtonAction {
    /// Start a new game session
    NewGame,
    /// Load a previously saved game
    LoadGame,
    /// Enter multiplayer mode
    Multiplayer,
    /// Open settings menu
    Settings,
    /// Exit the game
    Quit,
    /// Resume the current game
    Resume,
    /// Restart the current game with a new hand
    Restart,
    /// Return to the main menu
    MainMenu,
    /// Save the current game
    SaveGame,
}
