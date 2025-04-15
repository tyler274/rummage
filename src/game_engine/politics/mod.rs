mod combat_restrictions;
mod deals;
mod goad;
mod monarch;
pub mod types;
mod voting;

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::game_engine::game_state_condition;

pub use deals::*;
pub use monarch::*;
pub use types::*;
pub use voting::*;

// Make these modules public and explicitly re-export their events
pub use combat_restrictions::{
    ApplyCombatRestrictionEvent, RemoveCombatRestrictionEvent, combat_restrictions_system,
    manage_combat_restrictions,
};
pub use goad::{GoadEvent, goad_system};

/// Resource that manages multiplayer politics in Commander games
#[derive(Resource, Default)]
pub struct PoliticsSystem {
    /// The current monarch player (if any)
    pub monarch: Option<Entity>,

    /// The player who currently has the initiative
    #[allow(dead_code)]
    pub initiative_holder: Option<Entity>,

    /// Tracks goad effects on creatures
    pub goad_effects: HashMap<Entity, Vec<GoadEffect>>,

    /// Tracks vow effects on creatures
    #[allow(dead_code)]
    pub vow_effects: HashMap<Entity, Vec<VowEffect>>,

    /// The currently active vote (if any)
    pub active_vote: Option<Vote>,

    /// Tracks votes cast by each player
    pub votes_cast: HashMap<Entity, VoteChoice>,

    /// Tracks vote weights for each player
    pub vote_weights: HashMap<Entity, u32>,

    /// Deals that have been proposed but not yet accepted/rejected
    pub pending_deals: Vec<Deal>,

    /// Deals that are currently active
    pub active_deals: Vec<Deal>,

    /// Tracks alliances between players (for team play variants)
    #[allow(dead_code)]
    pub alliances: HashMap<Entity, Vec<Entity>>,

    /// Tracks combat restrictions from political effects
    pub combat_restrictions: HashMap<Entity, Vec<CombatRestriction>>,
}

/// Event for when a player becomes the monarch
#[derive(Event)]
pub struct MonarchChangedEvent {
    /// The new monarch
    pub new_monarch: Entity,

    /// The previous monarch (if any)
    pub previous_monarch: Option<Entity>,

    /// The source of the monarchy change
    pub source: Option<Entity>,
}

/// Event for when a player starts a vote
#[derive(Event)]
pub struct VoteStartedEvent {
    /// The vote being started
    pub vote: Vote,
}

/// Event for when a player casts a vote
#[derive(Event)]
pub struct VoteCastEvent {
    /// The vote ID
    pub vote_id: Uuid,

    /// The player casting the vote
    pub player: Entity,

    /// The chosen option
    pub choice: VoteChoice,
}

/// Event for when a vote is completed
#[derive(Event)]
pub struct VoteCompletedEvent {
    /// The vote ID
    #[allow(dead_code)]
    pub vote_id: Uuid,

    /// The winning choice
    #[allow(dead_code)]
    pub winning_choice: VoteChoice,

    /// The number of votes for the winning choice
    #[allow(dead_code)]
    pub vote_count: u32,
}

/// Event for when a deal is proposed
#[derive(Event)]
pub struct DealProposedEvent {
    /// The deal being proposed
    pub deal: Deal,
}

/// Event for when a deal is accepted/rejected
#[derive(Event)]
pub struct DealResponseEvent {
    /// The deal ID
    pub deal_id: Uuid,

    /// Whether the deal was accepted
    pub accepted: bool,

    /// The player who responded
    pub responder: Entity,
}

/// Event for when a deal is broken
#[derive(Event)]
pub struct DealBrokenEvent {
    /// The deal ID
    pub deal_id: Uuid,

    /// The player who broke the deal
    pub breaker: Entity,

    /// Reason the deal was broken
    pub reason: String,
}

/// Register politics-related systems
pub fn register_politics_systems(app: &mut App) {
    app.insert_resource(PoliticsSystem::default())
        .add_event::<MonarchChangedEvent>()
        .add_event::<VoteStartedEvent>()
        .add_event::<VoteCastEvent>()
        .add_event::<VoteCompletedEvent>()
        .add_event::<DealProposedEvent>()
        .add_event::<DealResponseEvent>()
        .add_event::<DealBrokenEvent>()
        .add_systems(
            Update,
            (
                monarch_system,
                voting_system,
                goad_system,
                deal_system,
                combat_restrictions_system,
                manage_combat_restrictions,
            )
                .run_if(game_state_condition),
        );
}
