use bevy::prelude::*;

/// Marker component for star of david elements
#[derive(Component, Debug)]
pub struct StarOfDavid;

pub mod components;
pub mod plugin;
pub mod star;
pub mod systems;
pub mod text;

pub use plugin::LogoPlugin;
pub use star::*;
pub use text::*;
