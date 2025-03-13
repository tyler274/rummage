use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_black_cards() -> Vec<Card> {
    vec![
        Card::builder("Dark Ritual")
            .cost(Mana::new_with_colors(0, 0, 0, 1, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Add {B}{B}{B}.")
            .build_or_panic(),
        Card::builder("Demonic Tutor")
            .cost(Mana::new_with_colors(1, 0, 0, 1, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Search your library for a card and put that card into your hand. Then shuffle your library.")
            .build_or_panic(),
        Card::builder("Black Knight")
            .cost(Mana::new_with_colors(0, 0, 0, 2, 0, 0))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 2,
                creature_type: CreatureType::HUMAN | CreatureType::WIZARD,
            }))
            .rules_text("First strike, protection from white")
            .build_or_panic(),
        Card::builder("Mind Twist")
            .cost(Mana::new_with_colors(1, 0, 0, 1, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Target player discards X cards at random.")
            .build_or_panic(),
        Card::builder("Hypnotic Specter")
            .cost(Mana::new_with_colors(0, 0, 0, 2, 0, 0))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 2,
                creature_type: CreatureType::NONE,
            }))
            .rules_text("Flying\nWhenever Hypnotic Specter deals damage to an opponent, that player discards a card at random.")
            .build_or_panic(),
    ]
}
