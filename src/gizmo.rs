use crate::hot_despawn;
use bevy::{color::palettes::tailwind, prelude::*};
use bevy_simple_subsecond_system::hot;
use bevy_text_mesh::{
    SizeUnit, TextMesh, TextMeshBundle, TextMeshFont, TextMeshSize, TextMeshStyle,
};
use eframe::epaint::text;

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
    hot_despawn(&mut commands, prev_setup);

    let longarm = 2.0;
    let diam = 0.05;
    let text_offset = 0.4;
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf");

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
                text: String::from("X"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(50.),
                    color: x_color,
                    ..Default::default()
                },
                size: TextMeshSize {
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(longarm, 0.0, -text_offset))
                .looking_at(Vec3::new(999.0, -1.0, -text_offset), Vec3::X),
            ..Default::default()
        },
        GizmoArm,
    ));

    // Y GREEN
    let color_y = Color::from(tailwind::EMERALD_500);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(color_y)),
        Transform::from_scale(Vec3::new(diam, longarm, diam)).with_translation(Vec3::new(
            0.0,
            longarm / 2.0,
            0.0,
        )),
        GizmoArm,
    ));
    commands.spawn((
        TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("Y"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(50.),
                    color: color_y,
                    ..Default::default()
                },
                size: TextMeshSize {
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, longarm, -text_offset))
                .looking_at(Vec3::new(999.0, 0.0, -text_offset), Vec3::X),
            ..Default::default()
        },
        GizmoArm,
    ));

    // Z BLUE
    let color_z = Color::from(tailwind::SKY_500);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(color_z)),
        Transform::from_scale(Vec3::new(diam, diam, longarm)).with_translation(Vec3::new(
            0.0,
            0.0,
            longarm / 2.0,
        )),
        GizmoArm,
    ));

    commands.spawn((
        TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from("Z"),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(50.),
                    color: color_z,
                    ..Default::default()
                },
                size: TextMeshSize {
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, text_offset * 1.5, longarm))
                .looking_at(Vec3::new(-999.0, 0.0, 0.0), -Vec3::Y),
            ..Default::default()
        },
        GizmoArm,
    ));
}
