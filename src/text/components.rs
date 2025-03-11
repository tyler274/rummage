use bevy::prelude::*;
use bevy::text::JustifyText;

/// Marker component for text that has been spawned
#[derive(Component)]
pub struct SpawnedText;

/// Component for storing card text content
#[derive(Component, Debug, Clone)]
pub struct CardTextContent {
    /// The name of the card
    pub name: String,
    /// The mana cost of the card
    pub mana_cost: String,
    /// The type line of the card
    pub type_line: String,
    /// The rules text of the card
    pub rules_text: String,
    /// The power/toughness of the card (if applicable)
    pub power_toughness: Option<String>,
}

/// Configuration for debug visualization
#[derive(Resource, Default)]
pub struct DebugConfig {
    /// Whether to show text positions
    pub show_text_positions: bool,
}

/// Enum for different types of card text
#[derive(Component, Debug, Clone)]
pub enum CardTextType {
    /// The name of the card
    Name,
    /// The mana cost of the card
    ManaCost,
    /// The type line of the card
    TypeLine,
    /// The rules text of the card
    RulesText,
    /// The power/toughness of the card
    PowerToughness,
    /// Debug visualization
    Debug,
}

/// Component for storing text layout information
#[derive(Component, Debug, Clone)]
pub struct TextLayoutInfo {
    /// The position of the text relative to the card
    pub position: Vec2,
    /// The size of the text bounds
    pub size: Vec2,
    /// The alignment of the text
    pub alignment: JustifyText,
}

/// Default implementations for text layout
impl Default for TextLayoutInfo {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::new(100.0, 50.0),
            alignment: JustifyText::Left,
        }
    }
}
