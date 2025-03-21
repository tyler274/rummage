use bevy::prelude::*;

// Temporary enum for multiplayer state until the actual implementation is created
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum MultiplayerState {
    #[default]
    None,
    Menu,
}
