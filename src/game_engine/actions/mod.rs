mod systems;
mod types;
mod validation;

// Re-export everything needed by other modules
pub use systems::process_game_actions;
pub use types::GameAction;

// TODO: Implement validation functions and expose them as needed
// Currently these functions are defined but not used
// pub use validation::{
//     valid_time_to_play_land,
//     valid_time_for_sorcery,
//     is_instant_cast,
//     can_pay_mana,
// };
