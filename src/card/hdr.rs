use std::time::Duration;

use bevy::{prelude::*, render::mesh::Mesh};

use crate::card::Card;
use crate::menu::GameMenuState;

/// Component to mark an entity as an HDR emissive card for visual effects
#[derive(Component)]
pub struct EmissiveCard {
    pub original_card: Entity,
    pub glow_intensity: f32,
    pub pulse_timer: Timer,
}

#[derive(Component)]
pub struct EmissiveCardMaterial(pub Handle<StandardMaterial>);

// Type that will store our card glow color rules
type CardColorRule = Box<dyn Fn(&Card) -> bool + Send + Sync>;

// Struct to make the trait object approach cleaner
pub struct CardEmissiveProperties {
    pub matcher: CardColorRule,
    pub color: Color,
    pub intensity: f32,
}

/// Spawns 3D emissive cards that demonstrate HDR rendering and bloom effects
///
/// Places 3D card meshes in the scene with emissive materials to show HDR capabilities:
/// - Cards with varying emissive intensities
/// - Pulsing glow effects
/// - Bloom visible on bright card parts
pub fn spawn_emissive_cards(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cards: Query<(Entity, &Card, &Transform)>,
) {
    // Define card color rules - using boxed trait objects to allow different closures
    let color_rules = vec![
        // White cards - bright white/gold glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| card.cost.white > 0),
            color: Color::srgb(1.0, 0.95, 0.8),
            intensity: 5.0,
        },
        // Blue cards - cool blue glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| card.cost.blue > 0),
            color: Color::srgb(0.2, 0.5, 1.0),
            intensity: 4.0,
        },
        // Black cards - dark purple/black glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| card.cost.black > 0),
            color: Color::srgb(0.5, 0.1, 0.8),
            intensity: 3.0,
        },
        // Red cards - fiery red/orange glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| card.cost.red > 0),
            color: Color::srgb(1.0, 0.3, 0.1),
            intensity: 4.5,
        },
        // Green cards - nature green glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| card.cost.green > 0),
            color: Color::srgb(0.2, 1.0, 0.3),
            intensity: 3.5,
        },
        // Artifact/colorless - silver/chrome glow
        CardEmissiveProperties {
            matcher: Box::new(|card: &Card| {
                card.cost.white == 0
                    && card.cost.blue == 0
                    && card.cost.black == 0
                    && card.cost.red == 0
                    && card.cost.green == 0
            }),
            color: Color::srgb(0.8, 0.8, 0.9),
            intensity: 3.0,
        },
    ];

    // Create 3D card mesh for each card
    for (entity, card, transform) in cards.iter() {
        // Find the matching color rule
        let mut matched_color = Color::srgb(0.8, 0.8, 0.8);
        let mut matched_intensity = 3.0;

        for rule in &color_rules {
            if (rule.matcher)(card) {
                matched_color = rule.color;
                matched_intensity = rule.intensity;
                break;
            }
        }

        // Adjust the emissive color based on the card properties
        let emissive_color = matched_color;

        // Create a simple card mesh
        let card_mesh = meshes.add(Cuboid::new(100.0, 140.0, 2.0));

        // Position slightly behind the actual card
        let mut new_transform = transform.clone();
        new_transform.translation.z -= 5.0;

        // Add slight tilt for a more dynamic look
        new_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.05, 0.05, 0.0);

        let card_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.1),
            emissive: emissive_color.into(),
            reflectance: 0.2, // Slight reflectivity
            perceptual_roughness: 0.8,
            ..default()
        });

        // Spawn the 3D card with emissive properties
        commands.spawn((
            Mesh3d(card_mesh),
            MeshMaterial3d(card_material.clone()),
            new_transform,
            EmissiveCard {
                original_card: entity,
                glow_intensity: matched_intensity,
                pulse_timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
            },
            EmissiveCardMaterial(card_material),
        ));
    }
}

/// Updates the emissive cards with pulsing effects to demonstrate dynamic HDR
pub fn update_emissive_cards(
    time: Res<Time>,
    mut cards: Query<(&mut EmissiveCard, &mut Transform, &EmissiveCardMaterial)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Update pulse timers and materials for each emissive card
    for (mut emissive_card, mut transform, card_material) in cards.iter_mut() {
        // Update pulse timer
        emissive_card.pulse_timer.tick(time.delta());

        // Update the material if we found it
        if let Some(material) = materials.get_mut(&card_material.0) {
            // Calculate pulse intensity based on sine wave
            let pulse_completion = emissive_card.pulse_timer.elapsed().as_secs_f32()
                / emissive_card.pulse_timer.duration().as_secs_f32();

            let base_intensity = emissive_card.glow_intensity;

            // Vary between 75% and 125% of base intensity
            let pulse_intensity = base_intensity
                * (0.75 + 0.5 * (pulse_completion * std::f32::consts::TAU).sin().abs());

            // In Bevy 0.15, create a new color instead of manipulating the existing one
            let intensity_ratio = pulse_intensity / base_intensity;
            material.emissive = Color::srgb(
                0.8 * intensity_ratio,
                0.8 * intensity_ratio,
                0.8 * intensity_ratio,
            )
            .into();
        }

        // Add subtle hovering motion
        let hover_height =
            (time.elapsed().as_secs_f32() * 0.5 + transform.translation.x * 0.1).sin() * 0.03;
        transform.translation.y += hover_height * time.delta_secs();
    }
}

/// Plugin to add HDR emissive cards to the game
pub struct HDRCardsPlugin;

impl Plugin for HDRCardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameMenuState::InGame), spawn_emissive_cards)
            .add_systems(
                Update,
                update_emissive_cards.run_if(in_state(GameMenuState::InGame)),
            );
    }
}
