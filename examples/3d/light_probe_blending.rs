//! Demonstrates parallax-corrected cubemap reflections.

use core::f32;

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
    render::view::Hdr,
};

/// The brightness of the cubemap.
///
/// Since the cubemap images were baked in Blender, which uses a different
/// exposure setting than that of Bevy, we need this factor in order to make the
/// exposure of the baked image match ours.
const ENVIRONMENT_MAP_INTENSITY: f32 = 100.0;

const TWO_ROOMS_URL: &str =
    "https://github.com/atlv24/bevy_asset_files/raw/ad/lpb/light_probe_blending/two_rooms.glb#Scene0";
const ENV_DIFFUSE_1_URL: &str =
    "https://github.com/atlv24/bevy_asset_files/raw/ad/lpb/light_probe_blending/diffuse_room1.ktx2";
const ENV_SPECULAR_1_URL: &str =
    "https://github.com/atlv24/bevy_asset_files/raw/ad/lpb/light_probe_blending/specular_room1.ktx2";
const ENV_DIFFUSE_2_URL: &str =
    "https://github.com/atlv24/bevy_asset_files/raw/ad/lpb/light_probe_blending/diffuse_room2.ktx2";
const ENV_SPECULAR_2_URL: &str =
    "https://github.com/atlv24/bevy_asset_files/raw/ad/lpb/light_probe_blending/specular_room2.ktx2";

/// The example entry point.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Light Probe Blending Example".into(),
                    ..default()
                }),
                ..default()
            }),
            FreeCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Creates the initial scene.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the glTF scene.
    commands.spawn(SceneRoot(asset_server.load(TWO_ROOMS_URL)));

    spawn_camera(&mut commands);
    spawn_inner_cube(&mut commands, &mut meshes, &mut materials);
    spawn_reflection_probe(&mut commands, &asset_server);
}

/// Spawns the camera.
fn spawn_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        FreeCamera::default(),
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::new(0.0, -2.5, 0.0), Dir3::Y),
        Hdr,
    ));
}

/// Spawns the inner reflective cube in the scene.
fn spawn_inner_cube(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let cube_mesh = meshes.add(
        Cuboid {
            half_size: Vec3::new(2.0, 1.0, 10.0),
        }
        .mesh()
        .build()
        .with_duplicated_vertices()
        .with_computed_flat_normals(),
    );
    let cube_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 1.0,
        reflectance: 1.0,
        perceptual_roughness: 0.0,
        ..default()
    });

    commands.spawn((
        Mesh3d(cube_mesh),
        MeshMaterial3d(cube_material),
        Transform::from_xyz(0.0, -4.0, -5.5),
    ));
}

/// Spawns the reflection probe (i.e. cubemap reflection) in the center of the scene.
fn spawn_reflection_probe(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        LightProbe,
        EnvironmentMapLight {
            diffuse_map: asset_server.load(ENV_DIFFUSE_1_URL),
            specular_map: asset_server.load(ENV_SPECULAR_1_URL),
            intensity: ENVIRONMENT_MAP_INTENSITY,
            ..default()
        },
        Transform::from_scale(Vec3::new(10.0, -10.0, 10.0))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
    ));
    commands.spawn((
        LightProbe,
        EnvironmentMapLight {
            diffuse_map: asset_server.load(ENV_DIFFUSE_2_URL),
            specular_map: asset_server.load(ENV_SPECULAR_2_URL),
            intensity: ENVIRONMENT_MAP_INTENSITY,
            ..default()
        },
        Transform::from_scale(Vec3::new(10.0, -10.0, 10.0))
            .with_translation(-Vec3::Z * 11.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
    ));
}
