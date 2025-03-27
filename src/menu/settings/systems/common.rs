use bevy::prelude::*;
use bevy::text::JustifyText;
use bevy::ui::{AlignItems, JustifyContent, UiRect, Val};

use crate::camera::components::AppLayer;
use crate::menu::components::MenuItem;
use crate::menu::settings::components::*;
use crate::menu::styles::*;

/// Text color for settings menu
pub const TEXT_COLOR: Color = Color::WHITE;

/// Creates a settings button with text
pub fn spawn_settings_button(parent: &mut ChildBuilder, text: &str, action: SettingsButtonAction) {
    info!("Spawning settings button: {}", text);
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(40.0),
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            action,
            AppLayer::Menu.layer(),
            SettingsMenuItem,
            MenuItem,
            ZIndex::from(crate::menu::components::ZLayers::MenuButtons),
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new(format!("{} Button", text)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                AppLayer::Menu.layer(),
                SettingsMenuItem,
                MenuItem,
                ZIndex::from(crate::menu::components::ZLayers::MenuButtonText),
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
            ));
        });
}

/// Creates a settings container node
pub fn spawn_settings_container(parent: &mut ChildBuilder) -> Entity {
    parent
        .spawn((
            Node {
                width: Val::Px(400.0),
                height: Val::Px(300.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            AppLayer::Menu.layer(),
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new("Settings Container"),
            MenuItem,
            SettingsMenuItem,
            ZIndex::from(crate::menu::components::ZLayers::MenuContainer),
        ))
        .id()
}

/// Creates a settings title
pub fn spawn_settings_title(parent: &mut ChildBuilder, title: &str) {
    parent.spawn((
        Text::new(title),
        TextFont {
            font_size: 35.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(TEXT_COLOR),
        AppLayer::Menu.layer(),
        MenuItem,
        SettingsMenuItem,
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        Name::new(format!("{} Title", title)),
        ZIndex::from(crate::menu::components::ZLayers::MenuButtonText),
    ));
}

/// Creates a settings root node
pub fn spawn_settings_root(commands: &mut Commands, background_color: Color, name: &str) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(background_color),
            MenuItem,
            SettingsMenuItem,
            AppLayer::Menu.layer(),
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new(format!("{} Root Node", name)),
            ZIndex::from(crate::menu::components::ZLayers::Background),
        ))
        .id()
}

/// Creates a toggle setting with a label and current value
pub fn create_toggle_setting(parent: &mut ChildBuilder, label: &str, value: bool) {
    parent
        .spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            AppLayer::Menu.layer(),
            MenuItem,
            SettingsMenuItem,
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
            Name::new("Toggle Setting"),
        ))
        .with_children(|parent| {
            // Label
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Toggle Setting Label"),
            ));

            // Value
            parent.spawn((
                Text::new(if value { "On" } else { "Off" }),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                AppLayer::Menu.layer(),
                MenuItem,
                SettingsMenuItem,
                Visibility::Visible,
                InheritedVisibility::VISIBLE,
                Name::new("Toggle Setting Value"),
            ));
        });
}
