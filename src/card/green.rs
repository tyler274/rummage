use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_green_cards() -> Vec<Card> {
    vec![
        Card::builder("Birds of Paradise")
            .cost(Mana::new_with_colors(0, 0, 0, 0, 0, 1))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 0,
                toughness: 1,
                creature_type: CreatureType::NONE,
            }))
            .rules_text("Flying\n{T}: Add one mana of any color.")
            .build_or_panic(),
        Card::builder("Llanowar Elves")
            .cost(Mana::new_with_colors(0, 0, 0, 0, 0, 1))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 1,
                toughness: 1,
                creature_type: CreatureType::NONE,
            }))
            .rules_text("{T}: Add {G}.")
            .build_or_panic(),
        Card::builder("Tarmogoyf")
            .cost(Mana::new_with_colors(1, 0, 0, 0, 0, 1))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 2,
                toughness: 3,
                creature_type: CreatureType::NONE,
            }))
            .rules_text("Tarmogoyf's power is equal to the number of card types among cards in all graveyards and its toughness is that number plus 1.")
            .build_or_panic(),
        Card::builder("Giant Growth")
            .cost(Mana::new_with_colors(0, 0, 0, 0, 0, 1))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Target creature gets +3/+3 until end of turn.")
            .build_or_panic(),
    ]
}
