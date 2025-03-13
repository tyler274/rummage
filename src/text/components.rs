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

impl Default for CardTextType {
    fn default() -> Self {
        Self::Name
    }
}

/// Information about text layout, used by multiple text components
#[derive(Component, Clone, Debug)]
pub struct TextLayoutInfo {
    /// The alignment of the text
    #[allow(dead_code)]
    pub alignment: JustifyText,
}

/// Default implementations for text layout
impl Default for TextLayoutInfo {
    fn default() -> Self {
        Self {
            alignment: JustifyText::Left,
        }
    }
}

/// Bundle for card text components
#[derive(Bundle)]
pub struct CardTextBundle {
    pub text_2d: Text2d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
    pub card_text_type: CardTextType,
    pub text_layout_info: TextLayoutInfo,
    pub name: Name,
}

/// Bundle for text style components
#[derive(Bundle)]
pub struct CardTextStyleBundle {
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}
