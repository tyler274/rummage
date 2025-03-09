use crate::game_engine::GameState;
use crate::game_engine::combat::CombatState;
use crate::game_engine::turns::TurnManager;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Resource that manages multiplayer politics in Commander games
#[derive(Resource)]
pub struct PoliticsSystem {
    /// The current monarch player (if any)
    pub monarch: Option<Entity>,

    /// The player who currently has the initiative
    pub initiative_holder: Option<Entity>,

    /// Tracks goad effects on creatures
    pub goad_effects: HashMap<Entity, Vec<GoadEffect>>,

    /// Tracks vow effects on creatures
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
    pub alliances: HashMap<Entity, Vec<Entity>>,

    /// Tracks combat restrictions from political effects
    pub combat_restrictions: HashMap<Entity, Vec<CombatRestriction>>,
}

impl Default for PoliticsSystem {
    fn default() -> Self {
        Self {
            monarch: None,
            initiative_holder: None,
            goad_effects: HashMap::new(),
            vow_effects: HashMap::new(),
            active_vote: None,
            votes_cast: HashMap::new(),
            vote_weights: HashMap::new(),
            pending_deals: Vec::new(),
            active_deals: Vec::new(),
            alliances: HashMap::new(),
            combat_restrictions: HashMap::new(),
        }
    }
}

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
    pub vote_id: Uuid,

    /// The winning choice
    pub winning_choice: VoteChoice,

    /// The number of votes for the winning choice
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

/// System to handle the monarch mechanic
pub fn monarch_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut monarch_events: EventReader<MonarchChangedEvent>,
    game_state: Res<GameState>,
    turn_manager: Res<TurnManager>,
) {
    // Process monarch change events
    for event in monarch_events.read() {
        politics.monarch = Some(event.new_monarch);

        // TODO: Handle trigger effects when monarch changes
    }

    // At the end of a monarch's turn, they draw a card
    if let Some(monarch) = politics.monarch {
        if monarch == turn_manager.active_player {
            // TODO: Trigger card draw effect at end of turn
        }
    }
}

/// System to handle voting
pub fn voting_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut vote_started_events: EventReader<VoteStartedEvent>,
    mut vote_cast_events: EventReader<VoteCastEvent>,
    player_query: Query<Entity, With<Player>>,
) {
    // Start new votes
    for event in vote_started_events.read() {
        // Only one vote at a time
        if politics.active_vote.is_some() {
            continue;
        }

        politics.active_vote = Some(event.vote.clone());
        politics.votes_cast.clear();

        // Initialize vote weights (default to 1)
        for player in player_query.iter() {
            politics.vote_weights.insert(player, 1);
        }
    }

    // Process new votes
    for event in vote_cast_events.read() {
        // Ignore votes for inactive/different votes
        if let Some(active_vote) = &politics.active_vote {
            if active_vote.id != event.vote_id {
                continue;
            }

            // Record the vote
            politics
                .votes_cast
                .insert(event.player, event.choice.clone());

            // Check if voting is complete
            if politics.votes_cast.len() >= active_vote.eligible_voters.len()
                || (!active_vote.requires_all_players && politics.is_vote_decisive())
            {
                // Determine the winner
                if let Some(winning_choice) = politics.tally_votes() {
                    // Send completion event
                    commands.spawn(VoteCompletedEvent {
                        vote_id: active_vote.id,
                        winning_choice: winning_choice.0,
                        vote_count: winning_choice.1,
                    });

                    // Clear the active vote
                    politics.active_vote = None;
                    politics.votes_cast.clear();
                }
            }
        }
    }

    // Check for vote timeout
    if let Some(vote) = &politics.active_vote {
        if let Some(timer) = vote.timer {
            let elapsed = Instant::now().duration_since(vote.created_at);
            if elapsed > timer {
                // Vote timed out, tally what we have
                if let Some(winning_choice) = politics.tally_votes() {
                    commands.spawn(VoteCompletedEvent {
                        vote_id: vote.id,
                        winning_choice: winning_choice.0,
                        vote_count: winning_choice.1,
                    });
                }

                politics.active_vote = None;
                politics.votes_cast.clear();
            }
        }
    }
}

impl PoliticsSystem {
    /// Check if a vote is decisive (has a clear winner that cannot be changed)
    fn is_vote_decisive(&self) -> bool {
        if let Some(vote) = &self.active_vote {
            // Count current votes
            let mut vote_counts: HashMap<&VoteChoice, u32> = HashMap::new();

            for (player, choice) in &self.votes_cast {
                let weight = self.vote_weights.get(player).cloned().unwrap_or(1);
                *vote_counts.entry(choice).or_insert(0) += weight;
            }

            // Count remaining potential votes
            let remaining_voters: Vec<_> = vote
                .eligible_voters
                .iter()
                .filter(|p| !self.votes_cast.contains_key(p))
                .collect();

            let remaining_vote_power: u32 = remaining_voters
                .iter()
                .map(|p| self.vote_weights.get(p).cloned().unwrap_or(1))
                .sum();

            // Check if leading choice cannot be overtaken
            if let Some((leading_choice, leading_count)) =
                vote_counts.iter().max_by_key(|(_, count)| *count)
            {
                // Find second highest
                let second_highest = vote_counts
                    .iter()
                    .filter(|(c, _)| *c != leading_choice)
                    .map(|(_, count)| *count)
                    .max()
                    .unwrap_or(0);

                // If lead is greater than possible remaining votes + second highest, it's decisive
                if *leading_count > second_highest + remaining_vote_power {
                    return true;
                }
            }
        }

        false
    }

    /// Count votes and determine the winner
    fn tally_votes(&self) -> Option<(VoteChoice, u32)> {
        if self.votes_cast.is_empty() {
            return None;
        }

        // Count votes with weighting
        let mut vote_counts: HashMap<&VoteChoice, u32> = HashMap::new();

        for (player, choice) in &self.votes_cast {
            let weight = self.vote_weights.get(player).cloned().unwrap_or(1);
            *vote_counts.entry(choice).or_insert(0) += weight;
        }

        // Find the winner (max by count)
        vote_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(choice, count)| ((*choice).clone(), *count))
    }
}

/// System to handle goad effects
pub fn goad_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut combat_state: ResMut<CombatState>,
    turn_manager: Res<TurnManager>,
) {
    // Clear expired goad effects
    politics.goad_effects.retain(|_, effects| {
        effects.retain(|effect| effect.created_at + effect.duration > turn_manager.turn_number);
        !effects.is_empty()
    });

    // Apply goad effects to combat restrictions
    for (creature, effects) in &politics.goad_effects {
        if !effects.is_empty() {
            // Get the active player
            let active_player = turn_manager.active_player;

            // A goaded creature must attack if able
            combat_state
                .must_attack
                .entry(*creature)
                .or_insert_with(Vec::new);

            // A goaded creature must attack someone other than the player who goaded it
            for effect in effects {
                // Only restrict creatures owned by the active player
                // (in a real implementation we'd check creature controller)

                // Add combat restriction: cannot attack the source of the goad
                if let Some(restrictions) = combat_state.cannot_attack.get_mut(creature) {
                    if !restrictions.contains(&effect.source) {
                        restrictions.push(effect.source);
                    }
                } else {
                    combat_state
                        .cannot_attack
                        .insert(*creature, vec![effect.source]);
                }
            }
        }
    }
}

/// System to handle deals between players
pub fn deal_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut deal_proposed_events: EventReader<DealProposedEvent>,
    mut deal_response_events: EventReader<DealResponseEvent>,
    mut deal_broken_events: EventReader<DealBrokenEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Process new deal proposals
    for event in deal_proposed_events.read() {
        politics.pending_deals.push(event.deal.clone());
    }

    // Process deal responses
    for event in deal_response_events.read() {
        let deal_index = politics
            .pending_deals
            .iter()
            .position(|deal| deal.id == event.deal_id);

        if let Some(index) = deal_index {
            let mut deal = politics.pending_deals.remove(index);

            if event.accepted {
                deal.status = DealStatus::Accepted;
                politics.active_deals.push(deal);
            } else {
                deal.status = DealStatus::Rejected;
                // Rejected deals just disappear
            }
        }
    }

    // Process broken deals
    for event in deal_broken_events.read() {
        let deal_index = politics
            .active_deals
            .iter()
            .position(|deal| deal.id == event.deal_id);

        if let Some(index) = deal_index {
            let mut deal = politics.active_deals.remove(index);
            deal.status = DealStatus::Broken(event.breaker);
            // TODO: Handle consequences of broken deals
        }
    }

    // Update deal durations
    politics.active_deals.retain(|deal| {
        match deal.duration {
            DealDuration::Turns(turns) => {
                // Check if the deal has expired
                let deal_turn = deal.created_at.elapsed().as_secs() / 5; // Approximate turn tracking
                deal_turn < turns as u64
            }
            DealDuration::UntilEndOfGame => true,
            DealDuration::UntilPlayerEliminated(player) => {
                // Check if the player is still in the game
                !turn_manager.eliminated_players.contains(&player)
            }
            DealDuration::Custom(_) => true, // Custom durations need special handling
        }
    });
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
            (monarch_system, voting_system, goad_system, deal_system),
        );
}
