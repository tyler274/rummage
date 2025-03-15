use bevy::prelude::*;

/// Marker component for text that has been spawned
#[derive(Component)]
pub struct SpawnedText;

/// Specialized component for card name text
#[derive(Component, Debug, Clone)]
pub struct CardNameText {
    /// The name of the card
    pub name: String,
}

/// Specialized component for card mana cost text
#[derive(Component, Debug, Clone)]
pub struct CardManaCostText {
    /// The mana cost of the card
    pub mana_cost: String,
}

/// Specialized component for card type line text
#[derive(Component, Debug, Clone)]
pub struct CardTypeLine {
    /// The type line of the card
    pub type_line: String,
}

/// Specialized component for card rules text
#[derive(Component, Debug, Clone)]
pub struct CardRulesText {
    /// The rules text of the card
    pub rules_text: String,
}

/// Specialized component for card power/toughness text
#[derive(Component, Debug, Clone)]
pub struct CardPowerToughness {
    /// The power/toughness of the card
    pub power_toughness: String,
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
    pub name: Name,
}

/// Bundle for text style components
#[derive(Bundle)]
pub struct CardTextStyleBundle {
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}

/// Marker component for card name text
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CardNameTextMarker;

/// Marker component for card mana cost text
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CardManaCostTextMarker;

/// Marker component for card type line text
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CardTypeLineMarker;

/// Marker component for card rules text
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CardRulesTextMarker;

/// Marker component for card power/toughness text
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct CardPowerToughnessMarker;
