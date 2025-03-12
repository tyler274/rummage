use super::{CombatRestriction, GoadEffect, PoliticsSystem};
use crate::game_engine::combat::CombatState;
use crate::game_engine::turns::TurnManager;
use bevy::prelude::*;

/// Event for when a creature is goaded
#[derive(Event)]
pub struct GoadEvent {
    /// The creature being goaded
    pub target: Entity,

    /// The player doing the goading
    pub source: Entity,

    /// How many turns it lasts
    pub duration: u32,
}

/// System to handle goad effects
pub fn goad_system(
    _commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    _combat_state: Option<ResMut<CombatState>>,
    mut goad_events: EventReader<GoadEvent>,
    turn_manager: Res<TurnManager>,
) {
    // Process new goad effects
    for event in goad_events.read() {
        info!(
            "Creature {:?} goaded by {:?} for {} turns",
            event.target, event.source, event.duration
        );

        // Create new goad effect
        let goad_effect = GoadEffect {
            target: event.target,
            source: event.source,
            duration: event.duration,
            created_at: turn_manager.turn_number,
        };

        // Add to goad effects
        politics
            .goad_effects
            .entry(event.target)
            .or_insert_with(Vec::new)
            .push(goad_effect);

        // Apply combat restrictions for goaded creatures
        if let Some(combat_restrictions) = politics.combat_restrictions.get_mut(&event.target) {
            // A goaded creature must attack if able
            if !combat_restrictions.contains(&CombatRestriction::MustAttack) {
                combat_restrictions.push(CombatRestriction::MustAttack);
            }

            // A goaded creature cannot attack the player who goaded it if possible
            let cannot_attack = CombatRestriction::CannotAttackPlayer(event.source);
            if !combat_restrictions.contains(&cannot_attack) {
                combat_restrictions.push(cannot_attack);
            }
        } else {
            politics.combat_restrictions.insert(
                event.target,
                vec![
                    CombatRestriction::MustAttack,
                    CombatRestriction::CannotAttackPlayer(event.source),
                ],
            );
        }
    }

    // Check for expired goad effects
    let current_turn = turn_manager.turn_number;

    // First collect expired effects
    let mut expired_goads: Vec<(Entity, usize)> = Vec::new();
    for (creature, effects) in &mut politics.goad_effects {
        for (index, effect) in effects.iter().enumerate().rev() {
            let expired = current_turn >= effect.created_at + effect.duration;
            if expired {
                expired_goads.push((*creature, index));
                info!("Goad effect on creature {:?} has expired", creature);
            }
        }
    }

    // Now remove the expired effects and update restrictions
    for (creature, index) in expired_goads {
        if let Some(effects) = politics.goad_effects.get_mut(&creature) {
            // Only remove if index is valid
            if index < effects.len() {
                effects.remove(index);
            }

            // If all goad effects are gone, remove combat restrictions
            if effects.is_empty() {
                if let Some(restrictions) = politics.combat_restrictions.get_mut(&creature) {
                    restrictions.retain(|r| {
                        !matches!(
                            r,
                            CombatRestriction::MustAttack
                                | CombatRestriction::CannotAttackPlayer(_)
                        )
                    });
                }
            }
        }
    }

    // Clean up empty entries
    politics
        .goad_effects
        .retain(|_, effects| !effects.is_empty());
    politics
        .combat_restrictions
        .retain(|_, restrictions| !restrictions.is_empty());
}
