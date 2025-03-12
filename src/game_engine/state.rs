use crate::card::{Card, CreatureOnField};
use crate::game_engine::commander::{Commander, EliminationReason, PlayerEliminatedEvent};
use crate::game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager};
use crate::player::Player;
use bevy::prelude::*;
use std::collections::VecDeque;

/// The global game state for an MTG game
#[derive(Resource)]
pub struct GameState {
    /// The current turn number
    pub turn_number: u32,

    /// The player whose turn it is
    pub active_player: Entity,

    /// The player currently with priority
    pub priority_holder: Entity,

    /// The turn order of players
    pub turn_order: VecDeque<Entity>,

    /// Lands played this turn by each player
    pub lands_played: Vec<(Entity, u32)>,

    /// Whether a main phase action has been taken (for "one per turn" effects)
    pub main_phase_action_taken: bool,

    /// Tracks which players have drawn from their library this turn
    pub drawn_this_turn: Vec<Entity>,

    /// Tracks if any state-based actions were performed in the last check
    pub state_based_actions_performed: bool,

    /// Tracks players who have been eliminated
    pub eliminated_players: Vec<Entity>,

    /// Commander specific rule - whether commander damage is tracked
    pub use_commander_damage: bool,

    /// Commander specific rule - commander damage threshold (typically 21)
    pub commander_damage_threshold: u32,

    /// Commander specific rule - starting life total (typically 40)
    pub starting_life: i32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            turn_number: 1,
            active_player: Entity::PLACEHOLDER,
            priority_holder: Entity::PLACEHOLDER,
            turn_order: VecDeque::new(),
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            state_based_actions_performed: false,
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        }
    }
}

impl GameState {
    /// Set the turn order of players
    pub fn set_turn_order(&mut self, players: Vec<Entity>) {
        self.turn_order = VecDeque::from(players);

        // Set the first player as active
        if let Some(&first_player) = self.turn_order.front() {
            self.active_player = first_player;
            self.priority_holder = first_player;
        }
    }

    /// Advance to the next player in turn order
    pub fn advance_active_player(&mut self) {
        if !self.turn_order.is_empty() {
            // Skip eliminated players
            let mut next_player = None;
            let mut rotations = 0;

            while next_player.is_none() && rotations < self.turn_order.len() {
                // Rotate the turn order so the next player becomes active
                let current_player = self.turn_order.pop_front().unwrap();
                self.turn_order.push_back(current_player);
                rotations += 1;

                if let Some(&potential_next) = self.turn_order.front() {
                    if !self.eliminated_players.contains(&potential_next) {
                        next_player = Some(potential_next);
                    }
                }
            }

            // Set the new active player if we found one
            if let Some(player) = next_player {
                self.active_player = player;
                self.priority_holder = player;
                self.turn_number += 1;
                self.reset_turn_tracking();
            }
        }
    }

    /// Reset per-turn state tracking
    pub fn reset_turn_tracking(&mut self) {
        // Clear lands played counters
        self.lands_played.clear();

        // Reset action flags
        self.main_phase_action_taken = false;

        // Reset drawn cards tracking
        self.drawn_this_turn.clear();
    }

    /// Record that a player played a land
    pub fn record_land_played(&mut self, player: Entity) {
        // Find this player in the lands_played list or add a new entry
        if let Some(entry) = self.lands_played.iter_mut().find(|(p, _)| *p == player) {
            entry.1 += 1;
        } else {
            self.lands_played.push((player, 1));
        }
    }

    /// Check if a player can play another land this turn
    pub fn can_play_land(&self, player: Entity) -> bool {
        // Default is one land per turn
        let max_lands = 1;

        // Find the player's entry in lands_played
        let lands_played = self
            .lands_played
            .iter()
            .find(|(p, _)| *p == player)
            .map(|(_, count)| *count)
            .unwrap_or(0);

        lands_played < max_lands
    }

    /// Record that a player was eliminated
    pub fn eliminate_player(&mut self, player: Entity, reason: EliminationReason) {
        if !self.eliminated_players.contains(&player) {
            self.eliminated_players.push(player);
            info!("Player {:?} eliminated due to {:?}", player, reason);
        }
    }

    /// Check if the game is over (only one player remains)
    pub fn is_game_over(&self) -> bool {
        self.turn_order.len() - self.eliminated_players.len() <= 1
    }

    /// Get the winner if the game is over
    pub fn get_winner(&self) -> Option<Entity> {
        if self.is_game_over() {
            self.turn_order
                .iter()
                .find(|&&player| !self.eliminated_players.contains(&player))
                .copied()
        } else {
            None
        }
    }

    /// Record a player drawing a card
    pub fn record_draw(&mut self, player: Entity) {
        if !self.drawn_this_turn.contains(&player) {
            self.drawn_this_turn.push(player);
        }
    }
}

/// System that handles state-based actions
pub fn state_based_actions_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    zone_manager: ResMut<ZoneManager>,
    player_query: Query<(Entity, &Player)>,
    creature_query: Query<(Entity, &CreatureOnField, Option<&Card>)>,
    commander_query: Query<(Entity, &Commander)>,
) {
    // Reset the state-based actions performed flag
    game_state.state_based_actions_performed = false;

    // 1. Check for players with 0 or less life
    for (entity, player) in player_query.iter() {
        if player.life <= 0 && !game_state.eliminated_players.contains(&entity) {
            // Eliminate player due to life loss
            game_state.eliminate_player(entity, EliminationReason::LifeLoss);

            // Spawn elimination event
            commands.spawn(PlayerEliminatedEvent {
                player: entity,
                reason: EliminationReason::LifeLoss,
            });

            game_state.state_based_actions_performed = true;
        }
    }

    // 2. Check for commander damage eliminations
    if game_state.use_commander_damage {
        for (entity, _player) in player_query.iter() {
            for (commander_entity, commander) in commander_query.iter() {
                // Check if this commander has dealt lethal damage to the player
                if let Some((_, damage)) = commander.damage_dealt.iter().find(|(p, _)| *p == entity)
                {
                    if *damage >= game_state.commander_damage_threshold
                        && !game_state.eliminated_players.contains(&entity)
                    {
                        // Eliminate player due to commander damage
                        game_state.eliminate_player(
                            entity,
                            EliminationReason::CommanderDamage(commander_entity),
                        );

                        // Spawn elimination event
                        commands.spawn(PlayerEliminatedEvent {
                            player: entity,
                            reason: EliminationReason::CommanderDamage(commander_entity),
                        });

                        game_state.state_based_actions_performed = true;
                    }
                }
            }
        }
    }

    // 3. Check for players that have tried to draw from an empty library
    for (entity, _player) in player_query.iter() {
        if game_state.drawn_this_turn.contains(&entity) {
            if let Some(library) = zone_manager.get_player_zone(entity, Zone::Library) {
                if library.is_empty() && !game_state.eliminated_players.contains(&entity) {
                    // Eliminate player due to empty library
                    game_state.eliminate_player(entity, EliminationReason::EmptyLibrary);

                    // Spawn elimination event
                    commands.spawn(PlayerEliminatedEvent {
                        player: entity,
                        reason: EliminationReason::EmptyLibrary,
                    });

                    game_state.state_based_actions_performed = true;
                }
            }
        }
    }

    // 4. Check for creatures with lethal damage or 0 or less toughness
    let mut dead_creatures = Vec::new();

    for (entity, creature_on_field, card_opt) in creature_query.iter() {
        if let Some(card) = card_opt {
            if let crate::card::CardDetails::Creature(creature_card) = &card.card_details {
                // Get the base power and toughness from the card
                let base_toughness = creature_card.toughness;

                // Apply modifiers from CreatureOnField
                let effective_toughness =
                    base_toughness + creature_on_field.toughness_modifier as i32;

                // Check for lethal damage
                if creature_on_field.battle_damage as i32 >= effective_toughness {
                    dead_creatures.push(entity);
                    game_state.state_based_actions_performed = true;
                }

                // Check for 0 or less toughness
                if effective_toughness <= 0 {
                    dead_creatures.push(entity);
                    game_state.state_based_actions_performed = true;
                }
            }
        }
    }

    // Process dead creatures
    for entity in dead_creatures {
        // Get the card owner
        if let Some(owner) = zone_manager.get_card_owner(entity) {
            // Move the creature to the graveyard
            commands.spawn(ZoneChangeEvent {
                card: entity,
                owner,
                source: Zone::Battlefield,
                destination: Zone::Graveyard,
                was_visible: true,
                is_visible: true,
            });
        }
    }

    // 5. Check if game is over and handle winner
    if game_state.is_game_over() {
        if let Some(winner) = game_state.get_winner() {
            info!("Game over! Player {:?} wins!", winner);
            // Here you could trigger a game-over event if needed
        }
    }

    // If any state-based actions were performed, we need to check again
    // before players receive priority. This is handled by the priority system.
}

/// Event that triggers when state-based actions need to be checked
#[derive(Event)]
pub struct CheckStateBasedActionsEvent;

/// System that triggers state-based action checks at appropriate times
pub fn trigger_state_based_actions_system(
    mut commands: Commands,
    stack_events: EventReader<crate::game_engine::stack::StackItemResolvedEvent>,
    zone_change_events: EventReader<ZoneChangeEvent>,
) {
    // Trigger state-based action checks whenever:
    // 1. A spell or ability has resolved
    // 2. A zone change has occurred
    // 3. A player would receive priority

    if !stack_events.is_empty() || !zone_change_events.is_empty() {
        commands.spawn(CheckStateBasedActionsEvent);
    }
}
