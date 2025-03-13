use crate::card::{ArtifactCard, Card, CardDetails, CardTypes};
use crate::mana::Mana;

pub fn get_artifact_cards() -> Vec<Card> {
    vec![sol_ring(), mana_crypt(), arcane_signet()]
}

pub fn sol_ring() -> Card {
    Card::builder("Sol Ring")
        .cost(Mana::new_with_colors(1, 0, 0, 0, 0, 0))
        .types(CardTypes::ARTIFACT)
        .details(CardDetails::Artifact(ArtifactCard {
            artifact_type: None,
        }))
        .rules_text("{T}: Add {C}{C}.")
        .build_or_panic()
}

pub fn mana_crypt() -> Card {
    Card::builder("Mana Crypt")
        .cost(Mana::new_with_colors(0, 0, 0, 0, 0, 0))
        .types(CardTypes::ARTIFACT)
        .details(CardDetails::Artifact(ArtifactCard {
            artifact_type: None,
        }))
        .rules_text("{T}: Add {C}{C}.\nAt the beginning of your upkeep, flip a coin. If you lose the flip, Mana Crypt deals 3 damage to you.")
        .build_or_panic()
}

pub fn arcane_signet() -> Card {
    Card::builder("Arcane Signet")
        .cost(Mana::new_with_colors(2, 0, 0, 0, 0, 0))
        .types(CardTypes::ARTIFACT)
        .details(CardDetails::Artifact(ArtifactCard {
            artifact_type: None,
        }))
        .rules_text("{T}: Add one mana of any color in your commander's color identity.")
        .build_or_panic()
}
