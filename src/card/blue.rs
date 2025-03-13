use crate::card::{Card, CardDetails, CardTypes};
use crate::mana::{Color, Mana};

pub fn get_blue_cards() -> Vec<Card> {
    vec![
        Card::builder("Counterspell")
            .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Counter target spell.")
            .build_or_panic(),
        Card::builder("Force of Will")
            .cost(Mana::new_with_colors(3, 0, 2, 0, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("You may pay 1 life and exile a blue card from your hand rather than pay Force of Will's mana cost.\nCounter target spell.")
            .build_or_panic(),
        Card::builder("Ancestral Recall")
            .cost(Mana::new_with_colors(0, 0, 1, 0, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Target player draws three cards.")
            .build_or_panic(),
        Card::builder("Time Walk")
            .cost(Mana::new_with_colors(1, 0, 1, 0, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Take an extra turn after this one.")
            .build_or_panic(),
        Card::builder("Mana Drain")
            .cost(Mana::new_with_colors(0, 0, 2, 0, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.")
            .build_or_panic(),
    ]
}
