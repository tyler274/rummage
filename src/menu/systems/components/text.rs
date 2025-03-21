use crate::menu::{
    components::{MenuItem, MenuTextBundle},
    styles::text_font_with_font,
};
use bevy::prelude::*;
use bevy::text::JustifyText;

/// Creates a Text2d component bundle with the specified text
#[derive(Bundle)]
pub struct CustomText2dBundle {
    /// 2D text marker component
    pub text_2d_marker: Text2d,
    /// Text content
    pub text: Text,
    /// Font configuration
    pub font: TextFont,
    /// Text color
    pub color: TextColor,
    /// Name component
    pub name: Name,
}

impl CustomText2dBundle {
    /// Create a new Text2d bundle with the given text
    pub fn new(text: &str, asset_server: &AssetServer, font_size: f32) -> Self {
        Self {
            text_2d_marker: Text2d::default(),
            text: Text::new(text),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                ..default()
            },
            color: TextColor(Color::WHITE),
            name: Name::new(format!("{} Text", text)),
        }
    }
}

/// Creates a menu text bundle with standard styling
pub fn create_menu_text(text: &str, _asset_server: &AssetServer, _z_index: i32) -> MenuTextBundle {
    MenuTextBundle {
        text: Text::new(text),
        text_layout: TextLayout::default(),
        menu_item: MenuItem,
        visibility: Visibility::Visible,
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
        z_index: ZIndex::default(),
    }
}

/// Creates a menu text bundle with justification
pub fn create_menu_text_justified(
    text: &str,
    _asset_server: &AssetServer,
    _z_index: i32,
    justify: JustifyText,
) -> MenuTextBundle {
    MenuTextBundle {
        text: Text::new(text),
        text_layout: TextLayout::new_with_justify(justify),
        menu_item: MenuItem,
        visibility: Visibility::Visible,
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
        z_index: ZIndex::default(),
    }
}

/// Bundle for menu button text
#[derive(Bundle)]
pub struct MenuButtonTextBundle {
    /// Text component
    pub text: Text,
    /// Font component
    pub font: TextFont,
    /// Color component
    pub color: TextColor,
    /// Layout component
    pub layout: TextLayout,
    /// Text2d component for 2D rendering
    pub text_2d_marker: Text2d,
    /// Name component
    pub name: Name,
    /// Menu item marker
    pub menu_item: MenuItem,
    /// Visibility component
    pub visibility: Visibility,
    /// Inherited visibility component
    pub inherited_visibility: InheritedVisibility,
    /// View visibility component
    pub view_visibility: ViewVisibility,
}

impl MenuButtonTextBundle {
    /// Create a new menu button text bundle
    pub fn new(asset_server: &AssetServer, text_str: &str, _z_index: i32) -> Self {
        Self {
            text: Text::new(text_str),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::default(),
            text_2d_marker: Text2d::default(),
            name: Name::new(format!("{} Button Text", text_str)),
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
