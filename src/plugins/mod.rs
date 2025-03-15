mod game_plugin;

use bevy::prelude::*;

use crate::camera::CameraPlugin;
use crate::game_engine::commander::CommanderPlugin;
use crate::game_engine::phase::PhasePlugin;
use crate::game_engine::priority::PriorityPlugin;
use crate::game_engine::save::SaveLoadPlugin;
use crate::game_engine::stack::StackPlugin;
use crate::game_engine::state::StatePlugin;
use crate::game_engine::turns::TurnPlugin;
use crate::game_engine::zones::ZonePlugin;
use crate::player::PlayerPlugin;

pub use game_plugin::RummagePlugin;

pub struct RummagePlugin;

impl Plugin for RummagePlugin {
    fn build(&self, app: &mut App) {
        // Core game state
        app.add_plugins(StatePlugin);

        // Core game mechanics
        app.add_plugins(TurnPlugin);
        app.add_plugins(PhasePlugin);
        app.add_plugins(PriorityPlugin);
        app.add_plugins(StackPlugin);
        app.add_plugins(ZonePlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(CommanderPlugin);

        // Save/Load system
        app.add_plugins(SaveLoadPlugin);

        // Add your additional plugins here
    }
}
