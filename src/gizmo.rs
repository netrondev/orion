use crate::hot_reload_system;
use bevy::{color::palettes::tailwind, prelude::*};
use bevy_simple_subsecond_system::hot;
use bevy_text_mesh::{
    SizeUnit, TextMesh, TextMeshBundle, TextMeshFont, TextMeshSize, TextMeshStyle,
};

#[derive(Component)]
pub struct GizmoArm;

#[hot(rerun_on_hot_patch = true)]
pub fn create_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    prev_setup: Query<Entity, With<GizmoArm>>,
    asset_server: Res<AssetServer>,
) {
    // for entity in prev_setup.iter() {
    //     commands.entity(entity).despawn();
    // }

    hot_reload_system(&mut commands, prev_setup);

    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf");

    let longarm = 3.0;
    let diam = 0.05;

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(diam * 5.0, diam * 5.0, diam * 5.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GizmoArm,
    ));

    // X RED
    let x_color = Color::from(tailwind::RED_500);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(x_color)),
        Transform::from_scale(Vec3::new(longarm, diam, diam)).with_translation(Vec3::new(
            longarm / 2.0,
            0.0,
            0.0,
        )),
        GizmoArm,
    ));

    commands.spawn((
        TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("Xasndjk"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(10.),
                    color: Color::srgb(0.0, 0.5, 0.0),
                    ..Default::default()
                },
                size: TextMeshSize {
                    ..Default::default()
                },
                ..Default::default()
            },

            transform: Transform::from_xyz(longarm, 0.0, 0.0)
                .looking_at(Vec3::new(0.1, -1.0, 0.0), -Vec3::X),
            ..Default::default()
        },
        GizmoArm,
    ));

    // Y GREEN
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(tailwind::EMERALD_500))),
        Transform::from_scale(Vec3::new(diam, longarm, diam)).with_translation(Vec3::new(
            0.0,
            longarm / 2.0,
            0.0,
        )),
        GizmoArm,
    ));

    // Z BLUE
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::from(tailwind::SKY_500))),
        Transform::from_scale(Vec3::new(diam, diam, longarm)).with_translation(Vec3::new(
            0.0,
            0.0,
            longarm / 2.0,
        )),
        GizmoArm,
    ));
}
