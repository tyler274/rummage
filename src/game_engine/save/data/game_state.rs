use serde::{Deserialize, Serialize};

/// Serializable game state data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateData {
    pub turn_number: u32,
    pub active_player_index: usize,
    pub priority_holder_index: usize,
    pub turn_order_indices: Vec<usize>,
    pub lands_played: Vec<(usize, u32)>,
    pub main_phase_action_taken: bool,
    pub drawn_this_turn: Vec<usize>,
    pub eliminated_players: Vec<usize>,
    pub use_commander_damage: bool,
    pub commander_damage_threshold: u32,
    pub starting_life: i32,
}

impl Default for GameStateData {
    fn default() -> Self {
        Self {
            turn_number: 1,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(),
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        }
    }
}

/// Builder for GameStateData
#[derive(Default)]
pub struct GameStateDataBuilder {
    turn_number: u32,
    active_player_index: usize,
    priority_holder_index: usize,
    turn_order_indices: Vec<usize>,
    lands_played: Vec<(usize, u32)>,
    main_phase_action_taken: bool,
    drawn_this_turn: Vec<usize>,
    eliminated_players: Vec<usize>,
    use_commander_damage: bool,
    commander_damage_threshold: u32,
    starting_life: i32,
}

impl GameStateDataBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            turn_number: 1,
            active_player_index: 0,
            priority_holder_index: 0,
            turn_order_indices: Vec::new(),
            lands_played: Vec::new(),
            main_phase_action_taken: false,
            drawn_this_turn: Vec::new(),
            eliminated_players: Vec::new(),
            use_commander_damage: true,
            commander_damage_threshold: 21,
            starting_life: 40,
        }
    }

    /// Set the turn number
    pub fn turn_number(mut self, turn_number: u32) -> Self {
        self.turn_number = turn_number;
        self
    }

    /// Set the active player index
    pub fn active_player_index(mut self, active_player_index: usize) -> Self {
        self.active_player_index = active_player_index;
        self
    }

    /// Set the priority holder index
    pub fn priority_holder_index(mut self, priority_holder_index: usize) -> Self {
        self.priority_holder_index = priority_holder_index;
        self
    }

    /// Set the turn order indices
    pub fn turn_order_indices(mut self, turn_order_indices: Vec<usize>) -> Self {
        self.turn_order_indices = turn_order_indices;
        self
    }

    /// Set the lands played
    pub fn lands_played(mut self, lands_played: Vec<(usize, u32)>) -> Self {
        self.lands_played = lands_played;
        self
    }

    /// Set the main phase action taken flag
    pub fn main_phase_action_taken(mut self, main_phase_action_taken: bool) -> Self {
        self.main_phase_action_taken = main_phase_action_taken;
        self
    }

    /// Set the drawn this turn list
    pub fn drawn_this_turn(mut self, drawn_this_turn: Vec<usize>) -> Self {
        self.drawn_this_turn = drawn_this_turn;
        self
    }

    /// Set the eliminated players list
    pub fn eliminated_players(mut self, eliminated_players: Vec<usize>) -> Self {
        self.eliminated_players = eliminated_players;
        self
    }

    /// Set whether to use commander damage
    pub fn use_commander_damage(mut self, use_commander_damage: bool) -> Self {
        self.use_commander_damage = use_commander_damage;
        self
    }

    /// Set the commander damage threshold
    pub fn commander_damage_threshold(mut self, commander_damage_threshold: u32) -> Self {
        self.commander_damage_threshold = commander_damage_threshold;
        self
    }

    /// Set the starting life
    pub fn starting_life(mut self, starting_life: i32) -> Self {
        self.starting_life = starting_life;
        self
    }

    /// Build the GameStateData instance
    pub fn build(self) -> GameStateData {
        GameStateData {
            turn_number: self.turn_number,
            active_player_index: self.active_player_index,
            priority_holder_index: self.priority_holder_index,
            turn_order_indices: self.turn_order_indices,
            lands_played: self.lands_played,
            main_phase_action_taken: self.main_phase_action_taken,
            drawn_this_turn: self.drawn_this_turn,
            eliminated_players: self.eliminated_players,
            use_commander_damage: self.use_commander_damage,
            commander_damage_threshold: self.commander_damage_threshold,
            starting_life: self.starting_life,
        }
    }
}

impl GameStateData {
    /// Create a new builder for GameStateData
    pub fn builder() -> GameStateDataBuilder {
        GameStateDataBuilder::new()
    }
}
