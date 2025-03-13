use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// Enum representing the different rendering layers in the application
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLayer {
    /// Layer for the main game elements (cards, game UI, etc.)
    Game = 0,
    /// Layer for menu elements (menus, Stars of David, etc.)
    Menu = 1,
    /// Shared layer for elements that should appear in both views
    #[allow(dead_code)]
    Shared = 2,

    // Extended layers for more granular control
    /// Layer for game background elements
    Background = 3,
    /// Layer for the main game world (board, table)
    GameWorld = 4,
    /// Layer for cards
    Cards = 5,
    /// Layer for game UI elements
    GameUI = 6,
    /// Layer for popup dialogs
    Popup = 7,
    /// Layer for visual effects
    Effects = 8,
    /// Layer for overlay notifications
    Overlay = 9,
    /// Layer for wireframe rendering
    #[allow(dead_code)]
    Wireframe = 10,
    /// Layer for debug visuals
    #[allow(dead_code)]
    Debug = 11,
    /// Layer for debug text
    #[allow(dead_code)]
    DebugText = 12,
    /// Layer for debug gizmos/shapes
    #[allow(dead_code)]
    DebugGizmo = 13,
}

impl AppLayer {
    /// Convert the layer to usize for comparison
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    /// Get the RenderLayers mask for this single layer
    pub fn layer(&self) -> RenderLayers {
        RenderLayers::layer(self.as_usize())
    }

    /// Get a RenderLayers component including the shared layer
    #[allow(dead_code)]
    pub fn with_shared(&self) -> RenderLayers {
        RenderLayers::from_layers(&[self.as_usize(), AppLayer::Shared.as_usize()])
    }

    /// Get the combined RenderLayers for all game layers
    pub fn game_layers() -> RenderLayers {
        // Include the old Game layer for backward compatibility
        Self::Game.layer()
            | Self::Background.layer()
            | Self::GameWorld.layer()
            | Self::Cards.layer()
            | Self::GameUI.layer()
            | Self::Effects.layer()
            | Self::Overlay.layer()
    }

    /// Get the combined RenderLayers for menu layers
    pub fn menu_layers() -> RenderLayers {
        // Include the old Menu layer for backward compatibility
        Self::Menu.layer() | Self::Popup.layer()
    }

    /// Get the combined RenderLayers for debug layers
    #[allow(dead_code)]
    pub fn debug_layers() -> RenderLayers {
        Self::Debug.layer() | Self::DebugText.layer() | Self::DebugGizmo.layer()
    }

    /// Get the RenderLayers for wireframe visualization
    #[allow(dead_code)]
    pub fn wireframe_layers() -> RenderLayers {
        Self::Wireframe.layer()
    }

    /// Get the combined RenderLayers for all layers
    #[allow(dead_code)]
    pub fn all_layers() -> RenderLayers {
        Self::game_layers()
            | Self::menu_layers()
            | Self::debug_layers()
            | Self::wireframe_layers()
            | Self::Shared.layer()
    }
}

/// Component for marking an entity as a game camera
#[derive(Component, Debug)]
pub struct GameCamera;

/// Component for marking an entity as a menu camera
#[derive(Component, Debug)]
pub struct MenuCamera;
