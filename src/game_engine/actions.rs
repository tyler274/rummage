use crate::card::{Card, CardTypes};
use crate::game_engine::state::GameState;
use crate::game_engine::{GameStack, Phase, PrioritySystem};
use crate::mana::Mana;
use crate::player::Player;
use bevy::prelude::*;

/// Different types of game actions a player can take
#[derive(Debug, Clone, Event)]
pub enum GameAction {
    /// Play a land
    PlayLand { player: Entity, land_card: Entity },
    /// Cast a spell
    CastSpell {
        player: Entity,
        spell_card: Entity,
        targets: Vec<Entity>,
        mana_payment: Mana,
    },
    /// Activate an ability
    ActivateAbility {
        player: Entity,
        source: Entity,
        ability_index: usize,
        targets: Vec<Entity>,
        mana_payment: Mana,
    },
    /// Pass priority
    PassPriority { player: Entity },
}

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
                player,
                source,
                ability_index,
                targets: _,
                mana_payment: _,
            } => {
                // Similar to cast spell, but for abilities
                // Would check activation restrictions, costs, etc.
            }

            GameAction::PassPriority { player } => {
                // Check if it's this player's priority
                if priority.active_player == *player && priority.has_priority {
                    // Pass priority to the next player
                    priority.pass_priority();
                }
            }
        }
    }
}

/// Checks if it's a valid time to play a land
fn valid_time_to_play_land(game_state: &GameState, phase: &Phase, player: Entity) -> bool {
    // Can only play lands during your own turn
    if game_state.active_player != player {
        return false;
    }

    // Can only play lands during main phases
    match phase {
        Phase::Precombat(crate::game_engine::PrecombatStep::Main) => true,
        Phase::Postcombat(crate::game_engine::PostcombatStep::Main) => true,
        _ => false,
    }
}

/// Checks if it's a valid time to cast a sorcery-speed spell
fn valid_time_for_sorcery(
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
fn is_instant_cast(card: &Card) -> bool {
    card.types.contains(CardTypes::INSTANT) ||
    // Flash would be checked here
    false
}

/// Checks if a player can pay a mana cost
fn can_pay_mana(_player: &Player, _cost: &Mana) -> bool {
    // Placeholder implementation
    true
}
