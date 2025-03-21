use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, PositionType, UiRect, Val};

/// Component for the Star of David image
#[derive(Component)]
pub struct StarOfDavid;

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    info!("Creating StarOfDavid bundle as UI component");
    (
        // Use individual components instead of NodeBundle
        Node {
            width: Val::Px(120.0),
            height: Val::Px(120.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Column,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::NONE),
        StarOfDavid,
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
        // Add this to ensure we're in a UI hierarchy
        ZIndex::default(),
        Transform::default(),
        GlobalTransform::default(),
    )
}
