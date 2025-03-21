use bevy::prelude::*;
use crate::menu::{
    components::{MenuItem, MenuTextBundle},
    styles::text_style_with_font,
};

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