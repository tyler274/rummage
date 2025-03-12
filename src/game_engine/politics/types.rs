use bevy::prelude::*;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Structure representing a vote in progress
#[derive(Debug, Clone)]
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

/// A choice in a vote
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VoteChoice {
    /// Identifier for this choice
    pub id: usize,

    /// Text description of the choice
    pub text: String,

    /// Optional target entity for this choice
    pub target: Option<Entity>,
}

/// Structure representing a deal between players
#[derive(Debug, Clone)]
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

/// Possible terms in a deal between players
#[derive(Debug, Clone, PartialEq, Eq)]
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
pub enum ActionType {
    Attack,
    CastSpell,
    ActivateAbility,
    DestroyPermanent,
    Other,
}

/// Current status of a deal
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Structure representing a vow effect
#[derive(Debug, Clone)]
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

/// Combat restriction created by political effects
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CombatRestriction {
    /// Creature must attack if able
    MustAttack,

    /// Creature must attack a specific player if able
    MustAttackPlayer(Entity),

    /// Creature cannot attack a specific player
    CannotAttackPlayer(Entity),

    /// Creature cannot block
    CannotBlock,

    /// Creature cannot block attacks against a specific player
    CannotBlockAttacksAgainst(Entity),
}
