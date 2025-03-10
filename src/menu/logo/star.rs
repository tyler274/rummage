use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::{
    Extract, Render, RenderApp, RenderSet,
    render_phase::{
        AddRenderCommand, DrawFunctions, PhaseItem, PhaseItemExtraIndex, RenderCommand,
        RenderCommandResult, SetItemPipeline,
    },
    render_resource::{
        BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
        DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCache,
        PolygonMode, PrimitiveState, RenderPipelineDescriptor, SpecializedRenderPipeline,
        SpecializedRenderPipelines, StencilFaceState, StencilState, TextureFormat,
        VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
    },
    view::{ExtractedView, RenderVisibleEntities, ViewTarget},
};
use bevy::sprite::{
    DrawMesh2d, Material2dBindGroupId, Mesh2dPipeline, Mesh2dPipelineKey, Mesh2dTransforms,
    MeshFlags, MeshMaterial2d, RenderMesh2dInstance, SetMesh2dBindGroup, SetMesh2dViewBindGroup,
};
use bevy::ui::{PositionType, Val};

/// Component for the Star of David shape
#[derive(Component)]
pub struct StarOfDavid;

/// Plugin that renders the Star of David
pub struct StarOfDavidPlugin;

impl Plugin for StarOfDavidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_star_of_david);
    }

    fn finish(&self, _app: &mut App) {
        // No custom pipeline setup needed
    }
}

/// System to create and render the Star of David
pub fn render_star_of_david(mut commands: Commands, query: Query<Entity, With<StarOfDavid>>) {
    for entity in &query {
        // Spawn the Star of David using a sprite
        commands.entity(entity).with_children(|parent| {
            // In Bevy 0.15, Sprite component automatically adds Transform and Visibility
            parent.spawn((
                Sprite {
                    color: Color::srgb(1.0, 0.84, 0.0), // Gold color
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                Transform::from_scale(Vec3::splat(1.0)),
            ));
        });
    }
}

/// Create a Star of David bundle for spawning
pub fn create_star_of_david() -> impl Bundle {
    (Transform::default(), StarOfDavid)
}
