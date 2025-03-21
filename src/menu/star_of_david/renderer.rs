use crate::camera::components::AppLayer;
use bevy::prelude::*;

/// Creates a container for the logo group (Star of David + text)
fn create_logo() -> impl Bundle {
    (
        Node {
            width: Val::Px(300.0),
            height: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        AppLayer::Menu.layer(), // Top-level element needs explicit RenderLayers
        Interaction::None,
    )
}
