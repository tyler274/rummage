use crate::card::{Card, CardTypes};
use crate::game_engine::state::GameState;
use crate::game_engine::{GameStack, Phase, PostcombatStep, PrecombatStep};
use crate::mana::Mana;
use crate::player::Player;
use bevy::prelude::*;

/// Checks if it's a valid time to play a land
pub fn valid_time_to_play_land(game_state: &GameState, phase: &Phase, player: Entity) -> bool {
    // Can only play lands during your own turn
    if game_state.active_player != player {
        return false;
    }

    // Can only play lands during main phases
    match phase {
        Phase::Precombat(PrecombatStep::Main) => true,
        Phase::Postcombat(PostcombatStep::Main) => true,
        _ => false,
    }
}

/// Checks if it's a valid time to cast a sorcery-speed spell
pub fn valid_time_for_sorcery(
    game_state: &GameState,
    phase: &Phase,
    stack: &GameStack,
    player: Entity,
) -> bool {
    // Must be your turn
    if game_state.active_player != player {
        return false;
    }

    // Must be main phase
    if !phase.allows_sorcery_speed() {
        return false;
    }

    // Stack must be empty
    if !stack.is_empty() {
        return false;
    }

    true
}

/// Checks if a card can be cast at instant speed
pub fn is_instant_cast(card: &Card) -> bool {
    card.types.contains(CardTypes::INSTANT) ||
    // Flash would be checked here
    false
}

/// Checks if a player can pay a mana cost
pub fn can_pay_mana(_player: &Player, _cost: &Mana) -> bool {
    // Placeholder implementation
    true
}
