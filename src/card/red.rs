use crate::card::{Card, CardDetails, CardTypes, CreatureCard, CreatureType};
use crate::mana::Mana;

pub fn get_red_cards() -> Vec<Card> {
    vec![
        Card::builder("Dragon Mage")
            .cost(Mana::new_with_colors(5, 0, 0, 0, 2, 0))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 5,
                toughness: 5,
                creature_type: CreatureType::DRAGON | CreatureType::WIZARD,
            }))
            .rules_text("Flying\nWhenever Dragon Mage deals combat damage to a player, each player discards their hand, then draws seven cards.")
            .build_or_panic(),
        Card::builder("Lightning Bolt")
            .cost(Mana::new_with_colors(0, 0, 0, 0, 1, 0))
            .types(CardTypes::INSTANT)
            .details(CardDetails::Other)
            .rules_text("Lightning Bolt deals 3 damage to any target.")
            .build_or_panic(),
        Card::builder("Shivan Dragon")
            .cost(Mana::new_with_colors(4, 0, 0, 0, 2, 0))
            .types(CardTypes::CREATURE)
            .details(CardDetails::Creature(CreatureCard {
                power: 5,
                toughness: 5,
                creature_type: CreatureType::DRAGON,
            }))
            .rules_text("Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.")
            .build_or_panic(),
        Card::builder("Fireball")
            .cost(Mana::new_with_colors(1, 0, 0, 0, 1, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Fireball deals X damage divided evenly, rounded down, among any number of targets.\nFireball costs {1} more to cast for each target beyond the first.")
            .build_or_panic(),
        Card::builder("Wheel of Fortune")
            .cost(Mana::new_with_colors(2, 0, 0, 0, 1, 0))
            .types(CardTypes::SORCERY)
            .details(CardDetails::Other)
            .rules_text("Each player discards their hand, then draws seven cards.")
            .build_or_panic(),
    ]
}
