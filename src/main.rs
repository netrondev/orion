use bevy::{color::palettes::tailwind, prelude::*};
use bevy_simple_subsecond_system::prelude::*;
use std::sync::{Arc, Mutex};
mod bevy_midi;
use bevy_midi::prelude::*;
mod piano;
use bevy_procedural_audio::prelude::*;
mod audio;
mod bevy_mic;
pub mod gizmo;
mod keys;
mod mic;
mod record_visualizer;
mod songs;
mod synth;
use bevy_text_mesh::prelude::*;

fn main() {
    // keys::main();

    let synth_mutex = Arc::new(Mutex::new(0.0f32));
    let micamp = mic::MicAmplitude(synth_mutex.clone());

    let audio_buffer = mic::AudioBuffer(Arc::new(Mutex::new(Vec::new())));

    App::new()
        // .insert_resource(AmbientLight {
        //     color: Color::WHITE,
        //     brightness: 1.0 / 5.0f32,
        //     ..default()
        // })
        // PLUGINS
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "/home/rouan/work/orion/assets".to_string(),
            unapproved_path_mode: bevy_asset::UnapprovedPathMode::Allow,
            ..default()
        }))
        .add_plugins(TextMeshPlugin)
        .add_plugins(DspPlugin::default())
        // HOT RELOAD
        .add_plugins(SimpleSubsecondPlugin::default())
        // MIDI
        .add_plugins(MidiInputPlugin)
        .add_plugins(bevy_mic::ModAudioPlugins)
        .add_plugins(audio::PianoPlugin)
        .add_plugins(record_visualizer::RecordVisualizerPlugin)
        .add_plugins(MidiOutputPlugin)
        .add_plugins(songs::SongLoaderPlugin)
        // RESOURCES
        .init_resource::<MidiInputSettings>()
        .init_resource::<MidiOutputSettings>()
        // Add RecordingState resource
        .insert_resource(micamp)
        .insert_resource(audio_buffer)
        // .init_resource::<mic::RecordingState>()
        // STARTUP
        .add_systems(
            Startup,
            (
                setup,
                // create_piano,
                mic::ui_system_startup,
                gizmo::create_gizmo,
            ),
        )
        // UPDATE
        .add_systems(
            Update,
            (
                handle_midi_input,
                connect_to_first_input_port,
                connect_to_first_output_port,
                display_press,
                display_release,
                // mic::ui_system_update_button,
                // mic::mic_update,
            ),
        )
        .run();
}

pub fn hot_despawn<T: Component>(
    commands: &mut Commands,
    mut hot_reloaded: Query<Entity, With<T>>,
) {
    for entity in hot_reloaded.iter_mut() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, Debug)]
struct Key {
    key_val: String,
    y_reset: f32,
}

#[derive(Component)]
struct PressedKey;

#[derive(Component, Debug, Clone)]
enum NoteType {
    White,
    Black,
}

#[rustfmt::skip]
#[hot(rerun_on_hot_patch = true)]
fn setup(
    mut cmds: Commands,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    hot_reload_keys: Query<Entity, With<Key>>,
    hot_reload_camera: Query<Entity, With<Camera3d>>,
    hot_reload_pointlights: Query<Entity, With<PointLight>>,
) {

    hot_despawn(&mut cmds, hot_reload_keys);
    hot_despawn(&mut cmds, hot_reload_camera);
    hot_despawn(&mut cmds, hot_reload_pointlights);

    let mid = -6.3;   

    cmds.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(1.0, 7.0, mid).looking_at(Vec3::new(-1.8, 0., mid), Vec3::Y),
    ));

    // light
    // cmds.spawn((
    //     PointLight::default(),
    //     Transform::from_xyz(0.0, 6.0, mid),        
    // ));

    cmds.spawn((
        PointLight {
            intensity: 3000000.0,

            shadows_enabled: true,
            // range: 10.0,
            color: Color::from(tailwind::SKY_200),
            ..Default::default()
        },
        Transform::from_xyz(3.0, 5.0, mid*2.0),        
    ));
    
    cmds.spawn((
        PointLight {
            intensity: 3000000.0,

            shadows_enabled: true,
            // range: 10.0,
            color: Color::from(tailwind::AMBER_200),
            ..Default::default()
        },
        Transform::from_xyz(-3.0, 5.0, 0.0),        
    ));

        cmds.spawn((
        PointLight {
            intensity: 24000000.0,

            shadows_enabled: true,
            // range: 10.0,
            color: Color::from(tailwind::NEUTRAL_100),
            ..Default::default()
        },
        Transform::from_xyz(10.0, 10.0, mid),        
    ));

    let pos: Vec3 = Vec3::new(0., 0., 0.);

    let mut black_key: Handle<Mesh> = asset_server.load(GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }.from_asset("models/black_key_3.gltf"));
    let mut white_key_0: Handle<Mesh> = asset_server.load("models/white_key_0.gltf#Mesh0/Primitive0");
    let mut white_key_1: Handle<Mesh> = asset_server.load("models/white_key_1.gltf#Mesh0/Primitive0");
    let mut white_key_2: Handle<Mesh> = asset_server.load("models/white_key_2.gltf#Mesh0/Primitive0");
    
    let colour_white = Color::from(tailwind::NEUTRAL_300);
    let colour_black = Color::from(tailwind::NEUTRAL_800);
    
    

    //Create keyboard layout
    let pos_black = pos + Vec3::new(0., 0.06, 0.);

    for i in 0..8 {

        if i == 5 {
            spawn_note(&mut cmds, &standard_materials.add(Color::srgb(0.75, 0.0, 0.0)), 0.00, pos, &mut white_key_0, i, "C", &asset_server, NoteType::White);
        } else {
            spawn_note(&mut cmds, &standard_materials.add(colour_white), 0.00, pos, &mut white_key_0, i, "C", &asset_server, NoteType::White);
        }

        
        spawn_note(&mut cmds, &standard_materials.add(colour_black), 0.15, pos_black, &mut black_key, i, "C#/Db", &asset_server , NoteType::Black);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 0.27, pos, &mut white_key_1, i, "D", &asset_server , NoteType::White);
        spawn_note(&mut cmds, &standard_materials.add(colour_black), 0.39, pos_black, &mut black_key, i, "D#/Eb", &asset_server , NoteType::Black);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 0.54, pos, &mut white_key_2, i, "E", &asset_server , NoteType::White);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 0.69, pos, &mut white_key_0, i, "F", &asset_server , NoteType::White);
        spawn_note(&mut cmds, &standard_materials.add(colour_black), 0.85, pos_black, &mut black_key, i, "F#/Gb", &asset_server , NoteType::Black);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 0.96, pos, &mut white_key_1, i, "G", &asset_server , NoteType::White);
        spawn_note(&mut cmds, &standard_materials.add(colour_black), 1.08, pos_black, &mut black_key, i, "G#/Ab", &asset_server , NoteType::Black);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 1.19, pos, &mut white_key_1, i, "A", &asset_server , NoteType::White);
        spawn_note(&mut cmds, &standard_materials.add(colour_black), 1.31, pos_black, &mut black_key, i, "A#/Bb", &asset_server , NoteType::Black);
        spawn_note(&mut cmds, &standard_materials.add(colour_white), 1.46, pos, &mut white_key_2, i, "B", &asset_server , NoteType::White);
    }

    // My Piano
}

fn spawn_note(
    commands: &mut Commands,
    mat: &Handle<StandardMaterial>,
    offset_z: f32,
    pos: Vec3,
    asset: &mut Handle<Mesh>,
    oct: i32,
    key: &str,
    asset_server: &Res<AssetServer>,
    note_type: NoteType,
) {
    let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf");

    commands.spawn((
        Mesh3d(asset.clone()),
        MeshMaterial3d(mat.clone()),
        Transform {
            translation: Vec3::new(pos.x, pos.y, pos.z - offset_z - (1.61 * oct as f32)),
            scale: Vec3::new(10., 10., 10.),
            ..Default::default()
        },
        Key {
            key_val: format!("{}{}", key, oct),
            y_reset: pos.y,
        },
        note_type.clone(),
        children![TextMeshBundle {
            text_mesh: TextMesh {
                text: String::from(key),
                style: TextMeshStyle {
                    font: font.clone(),
                    font_size: SizeUnit::NonStandard(1.),

                    color: match note_type {
                        NoteType::White => Color::srgb(0.0, 0.5, 0.0),
                        NoteType::Black => Color::srgb(0.5, 0.0, 0.0),
                    },
                    ..Default::default()
                },
                size: TextMeshSize {
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0.1, 0.015, 0.0)
                .looking_at(Vec3::new(0.1, -1.0, 0.0), -Vec3::X),
            ..Default::default()
        }],
    ));
}

fn display_press(
    mut query: Query<
        (&mut Transform, &MeshMaterial3d<StandardMaterial>, &NoteType),
        With<PressedKey>,
    >,
    timer: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut t, mut mat, notetype) in &mut query {
        println!("Pressed Key: {:?} {:?}", t.translation.y, notetype);
        t.translation.y = (t.translation.y - 0.03).max(-0.06);
        println!("Material: {:?}", mat);

        if let Some(material) = materials.get_mut(mat) {
            println!("Material Color: {:?}", material.base_color);
            material.base_color = Color::from(tailwind::RED_500);
        }
    }
}

fn display_release(
    mut query: Query<(&mut Transform, &Key), Without<PressedKey>>,
    timer: Res<Time>,
) {
    for (mut t, k) in &mut query {
        // t.translation.y = k.y_reset;
        t.translation.y = (t.translation.y + 0.03).min(k.y_reset);
    }
}

fn handle_midi_input(
    mut commands: Commands,
    mut midi_events: EventReader<MidiData>,
    query: Query<(Entity, &Key)>,
) {
    for data in midi_events.read() {
        let [_, index, _value] = data.message.msg;
        let off = index % 12;
        let oct = index.overflowing_div(12).0;
        let key_str = KEY_RANGE.iter().nth(off.into()).unwrap();

        if data.message.is_note_on() {
            for (entity, key) in query.iter() {
                if key.key_val.eq(&format!("{}{}", key_str, oct).to_string()) {
                    commands.entity(entity).insert(PressedKey);
                }
            }
        } else if data.message.is_note_off() {
            for (entity, key) in query.iter() {
                if key.key_val.eq(&format!("{}{}", key_str, oct).to_string()) {
                    commands.entity(entity).remove::<PressedKey>();
                }
            }
        } else {
        }
    }
}

fn connect_to_first_input_port(input: Res<MidiInput>) {
    if input.is_changed() {
        if let Some((_, port)) = input.ports().get(1) {
            input.connect(port.clone());
        }
    }
}

fn connect_to_first_output_port(input: Res<MidiOutput>) {
    if input.is_changed() {
        if let Some((_, port)) = input.ports().get(0) {
            input.connect(port.clone());
        }
    }
}
