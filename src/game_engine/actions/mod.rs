mod types;
mod systems;
mod validation;

// Re-export everything needed by other modules
pub use types::GameAction;
pub use systems::process_game_actions;
pub use validation::{
    valid_time_to_play_land,
    valid_time_for_sorcery,
    is_instant_cast,
    can_pay_mana,
}; 