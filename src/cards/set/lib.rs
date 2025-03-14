use bevy::prelude::*;

/// Information about a card set
#[derive(Component, Debug, Clone, Reflect)]
pub struct CardSet {
    /// Set code (e.g., "MID" for Innistrad: Midnight Hunt)
    pub code: String,
    /// Full name of the set
    pub name: String,
    /// Release date of the set
    pub release_date: String,
}
