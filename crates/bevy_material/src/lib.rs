//! Provides a material abstraction for bevy
#![allow(missing_docs)]

use bevy_ecs::component::Component;
use bevy_mesh::MeshVertexBufferLayoutRef;
use bevy_shader::ShaderRef;

use crate::alpha::AlphaMode;
use crate::opaque::OpaqueRendererMethod;

pub mod alpha;
pub mod opaque;

/// The material prelude.
///
/// This includes the most common types in this crate, re-exported for your convenience.
pub mod prelude {
    #[doc(hidden)]
    pub use crate::alpha::AlphaMode;
}

// we basically already have a good material api, its just hidden inside the renderer
// and named MaterialProperties. All our Material generic soup stuff just makes a
// MaterialProperties and thats what actually gets used.

// I think Materials as Entities makes a lot of sense (we dont have to wait for assets
// as entities, but it would improve performance i think?)

// In this model, we can define a material with components

/// Base material properties
#[derive(Component, Default)]
pub struct Material {
    // Note: 2d can either be made to support the rest of the AlphaModes it doesnt,
    // or we can break stuff out to a more granular level here, making each of these
    // its own component.
    pub alpha_mode: AlphaMode,
    // Will 2d ever want deferred?
    pub opaque_method: OpaqueRendererMethod,
    pub depth_bias: f32,
    // This could be a marker ZST component
    pub reads_view_transmission_texture: bool,
    // Option would go away if this was its own component
    pub specialize: Option<
        fn(
            &MaterialPipeline,
            &mut RenderPipelineDescriptor,
            &MeshVertexBufferLayoutRef,
            ErasedMaterialPipelineKey,
        ) -> Result<(), SpecializedMeshPipelineError>,
    >,
}

/// Forward shader pair
#[derive(Component, Default)]
pub struct ForwardShaders {
    pub vertex: ShaderRef,
    pub fragment: ShaderRef,
}

/// Prepass shader pair
#[derive(Component, Default)]
pub struct PrepassShaders {
    pub vertex: ShaderRef,
    pub fragment: ShaderRef,
}

/// Deferred shader pair
#[derive(Component, Default)]
pub struct DeferredShaders {
    pub vertex: ShaderRef,
    pub fragment: ShaderRef,
}

/// Meshlet shaders
#[derive(Component, Default)]
pub struct MeshletShaders {
    pub fragment: ShaderRef,
    pub prepass_fragment: ShaderRef,
    pub deferred_fragment: ShaderRef,
}

// The api doesnt necessarily have to be split into specifically these Components,
// we can workshop it. The motivation for this split however is ExtendedMaterials,
// and defining it in a more modular way that may be amenable to 2d materials too.

// ExtendedMaterials might make sense to model as Materials that are a child of
// another Material entity, and inherit parent settings unless specified. Using
// bevy_hierarchy on assets is worth exploring imo.

// 2d obviously doesnt care about meshlets, but by having these be components we
// don't force them to be present. 2d can have its own Handle<Shader> component,
// or we could just try to adhere to MaterialProperties as closely as possible,
// and use ShaderLabel -> Handle<Shader> maps.

// The path to get us here doesnt look too arduous, we'd first introduce the new api
// which generates the MaterialProperties struct and can coexist with the current
// material generic soup, and then migrate usages case by case, then yeet the old
// system.

// One potential pain point is getting these bevy_pbr types, needed for specialization,
// to be usable without the renderer:

pub struct MaterialPipeline; // holds a metric fuckton of BindGroupLayouts + a GpuImage
pub struct RenderPipelineDescriptor; // holds BindGroupLayout. the rest is fine

// Perhaps we can store BindGroupLayoutEntries on a Material/RenderPipelineDescriptor
// instead of an actual BindGroupLayout, and then have a hashmap conversion step when
// we create MaterialProperties, creating new/reusing old layouts as needed? Not sure
// about the potential performance impacts of doing that.

// The GpuImage on MaterialPipeline is just dummy_white_gpu_image, that can just be a
// Handle<Image> instead i think.

// We might have to add some concepts to bevy_shader. MVP of bevy_material can depend
// on bevy_render if its too hard, but we should have a plan for making it render-free
// so that bevy_gltf can finally be render-free.

// These also live in bevy_pbr, but can be extracted to bevy_material easily:

pub struct ErasedMaterialPipelineKey;
pub struct SpecializedMeshPipelineError;
