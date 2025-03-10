use bevy::prelude::*;

/// Game states for managing transitions between different parts of the game.
///
/// State transitions:
/// ```plaintext
///                    ┌─────────┐
///                    │         │
///                    ▼         │
/// MainMenu ──► Loading ──► InGame ◄─┐
///    ▲         │                    │
///    │         │                    │
///    └─────────┘              PausedGame
/// ```
///
/// - New Game: MainMenu -> Loading -> InGame
/// - Pause: InGame -> PausedGame
/// - Resume: PausedGame -> InGame
/// - Restart: PausedGame -> Loading -> InGame
/// - Main Menu: PausedGame -> MainMenu
#[derive(States, Resource, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameMenuState {
    /// Initial state, showing the main menu
    #[default]
    MainMenu,
    /// Transitional state for loading game assets
    Loading,
    /// Active gameplay state
    InGame,
    /// Game is paused, showing pause menu
    PausedGame,
}
