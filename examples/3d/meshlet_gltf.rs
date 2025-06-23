//! Loads and renders a glTF file as a meshlet scene.

#[path = "../helpers/camera_controller.rs"]
mod camera_controller;
use bevy::{
    pbr::{
        experimental::meshlet::{MeshletMesh, MeshletMesh3d, MeshletMeshSaver, MeshletPlugin},
        CascadeShadowConfigBuilder, DirectionalLightShadowMap,
    },
    platform::collections::HashMap,
    prelude::*,
};
use bevy_asset::{
    io::{file::FileAssetWriter, AssetWriter},
    saver::ErasedAssetSaver,
    ErasedLoadedAsset, LoadedAsset,
};
use std::path::Path;

use crate::camera_controller::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .init_resource::<MeshletMapper>()
        .add_plugins((
            DefaultPlugins,
            MeshletPlugin {
                cluster_buffer_slots: 65535,
            },
            CameraControllerPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, process_meshes::<true>)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Off,
        CameraController::default(),
        Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 250.0,
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 150.0,
            ..default()
        }
        .build(),
        Transform::from_xyz(0.5, 2.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Download from https://drive.proton.me/urls/F6WTNGN74C#VbS2fs2KVnni
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("external/models/sponza.glb"),
    )));
}

#[derive(Resource, Default)]
struct MeshletMapper(HashMap<AssetId<Mesh>, (Handle<MeshletMesh>, String)>);

fn process_meshes<const SAVE: bool>(
    query: Query<(Entity, &ChildOf, &GlobalTransform, &Mesh3d)>,
    parents: Query<(Entity, &Name, &Children)>,
    assets: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut mapper: ResMut<MeshletMapper>,
    mut commands: Commands,
) {
    for (e, childof, transform, Mesh3d(mesh_handle)) in query {
        if let Some(mesh) = meshes.get(mesh_handle.id()) {
            let (meshlet_mesh_handle, name) =
                mapper.0.entry(mesh_handle.id()).or_insert_with(|| {
                    let name = parents
                        .get(childof.parent())
                        .map(|(_, name, children)| {
                            format!(
                                "{name}_{}.meshlet_mesh",
                                children
                                    .iter()
                                    .enumerate()
                                    .find(|(_, entity)| e == *entity)
                                    .unwrap()
                                    .0
                            )
                        })
                        .unwrap_or_default();

                    let meshlet_mesh = MeshletMesh::from_mesh(mesh, 3).unwrap();
                    if SAVE {
                        let loaded_asset = LoadedAsset::new_with_dependencies(meshlet_mesh.clone());
                        let erased_asset = ErasedLoadedAsset::from(loaded_asset);
                        let writer = FileAssetWriter::new("assets/external/models/sponza/", false);
                        let mut write =
                            bevy::tasks::block_on(writer.write(&Path::new(&name))).unwrap();
                        bevy::tasks::block_on(MeshletMeshSaver.save(
                            &mut write,
                            &erased_asset,
                            &(),
                        ))
                        .unwrap();
                    }
                    (assets.add(meshlet_mesh), name)
                });

            commands
                .entity(e)
                .remove::<Mesh3d>()
                .insert(MeshletMesh3d(meshlet_mesh_handle.clone()));

            if SAVE {
                println!(
                        "commands.spawn((MeshletMesh3d(asset_server.load(\"external/models/sponza/{name}\")),Transform::default().with_translation(Vec3::from_array({:?})).with_rotation(Quat::from_array({:?})),MeshMaterial3d(debug_material.clone())));",
                        transform.translation().to_array(),
                        transform.rotation().to_array()
                    );
            }
        }
    }
}
