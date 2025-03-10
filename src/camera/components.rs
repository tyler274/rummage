use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// Enum representing the different rendering layers in the application
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLayer {
    /// Layer for game background elements
    Background = 0,
    /// Layer for the main game world (board, table)
    GameWorld = 1,
    /// Layer for cards
    Cards = 2,
    /// Layer for game UI elements
    GameUI = 3,
    /// Layer for menu elements
    Menu = 4,
    /// Layer for popup dialogs
    Popup = 5,
    /// Layer for visual effects
    Effects = 6,
    /// Layer for overlay notifications
    Overlay = 7,
    /// Layer for wireframe rendering
    Wireframe = 8,
    /// Layer for debug visuals
    Debug = 9,
    /// Layer for debug text
    DebugText = 10,
    /// Layer for debug gizmos/shapes
    DebugGizmo = 11,
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

    /// Get the combined RenderLayers for all game layers
    pub fn game_layers() -> RenderLayers {
        Self::Background.layer()
            | Self::GameWorld.layer()
            | Self::Cards.layer()
            | Self::GameUI.layer()
            | Self::Effects.layer()
            | Self::Overlay.layer()
    }

    /// Get the combined RenderLayers for menu layers
    pub fn menu_layers() -> RenderLayers {
        Self::Menu.layer() | Self::Popup.layer()
    }

    /// Get the combined RenderLayers for debug layers
    pub fn debug_layers() -> RenderLayers {
        Self::Debug.layer() | Self::DebugText.layer() | Self::DebugGizmo.layer()
    }

    /// Get the RenderLayers for wireframe visualization
    pub fn wireframe_layers() -> RenderLayers {
        Self::Wireframe.layer()
    }

    /// Get the combined RenderLayers for all layers
    pub fn all_layers() -> RenderLayers {
        Self::game_layers() | Self::menu_layers() | Self::debug_layers() | Self::wireframe_layers()
    }
}

/// Component for marking an entity as a game camera
#[derive(Component, Debug)]
pub struct GameCamera;

/// Component for marking an entity as a menu camera
#[derive(Component, Debug)]
pub struct MenuCamera;
