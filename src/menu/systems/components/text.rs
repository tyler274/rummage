use crate::menu::{
    components::{MenuItem, MenuTextBundle},
    styles::text_style_with_font,
};
use bevy::prelude::*;
use bevy::ui::JustifyText;

/// Creates a Text2d component bundle with the specified text
pub fn create_text_2d_bundle(
    text: &str,
    asset_server: &AssetServer,
    font_size: f32,
) -> impl Bundle {
    (
        Text::from_section(text, text_style_with_font(asset_server, font_size)),
        Text2d,
        Name::new(format!("{} Text", text)),
    )
}

/// Creates a menu text bundle with standard styling
pub fn create_menu_text(text: &str, asset_server: &AssetServer, z_index: i32) -> MenuTextBundle {
    MenuTextBundle {
        text: Text::from_section(text, text_style_with_font(asset_server, 24.0)),
        text_layout: TextLayout::default(),
        menu_item: MenuItem,
        visibility: Visibility::Visible,
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
        z_index: ZIndex::Global(z_index),
    }
}

/// Creates a menu text bundle with justified text
pub fn create_menu_text_justified(
    text: &str,
    asset_server: &AssetServer,
    z_index: i32,
    justify: JustifyText,
) -> MenuTextBundle {
    MenuTextBundle {
        text: Text::from_section(text, text_style_with_font(asset_server, 24.0)),
        text_layout: TextLayout::new_with_justify(justify),
        menu_item: MenuItem,
        visibility: Visibility::Visible,
        inherited_visibility: InheritedVisibility::default(),
        view_visibility: ViewVisibility::default(),
        z_index: ZIndex::Global(z_index),
    }
}

/// Bundle for text components in menu buttons
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
    /// Text2d component
    pub text_2d: Text2d,
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
    /// Z-index for layering
    pub z_index: ZIndex,
}

impl MenuButtonTextBundle {
    /// Create a new menu button text bundle
    pub fn new(asset_server: &AssetServer, text_str: &str, z_index: i32) -> Self {
        Self {
            text: Text::new(text_str.to_string()),
            font: TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            text_2d: Text2d,
            name: Name::new(format!("{} Button Text", text_str)),
            menu_item: MenuItem,
            visibility: Visibility::Visible,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            z_index: ZIndex::Global(z_index),
        }
    }
}
