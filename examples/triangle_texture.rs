use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::schedule::camera_driver,
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    mesh::Indices,
    prelude::*,
    render::{
        Render, RenderApp, RenderStartup, RenderSystems,
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, BindGroup, BindGroupEntries, BindGroupLayoutDescriptor,
            BindGroupLayoutEntries, CachedComputePipelineId, CachedPipelineState,
            ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, SamplerBindingType,
            ShaderStages, StorageTextureAccess, TextureFormat, TextureSampleType, TextureUsages,
            binding_types::{sampler, texture_2d, texture_storage_2d},
        },
        renderer::{RenderContext, RenderDevice, RenderGraph},
        texture::GpuImage,
    },
    shader::{ShaderCacheError, ShaderRef},
};
use std::borrow::Cow;

const TRIANGLE_TEXTURE_PATH: &str = "examples/triangle_texture/shaders/triangle_texture.wgsl";
const TRIANGULAR_DRAW_PATH: &str = "examples/triangle_texture/shaders/triangular_draw.wgsl";

const SIZE: u32 = 32;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<TriangleMaterial>::default(),
            TriangleRenderPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Clone, ExtractResource)]
struct SabiCats {
    src: Handle<Image>,
    dst: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut s_materials: ResMut<Assets<StandardMaterial>>,
    mut t_materials: ResMut<Assets<TriangleMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 30.0,
        }),
    ));

    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
    let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

    let src_mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-20.0, 5.0, 0.0],
            [-20.0, -5.0, 0.0],
            [-10.0, -5.0, 0.0],
            [-10.0, 5.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(indices.clone()))
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());

    let dst_mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[10.0, 5.0, 0.0], [10.0, -5.0, 0.0], [20.0, 5.0, 0.0]],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [0.0, 1.0], [1.0, 0.0]],
    );

    let dst_mesh2 = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-5.0, 5.0, 0.0],
            [-5.0, -5.0, 0.0],
            [5.0, -5.0, 0.0],
            [5.0, 5.0, 0.0],
        ],
    )
    .with_inserted_indices(Indices::U32(indices.clone()))
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs.clone());

    let src_cat = asset_server
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| {
            s.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::ClampToBorder,
                address_mode_v: ImageAddressMode::ClampToBorder,
                mag_filter: ImageFilterMode::Linear,
                min_filter: ImageFilterMode::Linear,
                ..default()
            })
        })
        .load::<Image>("images/sabi_cat.png");

    let mut dst_cat = Image::new_target_texture(SIZE, SIZE, TextureFormat::Rgba32Float, None);
    dst_cat.asset_usage = RenderAssetUsages::RENDER_WORLD;
    dst_cat.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    dst_cat.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        ..default()
    });
    let dst_cat = images.add(dst_cat);

    commands.insert_resource(SabiCats {
        src: src_cat.clone(),
        dst: dst_cat.clone(),
    });

    commands.spawn((
        Mesh3d(meshes.add(src_mesh)),
        MeshMaterial3d(s_materials.add(StandardMaterial {
            base_color_texture: Some(src_cat),
            unlit: true,
            ..default()
        })),
        Transform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(dst_mesh)),
        MeshMaterial3d(t_materials.add(TriangleMaterial {
            size: SIZE,
            base_texture: dst_cat.clone(),
        })),
        Transform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(dst_mesh2)),
        MeshMaterial3d(s_materials.add(StandardMaterial {
            base_color_texture: Some(dst_cat),
            unlit: true,
            ..default()
        })),
        Transform::default(),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct TriangleMaterial {
    #[uniform(0)]
    size: u32,
    #[texture(1)]
    base_texture: Handle<Image>,
}

impl Material for TriangleMaterial {
    fn fragment_shader() -> ShaderRef {
        TRIANGLE_TEXTURE_PATH.into()
    }
}

#[derive(Resource)]
struct TrianglePipeline {
    layout: BindGroupLayoutDescriptor,
    pipeline_id: CachedComputePipelineId,
}

struct TriangleRenderPlugin;

impl Plugin for TriangleRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<SabiCats>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<TriangleState>()
            .add_systems(RenderStartup, init_pipeline)
            .add_systems(
                Render,
                prepare_bind_group
                    .run_if(not(resource_exists::<TriangleBindGroup>))
                    .in_set(RenderSystems::PrepareBindGroups),
            )
            .add_systems(Render, update.in_set(RenderSystems::Prepare))
            .add_systems(
                RenderGraph,
                triangular_draw
                    .before(camera_driver)
                    .run_if(resource_exists::<TriangleBindGroup>),
            );
    }
}

fn init_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pipeline_cache: Res<PipelineCache>,
) {
    let layout = BindGroupLayoutDescriptor::new(
        "triangle",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                texture_storage_2d(TextureFormat::Rgba32Float, StorageTextureAccess::WriteOnly),
            ),
        ),
    );

    let pipeline_id = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
        layout: vec![layout.clone()],
        shader: asset_server.load(TRIANGULAR_DRAW_PATH),
        entry_point: Some(Cow::from("draw")),
        immediate_size: 4,
        ..default()
    });

    commands.insert_resource(TrianglePipeline {
        layout,
        pipeline_id,
    });
}

#[derive(Resource)]
struct TriangleBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<TrianglePipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    sabi_cats: Res<SabiCats>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
) {
    if let (Some(src_cat), Some(dst_cat)) = (
        gpu_images.get(&sabi_cats.src),
        gpu_images.get(&sabi_cats.dst),
    ) {
        let bind_group = render_device.create_bind_group(
            None,
            &pipeline_cache.get_bind_group_layout(&pipeline.layout),
            &BindGroupEntries::sequential((
                &src_cat.texture_view,
                &src_cat.sampler,
                &dst_cat.texture_view,
            )),
        );

        commands.insert_resource(TriangleBindGroup(bind_group));
    }
}

#[derive(Resource, Default)]
enum TriangleState {
    #[default]
    Loading,
    Update,
}

fn update(
    pipeline: Res<TrianglePipeline>,
    pipeline_cache: Res<PipelineCache>,
    mut state: ResMut<TriangleState>,
) {
    match *state {
        TriangleState::Loading => {
            match pipeline_cache.get_compute_pipeline_state(pipeline.pipeline_id) {
                CachedPipelineState::Ok(_) => {
                    *state = TriangleState::Update;
                }
                CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_)) => {}
                CachedPipelineState::Err(err) => {
                    panic!("Initializing assets/{TRIANGULAR_DRAW_PATH}:\n{err}");
                }
                _ => {}
            }
        }
        TriangleState::Update => {}
    }
}

fn triangular_draw(
    mut render_context: RenderContext,
    bind_group: Res<TriangleBindGroup>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<TrianglePipeline>,
    state: Res<TriangleState>,
) {
    match *state {
        TriangleState::Loading => {}
        TriangleState::Update => {
            const XY: u32 = if SIZE < 16 { 1 } else { SIZE / 16 };
            let size = bytemuck::bytes_of(&SIZE);
            let pipeline = pipeline_cache
                .get_compute_pipeline(pipeline.pipeline_id)
                .unwrap();

            let mut pass = render_context
                .command_encoder()
                .begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_bind_group(0, &bind_group.0, &[]);
            pass.set_pipeline(pipeline);
            pass.set_immediates(0, size);
            pass.dispatch_workgroups(XY, XY, 1);
        }
    }
}
