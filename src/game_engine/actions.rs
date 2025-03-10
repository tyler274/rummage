use crate::card::{Card, CardDetails, CardTypes};
use crate::game_engine::state::GameState;
use crate::game_engine::{GameStack, Phase, PrioritySystem};
use crate::mana::Mana;
use crate::menu::GameMenuState;
use crate::player::Player;
use bevy::prelude::*;

/// Different types of game actions a player can take
#[derive(Debug, Clone)]
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
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut stack: ResMut<GameStack>,
    mut priority: ResMut<PrioritySystem>,
    phase: Res<Phase>,
    // Add an event reader for GameAction events when you implement the event system
    player_query: Query<&Player>,
    card_query: Query<&Card>,
) {
    // This would normally read actions from an event queue or similar
    // For now, this is a placeholder showing how action processing would work

    // Example: Processing a PlayLand action
    let action = GameAction::PlayLand {
        player: Entity::PLACEHOLDER,
        land_card: Entity::PLACEHOLDER,
    };

    match action {
        GameAction::PlayLand { player, land_card } => {
            // Check if it's a valid time to play a land
            if valid_time_to_play_land(&game_state, &phase, player) {
                // Check if the player has already played their land for the turn
                if game_state.can_play_land(player) {
                    // Check if the card is actually a land
                    if let Ok(card) = card_query.get(land_card) {
                        if card.types.contains(CardTypes::LAND) {
                            // Process playing the land
                            // This would involve adding it to the battlefield, etc.

                            // Record that a land was played
                            game_state.record_land_played(player);

                            // No priority is passed when playing a land
                        } else {
                            warn!("Attempted to play a non-land card as a land");
                        }
                    }
                } else {
                    warn!("Player has already played their land for this turn");
                }
            } else {
                warn!("Not a valid time to play a land");
            }
        }

        GameAction::CastSpell {
            player,
            spell_card,
            targets,
            mana_payment,
        } => {
            // Check if it's this player's priority
            if priority.active_player == player && priority.has_priority {
                // Check if it's a valid time to cast this spell
                if let Ok(card) = card_query.get(spell_card) {
                    let is_sorcery_speed = card.types.contains(CardTypes::SORCERY)
                        || (card.types.contains(CardTypes::CREATURE) && !is_instant_cast(&card));

                    // Check timing restrictions
                    if is_sorcery_speed
                        && !valid_time_for_sorcery(&game_state, &phase, &stack, player)
                    {
                        warn!("Not a valid time to cast a sorcery-speed spell");
                        return;
                    }

                    // Check mana payment
                    if let Ok(player_entity) = player_query.get(player) {
                        if !can_pay_mana(&player_entity, &mana_payment) {
                            warn!("Player cannot pay the mana cost for this spell");
                            return;
                        }

                        // Process casting the spell (would add to stack, etc.)

                        // Reset priority system after spell is cast
                        let players: Vec<Entity> = game_state.turn_order.iter().copied().collect();
                        priority.reset_after_stack_action(&players, game_state.active_player);
                    }
                }
            }
        }

        GameAction::ActivateAbility {
            player,
            source,
            ability_index,
            targets,
            mana_payment,
        } => {
            // Similar to cast spell, but for abilities
            // Would check activation restrictions, costs, etc.
        }

        GameAction::PassPriority { player } => {
            // Check if it's this player's priority
            if priority.active_player == player && priority.has_priority {
                // Pass priority to the next player
                priority.pass_priority();
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
fn can_pay_mana(player: &Player, cost: &Mana) -> bool {
    // In a full implementation, this would check the player's mana pool
    // For now, assume the player can pay
    true
}
