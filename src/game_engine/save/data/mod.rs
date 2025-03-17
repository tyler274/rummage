// Re-export data structures from submodules
mod commander;
mod game_save;
mod game_state;
mod player;
mod zone;

// Re-export specific types for backward compatibility
pub use commander::{CommanderData, CommanderPairData};
pub use game_save::{
    GameSaveData, GameSaveDataBuilder, SaveInfo, convert_entity_to_index, convert_index_to_entity,
};
pub use game_state::{GameStateData, GameStateDataBuilder};
pub use player::{PlayerData, PlayerDataBuilder};
pub use zone::{CardData, ZoneData, ZoneType};
