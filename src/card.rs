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

impl std::fmt::Display for CreatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut types = Vec::new();

        // Check each creature type in alphabetical order
        if self.contains(CreatureType::ADVISOR) {
            types.push("Advisor");
        }
        if self.contains(CreatureType::ALLY) {
            types.push("Ally");
        }
        if self.contains(CreatureType::ANGEL) {
            types.push("Angel");
        }
        if self.contains(CreatureType::ARCHON) {
            types.push("Archon");
        }
        if self.contains(CreatureType::ARTIFICER) {
            types.push("Artificer");
        }
        if self.contains(CreatureType::ASSASSIN) {
            types.push("Assassin");
        }
        if self.contains(CreatureType::BEAST) {
            types.push("Beast");
        }
        if self.contains(CreatureType::BERSERKER) {
            types.push("Berserker");
        }
        if self.contains(CreatureType::BIRD) {
            types.push("Bird");
        }
        if self.contains(CreatureType::CAT) {
            types.push("Cat");
        }
        if self.contains(CreatureType::CLERIC) {
            types.push("Cleric");
        }
        if self.contains(CreatureType::CONSTRUCT) {
            types.push("Construct");
        }
        if self.contains(CreatureType::DEMON) {
            types.push("Demon");
        }
        if self.contains(CreatureType::DINOSAUR) {
            types.push("Dinosaur");
        }
        if self.contains(CreatureType::DRAGON) {
            types.push("Dragon");
        }
        if self.contains(CreatureType::DRAKE) {
            types.push("Drake");
        }
        if self.contains(CreatureType::DRUID) {
            types.push("Druid");
        }
        if self.contains(CreatureType::DRYAD) {
            types.push("Dryad");
        }
        if self.contains(CreatureType::EFREET) {
            types.push("Efreet");
        }
        if self.contains(CreatureType::ELF) {
            types.push("Elf");
        }
        if self.contains(CreatureType::ELEMENTAL) {
            types.push("Elemental");
        }
        if self.contains(CreatureType::GARGOYLE) {
            types.push("Gargoyle");
        }
        if self.contains(CreatureType::GIANT) {
            types.push("Giant");
        }
        if self.contains(CreatureType::GNOME) {
            types.push("Gnome");
        }
        if self.contains(CreatureType::GOBLIN) {
            types.push("Goblin");
        }
        if self.contains(CreatureType::GOLEM) {
            types.push("Golem");
        }
        if self.contains(CreatureType::HORROR) {
            types.push("Horror");
        }
        if self.contains(CreatureType::HUMAN) {
            types.push("Human");
        }
        if self.contains(CreatureType::HYDRA) {
            types.push("Hydra");
        }
        if self.contains(CreatureType::IMP) {
            types.push("Imp");
        }
        if self.contains(CreatureType::INCARNATION) {
            types.push("Incarnation");
        }
        if self.contains(CreatureType::INSECT) {
            types.push("Insect");
        }
        if self.contains(CreatureType::KAVU) {
            types.push("Kavu");
        }
        if self.contains(CreatureType::KNIGHT) {
            types.push("Knight");
        }
        if self.contains(CreatureType::LHURGOYF) {
            types.push("Lhurgoyf");
        }
        if self.contains(CreatureType::LIZARD) {
            types.push("Lizard");
        }
        if self.contains(CreatureType::MERCENARY) {
            types.push("Mercenary");
        }
        if self.contains(CreatureType::MERFOLK) {
            types.push("Merfolk");
        }
        if self.contains(CreatureType::MONK) {
            types.push("Monk");
        }
        if self.contains(CreatureType::MONKEY) {
            types.push("Monkey");
        }
        if self.contains(CreatureType::NYMPH) {
            types.push("Nymph");
        }
        if self.contains(CreatureType::OOZE) {
            types.push("Ooze");
        }
        if self.contains(CreatureType::PHOENIX) {
            types.push("Phoenix");
        }
        if self.contains(CreatureType::PHYREXIAN) {
            types.push("Phyrexian");
        }
        if self.contains(CreatureType::REBEL) {
            types.push("Rebel");
        }
        if self.contains(CreatureType::ROGUE) {
            types.push("Rogue");
        }
        if self.contains(CreatureType::SCOUT) {
            types.push("Scout");
        }
        if self.contains(CreatureType::SHAMAN) {
            types.push("Shaman");
        }
        if self.contains(CreatureType::SLIVER) {
            types.push("Sliver");
        }
        if self.contains(CreatureType::SNAKE) {
            types.push("Snake");
        }
        if self.contains(CreatureType::SOLDIER) {
            types.push("Soldier");
        }
        if self.contains(CreatureType::SPIDER) {
            types.push("Spider");
        }
        if self.contains(CreatureType::SPIRIT) {
            types.push("Spirit");
        }
        if self.contains(CreatureType::SQUIRREL) {
            types.push("Squirrel");
        }
        if self.contains(CreatureType::TREEFOLK) {
            types.push("Treefolk");
        }
        if self.contains(CreatureType::VAMPIRE) {
            types.push("Vampire");
        }
        if self.contains(CreatureType::WARRIOR) {
            types.push("Warrior");
        }
        if self.contains(CreatureType::WEREWOLF) {
            types.push("Werewolf");
        }
        if self.contains(CreatureType::WIZARD) {
            types.push("Wizard");
        }
        if self.contains(CreatureType::WOLF) {
            types.push("Wolf");
        }
        if self.contains(CreatureType::WURM) {
            types.push("Wurm");
        }
        if self.contains(CreatureType::ZOMBIE) {
            types.push("Zombie");
        }

        if types.is_empty() {
            write!(f, "None")
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

        // Add supertypes first
        if self.contains(CardTypes::BASIC) {
            parts.push("Basic");
        }
        if self.contains(CardTypes::LEGENDARY) {
            parts.push("Legendary");
        }
        if self.contains(CardTypes::SNOW) {
            parts.push("Snow");
        }
        if self.contains(CardTypes::WORLD) {
            parts.push("World");
        }

        // Add main types in canonical order
        if self.contains(CardTypes::ARTIFACT) {
            parts.push("Artifact");
        }
        if self.contains(CardTypes::CREATURE) {
            parts.push("Creature");
        }
        if self.contains(CardTypes::ENCHANTMENT) {
            parts.push("Enchantment");
        }
        if self.contains(CardTypes::INSTANT) {
            parts.push("Instant");
        }
        if self.contains(CardTypes::LAND) {
            parts.push("Land");
        }
        if self.contains(CardTypes::PLANESWALKER) {
            parts.push("Planeswalker");
        }
        if self.contains(CardTypes::SORCERY) {
            parts.push("Sorcery");
        }
        if self.contains(CardTypes::TRIBAL) {
            parts.push("Tribal");
        }

        // Add subtypes
        if self.contains(CardTypes::SAGA) {
            parts.push("Saga");
        }
        if self.contains(CardTypes::EQUIPMENT) {
            parts.push("Equipment");
        }
        if self.contains(CardTypes::AURA) {
            parts.push("Aura");
        }
        if self.contains(CardTypes::VEHICLE) {
            parts.push("Vehicle");
        }
        if self.contains(CardTypes::FOOD) {
            parts.push("Food");
        }
        if self.contains(CardTypes::CLUE) {
            parts.push("Clue");
        }
        if self.contains(CardTypes::TREASURE) {
            parts.push("Treasure");
        }
        if self.contains(CardTypes::FORTIFICATION) {
            parts.push("Fortification");
        }
        if self.contains(CardTypes::CONTRAPTION) {
            parts.push("Contraption");
        }

        // Add land subtypes
        if self.contains(CardTypes::PLAINS) {
            parts.push("Plains");
        }
        if self.contains(CardTypes::ISLAND) {
            parts.push("Island");
        }
        if self.contains(CardTypes::SWAMP) {
            parts.push("Swamp");
        }
        if self.contains(CardTypes::MOUNTAIN) {
            parts.push("Mountain");
        }
        if self.contains(CardTypes::FOREST) {
            parts.push("Forest");
        }

        if parts.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", parts.join(" "))
        }
    }
}

/// Formats a card's complete type line, including creature types if applicable
pub fn format_type_line(types: &CardTypes, card_details: &CardDetails) -> String {
    let mut result = types.to_string();

    // If it's a creature, append the creature types
    if let CardDetails::Creature(creature) = card_details {
        if !creature.creature_type.is_empty() {
            result.push_str(" — ");
            result.push_str(&creature.creature_type.to_string());
        }
    }
    // Add other type-specific additions here if needed
    // For example, for Enchantment - Aura, Artifact - Equipment, etc.
    else if let CardDetails::Enchantment(enchantment) = card_details {
        if let Some(enchantment_type) = &enchantment.enchantment_type {
            result.push_str(" — ");
            result.push_str(enchantment_type);
        }
    } else if let CardDetails::Artifact(artifact) = card_details {
        if let Some(artifact_type) = &artifact.artifact_type {
            result.push_str(" — ");
            result.push_str(artifact_type);
        }
    } else if let CardDetails::Land(land) = card_details {
        if let Some(land_type) = &land.land_type {
            result.push_str(" — ");
            result.push_str(land_type);
        }
    }

    result
}

#[derive(Bundle)]
pub struct CardBundle {
    pub sprite: Sprite,
    pub card: Card,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub cost: Mana,
    pub types: CardTypes,
    pub card_details: CardDetails,
    pub rules_text: String,
}

impl Card {
    /// Gets the complete type line for the card, including subtypes
    ///
    /// This includes:
    /// - Supertypes (e.g., "Legendary", "Basic")
    /// - Card types (e.g., "Creature", "Instant")
    /// - Subtypes (e.g., "Human Wizard", "Equipment")
    ///
    /// Examples:
    /// - "Legendary Creature — Human Wizard"
    /// - "Artifact — Equipment"
    /// - "Basic Land — Forest"
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

#[derive(Component, Debug)]
#[allow(dead_code)] // These fields are used for text rendering and may be needed later
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
    #[allow(dead_code)]
    pub fn infer_from_name_and_text(text: &str, existing_types: Self) -> Self {
        let types = existing_types;
        let _text = text.to_lowercase();

        // Add type inference logic here
        // For now, just return existing types
        types
    }

    #[allow(dead_code)]
    pub fn apply_retroactive_types(name: &str, existing_types: Self) -> Self {
        let types = existing_types;
        let _name = name.to_lowercase();

        // Add retroactive type application logic here
        // For now, just return existing types
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
