use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// Import from other modules
use super::details::CardDetails;

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Hash)]
    pub struct CreatureType: u64 {
        const NONE = 0;
        const HUMAN = 1 << 0;
        const WIZARD = 1 << 1;
        const DRAGON = 1 << 2;
        const ANGEL = 1 << 3;
        const DEMON = 1 << 4;
        // Common creature types
        const WARRIOR = 1 << 5;
        const SOLDIER = 1 << 6;
        const CLERIC = 1 << 7;
        const ROGUE = 1 << 8;
        const SHAMAN = 1 << 9;
        const BEAST = 1 << 10;
        const ELEMENTAL = 1 << 11;
        const VAMPIRE = 1 << 12;
        const ZOMBIE = 1 << 13;
        const GOBLIN = 1 << 14;
        const ELF = 1 << 15;
        const MERFOLK = 1 << 16;
        const BIRD = 1 << 17;
        const SPIRIT = 1 << 18;
        const KNIGHT = 1 << 19;
        const DRUID = 1 << 20;
        const ASSASSIN = 1 << 21;
        const ARTIFICER = 1 << 22;
        const MONK = 1 << 23;
        const HORROR = 1 << 24;
        const GIANT = 1 << 25;
        const DINOSAUR = 1 << 26;
        const HYDRA = 1 << 27;
        const PHOENIX = 1 << 28;
        const WURM = 1 << 29;
        const PHYREXIAN = 1 << 30;
        const BERSERKER = 1 << 31;
        const SPHINX = 1 << 32;
        const IMP = 1 << 33;
        const GARGOYLE = 1 << 34;
        const LHURGOYF = 1 << 35;
        const OOZE = 1 << 36;
        const SQUIRREL = 1 << 37;
        const KAVU = 1 << 38;
        const CAT = 1 << 39;
        const DRAKE = 1 << 40;
        const GNOME = 1 << 41;
        const ARCHON = 1 << 42;
        const LIZARD = 1 << 43;
        const INSECT = 1 << 44;
        const CONSTRUCT = 1 << 45;
        const GOLEM = 1 << 46;
        const MONKEY = 1 << 47;
        const NYMPH = 1 << 48;
        const EFREET = 1 << 49;
        const INCARNATION = 1 << 50;
        const DRYAD = 1 << 51;
        // New types
        const TREEFOLK = 1 << 52;
        const SLIVER = 1 << 53;
        const SNAKE = 1 << 54;
        const WOLF = 1 << 55;
        const WEREWOLF = 1 << 56;
        const SCOUT = 1 << 58;
        const ADVISOR = 1 << 59;
        const ALLY = 1 << 60;
        const MERCENARY = 1 << 61;
        const REBEL = 1 << 62;
        const SPIDER = 1 << 63;
    }
}

impl std::fmt::Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut types = Vec::new();

        if self.contains(Self::HUMAN) {
            types.push("Human");
        }
        if self.contains(Self::WIZARD) {
            types.push("Wizard");
        }
        if self.contains(Self::DRAGON) {
            types.push("Dragon");
        }
        if self.contains(Self::ANGEL) {
            types.push("Angel");
        }
        if self.contains(Self::DEMON) {
            types.push("Demon");
        }
        if self.contains(Self::WARRIOR) {
            types.push("Warrior");
        }
        if self.contains(Self::SOLDIER) {
            types.push("Soldier");
        }
        if self.contains(Self::CLERIC) {
            types.push("Cleric");
        }
        if self.contains(Self::ROGUE) {
            types.push("Rogue");
        }
        if self.contains(Self::SHAMAN) {
            types.push("Shaman");
        }
        if self.contains(Self::BEAST) {
            types.push("Beast");
        }
        if self.contains(Self::ELEMENTAL) {
            types.push("Elemental");
        }
        if self.contains(Self::VAMPIRE) {
            types.push("Vampire");
        }
        if self.contains(Self::ZOMBIE) {
            types.push("Zombie");
        }
        if self.contains(Self::GOBLIN) {
            types.push("Goblin");
        }
        if self.contains(Self::ELF) {
            types.push("Elf");
        }
        if self.contains(Self::MERFOLK) {
            types.push("Merfolk");
        }
        if self.contains(Self::BIRD) {
            types.push("Bird");
        }
        if self.contains(Self::SPIRIT) {
            types.push("Spirit");
        }
        if self.contains(Self::KNIGHT) {
            types.push("Knight");
        }
        if self.contains(Self::DRUID) {
            types.push("Druid");
        }
        if self.contains(Self::ASSASSIN) {
            types.push("Assassin");
        }
        if self.contains(Self::ARTIFICER) {
            types.push("Artificer");
        }
        if self.contains(Self::MONK) {
            types.push("Monk");
        }
        if self.contains(Self::HORROR) {
            types.push("Horror");
        }
        if self.contains(Self::GIANT) {
            types.push("Giant");
        }
        if self.contains(Self::DINOSAUR) {
            types.push("Dinosaur");
        }
        if self.contains(Self::HYDRA) {
            types.push("Hydra");
        }
        if self.contains(Self::PHOENIX) {
            types.push("Phoenix");
        }
        if self.contains(Self::WURM) {
            types.push("Wurm");
        }
        if self.contains(Self::PHYREXIAN) {
            types.push("Phyrexian");
        }
        if self.contains(Self::BERSERKER) {
            types.push("Berserker");
        }
        if self.contains(Self::SPHINX) {
            types.push("Sphinx");
        }
        if self.contains(Self::IMP) {
            types.push("Imp");
        }
        if self.contains(Self::GARGOYLE) {
            types.push("Gargoyle");
        }
        if self.contains(Self::LHURGOYF) {
            types.push("Lhurgoyf");
        }
        if self.contains(Self::OOZE) {
            types.push("Ooze");
        }
        if self.contains(Self::SQUIRREL) {
            types.push("Squirrel");
        }
        if self.contains(Self::KAVU) {
            types.push("Kavu");
        }
        if self.contains(Self::CAT) {
            types.push("Cat");
        }
        if self.contains(Self::DRAKE) {
            types.push("Drake");
        }
        if self.contains(Self::GNOME) {
            types.push("Gnome");
        }
        if self.contains(Self::ARCHON) {
            types.push("Archon");
        }
        if self.contains(Self::LIZARD) {
            types.push("Lizard");
        }
        if self.contains(Self::INSECT) {
            types.push("Insect");
        }
        if self.contains(Self::CONSTRUCT) {
            types.push("Construct");
        }
        if self.contains(Self::GOLEM) {
            types.push("Golem");
        }
        if self.contains(Self::MONKEY) {
            types.push("Monkey");
        }
        if self.contains(Self::NYMPH) {
            types.push("Nymph");
        }
        if self.contains(Self::EFREET) {
            types.push("Efreet");
        }
        if self.contains(Self::INCARNATION) {
            types.push("Incarnation");
        }
        if self.contains(Self::DRYAD) {
            types.push("Dryad");
        }
        if self.contains(Self::TREEFOLK) {
            types.push("Treefolk");
        }
        if self.contains(Self::SLIVER) {
            types.push("Sliver");
        }
        if self.contains(Self::SNAKE) {
            types.push("Snake");
        }
        if self.contains(Self::WOLF) {
            types.push("Wolf");
        }
        if self.contains(Self::WEREWOLF) {
            types.push("Werewolf");
        }
        if self.contains(Self::SCOUT) {
            types.push("Scout");
        }
        if self.contains(Self::ADVISOR) {
            types.push("Advisor");
        }
        if self.contains(Self::ALLY) {
            types.push("Ally");
        }
        if self.contains(Self::MERCENARY) {
            types.push("Mercenary");
        }
        if self.contains(Self::REBEL) {
            types.push("Rebel");
        }
        if self.contains(Self::SPIDER) {
            types.push("Spider");
        }

        if types.is_empty() {
            write!(f, "")
        } else {
            write!(f, "{}", types.join(" "))
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, Hash)]
    pub struct CardTypes: u32 {
        const NONE = 0;
        // Basic types
        const ARTIFACT = 1 << 0;
        const CONSPIRACY = 1 << 1;
        const CREATURE = 1 << 2;
        const ENCHANTMENT = 1 << 3;
        const INSTANT = 1 << 4;
        const LAND = 1 << 5;
        const PHENOMENON = 1 << 6;
        const PLANE = 1 << 7;
        const PLANESWALKER = 1 << 8;
        const SCHEME = 1 << 9;
        const SORCERY = 1 << 10;
        const TRIBAL = 1 << 11;
        const VANGUARD = 1 << 12;

        // Supertypes
        const BASIC = 1 << 13;
        const LEGENDARY = 1 << 14;
        const ONGOING = 1 << 15;
        const SNOW = 1 << 16;
        const WORLD = 1 << 17;

        // Subtypes
        const SAGA = 1 << 18;
        const EQUIPMENT = 1 << 19;
        const AURA = 1 << 20;
        const VEHICLE = 1 << 21;
        const FOOD = 1 << 22;
        const CLUE = 1 << 23;
        const TREASURE = 1 << 24;
        const FORTIFICATION = 1 << 25;
        const CONTRAPTION = 1 << 26;

        // Land subtypes
        const PLAINS = 1 << 27;
        const ISLAND = 1 << 28;
        const SWAMP = 1 << 29;
        const MOUNTAIN = 1 << 30;
        const FOREST = 1 << 31;

        // Derived types
        const HISTORIC = Self::LEGENDARY.bits() | Self::ARTIFACT.bits() | Self::SAGA.bits();
    }
}

impl std::fmt::Display for CardTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();

        // Supertypes
        if self.contains(Self::LEGENDARY) {
            parts.push("Legendary");
        }
        if self.contains(Self::BASIC) {
            parts.push("Basic");
        }
        if self.contains(Self::SNOW) {
            parts.push("Snow");
        }
        if self.contains(Self::WORLD) {
            parts.push("World");
        }

        // Main types
        if self.contains(Self::ARTIFACT) {
            parts.push("Artifact");
        }
        if self.contains(Self::ENCHANTMENT) {
            parts.push("Enchantment");
        }
        if self.contains(Self::CREATURE) {
            parts.push("Creature");
        }
        if self.contains(Self::SORCERY) {
            parts.push("Sorcery");
        }
        if self.contains(Self::INSTANT) {
            parts.push("Instant");
        }
        if self.contains(Self::LAND) {
            parts.push("Land");
        }
        if self.contains(Self::PLANESWALKER) {
            parts.push("Planeswalker");
        }
        if self.contains(Self::TRIBAL) {
            parts.push("Tribal");
        }

        // Artifact subtypes
        if self.contains(Self::EQUIPMENT) {
            parts.push("Equipment");
        }
        if self.contains(Self::VEHICLE) {
            parts.push("Vehicle");
        }
        if self.contains(Self::FOOD) {
            parts.push("Food");
        }
        if self.contains(Self::CLUE) {
            parts.push("Clue");
        }
        if self.contains(Self::TREASURE) {
            parts.push("Treasure");
        }
        if self.contains(Self::CONTRAPTION) {
            parts.push("Contraption");
        }

        // Enchantment subtypes
        if self.contains(Self::AURA) {
            parts.push("Aura");
        }
        if self.contains(Self::SAGA) {
            parts.push("Saga");
        }

        // Land subtypes
        if self.contains(Self::PLAINS) {
            parts.push("Plains");
        }
        if self.contains(Self::ISLAND) {
            parts.push("Island");
        }
        if self.contains(Self::SWAMP) {
            parts.push("Swamp");
        }
        if self.contains(Self::MOUNTAIN) {
            parts.push("Mountain");
        }
        if self.contains(Self::FOREST) {
            parts.push("Forest");
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl CreatureType {
    // Infer creature types from card name and rules text
    pub fn infer_from_name_and_text(text: &str, existing_types: Self) -> Self {
        let mut types = existing_types;

        if text.contains("Human") || text.contains("human") {
            types |= Self::HUMAN;
        }

        // Add other rules as needed

        types
    }

    pub fn apply_retroactive_types(name: &str, existing_types: Self) -> Self {
        let mut types = existing_types;

        // Example: Cards with "Knight" in the name are Knights
        if name.contains("Knight") {
            types |= Self::KNIGHT;
        }

        types
    }
}

pub fn format_type_line(types: &CardTypes, card_details: &CardDetails) -> String {
    let mut type_line = types.to_string();

    // Add creature type if applicable
    if types.contains(CardTypes::CREATURE) {
        if let CardDetails::Creature(creature_card) = card_details {
            if creature_card.creature_type != CreatureType::NONE {
                type_line.push_str(" — ");
                type_line.push_str(&creature_card.creature_type.to_string());
            }
        }
    }

    // Add land type if applicable
    if types.contains(CardTypes::LAND) {
        if let CardDetails::Land(land_card) = card_details {
            if let Some(land_type) = &land_card.land_type {
                type_line.push_str(" — ");
                type_line.push_str(land_type);
            }
        }
    }

    // Add enchantment subtypes if applicable
    if types.contains(CardTypes::ENCHANTMENT)
        && !types.contains(CardTypes::AURA)
        && !types.contains(CardTypes::SAGA)
    {
        if let CardDetails::Enchantment(enchantment_card) = card_details {
            if let Some(enchantment_type) = &enchantment_card.enchantment_type {
                type_line.push_str(" — ");
                type_line.push_str(enchantment_type);
            }
        }
    }

    // Add artifact subtypes if applicable
    if types.contains(CardTypes::ARTIFACT)
        && !types.contains(CardTypes::EQUIPMENT)
        && !types.contains(CardTypes::VEHICLE)
        && !types.contains(CardTypes::FOOD)
        && !types.contains(CardTypes::CLUE)
        && !types.contains(CardTypes::TREASURE)
    {
        if let CardDetails::Artifact(artifact_card) = card_details {
            if let Some(artifact_type) = &artifact_card.artifact_type {
                type_line.push_str(" — ");
                type_line.push_str(artifact_type);
            }
        }
    }

    type_line
}
