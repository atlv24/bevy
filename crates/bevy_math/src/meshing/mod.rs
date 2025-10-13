//! Mesh generation for [primitive shapes](bevy_math::primitives).
//!
//! Primitives that support meshing implement the [`Meshable`] trait.
//! Calling [`mesh`](Meshable::mesh) will return either a [`Mesh`] or a builder
//! that can be used to specify shape-specific configuration for creating the [`Mesh`].
//!
//! ```
//! # use bevy_asset::Assets;
//! # use bevy_ecs::prelude::ResMut;
//! # use bevy_math::prelude::Circle;
//! # use bevy_mesh::*;
//! #
//! # fn setup(mut meshes: ResMut<Assets<Mesh>>) {
//! // Create circle mesh with default configuration
//! let circle = meshes.add(Circle { radius: 25.0 });
//!
//! // Specify number of vertices
//! let circle = meshes.add(Circle { radius: 25.0 }.mesh().resolution(64));
//! # }
//! ```

//mod dim2;
//pub use dim2::*;

mod dim3;
pub use dim3::*;

//mod extrusion;
//pub use extrusion::*;

/// A trait for shapes that can be turned into a `Mesh`.
pub trait Meshable {
    /// Creates a `Mesh` for a shape.
    fn mesh(&self, builder: &mut impl MeshBuilder);
}

/// A trait used to build `Mesh`es from a configuration
pub trait MeshBuilder {
    /// Push the elements of a u32 iterator as indices and the elements of a (Position, Normal, UV) iterator as vertices.
    fn triangles<I: Iterator<Item = u32>, V: Iterator<Item = ([f32; 3], [f32; 3], [f32; 2])>>(
        &mut self,
        indices: I,
        vertices: V,
    );
    /// Push the elements of a u32 iterator as indices and the elements of a position iterator as vertices.
    fn lines<I: Iterator<Item = u32>, V: Iterator<Item = [f32; 3]>>(
        &mut self,
        indices: I,
        vertices: V,
    );
}
