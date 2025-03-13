use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Magic keyword abilities
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct KeywordAbilities {
    pub abilities: HashSet<KeywordAbility>,
    /// For abilities with values like "Protection from X" or "Ward X"
    pub ability_values: HashMap<KeywordAbility, String>,
}

/// All keyword abilities in Magic: The Gathering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub enum KeywordAbility {
    // Evergreen keywords
    Deathtouch,
    Defender,
    DoubleStrike,
    Enchant,
    Equip,
    FirstStrike,
    Flash,
    Flying,
    Haste,
    Hexproof,
    Indestructible,
    Lifelink,
    Menace,
    Protection, // Requires a value
    Reach,
    Trample,
    Vigilance,
    Ward, // Requires a value

    // Deciduous keywords
    Cycling,
    Scry,
    ActivatedAbility,
    Fight,
    Landcycling,
    Typecycling,

    // Set-specific and other keywords
    Adapt,
    Affinity,
    Aftermath,
    Amplify,
    Annihilator,
    Ascend,
    Assist,
    AuraSwap,
    Awaken,
    Backup,
    Banding,
    Bargain,
    Battlecry,
    Blitz,
    Bloodrush,
    Bloodthirst,
    Boast,
    Bushido,
    Buyback,
    Cascade,
    Champion,
    Changeling,
    Cipher,
    Cleave,
    Companion,
    Complicity,
    Compleated,
    Conspire,
    Convoke,
    Craft,
    Crew,
    Cumulative,
    Dash,
    Daybound,
    Decayed,
    Delve,
    Demonstrate,
    Desertwalk,
    Devoid,
    Devour,
    Discover,
    Disturb,
    Domain,
    Doctor,
    Dredge,
    Echo,
    Embalm,
    Emerge,
    Eminence,
    Encore,
    Enlist,
    Entwine,
    Epic,
    Escalate,
    Escape,
    Eternalize,
    Evoke,
    Evolve,
    Exalted,
    Exploit,
    Explore,
    Extort,
    Fabricate,
    Fading,
    Fear,
    Flanking,
    Flashback,
    Foretell,
    Forestwalk,
    Fortify,
    Frenzy,
    Fuse,
    Graft,
    Gravestorm,
    Haunt,
    Hideaway,
    Historicentric,
    Improvise,
    Incubate,
    Infect,
    Investigate,
    Islandwalk,
    JumpStart,
    Kicker,
    Landfall,
    LevelUp,
    LivingWeapon,
    Madness,
    Melee,
    Metalcraft,
    Miracle,
    Modular,
    Monstrosity,
    Morph,
    Mountainwalk,
    Mutate,
    Myriad,
    Ninjutsu,
    Nightbound,
    Offering,
    Outlast,
    Overload,
    Persist,
    Phasing,
    Plainswalk,
    Poisonous,
    Populate,
    Proliferate,
    Provoke,
    Prowess,
    Prowl,
    Rampage,
    Ravenous,
    Rebound,
    Reconfigure,
    Recover,
    Reinforce,
    Renown,
    Replicate,
    Retrace,
    Riot,
    Ripple,
    Sleuthed,
    Skulk,
    Soulbond,
    Soulshift,
    Spectacle,
    Splice,
    SplitSecond,
    Storm,
    Sunburst,
    Surge,
    Suspend,
    Swampwalk,
    Threshold,
    TotemArmor,
    Transfigure,
    Transmute,
    Tribute,
    Undaunted,
    Undergrowth,
    Undying,
    Unearth,
    Unleash,
    Vanishing,
    Wither,
}

impl KeywordAbilities {
    /// Parse keywords from rules text
    pub fn from_rules_text(text: &str) -> Self {
        let mut abilities = HashSet::new();
        let mut ability_values = HashMap::new();

        // Simple keywords that would appear exactly in the text
        let simple_keywords = [
            (KeywordAbility::Deathtouch, "deathtouch"),
            (KeywordAbility::Defender, "defender"),
            (KeywordAbility::DoubleStrike, "double strike"),
            (KeywordAbility::FirstStrike, "first strike"),
            (KeywordAbility::Flash, "flash"),
            (KeywordAbility::Flying, "flying"),
            (KeywordAbility::Haste, "haste"),
            (KeywordAbility::Hexproof, "hexproof"),
            (KeywordAbility::Indestructible, "indestructible"),
            (KeywordAbility::Lifelink, "lifelink"),
            (KeywordAbility::Menace, "menace"),
            (KeywordAbility::Reach, "reach"),
            (KeywordAbility::Trample, "trample"),
            (KeywordAbility::Vigilance, "vigilance"),
            // Additional simple keywords
            (KeywordAbility::Changeling, "changeling"),
            (KeywordAbility::Infect, "infect"),
            (KeywordAbility::Devoid, "devoid"),
            (KeywordAbility::Wither, "wither"),
            (KeywordAbility::Fear, "fear"),
            (KeywordAbility::Flanking, "flanking"),
            (KeywordAbility::Phasing, "phasing"),
            (KeywordAbility::Skulk, "skulk"),
            (KeywordAbility::TotemArmor, "totem armor"),
            (KeywordAbility::Undying, "undying"),
        ];

        for (keyword, text_match) in simple_keywords {
            if text.to_lowercase().contains(text_match) {
                abilities.insert(keyword);
            }
        }

        // Keywords with values
        if let Some(protection_match) = text.to_lowercase().find("protection from ") {
            abilities.insert(KeywordAbility::Protection);
            let after_protection = &text[protection_match + "protection from ".len()..];
            if let Some(end) =
                after_protection.find(|c: char| c == '.' || c == ',' || c == '\n' || c == ';')
            {
                let protection_value = &after_protection[..end];
                ability_values.insert(
                    KeywordAbility::Protection,
                    protection_value.trim().to_string(),
                );
            }
        }

        if let Some(ward_match) = text.to_lowercase().find("ward ") {
            abilities.insert(KeywordAbility::Ward);
            let after_ward = &text[ward_match + "ward ".len()..];
            if let Some(end) =
                after_ward.find(|c: char| c == '.' || c == ',' || c == '\n' || c == ';')
            {
                let ward_value = &after_ward[..end];
                ability_values.insert(KeywordAbility::Ward, ward_value.trim().to_string());
            }
        }

        Self {
            abilities,
            ability_values,
        }
    }

    /// Parse keywords from a list of keyword strings (e.g., from MTGJSON)
    #[allow(dead_code)]
    pub fn from_keyword_list(keywords: &[String]) -> Self {
        let mut abilities = HashSet::new();
        let mut ability_values = HashMap::new();

        for keyword in keywords {
            let keyword_lower = keyword.to_lowercase();

            // Handle simple keywords
            match keyword_lower.as_str() {
                "deathtouch" => abilities.insert(KeywordAbility::Deathtouch),
                "defender" => abilities.insert(KeywordAbility::Defender),
                "double strike" => abilities.insert(KeywordAbility::DoubleStrike),
                "first strike" => abilities.insert(KeywordAbility::FirstStrike),
                "flash" => abilities.insert(KeywordAbility::Flash),
                "flying" => abilities.insert(KeywordAbility::Flying),
                "haste" => abilities.insert(KeywordAbility::Haste),
                "hexproof" => abilities.insert(KeywordAbility::Hexproof),
                "indestructible" => abilities.insert(KeywordAbility::Indestructible),
                "lifelink" => abilities.insert(KeywordAbility::Lifelink),
                "menace" => abilities.insert(KeywordAbility::Menace),
                "reach" => abilities.insert(KeywordAbility::Reach),
                "trample" => abilities.insert(KeywordAbility::Trample),
                "vigilance" => abilities.insert(KeywordAbility::Vigilance),
                "cascade" => abilities.insert(KeywordAbility::Cascade),
                // Add more simple keywords as needed
                _ => false,
            };

            // Handle keywords with values
            if keyword_lower.starts_with("protection from ") {
                abilities.insert(KeywordAbility::Protection);
                let value = keyword_lower
                    .trim_start_matches("protection from ")
                    .to_string();
                ability_values.insert(KeywordAbility::Protection, value);
            } else if keyword_lower.starts_with("ward ") {
                abilities.insert(KeywordAbility::Ward);
                let value = keyword_lower.trim_start_matches("ward ").to_string();
                ability_values.insert(KeywordAbility::Ward, value);
            }
        }

        Self {
            abilities,
            ability_values,
        }
    }
}
