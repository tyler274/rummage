use crate::mana::Mana;
use bevy::{input::mouse::MouseButton, prelude::*, sprite::Sprite};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

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

#[derive(Bundle)]
pub struct CardBundle {
    pub sprite_bundle: Sprite,
    pub card: Card,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub cost: Mana,
    pub types: CardTypes,
    pub card_details: CardDetails,
    pub rules_text: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDetails {
    Creature(CreatureCard),
    Planeswalker { loyalty: i32 },
    // Add other specific card type details as needed
    Other, // For cards that don't need additional details
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatureCard {
    pub power: i32,
    pub toughness: i32,
    pub creature_type: CreatureType,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct CreatureOnField {
    pub power_modifier: i64,
    pub toughness_modifier: i64,
    pub battle_damage: u64,
    pub token: bool,
}

#[derive(Component)]
pub struct Draggable {
    pub dragging: bool,
    pub drag_offset: Vec2,
    pub z_index: f32,
}

#[derive(Component)]
pub struct CardTextContent {
    pub text: String,
    pub text_type: CardTextType,
}

#[derive(Component)]
pub struct SpawnedText;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CardTextType {
    Name,
    Cost,
    Type,
    PowerToughness,
    RulesText,
}

#[derive(Resource)]
pub struct DebugConfig {
    pub show_text_positions: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_text_positions: false,
        }
    }
}

impl CreatureType {
    fn infer_from_name_and_text(name: &str, rules_text: &str) -> Self {
        let mut creature_types = Self::NONE;
        let text = format!("{} {}", name, rules_text).to_lowercase();

        // Phyrexian inference
        if text.contains("phyrexian") || text.contains("compleated") || text.contains("praetors") {
            creature_types |= Self::PHYREXIAN;
        }

        // Common race/class patterns
        let patterns = [
            ("artificer", Self::ARTIFICER),
            ("assassin", Self::ASSASSIN),
            ("berserker", Self::BERSERKER),
            ("knight", Self::KNIGHT),
            ("warrior", Self::WARRIOR),
            ("wizard", Self::WIZARD),
            ("cleric", Self::CLERIC),
            ("druid", Self::DRUID),
            ("shaman", Self::SHAMAN),
            ("rogue", Self::ROGUE),
            ("soldier", Self::SOLDIER),
            ("monk", Self::MONK),
            ("sphinx", Self::SPHINX),
            ("imp", Self::IMP),
            ("gargoyle", Self::GARGOYLE),
            ("goyf", Self::LHURGOYF),
            ("ooze", Self::OOZE),
            ("squirrel", Self::SQUIRREL),
            ("kavu", Self::KAVU),
            ("lion", Self::CAT),
            ("drake", Self::DRAKE),
            ("gnome", Self::GNOME),
            ("archon", Self::ARCHON),
            ("rootwalla", Self::LIZARD),
            ("insect", Self::INSECT),
            ("wasp", Self::INSECT),
            ("construct", Self::CONSTRUCT),
            ("golem", Self::GOLEM),
            ("treefolk", Self::TREEFOLK),
            ("sliver", Self::SLIVER),
            ("snake", Self::SNAKE),
            ("serpent", Self::SNAKE),
            ("wolf", Self::WOLF),
            ("werewolf", Self::WEREWOLF),
            ("vampire", Self::VAMPIRE),
            ("scout", Self::SCOUT),
            ("advisor", Self::ADVISOR),
            ("ally", Self::ALLY),
            ("mercenary", Self::MERCENARY),
            ("rebel", Self::REBEL),
            ("spider", Self::SPIDER),
        ];

        for (pattern, creature_type) in patterns.iter() {
            if text.contains(pattern) {
                creature_types |= *creature_type;
            }
        }

        // Special cases based on card names
        if text.contains("manufactor") || text.contains("cryptkeeper") || text.contains("millikin")
        {
            creature_types |= Self::CONSTRUCT;
        }

        if text.contains("thought monitor") || text.contains("sojourner's companion") {
            creature_types |= Self::CONSTRUCT;
        }

        if text.contains("sanctum weaver") {
            creature_types |= Self::DRYAD;
        }

        if text.contains("glimmer bairn") {
            creature_types |= Self::SPIRIT;
        }

        // Werewolf transformations
        if text.contains("transform") && text.contains("wolf") {
            creature_types |= Self::WEREWOLF;
        }

        creature_types
    }

    fn apply_retroactive_types(name: &str, existing_types: Self) -> Self {
        let mut types = existing_types;

        // Known retroactive additions
        let retroactive_types = [
            ("Urza", Self::HUMAN | Self::ARTIFICER),
            ("Mishra", Self::HUMAN | Self::ARTIFICER),
            ("Yawgmoth", Self::PHYREXIAN),
            ("Gix", Self::PHYREXIAN),
            ("Xantcha", Self::PHYREXIAN),
            ("Ashnod", Self::HUMAN | Self::ARTIFICER),
            ("Tawnos", Self::HUMAN | Self::ARTIFICER),
            ("Slobad", Self::GOBLIN | Self::ARTIFICER),
            ("Glissa", Self::PHYREXIAN | Self::ELF),
            ("Ragavan", Self::MONKEY),
            ("Sythis", Self::NYMPH),
            ("Yusri", Self::EFREET),
            ("Wonder", Self::INCARNATION),
        ];

        for (character_name, character_types) in retroactive_types.iter() {
            if name.contains(character_name) {
                types |= *character_types;
            }
        }

        types
    }
}

pub fn handle_card_dragging(
    mut card_query: Query<(Entity, &mut Transform, &mut Draggable, &GlobalTransform), With<Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Safely get window and camera
    let Ok(window) = windows.get_single() else {
        return; // No window available
    };

    let Ok((camera, camera_transform)) = camera_q.get_single() else {
        return; // No camera available
    };

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor position to world coordinates
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Handle mouse press - start dragging
            if mouse_button.just_pressed(MouseButton::Left) {
                let mut highest_z = f32::NEG_INFINITY;
                let mut top_card = None;

                // First pass: find the card with highest z-index at cursor position
                for (entity, _, draggable, global_transform) in card_query.iter() {
                    let card_pos = global_transform.translation().truncate();
                    // Using actual card dimensions (672x936) to match Magic card proportions
                    let card_size = Vec2::new(672.0, 936.0);
                    // No additional scaling needed since viewport_to_world_2d already gives us
                    // coordinates in the same space as our card positions
                    let scaled_size = card_size * 1.0;

                    // Check if the cursor is within the card bounds
                    // The hit detection area now perfectly matches the visible card boundaries
                    if world_pos.x >= card_pos.x - scaled_size.x / 2.0
                        && world_pos.x <= card_pos.x + scaled_size.x / 2.0
                        && world_pos.y >= card_pos.y - scaled_size.y / 2.0
                        && world_pos.y <= card_pos.y + scaled_size.y / 2.0
                    {
                        if draggable.z_index > highest_z {
                            highest_z = draggable.z_index;
                            top_card = Some(entity);
                        }
                    }
                }

                // Second pass: start dragging only the top card
                if let Some(top_entity) = top_card {
                    // Find the highest z-index among all cards
                    let mut max_z = highest_z;
                    for (_, _, draggable, _) in card_query.iter() {
                        max_z = max_z.max(draggable.z_index);
                    }

                    for (entity, mut transform, mut draggable, global_transform) in
                        card_query.iter_mut()
                    {
                        if entity == top_entity {
                            let card_pos = global_transform.translation().truncate();
                            draggable.dragging = true;
                            draggable.drag_offset = card_pos - world_pos;
                            // Set the dragged card's z-index higher than all others
                            draggable.z_index = max_z + 1.0;
                            transform.translation.z = max_z + 1.0;
                        }
                    }
                }
            }

            // Handle mouse release - stop dragging and update z-index
            if mouse_button.just_released(MouseButton::Left) {
                let mut max_z = f32::NEG_INFINITY;

                // First find the highest z-index
                for (_, _, draggable, _) in card_query.iter() {
                    max_z = max_z.max(draggable.z_index);
                }

                // Then update the previously dragged card
                for (_, _, mut draggable, _) in card_query.iter_mut() {
                    if draggable.dragging {
                        draggable.dragging = false;
                        draggable.z_index = max_z + 1.0; // Place it on top
                    }
                }
            }

            // Update position of dragged cards
            for (_, mut transform, draggable, _) in card_query.iter_mut() {
                if draggable.dragging {
                    let new_pos = world_pos + draggable.drag_offset;
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                    // Maintain the z-index we set when dragging started
                    transform.translation.z = draggable.z_index;
                }
            }
        }
    }
}

pub fn card_types_to_string(types: &CardTypes) -> String {
    let mut type_strings = Vec::new();

    // Add supertypes first
    if types.contains(CardTypes::BASIC) {
        type_strings.push("Basic");
    }
    if types.contains(CardTypes::LEGENDARY) {
        type_strings.push("Legendary");
    }
    if types.contains(CardTypes::HISTORIC) {
        type_strings.push("Historic");
    }
    if types.contains(CardTypes::SNOW) {
        type_strings.push("Snow");
    }
    if types.contains(CardTypes::WORLD) {
        type_strings.push("World");
    }

    // Add main types in canonical order
    if types.contains(CardTypes::ARTIFACT) {
        type_strings.push("Artifact");
    }
    if types.contains(CardTypes::CREATURE) {
        type_strings.push("Creature");
    }
    if types.contains(CardTypes::ENCHANTMENT) {
        type_strings.push("Enchantment");
    }
    if types.contains(CardTypes::INSTANT) {
        type_strings.push("Instant");
    }
    if types.contains(CardTypes::LAND) {
        type_strings.push("Land");
    }
    if types.contains(CardTypes::PLANESWALKER) {
        type_strings.push("Planeswalker");
    }
    if types.contains(CardTypes::SORCERY) {
        type_strings.push("Sorcery");
    }
    if types.contains(CardTypes::TRIBAL) {
        type_strings.push("Tribal");
    }

    type_strings.join(" ")
}

pub fn debug_render_text_positions(
    mut gizmos: Gizmos,
    card_query: Query<(&Transform, &Card), With<Card>>,
    config: Res<DebugConfig>,
) {
    if !config.show_text_positions {
        return;
    }

    for (transform, _) in card_query.iter() {
        let card_pos = transform.translation.truncate();
        let card_width = 100.0;
        let card_height = card_width * 1.4;

        // Note: Using Color::srgb instead of Color::rgb as rgb is deprecated

        // Name position (top left) - red dot
        let name_pos = card_pos + Vec2::new(-card_width * 0.25, card_height * 0.35);
        gizmos.circle_2d(name_pos, 3.0, Color::srgb(1.0, 0.0, 0.0));

        // Mana cost position (top right) - blue dot
        let cost_pos = card_pos + Vec2::new(card_width * 0.35, card_height * 0.35);
        gizmos.circle_2d(cost_pos, 3.0, Color::srgb(0.0, 0.0, 1.0));

        // Type position (middle center) - green dot
        let type_pos = card_pos + Vec2::new(0.0, card_height * 0.1);
        gizmos.circle_2d(type_pos, 3.0, Color::srgb(0.0, 1.0, 0.0));

        // Power/Toughness position (bottom right) - yellow dot
        let pt_pos = card_pos + Vec2::new(card_width * 0.35, -card_height * 0.35);
        gizmos.circle_2d(pt_pos, 3.0, Color::srgb(1.0, 1.0, 0.0));
    }
}
