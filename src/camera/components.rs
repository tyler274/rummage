use bevy::prelude::*;
use bevy::render::view::RenderLayers;

/// Enum defining the render layers used in the game
#[derive(Debug, Clone, Copy)]
pub enum AppLayer {
    /// Layer for the main game elements (cards, game UI, etc.)
    Game = 0,
    /// Layer for menu elements (menus, Stars of David, etc.)
    Menu = 1,
    /// Shared layer for elements that should appear in both views
    Shared = 2,
}

impl AppLayer {
    /// Convert the enum variant to a usize for use with RenderLayers
    pub fn as_usize(&self) -> usize {
        *self as usize
    }

    /// Get a RenderLayers component configured for this layer
    pub fn layer(&self) -> RenderLayers {
        RenderLayers::layer(self.as_usize())
    }

    /// Get a RenderLayers component including the shared layer
    pub fn with_shared(&self) -> RenderLayers {
        RenderLayers::from_layers(&[self.as_usize(), AppLayer::Shared.as_usize()])
    }
}

/// Marker component for the game's main camera
#[derive(Component)]
pub struct GameCamera;

/// Marker component for menu cameras
#[derive(Component)]
pub struct MenuCamera;
