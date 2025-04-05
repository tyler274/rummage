use super::{CombatRestriction, PoliticsSystem};
use crate::game_engine::combat::CombatState;
use crate::game_engine::turns::TurnManager;
use bevy::prelude::*;

/// System to enforce combat restrictions from political effects
pub fn combat_restrictions_system(
    politics: ResMut<PoliticsSystem>,
    combat_state: Option<ResMut<CombatState>>,
    _turn_manager: Res<TurnManager>,
) {
    // Only apply restrictions during combat
    let Some(mut combat_state) = combat_state else {
        return;
    };

    // Process all combat restrictions
    for (creature, restrictions) in &politics.combat_restrictions {
        for restriction in restrictions {
            match restriction {
                CombatRestriction::MustAttack => {
                    // Add creature to the list that must attack
                    if !combat_state.must_attack.contains_key(creature) {
                        combat_state.must_attack.insert(*creature, Vec::new());
                    }
                }
                CombatRestriction::MustAttackPlayer(player) => {
                    // Add player to the list that this creature must attack
                    if let Some(targets) = combat_state.must_attack.get_mut(creature) {
                        if !targets.contains(player) {
                            targets.push(*player);
                        }
                    } else {
                        combat_state.must_attack.insert(*creature, vec![*player]);
                    }
                }
                CombatRestriction::CannotAttackPlayer(player) => {
                    // Add player to the list that this creature cannot attack
                    if let Some(targets) = combat_state.cannot_attack.get_mut(creature) {
                        if !targets.contains(player) {
                            targets.push(*player);
                        }
                    } else {
                        combat_state.cannot_attack.insert(*creature, vec![*player]);
                    }
                }
                CombatRestriction::CannotBlock => {
                    // This is checked directly in the declare_blockers system
                    // Implementation would go in that system
                    info!(
                        "Creature {:?} cannot block due to political effects",
                        creature
                    );
                }
                CombatRestriction::CannotBlockAttacksAgainst(player) => {
                    // This would need to be checked in the declare_blockers system
                    // Implementation would go in that system
                    info!(
                        "Creature {:?} cannot block attacks against player {:?}",
                        creature, player
                    );
                }
            }
        }
    }

    // Log combat restrictions for debugging
    if !combat_state.must_attack.is_empty() {
        info!(
            "Combat restrictions active: {:?} creatures must attack",
            combat_state.must_attack.len()
        );
    }

    if !combat_state.cannot_attack.is_empty() {
        info!(
            "Combat restrictions active: {:?} creatures have attack restrictions",
            combat_state.cannot_attack.len()
        );
    }
}

/// Event for applying a new combat restriction to a creature
#[derive(Event)]
pub struct ApplyCombatRestrictionEvent {
    /// The creature to restrict
    pub target: Entity,

    /// The restriction to apply
    pub restriction: CombatRestriction,

    /// The source of the restriction (e.g., a spell or ability)
    #[allow(dead_code)]
    pub source: Option<Entity>,

    /// The duration of the restriction in turns (None = until end of turn)
    #[allow(dead_code)]
    pub duration: Option<u32>,
}

/// Event for removing a combat restriction from a creature
#[derive(Event)]
pub struct RemoveCombatRestrictionEvent {
    /// The creature to remove restrictions from
    pub target: Entity,

    /// The specific restriction to remove (None = all restrictions)
    pub restriction: Option<CombatRestriction>,
}

/// System to handle applying and removing combat restrictions
pub fn manage_combat_restrictions(
    _commands: Commands,
    mut politics: ResMut<PoliticsSystem>,
    mut apply_events: EventReader<ApplyCombatRestrictionEvent>,
    mut remove_events: EventReader<RemoveCombatRestrictionEvent>,
) {
    // Apply new restrictions
    for event in apply_events.read() {
        info!(
            "Applying combat restriction to creature {:?}: {:?}",
            event.target, event.restriction
        );

        let restrictions = politics
            .combat_restrictions
            .entry(event.target)
            .or_default();

        if !restrictions.contains(&event.restriction) {
            restrictions.push(event.restriction.clone());
        }
    }

    // Remove restrictions
    for event in remove_events.read() {
        if let Some(restriction) = &event.restriction {
            // Remove specific restriction
            if let Some(restrictions) = politics.combat_restrictions.get_mut(&event.target) {
                restrictions.retain(|r| r != restriction);

                // Clean up if empty
                if restrictions.is_empty() {
                    politics.combat_restrictions.remove(&event.target);
                }
            }
        } else {
            // Remove all restrictions
            politics.combat_restrictions.remove(&event.target);
        }
    }
}
