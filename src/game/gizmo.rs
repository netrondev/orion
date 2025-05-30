use bevy::{color::palettes::tailwind, prelude::*};

pub fn create_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // WORLD ORIGIN
    let longarm = 2.0;
    let diam = 0.05;

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(diam * 2.0, diam * 2.0, diam * 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // X
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(tailwind::RED_500))),
        Transform::from_scale(Vec3::new(longarm, diam, diam)).with_translation(Vec3::new(
            longarm / 2.0,
            0.0,
            0.0,
        )),
    ));

    // Y
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(tailwind::EMERALD_500))),
        Transform::from_scale(Vec3::new(diam, longarm, diam)).with_translation(Vec3::new(
            0.0,
            longarm / 2.0,
            0.0,
        )),
    ));

    // Z
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(tailwind::SKY_500))),
        Transform::from_scale(Vec3::new(diam, diam, longarm)).with_translation(Vec3::new(
            0.0,
            0.0,
            longarm / 2.0,
        )),
    ));
}
