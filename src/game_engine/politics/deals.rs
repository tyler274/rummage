use super::{
    DealBrokenEvent, DealDuration, DealProposedEvent, DealResponseEvent, DealStatus, PoliticsSystem,
};
use crate::game_engine::turns::TurnManager;
use bevy::prelude::*;

/// System to handle deals between players
pub fn deal_system(
    _commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut deal_proposed_events: EventReader<DealProposedEvent>,
    mut deal_response_events: EventReader<DealResponseEvent>,
    mut deal_broken_events: EventReader<DealBrokenEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Handle new deal proposals
    for event in deal_proposed_events.read() {
        info!(
            "New deal proposed from player {:?} to player {:?}",
            event.deal.proposer, event.deal.target
        );

        // Add to pending deals
        politics.pending_deals.push(event.deal.clone());
    }

    // Handle deal responses
    for event in deal_response_events.read() {
        info!(
            "Deal response from player {:?}: {}",
            event.responder,
            if event.accepted {
                "Accepted"
            } else {
                "Rejected"
            }
        );

        // Find the deal in pending deals
        if let Some(deal_idx) = politics
            .pending_deals
            .iter()
            .position(|d| d.id == event.deal_id)
        {
            let mut deal = politics.pending_deals.remove(deal_idx);

            // Update the deal status
            if event.accepted {
                deal.status = DealStatus::Accepted;
                politics.active_deals.push(deal);

                // Log that the deal was accepted, including the responder
                info!("Deal accepted by player {:?}", event.responder);
            } else {
                deal.status = DealStatus::Rejected;

                // Log that the deal was rejected, including the responder
                info!("Deal rejected by player {:?}", event.responder);
            }
        }
    }

    // Handle broken deals
    for event in deal_broken_events.read() {
        info!(
            "Deal broken by player {:?}: {}",
            event.breaker, event.reason
        );

        // Find the deal in active deals
        if let Some(deal_idx) = politics
            .active_deals
            .iter()
            .position(|d| d.id == event.deal_id)
        {
            let mut deal = politics.active_deals.remove(deal_idx);
            deal.status = DealStatus::Broken(event.breaker);

            // Would implement additional mechanics for deal-breaking here
            // such as triggering effects or applying penalties
        }
    }

    // Update deal durations
    let current_turn = turn_manager.turn_number;

    // Check for expired deals
    politics.active_deals.retain_mut(|deal| {
        let expired = match &deal.duration {
            DealDuration::Turns(turns) => {
                // Check if deal duration has passed based on turn number
                // This is a simple implementation; in reality, it would need to track
                // when the deal was made in terms of game turns
                current_turn > turn_manager.turn_number + *turns as u32
            }
            DealDuration::UntilEndOfGame => false,
            DealDuration::UntilPlayerEliminated(player) => {
                // Check if the specified player has been eliminated
                turn_manager.eliminated_players.contains(player)
            }
            DealDuration::Custom(_) => {
                // Custom durations would need specific implementation
                false
            }
        };

        if expired {
            deal.status = DealStatus::Expired;
            info!(
                "Deal between players {:?} and {:?} has expired",
                deal.proposer, deal.target
            );
            false // Remove from active deals
        } else {
            true // Keep in active deals
        }
    });
}
