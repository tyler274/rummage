use super::MonarchChangedEvent;
use super::PoliticsSystem;
use crate::game_engine::Phase;
use crate::game_engine::state::GameState;
use crate::game_engine::turns::TurnManager;
use bevy::prelude::*;

/// System to handle the monarch mechanic
pub fn monarch_system(
    _commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut monarch_events: EventReader<MonarchChangedEvent>,
    _game_state: Res<GameState>,
    turn_manager: Res<TurnManager>,
    current_phase: Res<Phase>,
) {
    // Process monarch change events
    for event in monarch_events.read() {
        // Store previous monarch for reference (even though currently unused)
        let _previous = politics.monarch;

        // Update the current monarch
        politics.monarch = Some(event.new_monarch);

        // TODO: Implement card draw trigger for when a player becomes monarch
        // This would be implemented when cards with monarch effects are added

        info!("Player {:?} has become the monarch", event.new_monarch);

        // Process previous_monarch for monarchy change effects
        if let Some(prev) = event.previous_monarch {
            // Handle any effects that trigger when losing monarch status
            info!("Player {:?} is no longer the monarch", prev);
        }

        // Track source of monarchy change for future effects
        if let Some(source) = event.source {
            // Would be used for effects that care about how monarchy changed
            info!("Monarchy changed due to source: {:?}", source);
        }
    }

    // At the end of a monarch's turn, they draw a card
    if let Some(monarch) = politics.monarch {
        // Check if it's the end phase
        let is_end_phase = match *current_phase {
            Phase::Ending(_) => true,
            _ => false,
        };

        if monarch == turn_manager.active_player && is_end_phase {
            // TODO: Implement card draw effect through a proper event system
            info!("Monarch draws a card at end of turn");

            // This will be replaced with an actual card draw event when implemented
            // commands.spawn(DrawCardEvent { player: monarch, amount: 1 });
        }
    }
}
