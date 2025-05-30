use bevy::{
    color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion,
    input::mouse::MouseButton,
    input::ButtonInput,
    pbr::NotShadowCaster,
    prelude::*,
    render::view::RenderLayers,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::songs::insomnia::SONG_NOTES;

#[derive(Component)]
struct FallingNote;

pub fn spawn_midi_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    // let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));

    let material = materials.add(Color::hsv(0.1, 0.5, 0.5));

    // The world model camera will render the floor and the cubes spawned in this system.
    // Assigning no `RenderLayers` component defaults to layer 0.
    // commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    // commands.spawn((
    //     Mesh3d(cube.clone()),
    //     MeshMaterial3d(material.clone()),
    //     Transform::from_xyz(0.0, 2.0, -3.0),
    // ));

    const NOTE_SPEED: f32 = 0.1; // pixels per second
    const NOTE_START_Y: f32 = 0.5;
    const NOTE_END_Y: f32 = -300.0;
    const NOTE_WIDTH: f32 = 0.040;
    const NOTE_HEIGHT: f32 = 20.0;
    const NOTE_LANE_WIDTH: f32 = 0.060;

    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    for (i, note) in SONG_NOTES.iter().enumerate() {
        // Each note is a colored rectangle in a "lane" based on its pitch
        let lane = (note.midi_note as i32); // - 60; // C4 = lane 0
        let x = lane as f32 * NOTE_LANE_WIDTH;
        let y = NOTE_START_Y + note.start_beat * NOTE_SPEED;

        commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(
                NOTE_WIDTH,
                note.duration_beats * NOTE_SPEED,
                0.001,
            )),
        ));
    }
}

// fn spawn_song_notes(mut commands: Commands) {
//     let start_time = time.elapsed_seconds();

//     for (i, note) in SONG_NOTES.iter().enumerate() {
//         // Each note is a colored rectangle in a "lane" based on its pitch
//         let lane = (note.midi_note as i32) - 60; // C4 = lane 0
//         let x = NOTE_OFFSET_X + lane as f32 * NOTE_LANE_WIDTH;
//         let y = NOTE_START_Y + note.start_beat * NOTE_SPEED;
//         commands.spawn((
//             SpriteBundle {
//                 sprite: Sprite {
//                     color: Color::CYAN,
//                     custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
//                     ..default()
//                 },
//                 transform: Transform::from_xyz(x, y, 0.0),
//                 ..default()
//             },
//             FallingNote,
//         ));
//     }
// }
