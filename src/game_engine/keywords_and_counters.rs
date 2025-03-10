use crate::card::{
    CounterChangeEvent, CounterType, Counters, CreatureOnField, Keyword, KeywordChangeEvent,
    Keywords,
};
use bevy::prelude::*;
use std::collections::HashSet;

/// System that processes keyword changes from events
pub fn process_keyword_changes(
    mut events: EventReader<KeywordChangeEvent>,
    mut keyword_query: Query<&mut Keywords>,
) {
    for event in events.read() {
        if let Ok(mut keywords) = keyword_query.get_mut(event.card) {
            let keyword = &event.keyword;

            if event.added {
                if event.until_end_of_turn {
                    keywords.add_until_end_of_turn(keyword.clone());
                } else if let Some(source) = event.source {
                    keywords.add_granted_by(keyword.clone(), source);
                } else {
                    keywords.add(keyword.clone());
                }
            } else {
                keywords.remove(keyword);
            }
        }
    }
}

/// System that processes counter changes from events
pub fn process_counter_changes(
    mut events: EventReader<CounterChangeEvent>,
    mut counter_query: Query<&mut Counters>,
) {
    for event in events.read() {
        if let Ok(mut counters) = counter_query.get_mut(event.card) {
            let counter_type = &event.counter_type;
            let amount = event.amount;

            if amount > 0 {
                if let Some(source) = event.source {
                    counters.add_from_source(counter_type.clone(), amount, source);
                } else {
                    counters.add(counter_type.clone(), amount);
                }
            } else if amount < 0 {
                // For removing counters, we use the absolute value
                counters.remove(counter_type, amount.abs());
            }
        }
    }
}

/// Handles the interaction of +1/+1 and -1/-1 counters
/// In MTG, these counters cancel each other out
pub fn handle_counter_interactions(mut counter_query: Query<&mut Counters>) {
    for mut counters in counter_query.iter_mut() {
        let plus_counters = counters.get(&CounterType::PlusOnePlusOne);
        let minus_counters = counters.get(&CounterType::MinusOneMinusOne);

        if plus_counters > 0 && minus_counters > 0 {
            let counters_to_remove = plus_counters.min(minus_counters);
            counters.remove(&CounterType::PlusOnePlusOne, counters_to_remove);
            counters.remove(&CounterType::MinusOneMinusOne, counters_to_remove);
        }
    }
}

/// Resource to track which entities have been affected by indestructible
#[derive(Resource, Default)]
pub struct IndestructibleTracker {
    pub entities: HashSet<Entity>,
}

/// System to check for indestructible entities during state-based actions
pub fn check_indestructible(
    mut tracker: ResMut<IndestructibleTracker>,
    keyword_query: Query<(Entity, &Keywords)>,
) {
    tracker.entities.clear();

    for (entity, keywords) in keyword_query.iter() {
        if keywords.has(&Keyword::Indestructible) {
            tracker.entities.insert(entity);
        }
    }
}

/// System to handle triggered abilities of keywords
pub fn handle_keyword_triggers(
    keyword_query: Query<(Entity, &Keywords, Option<&CreatureOnField>)>,
    // We'd also need other relevant queries for the game state
) {
    for (_entity, keywords, creature) in keyword_query.iter() {
        // Example: Handle landfall
        if keywords.has(&Keyword::Landfall) {
            // Implement landfall trigger - this would require knowledge of whether
            // a land was played this turn, which would be in a game state resource
        }

        // Example: Handle Prowess
        if keywords.has(&Keyword::Prowess) && creature.is_some() {
            // Implement prowess trigger - would need to track when noncreature spells are cast
        }

        // Many more triggers could be implemented here
    }
}

/// System to apply the effects of keyword abilities during combat
pub fn apply_combat_keywords(
    keyword_query: Query<(Entity, &Keywords, Option<&CreatureOnField>)>,
    // Would need access to combat-related state
) {
    for (_entity, keywords, _) in keyword_query.iter() {
        if keywords.has(&Keyword::FirstStrike) || keywords.has(&Keyword::DoubleStrike) {
            // Handle first strike damage
        }

        if keywords.has(&Keyword::Deathtouch) {
            // Handle deathtouch effect in combat
        }

        if keywords.has(&Keyword::Lifelink) {
            // Handle lifelink effect
        }

        // And so on for other combat-relevant keywords
    }
}

/// System to check if a creature can be blocked based on evasion keywords
pub fn check_evasion_keywords(
    keyword_query: Query<(Entity, &Keywords)>,
    // Would need blockers info too
) {
    for (_entity, keywords) in keyword_query.iter() {
        if keywords.has(&Keyword::Flying) {
            // Only creatures with Flying or Reach can block
        }

        if keywords.has(&Keyword::Shadow) {
            // Only creatures with Shadow can block
        }

        if keywords.has(&Keyword::Horsemanship) {
            // Only creatures with Horsemanship can block
        }

        // And so on for other evasion keywords
    }
}

/// Resource to track proliferate actions
#[derive(Resource, Default)]
pub struct ProliferateTracker {
    pub pending_proliferate: bool,
    pub affected_entities: Vec<Entity>,
}

/// System to handle proliferate mechanics
pub fn handle_proliferate(
    mut tracker: ResMut<ProliferateTracker>,
    mut counter_query: Query<(Entity, &mut Counters)>,
) {
    if !tracker.pending_proliferate {
        return;
    }

    // Process all entities with counters
    for (entity, mut counters) in counter_query.iter_mut() {
        if counters.counters.is_empty() {
            continue;
        }

        // Proliferate adds one counter of each kind already there
        for (counter_type, _) in counters.counters.clone().iter() {
            counters.add(counter_type.clone(), 1);
            tracker.affected_entities.push(entity);
        }
    }

    // Reset the tracker
    tracker.pending_proliferate = false;
}

/// Component to track experience counters for the player
#[derive(Component, Default)]
pub struct ExperienceCounters {
    pub count: i32,
}

/// System to handle experience counter effects
pub fn handle_experience_counters(counter_query: Query<(Entity, &Counters)>) {
    for (_, counters) in counter_query.iter() {
        // Custom counter for experience
        let exp_count = counters.get(&CounterType::Custom("Experience".to_string()));
        if exp_count > 0 {
            // Apply effects based on experience counter count
            // This might affect abilities or other game state
        }
    }
}

/// Applies static effects based on keywords
pub fn apply_keyword_effects(
    // This will be expanded based on game events and other queries
    _keywords_query: Query<(Entity, &Keywords)>,
) {
    // Implement static effects for keywords like vigilance, trample, etc.
}

/// Applies effects based on counters
pub fn apply_counter_effects(
    // This will be expanded based on game events and other queries
    _counters_query: Query<(Entity, &Counters)>,
) {
    // Implement effects based on counters like +1/+1, loyalty, etc.
}

/// Utility function to copy all keywords from one entity to another
/// Useful for effects like Clone or Copy effects
pub fn copy_keywords(
    source: Entity,
    target: Entity,
    keywords_query: &mut Query<&mut Keywords>,
) -> bool {
    // First, get and clone the source keywords
    let source_keywords = match keywords_query.get(source) {
        Ok(kw) => kw.clone(),
        Err(_) => return false,
    };

    // Then, apply to target
    if let Ok(mut target_keywords) = keywords_query.get_mut(target) {
        // Reset the target's keywords
        *target_keywords = Keywords::default();

        // Copy all active keywords
        for keyword in source_keywords.active.iter() {
            target_keywords.add(keyword.clone());

            // Copy details if any
            if let Some(details) = source_keywords.details.get(keyword) {
                target_keywords.add_with_details(keyword.clone(), details.clone());
            }
        }

        return true;
    }

    false
}

/// Utility function to copy all counters from one entity to another
pub fn copy_counters(
    source: Entity,
    target: Entity,
    counters_query: &mut Query<&mut Counters>,
) -> bool {
    // First, get and clone the source counters
    let source_counters = match counters_query.get(source) {
        Ok(c) => c.clone(),
        Err(_) => return false,
    };

    // Then, apply to target
    if let Ok(mut target_counters) = counters_query.get_mut(target) {
        // Reset the target's counters
        *target_counters = Counters::default();

        // Copy all counters
        for (counter_type, amount) in source_counters.counters.iter() {
            target_counters.add(counter_type.clone(), *amount);
        }

        return true;
    }

    false
}
