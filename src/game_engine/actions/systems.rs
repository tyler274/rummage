use crate::card::{Card, CardTypes};
use crate::game_engine::state::GameState;
use crate::game_engine::{GameStack, Phase, PrioritySystem};
use crate::player::Player;
use bevy::prelude::*;

use super::types::GameAction;
use super::validation::{
    can_pay_mana, is_instant_cast, valid_time_for_sorcery, valid_time_to_play_land,
};

/// System for validating and processing game actions
pub fn process_game_actions(
    _commands: Commands,
    mut game_state: ResMut<GameState>,
    _stack: ResMut<GameStack>,
    mut priority: ResMut<PrioritySystem>,
    phase: Res<Phase>,
    mut game_action_events: EventReader<GameAction>,
    _player_query: Query<&Player>,
    card_query: Query<&Card>,
) {
    // Process game actions from the event queue
    for action in game_action_events.read() {
        match action {
            GameAction::PlayLand { player, land_card } => {
                // Check if it's a valid time to play a land
                if valid_time_to_play_land(&game_state, &phase, *player) {
                    // Check if the player has already played a land this turn
                    if game_state.can_play_land(*player) {
                        // Check if the card is actually a land
                        if let Ok(card) = card_query.get(*land_card) {
                            if card.types.contains(CardTypes::LAND) {
                                // Mark that the player has played a land this turn
                                game_state.record_land_played(*player);
                                // In a full implementation, you would move the land from hand to battlefield
                                info!("Land played successfully");
                            }
                        }
                    }
                } else {
                    warn!("Not a valid time to play a land");
                }
            }

            GameAction::CastSpell {
                player,
                spell_card,
                targets: _,
                mana_payment: _,
            } => {
                // Check if it's a valid time to cast this spell
                if let Ok(card) = card_query.get(*spell_card) {
                    let is_instant = is_instant_cast(card);
                    if is_instant || valid_time_for_sorcery(&game_state, &phase, &_stack, *player) {
                        // In a full implementation, check if the player can pay the cost
                        if let Ok(player_entity) = _player_query.get(*player) {
                            if can_pay_mana(player_entity, &card.cost) {
                                // In a full implementation, you would move the spell to the stack
                                info!("Spell cast successfully");
                            }
                        }
                    }
                }
            }

            GameAction::ActivateAbility {
                player: _player,
                source: _source,
                ability_index: _ability_index,
                targets: _,
                mana_payment: _,
            } => {
                // Similar to cast spell, but for abilities
                // Would check activation restrictions, costs, etc.
            }

            GameAction::PassPriority { player } => {
                // Check if it's this player's priority
                if priority.has_priority(*player) {
                    // Pass priority to the next player
                    priority.pass_priority();
                }
            }
        }
    }
}
