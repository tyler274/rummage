use super::{PoliticsSystem, VoteCastEvent, VoteChoice, VoteCompletedEvent, VoteStartedEvent};
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

/// System to handle voting mechanics
pub fn voting_system(
    mut commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut vote_started_events: EventReader<VoteStartedEvent>,
    mut vote_cast_events: EventReader<VoteCastEvent>,
    _player_query: Query<Entity, With<Player>>,
) {
    // Handle any new votes started
    for event in vote_started_events.read() {
        info!("New vote started: {}", event.vote.title);
        politics.active_vote = Some(event.vote.clone());
        politics.votes_cast.clear();

        // Initialize voting weights (default to 1 per player)
        politics.vote_weights.clear();
        for player in &event.vote.eligible_voters {
            politics.vote_weights.insert(*player, 1);
        }
    }

    // Process votes being cast
    for event in vote_cast_events.read() {
        // Make sure this vote is for the active vote
        if let Some(active_vote) = &politics.active_vote {
            if active_vote.id == event.vote_id {
                info!(
                    "Player {:?} voted for choice {}",
                    event.player, event.choice.text
                );
                politics
                    .votes_cast
                    .insert(event.player, event.choice.clone());
            }
        }
    }

    // Check if voting is complete
    if let Some(active_vote) = &politics.active_vote {
        if politics.is_vote_decisive() {
            // Get the winning choice and vote count
            if let Some((winning_choice, vote_count)) = politics.tally_votes() {
                info!(
                    "Vote completed. Winning choice: {} with {} votes",
                    winning_choice.text, vote_count
                );

                // Spawn a VoteCompletedEvent
                commands.spawn(VoteCompletedEvent {
                    vote_id: active_vote.id,
                    winning_choice,
                    vote_count,
                });

                // Clear the active vote
                politics.active_vote = None;
                politics.votes_cast.clear();
                politics.vote_weights.clear();
            }
        }
    }
}

impl PoliticsSystem {
    /// Check if a vote is decisive (has a clear winner that cannot be changed)
    pub fn is_vote_decisive(&self) -> bool {
        if let Some(active_vote) = &self.active_vote {
            // Check if all required players have voted
            if active_vote.requires_all_players {
                let all_voted = active_vote
                    .eligible_voters
                    .iter()
                    .all(|player| self.votes_cast.contains_key(player));
                return all_voted;
            } else {
                // Check if we have a clear majority
                if let Some((_, count)) = self.tally_votes() {
                    let total_possible_votes = active_vote
                        .eligible_voters
                        .iter()
                        .map(|player| self.vote_weights.get(player).copied().unwrap_or(1))
                        .sum::<u32>();

                    return count > total_possible_votes / 2;
                }
            }
        }
        false
    }

    /// Tally votes and determine the winner
    pub fn tally_votes(&self) -> Option<(VoteChoice, u32)> {
        if self.votes_cast.is_empty() {
            return None;
        }

        // Count votes for each choice
        let mut vote_counts: HashMap<usize, u32> = HashMap::new();

        for (player, choice) in &self.votes_cast {
            let weight = self.vote_weights.get(player).copied().unwrap_or(1);
            *vote_counts.entry(choice.id).or_insert(0) += weight;
        }

        // Find the choice with the most votes
        let mut max_votes = 0;
        let mut winning_choice_id = 0;

        for (choice_id, count) in vote_counts {
            if count > max_votes {
                max_votes = count;
                winning_choice_id = choice_id;
            }
        }

        // Get the full choice object
        if let Some(active_vote) = &self.active_vote {
            if let Some(choice) = active_vote
                .choices
                .iter()
                .find(|c| c.id == winning_choice_id)
            {
                return Some((choice.clone(), max_votes));
            }
        }

        None
    }
}
