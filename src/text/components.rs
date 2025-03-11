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

/// Bundle for card text components to avoid having too many components in a tuple
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

impl CardTextBundle {
    /// Create a new text bundle for cards with standard configuration
    pub fn new(
        text: String,
        font: Handle<Font>,
        font_size: f32,
        color: Color,
        card_position: Vec3,
        local_offset: Vec2,
        alignment: JustifyText,
    ) -> Self {
        Self {
            text_2d: Text2d::new(text.clone()),
            transform: Transform::from_translation(Vec3::new(local_offset.x, local_offset.y, 0.1)),
            global_transform: GlobalTransform::default(),
            text_font: TextFont {
                font,
                font_size,
                ..default()
            },
            text_color: TextColor(color),
            text_layout: TextLayout::new_with_justify(alignment),
            card_text_type: CardTextType::default(),
            text_layout_info: TextLayoutInfo {
                position: card_position.truncate() + local_offset,
                size: Vec2::new(100.0, 50.0),
                alignment,
            },
            name: Name::new(format!("Card Text: {}", text)),
        }
    }
}

/// Bundle for text style components
#[derive(Bundle)]
pub struct CardTextStyleBundle {
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}
