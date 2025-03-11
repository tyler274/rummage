pub mod hdr;

// Add modules from cards
pub mod artifacts;
pub mod black;
pub mod blue;
pub mod green;
pub mod mtgjson;
pub mod penacony;
pub mod red;
pub mod white;

// Import external crates
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy::text::JustifyText;
use bevy::utils::HashMap;
use bitflags::bitflags;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

// Import internal modules
use crate::mana::Mana;
use crate::menu::GameMenuState;
use crate::text::{self, CardTextContent, CardTextType, SpawnedText};

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

#[derive(Component, Debug, Clone)]
pub struct CardBundle {
    pub sprite: Sprite,
    pub card: Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

/// Represents a Magic: The Gathering card with all its properties
#[derive(Component, Debug, Clone)]
pub struct Card {
    pub name: String,
    pub cost: Mana,
    pub types: CardTypes,
    pub card_details: CardDetails,
    pub rules_text: String,
}

impl Card {
    pub fn new(
        name: &str,
        cost: Mana,
        types: CardTypes,
        details: CardDetails,
        rules_text: &str,
    ) -> Self {
        Card {
            name: name.to_string(),
            cost,
            types,
            card_details: details,
            rules_text: rules_text.to_string(),
        }
    }

    pub fn type_line(&self) -> String {
        format_type_line(&self.types, &self.card_details)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDetails {
    Creature(CreatureCard),
    Planeswalker { loyalty: i32 },
    Instant(SpellCard),
    Sorcery(SpellCard),
    Enchantment(EnchantmentCard),
    Artifact(ArtifactCard),
    Land(LandCard),
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellCard {
    pub spell_type: SpellType,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellType {
    Instant,
    Sorcery,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnchantmentCard {
    pub enchantment_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactCard {
    pub artifact_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandCard {
    pub land_type: Option<String>,
    pub produces: Vec<String>,
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

pub fn handle_card_dragging(
    mut card_query: Query<(Entity, &mut Transform, &mut Draggable, &GlobalTransform), With<Card>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::camera::components::GameCamera>>,
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

pub fn debug_render_text_positions(
    mut gizmos: Gizmos,
    card_query: Query<(&Transform, &Card), With<Card>>,
    config: Res<text::DebugConfig>,
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

/// Plugin for card management
pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_card_dragging, debug_render_text_positions)
                .run_if(in_state(GameMenuState::InGame)),
        );
    }
}

// Add get_example_cards function from cards module
pub fn get_example_cards(owner: Entity) -> Vec<Card> {
    let mut cards = Vec::new();
    cards.extend(artifacts::get_artifact_cards());
    cards.extend(black::get_black_cards());
    cards.extend(blue::get_blue_cards());
    cards.extend(green::get_green_cards());
    cards.extend(red::get_red_cards());
    cards.extend(white::get_white_cards());
    cards
}
