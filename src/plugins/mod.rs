mod game_plugin;

pub use game_plugin::GamePlugin;

/// Re-export GamePlugin as RummagePlugin for better naming
pub type RummagePlugin = GamePlugin;
