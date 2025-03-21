use bevy::prelude::*;

/// The different menu states in the game
#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Copy, Default, Resource)]
pub enum MenuState {
    /// The main menu state shown at the start
    #[default]
    MainMenu,

    /// The state when a new game is started
    NewGame,

    /// The state when loading a saved game
    LoadGame,

    /// The state for settings menu
    Settings,

    /// The state for credits display
    Credits,

    /// The state for loading assets
    Loading,

    /// The state for the game
    InGame,

    /// The state for paused game
    PausedGame,
}

/// Type alias for backward compatibility during refactoring
pub type GameMenuState = MenuState;

/// Resource to track context around state transitions
#[derive(Resource, Debug, Default, Clone)]
pub struct StateTransitionContext {
    /// The originating state when entering settings
    pub settings_origin: Option<MenuState>,

    /// Whether we're transitioning back from settings
    pub returning_from_settings: bool,

    /// Whether transitioning from pause menu
    pub from_pause_menu: bool,
}
