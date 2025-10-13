use crate::meshing::{MeshBuilder, Meshable};
use crate::primitives::Segment3d;
use alloc::vec;
use alloc::vec::Vec;
use bevy_reflect::prelude::*;

/// A builder used for creating a [`Mesh`] with a [`Segment3d`] shape.
#[derive(Clone, Copy, Debug, Default, Reflect)]
#[reflect(Default, Debug, Clone)]
pub struct Segment3dMeshBuilder {
    segment: Segment3d,
}

impl Meshable for Segment3dMeshBuilder {
    fn mesh(&self, builder: &mut impl MeshBuilder) {
        let positions: Vec<_> = self.segment.vertices.into();
        let indices = vec![0, 1];

        builder.lines(indices.into_iter(), positions.into_iter().map(|v| v.into()));
    }
}

impl Meshable for Segment3d {
    fn mesh(&self, builder: &mut impl MeshBuilder) {
        Segment3dMeshBuilder { segment: *self }.mesh(builder);
    }
}
