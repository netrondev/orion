use bevy::{
    color::palettes::{css::CRIMSON, tailwind},
    prelude::*,
};
use bevy_gizmos::gizmos;

pub fn create_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_assets: ResMut<Assets<GizmoAsset>>,
) {
    // WORLD ORIGIN
    let longarm = 2.0;
    let diam = 0.05;

    // TRANSLATE

    let position: Vec3 = Vec3::new(0.1, 0.5, -1.0);

    // CENTER CUBE
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(diam * 2.0, diam * 2.0, diam * 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(position.x, position.y, position.z),
    ));

    // gizmos.line(Vec3::ZERO, Vec3::X, bevy::color::palettes::css::GREEN);
    let mut gizmo = GizmoAsset::new();
    gizmo
        .sphere(Isometry3d::IDENTITY, 0.5, CRIMSON)
        .resolution(30_000 / 3);

    commands.spawn((
        Gizmo {
            handle: gizmo_assets.add(gizmo),
            line_config: GizmoLineConfig {
                width: 5.,
                ..default()
            },
            ..default()
        },
        Transform::from_xyz(4., 1., 0.),
    ));

    // // X
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::from(tailwind::RED_500))),
    //     Transform::from_scale(Vec3::new(longarm, diam, diam)).with_translation(Vec3::new(
    //         position.x + longarm / 2.0,
    //         position.y,
    //         position.z,
    //     )),
    // ));

    // // Y
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::from(tailwind::EMERALD_500))),
    //     Transform::from_scale(Vec3::new(diam, longarm, diam)).with_translation(Vec3::new(
    //         position.x,
    //         position.y + longarm / 2.0,
    //         position.z,
    //     )),
    // ));

    // // Z
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::from(tailwind::SKY_500))),
    //     Transform::from_scale(Vec3::new(diam, diam, longarm)).with_translation(Vec3::new(
    //         position.x,
    //         position.y,
    //         position.z + longarm / 2.0,
    //     )),
    // ));
}
