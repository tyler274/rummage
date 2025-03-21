use bevy::prelude::*;

/// Marker component for star of david elements
#[derive(Component, Debug)]
pub struct StarOfDavid;

pub mod star;
pub mod text;

pub use star::*;
pub use text::*;
