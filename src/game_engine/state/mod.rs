// Remove the self-reference import
// pub use crate::game_engine::state::*;

use crate::cards::Card;
use crate::cards::details::CreatureOnField;
use crate::game_engine::commander::{Commander, EliminationReason, PlayerEliminatedEvent};
use crate::game_engine::zones::{Zone, ZoneChangeEvent, ZoneManager};
use crate::player::Player;
use bevy::prelude::*;
use std::collections::VecDeque;

// Re-export save module's event
pub use crate::game_engine::save::CheckStateBasedActionsEvent;

/// The global game state for an MTG game
#[derive(Resource)]
pub struct GameState {
    /// The current turn number
    #[allow(dead_code)]
    pub turn_number: u32,

    /// The player whose turn it is
    pub active_player: Entity,

    /// The player currently with priority
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub starting_life: i32,
}

impl GameState {
    /// Creates a new GameStateBuilder to use a chainable API for constructing a GameState
    #[allow(dead_code)]
    pub fn builder() -> GameStateBuilder {
        GameStateBuilder::new()
    }

    /// Set the turn order for the game
    #[allow(dead_code)]
    pub fn set_turn_order(&mut self, players: Vec<Entity>) {
        self.turn_order = VecDeque::from(players);

        // Set active player to the first player
        if !self.turn_order.is_empty() {
            self.active_player = *self.turn_order.front().unwrap();
            self.priority_holder = self.active_player;
        }
    }

    /// Move to the next player in turn order
    #[allow(dead_code)]
    pub fn advance_active_player(&mut self) {
        if self.turn_order.is_empty() {
            return;
        }

        // Only advance if there are more than one player left
        if self.turn_order.len() > 1 {
            // Remove any eliminated players from turn order
            self.turn_order
                .retain(|p| !self.eliminated_players.contains(p));

            // If the active player is eliminated, set it to the next player
            if self.eliminated_players.contains(&self.active_player) {
                if let Some(&first) = self.turn_order.front() {
                    self.active_player = first;
                    self.priority_holder = first;
                    return;
                }
            }

            // Get active player from front and put at back
            let current = self.turn_order.pop_front().unwrap();
            self.turn_order.push_back(current);

            // Set new active player
            if let Some(&next) = self.turn_order.front() {
                self.active_player = next;
                self.priority_holder = next;
            }
        }

        // Increment turn number when we get back to the first player
        self.turn_number += 1;
    }

    /// Reset per-turn tracking data
    pub fn reset_turn_tracking(&mut self) {
        self.lands_played.clear();
        self.main_phase_action_taken = false;
        self.drawn_this_turn.clear();
        self.state_based_actions_performed = false;
    }

    /// Record that a player has played a land
    pub fn record_land_played(&mut self, player: Entity) {
        // Find the player's entry or add a new one
        if let Some(entry) = self.lands_played.iter_mut().find(|(p, _)| *p == player) {
            entry.1 += 1;
        } else {
            self.lands_played.push((player, 1));
        }
    }

    /// Check if a player can play a land
    pub fn can_play_land(&self, player: Entity) -> bool {
        // By default, each player can play one land per turn
        let max_lands = 1;

        // Check how many lands this player has played
        let lands_played = self
            .lands_played
            .iter()
            .find(|(p, _)| *p == player)
            .map(|(_, count)| *count)
            .unwrap_or(0);

        lands_played < max_lands
    }

    /// Eliminate a player from the game
    pub fn eliminate_player(&mut self, player: Entity, _reason: EliminationReason) {
        if !self.eliminated_players.contains(&player) {
            self.eliminated_players.push(player);
        }
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        self.turn_order.len() - self.eliminated_players.len() <= 1
    }

    /// Get the player index in the turn order
    #[allow(dead_code)]
    pub fn get_player_index(&self, player: Entity) -> Option<usize> {
        self.turn_order.iter().position(|p| *p == player)
    }

    /// Get the winner of the game
    pub fn get_winner(&self) -> Option<Entity> {
        if self.is_game_over() {
            // Find the first player who isn't eliminated
            for player in &self.turn_order {
                if !self.eliminated_players.contains(player) {
                    return Some(*player);
                }
            }
        }
        None
    }

    /// Record that a player has drawn a card this turn
    #[allow(dead_code)]
    pub fn record_draw(&mut self, player: Entity) {
        if !self.drawn_this_turn.contains(&player) {
            self.drawn_this_turn.push(player);
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameStateBuilder::new().build()
    }
}

/// Builder for GameState to enable chainable construction
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GameStateBuilder {
    turn_number: u32,
    active_player: Entity,
    priority_holder: Entity,
    turn_order: VecDeque<Entity>,
    lands_played: Vec<(Entity, u32)>,
    main_phase_action_taken: bool,
    drawn_this_turn: Vec<Entity>,
    state_based_actions_performed: bool,
    eliminated_players: Vec<Entity>,
    use_commander_damage: bool,
    commander_damage_threshold: u32,
    starting_life: i32,
}

impl GameStateBuilder {
    /// Creates a new GameStateBuilder with default values
    pub fn new() -> Self {
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

    /// Sets the current turn number
    /// TODO: Implement turn number tracking and management
    #[allow(dead_code)]
    pub fn turn_number(mut self, turn_number: u32) -> Self {
        self.turn_number = turn_number;
        self
    }

    /// Sets the active player
    /// TODO: Implement active player management
    #[allow(dead_code)]
    pub fn active_player(mut self, active_player: Entity) -> Self {
        self.active_player = active_player;
        self
    }

    /// Sets the priority holder
    /// TODO: Implement priority system
    #[allow(dead_code)]
    pub fn priority_holder(mut self, priority_holder: Entity) -> Self {
        self.priority_holder = priority_holder;
        self
    }

    /// Sets the turn order
    /// TODO: Implement turn order management
    #[allow(dead_code)]
    pub fn turn_order(mut self, turn_order: VecDeque<Entity>) -> Self {
        self.turn_order = turn_order;
        self
    }

    /// Sets the lands played
    /// TODO: Implement land play tracking
    #[allow(dead_code)]
    pub fn lands_played(mut self, lands_played: Vec<(Entity, u32)>) -> Self {
        self.lands_played = lands_played;
        self
    }

    /// Sets whether a main phase action has been taken
    /// TODO: Implement main phase action tracking
    #[allow(dead_code)]
    pub fn main_phase_action_taken(mut self, main_phase_action_taken: bool) -> Self {
        self.main_phase_action_taken = main_phase_action_taken;
        self
    }

    /// Sets the players who have drawn this turn
    /// TODO: Implement card draw tracking
    #[allow(dead_code)]
    pub fn drawn_this_turn(mut self, drawn_this_turn: Vec<Entity>) -> Self {
        self.drawn_this_turn = drawn_this_turn;
        self
    }

    /// Sets whether state-based actions were performed
    /// TODO: Implement state-based actions system
    #[allow(dead_code)]
    pub fn state_based_actions_performed(mut self, state_based_actions_performed: bool) -> Self {
        self.state_based_actions_performed = state_based_actions_performed;
        self
    }

    /// Sets the eliminated players
    /// TODO: Implement player elimination system
    #[allow(dead_code)]
    pub fn eliminated_players(mut self, eliminated_players: Vec<Entity>) -> Self {
        self.eliminated_players = eliminated_players;
        self
    }

    /// Sets whether commander damage is tracked
    /// TODO: Implement commander damage tracking
    #[allow(dead_code)]
    pub fn use_commander_damage(mut self, use_commander_damage: bool) -> Self {
        self.use_commander_damage = use_commander_damage;
        self
    }

    /// Sets the commander damage threshold
    /// TODO: Implement commander damage threshold system
    #[allow(dead_code)]
    pub fn commander_damage_threshold(mut self, commander_damage_threshold: u32) -> Self {
        self.commander_damage_threshold = commander_damage_threshold;
        self
    }

    /// Sets the starting life total
    /// TODO: Implement starting life total configuration
    #[allow(dead_code)]
    pub fn starting_life(mut self, starting_life: i32) -> Self {
        self.starting_life = starting_life;
        self
    }

    /// Builds the GameState instance
    pub fn build(self) -> GameState {
        GameState {
            turn_number: self.turn_number,
            active_player: self.active_player,
            priority_holder: self.priority_holder,
            turn_order: self.turn_order,
            lands_played: self.lands_played,
            main_phase_action_taken: self.main_phase_action_taken,
            drawn_this_turn: self.drawn_this_turn,
            state_based_actions_performed: self.state_based_actions_performed,
            eliminated_players: self.eliminated_players,
            use_commander_damage: self.use_commander_damage,
            commander_damage_threshold: self.commander_damage_threshold,
            starting_life: self.starting_life,
        }
    }
}

/// System that checks for state-based actions
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

    // 1. Check for players at 0 or less life
    for (player_entity, player) in player_query.iter() {
        if player.life <= 0 && !game_state.eliminated_players.contains(&player_entity) {
            info!(
                "Player {:?} eliminated due to 0 or less life",
                player_entity
            );
            game_state.eliminate_player(player_entity, EliminationReason::LifeLoss);
            game_state.state_based_actions_performed = true;

            commands.send_event(PlayerEliminatedEvent {
                player: player_entity,
                reason: EliminationReason::LifeLoss,
            });
        }
    }

    // 2. Check for players who have attempted to draw from an empty library
    // This would be handled by a separate drawing system that triggers elimination

    // 3. Check for creature state-based actions
    for (creature_entity, creature_field, _card) in creature_query.iter() {
        // Check for creatures with damage >= toughness
        let battle_damage_i64 = creature_field.battle_damage as i64;
        if battle_damage_i64 >= creature_field.toughness_modifier {
            if let Some(owner) = zone_manager.get_card_owner(creature_entity) {
                info!(
                    "Creature {:?} destroyed due to lethal damage",
                    creature_entity
                );

                // Move the creature from battlefield to graveyard
                commands.send_event(ZoneChangeEvent {
                    card: creature_entity,
                    owner,
                    source: Zone::Battlefield,
                    destination: Zone::Graveyard,
                    was_visible: true,
                    is_visible: true,
                });

                game_state.state_based_actions_performed = true;
            }
        }

        // Check for creatures with 0 or less toughness
        if creature_field.toughness_modifier <= 0 {
            if let Some(owner) = zone_manager.get_card_owner(creature_entity) {
                info!(
                    "Creature {:?} destroyed due to 0 or less toughness",
                    creature_entity
                );

                // Move the creature from battlefield to graveyard
                commands.send_event(ZoneChangeEvent {
                    card: creature_entity,
                    owner,
                    source: Zone::Battlefield,
                    destination: Zone::Graveyard,
                    was_visible: true,
                    is_visible: true,
                });

                game_state.state_based_actions_performed = true;
            }
        }
    }

    // 4. Check for commander damage threshold
    if game_state.use_commander_damage {
        for (commander_entity, commander) in commander_query.iter() {
            for (player_entity, _) in player_query.iter() {
                // Skip if the player is the controller of this commander
                if player_entity == commander.owner {
                    continue;
                }

                // Get total commander damage from this commander
                let commander_damage = commander
                    .damage_dealt
                    .iter()
                    .find_map(|(target, damage)| {
                        if *target == player_entity {
                            Some(*damage)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(0);

                // Check if it exceeds the threshold
                if commander_damage >= game_state.commander_damage_threshold
                    && !game_state.eliminated_players.contains(&player_entity)
                {
                    info!(
                        "Player {:?} eliminated due to commander damage from {:?}",
                        player_entity, commander_entity
                    );
                    game_state.eliminate_player(
                        player_entity,
                        EliminationReason::CommanderDamage(commander_entity),
                    );
                    game_state.state_based_actions_performed = true;

                    commands.send_event(PlayerEliminatedEvent {
                        player: player_entity,
                        reason: EliminationReason::CommanderDamage(commander_entity),
                    });
                }
            }
        }
    }

    // 5. Check if the game is over
    if game_state.is_game_over() {
        if let Some(winner) = game_state.get_winner() {
            info!("Game over! Player {:?} wins!", winner);
            // Additional handling for game over could go here
        }
    }
}

/// System that triggers state-based action checks when needed
pub fn trigger_state_based_actions_system(
    mut commands: Commands,
    stack_events: EventReader<crate::game_engine::stack::StackItemResolvedEvent>,
    zone_change_events: EventReader<ZoneChangeEvent>,
) {
    // Trigger after stack items resolve
    let stack_resolved = !stack_events.is_empty();

    // Trigger after zone changes (cards changing zones)
    let zone_changed = !zone_change_events.is_empty();

    // Trigger SBA if any of these events occurred
    if stack_resolved || zone_changed {
        commands.send_event(CheckStateBasedActionsEvent);
    }
}
