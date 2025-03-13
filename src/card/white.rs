use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::{Color, Mana};

pub fn get_white_cards() -> Vec<Card> {
    vec![
        Card::builder("Serra Angel")
            .cost(Mana::new_with_colors(3, 2, 0, 0, 0, 0))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 4,
                toughness: 4,
                creature_type: CreatureType::ANGEL,
            }))
            .rules_text("Flying, vigilance")
            .build_or_panic(),
        Card::builder("Wrath of God")
            .cost(Mana::new_with_colors(2, 2, 0, 0, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Destroy all creatures. They can't be regenerated.")
            .build_or_panic(),
        Card::builder("Swords to Plowshares")
            .cost(Mana::new_with_colors(0, 1, 0, 0, 0, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Exile target creature. Its controller gains life equal to its power.")
            .build_or_panic(),
        Card::builder("Armageddon")
            .cost(Mana::new_with_colors(3, 1, 0, 0, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Destroy all lands.")
            .build_or_panic(),
        Card::builder("Balance")
            .cost(Mana::new_with_colors(1, 1, 0, 0, 0, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Each player chooses a number of lands they control equal to the number of lands controlled by the player who controls the fewest, then sacrifices the rest. Players discard cards and sacrifice creatures the same way.")
            .build_or_panic(),
    ]
}
