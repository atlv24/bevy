use crate::{
    meshing::{MeshBuilder, Meshable},
    primitives::Polyline3d,
};
use alloc::vec::Vec;
use bevy_reflect::prelude::*;

/// A builder used for creating a [`Mesh`] with a [`Polyline3d`] shape.
#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Default, Debug, Clone)]
pub struct Polyline3dMeshBuilder {
    polyline: Polyline3d,
}

impl Meshable for Polyline3dMeshBuilder {
    fn mesh(&self, builder: &mut impl MeshBuilder) {
        let positions: Vec<_> = self.polyline.vertices.clone();

        let indices = (0..self.polyline.vertices.len() as u32 - 1).flat_map(|i| [i, i + 1]);

        builder.lines(indices, positions.into_iter().map(|v| v.into()));
    }
}

impl Meshable for Polyline3d {
    fn mesh(&self, builder: &mut impl MeshBuilder) {
        Polyline3dMeshBuilder {
            polyline: self.clone(),
        }
        .mesh(builder);
    }
}
