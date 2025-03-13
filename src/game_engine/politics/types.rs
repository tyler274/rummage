use bevy::prelude::*;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Structure representing a vote in progress
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Vote {
    /// Unique identifier for this vote
    pub id: Uuid,

    /// Title/description of the vote
    pub title: String,

    /// The card or effect that created this vote
    pub source: Entity,

    /// The player who controls the voting effect
    pub controller: Entity,

    /// Available choices for the vote
    pub choices: Vec<VoteChoice>,

    /// Players eligible to participate in the vote
    pub eligible_voters: Vec<Entity>,

    /// Whether all players must vote
    pub requires_all_players: bool,

    /// Optional timer for voting
    pub timer: Option<Duration>,

    /// When the vote was created
    pub created_at: Instant,
}

impl Vote {
    /// Creates a new VoteBuilder for chainable construction
    /// TODO: Implement voting mechanics in multiplayer games
    #[allow(dead_code)]
    pub fn builder(title: &str, controller: Entity, source: Entity) -> VoteBuilder {
        VoteBuilder::new(title, controller, source)
    }
}

/// Builder for Vote with a chainable API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VoteBuilder {
    id: Uuid,
    title: String,
    source: Entity,
    controller: Entity,
    choices: Vec<VoteChoice>,
    eligible_voters: Vec<Entity>,
    requires_all_players: bool,
    timer: Option<Duration>,
    created_at: Instant,
}

impl VoteBuilder {
    /// Creates a new VoteBuilder with required values
    #[allow(dead_code)]
    pub fn new(title: &str, controller: Entity, source: Entity) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.to_string(),
            source,
            controller,
            choices: Vec::new(),
            eligible_voters: Vec::new(),
            requires_all_players: false,
            timer: None,
            created_at: Instant::now(),
        }
    }

    /// Sets a custom UUID (rarely needed)
    #[allow(dead_code)]
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    /// Adds a choice to the vote
    #[allow(dead_code)]
    pub fn add_choice(mut self, choice: VoteChoice) -> Self {
        self.choices.push(choice);
        self
    }

    /// Sets all the vote choices at once
    #[allow(dead_code)]
    pub fn choices(mut self, choices: Vec<VoteChoice>) -> Self {
        self.choices = choices;
        self
    }

    /// Adds a voter to the eligible voters list
    #[allow(dead_code)]
    pub fn add_voter(mut self, voter: Entity) -> Self {
        self.eligible_voters.push(voter);
        self
    }

    /// Sets all eligible voters at once
    #[allow(dead_code)]
    pub fn eligible_voters(mut self, voters: Vec<Entity>) -> Self {
        self.eligible_voters = voters;
        self
    }

    /// Sets whether all players must vote
    #[allow(dead_code)]
    pub fn requires_all_players(mut self, requires: bool) -> Self {
        self.requires_all_players = requires;
        self
    }

    /// Sets a timer for the vote
    #[allow(dead_code)]
    pub fn timer(mut self, duration: Duration) -> Self {
        self.timer = Some(duration);
        self
    }

    /// Sets a custom creation time (rarely needed)
    #[allow(dead_code)]
    pub fn created_at(mut self, time: Instant) -> Self {
        self.created_at = time;
        self
    }

    /// Builds the Vote instance
    #[allow(dead_code)]
    pub fn build(self) -> Vote {
        Vote {
            id: self.id,
            title: self.title,
            source: self.source,
            controller: self.controller,
            choices: self.choices,
            eligible_voters: self.eligible_voters,
            requires_all_players: self.requires_all_players,
            timer: self.timer,
            created_at: self.created_at,
        }
    }
}

/// A choice in a vote
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct VoteChoice {
    /// Identifier for this choice
    pub id: usize,

    /// Text description of the choice
    pub text: String,

    /// Optional target entity for this choice
    pub target: Option<Entity>,
}

impl VoteChoice {
    /// Creates a new VoteChoice with the given text
    #[allow(dead_code)]
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            target: None,
        }
    }

    /// Creates a new VoteChoiceBuilder for chainable construction
    #[allow(dead_code)]
    pub fn builder(id: usize, text: &str) -> VoteChoiceBuilder {
        VoteChoiceBuilder::new(id, text)
    }
}

/// Builder for VoteChoice with a chainable API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VoteChoiceBuilder {
    id: usize,
    text: String,
    target: Option<Entity>,
}

impl VoteChoiceBuilder {
    /// Creates a new VoteChoiceBuilder with required values
    #[allow(dead_code)]
    pub fn new(id: usize, text: &str) -> Self {
        Self {
            id,
            text: text.to_string(),
            target: None,
        }
    }

    /// Sets the target entity for this choice
    #[allow(dead_code)]
    pub fn target(mut self, target: Entity) -> Self {
        self.target = Some(target);
        self
    }

    /// Builds the VoteChoice instance
    #[allow(dead_code)]
    pub fn build(self) -> VoteChoice {
        VoteChoice {
            id: self.id,
            text: self.text,
            target: self.target,
        }
    }
}

/// Structure representing a deal between players
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Deal {
    /// Unique identifier for this deal
    pub id: Uuid,

    /// Player who proposed the deal
    pub proposer: Entity,

    /// Player who received the deal proposal
    pub target: Entity,

    /// Terms of the deal
    pub terms: Vec<DealTerm>,

    /// How long the deal remains in effect
    pub duration: DealDuration,

    /// Current status of the deal
    pub status: DealStatus,

    /// When the deal was created
    pub created_at: Instant,
}

impl Deal {
    /// Creates a new DealBuilder for chainable construction
    #[allow(dead_code)]
    pub fn builder(proposer: Entity, target: Entity) -> DealBuilder {
        DealBuilder::new(proposer, target)
    }
}

/// Builder for Deal with a chainable API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DealBuilder {
    id: Uuid,
    proposer: Entity,
    target: Entity,
    terms: Vec<DealTerm>,
    duration: DealDuration,
    status: DealStatus,
    created_at: Instant,
}

impl DealBuilder {
    /// Creates a new DealBuilder with required values
    #[allow(dead_code)]
    pub fn new(proposer: Entity, target: Entity) -> Self {
        Self {
            id: Uuid::new_v4(),
            proposer,
            target,
            terms: Vec::new(),
            duration: DealDuration::Turns(1), // Default to 1 turn
            status: DealStatus::Proposed,
            created_at: Instant::now(),
        }
    }

    /// Sets a custom UUID (rarely needed)
    #[allow(dead_code)]
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    /// Adds a term to the deal
    #[allow(dead_code)]
    pub fn add_term(mut self, term: DealTerm) -> Self {
        self.terms.push(term);
        self
    }

    /// Sets all deal terms at once
    #[allow(dead_code)]
    pub fn terms(mut self, terms: Vec<DealTerm>) -> Self {
        self.terms = terms;
        self
    }

    /// Sets the duration of the deal
    #[allow(dead_code)]
    pub fn duration(mut self, duration: DealDuration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the status of the deal
    #[allow(dead_code)]
    pub fn status(mut self, status: DealStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets a custom creation time (rarely needed)
    #[allow(dead_code)]
    pub fn created_at(mut self, time: Instant) -> Self {
        self.created_at = time;
        self
    }

    /// Builds the Deal instance
    #[allow(dead_code)]
    pub fn build(self) -> Deal {
        Deal {
            id: self.id,
            proposer: self.proposer,
            target: self.target,
            terms: self.terms,
            duration: self.duration,
            status: self.status,
            created_at: self.created_at,
        }
    }
}

/// Possible terms in a deal between players
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DealTerm {
    /// Agreement not to attack for N turns
    DoNotAttack(u32),

    /// No harmful actions for N turns
    Truce(u32),

    /// Agreement to defend against a specific player
    SharedDefense { against: Entity, turns: u32 },

    /// Agreement to target a specific player
    TargetOtherPlayer { target: Entity, turns: u32 },

    /// Permission to take a specific action
    AllowAction { action_type: ActionType, turns: u32 },

    /// Custom deal term
    Custom(String),
}

/// Types of actions that can be allowed in a deal
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ActionType {
    Attack,
    CastSpell,
    ActivateAbility,
    DestroyPermanent,
    Other,
}

/// Current status of a deal
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DealStatus {
    /// Deal has been proposed but not yet accepted/rejected
    Proposed,

    /// Deal has been accepted and is active
    Accepted,

    /// Deal has been rejected
    Rejected,

    /// Deal was broken by a player
    Broken(Entity),

    /// Deal has expired
    Expired,
}

/// Duration of a deal
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DealDuration {
    /// Lasts for a specific number of turns
    Turns(u32),

    /// Lasts until the end of the game
    UntilEndOfGame,

    /// Lasts until a specific player is eliminated
    UntilPlayerEliminated(Entity),

    /// Custom duration
    Custom(String),
}

/// Structure representing a goad effect
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GoadEffect {
    /// The creature that is goaded
    pub target: Entity,

    /// The player who goaded it
    pub source: Entity,

    /// How many turns it lasts
    pub duration: u32,

    /// Turn number when created
    pub created_at: u32,
}

impl GoadEffect {
    /// Creates a new GoadEffect builder
    /// TODO: Implement goad mechanics in combat phase
    #[allow(dead_code)]
    pub fn builder(target: Entity, source: Entity) -> GoadEffectBuilder {
        GoadEffectBuilder::new(target, source)
    }
}

/// Builder for GoadEffect with a chainable API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GoadEffectBuilder {
    target: Entity,
    source: Entity,
    duration: u32,
    created_at: u32,
}

impl GoadEffectBuilder {
    /// Creates a new GoadEffectBuilder with required fields
    #[allow(dead_code)]
    pub fn new(target: Entity, source: Entity) -> Self {
        Self {
            target,
            source,
            duration: 1,   // Default duration is 1 turn
            created_at: 0, // Default turn number
        }
    }

    /// Sets the duration of the goad effect
    #[allow(dead_code)]
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the turn number when the effect was created
    #[allow(dead_code)]
    pub fn created_at(mut self, turn_number: u32) -> Self {
        self.created_at = turn_number;
        self
    }

    /// Builds the GoadEffect instance
    #[allow(dead_code)]
    pub fn build(self) -> GoadEffect {
        GoadEffect {
            target: self.target,
            source: self.source,
            duration: self.duration,
            created_at: self.created_at,
        }
    }
}

/// Structure representing a vow effect
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VowEffect {
    /// The creature with the vow
    pub target: Entity,

    /// The player who is protected by the vow
    pub protected_player: Entity,

    /// How many turns it lasts (None = permanent)
    pub duration: Option<u32>,

    /// Turn number when created
    pub created_at: u32,
}

impl VowEffect {
    /// Creates a new VowEffect builder
    /// TODO: Implement vow mechanics in combat phase
    #[allow(dead_code)]
    pub fn builder(target: Entity, protected_player: Entity) -> VowEffectBuilder {
        VowEffectBuilder::new(target, protected_player)
    }
}

/// Builder for VowEffect with a chainable API
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VowEffectBuilder {
    target: Entity,
    protected_player: Entity,
    duration: Option<u32>,
    created_at: u32,
}

impl VowEffectBuilder {
    /// Creates a new VowEffectBuilder with required fields
    #[allow(dead_code)]
    pub fn new(target: Entity, protected_player: Entity) -> Self {
        Self {
            target,
            protected_player,
            duration: None,
            created_at: 0,
        }
    }

    /// Sets how many turns the vow lasts (None = permanent)
    #[allow(dead_code)]
    pub fn duration(mut self, duration: Option<u32>) -> Self {
        self.duration = duration;
        self
    }

    /// Sets when the vow was created
    #[allow(dead_code)]
    pub fn created_at(mut self, turn_number: u32) -> Self {
        self.created_at = turn_number;
        self
    }

    /// Builds the VowEffect instance
    #[allow(dead_code)]
    pub fn build(self) -> VowEffect {
        VowEffect {
            target: self.target,
            protected_player: self.protected_player,
            duration: self.duration,
            created_at: self.created_at,
        }
    }
}

/// Combat restriction created by political effects
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatRestriction {
    /// Creature must attack if able
    MustAttack,

    /// Creature must attack a specific player if able
    #[allow(dead_code)]
    MustAttackPlayer(Entity),

    /// Creature cannot attack a specific player
    CannotAttackPlayer(Entity),

    /// Creature cannot block
    #[allow(dead_code)]
    CannotBlock,

    /// Creature cannot block attacks against a specific player
    #[allow(dead_code)]
    CannotBlockAttacksAgainst(Entity),
}
