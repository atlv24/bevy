#![expect(missing_docs, reason = "Not all docs are written yet, see #3492.")]

extern crate alloc;
extern crate core;

mod components;
mod conversions;
mod index;
mod mesh;
#[cfg(feature = "bevy_mikktspace")]
mod mikktspace;
#[cfg(feature = "morph")]
pub mod morph;
pub mod skinning;
mod vertex;
use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetApp, AssetEventSystems, RenderAssetUsages};
use bevy_ecs::schedule::{IntoScheduleConfigs, SystemSet};
use bevy_math::meshing::{MeshBuilder, Meshable};
use bitflags::bitflags;
pub use components::*;
pub use index::*;
pub use mesh::*;
#[cfg(feature = "bevy_mikktspace")]
pub use mikktspace::*;
pub use vertex::*;
pub use wgpu_types::VertexFormat;

/// The mesh prelude.
///
/// This includes the most common types in this crate, re-exported for your convenience.
pub mod prelude {
    #[cfg(feature = "morph")]
    pub use crate::morph::MorphWeights;
    #[doc(hidden)]
    pub use crate::{Mesh, Mesh2d, Mesh3d};
}

bitflags! {
    /// Our base mesh pipeline key bits start from the highest bit and go
    /// downward. The PBR mesh pipeline key bits start from the lowest bit and
    /// go upward. This allows the PBR bits in the downstream crate `bevy_pbr`
    /// to coexist in the same field without any shifts.
    #[derive(Clone, Debug)]
    pub struct BaseMeshPipelineKey: u64 {
        const MORPH_TARGETS = 1 << (u64::BITS - 1);
    }
}

/// Adds [`Mesh`] as an asset.
#[derive(Default)]
pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Mesh>()
            .init_asset::<skinning::SkinnedMeshInverseBindposes>()
            .register_asset_reflect::<Mesh>()
            .add_systems(
                PostUpdate,
                mark_3d_meshes_as_changed_if_their_assets_changed.after(AssetEventSystems),
            );
    }
}

impl BaseMeshPipelineKey {
    pub const PRIMITIVE_TOPOLOGY_MASK_BITS: u64 = 0b111;
    pub const PRIMITIVE_TOPOLOGY_SHIFT_BITS: u64 =
        (u64::BITS - 1 - Self::PRIMITIVE_TOPOLOGY_MASK_BITS.count_ones()) as u64;

    pub fn from_primitive_topology(primitive_topology: PrimitiveTopology) -> Self {
        let primitive_topology_bits = ((primitive_topology as u64)
            & Self::PRIMITIVE_TOPOLOGY_MASK_BITS)
            << Self::PRIMITIVE_TOPOLOGY_SHIFT_BITS;
        Self::from_bits_retain(primitive_topology_bits)
    }

    pub fn primitive_topology(&self) -> PrimitiveTopology {
        let primitive_topology_bits = (self.bits() >> Self::PRIMITIVE_TOPOLOGY_SHIFT_BITS)
            & Self::PRIMITIVE_TOPOLOGY_MASK_BITS;
        match primitive_topology_bits {
            x if x == PrimitiveTopology::PointList as u64 => PrimitiveTopology::PointList,
            x if x == PrimitiveTopology::LineList as u64 => PrimitiveTopology::LineList,
            x if x == PrimitiveTopology::LineStrip as u64 => PrimitiveTopology::LineStrip,
            x if x == PrimitiveTopology::TriangleList as u64 => PrimitiveTopology::TriangleList,
            x if x == PrimitiveTopology::TriangleStrip as u64 => PrimitiveTopology::TriangleStrip,
            _ => PrimitiveTopology::default(),
        }
    }
}

/// `bevy_render::mesh::inherit_weights` runs in this `SystemSet`
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct InheritWeightSystems;

impl<T: Meshable> From<T> for Mesh {
    fn from(meshable: T) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        meshable.mesh(&mut mesh);
        mesh
    }
}

impl MeshBuilder for Mesh {
    fn triangles<I: Iterator<Item = u32>, V: Iterator<Item = ([f32; 3], [f32; 3], [f32; 2])>>(
        &mut self,
        indices: I,
        vertices: V,
    ) {
        let mut vs = Vec::new();
        let mut vns = Vec::new();
        let mut vts = Vec::new();
        vertices.for_each(|(v, vn, vt)| {
            vs.push(v);
            vns.push(vn);
            vts.push(vt);
        });
        *self = Mesh::new(PrimitiveTopology::TriangleList, self.asset_usage)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vs)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vns)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vts)
            .with_inserted_indices(Indices::U32(indices.collect()));
    }

    fn lines<I: Iterator<Item = u32>, V: Iterator<Item = [f32; 3]>>(
        &mut self,
        indices: I,
        vertices: V,
    ) {
        *self = Mesh::new(PrimitiveTopology::LineList, self.asset_usage)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices.collect::<Vec<_>>())
            .with_inserted_indices(Indices::U32(indices.collect()));
    }
}
