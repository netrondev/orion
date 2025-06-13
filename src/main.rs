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
                Key::handle_midi_input,
                connect_to_first_input_port,
                connect_to_first_output_port,
                Key::display_press,
                Key::display_release,
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

#[derive(Component, Debug, Clone)]
struct Key {
    key_val: String,
    y_reset: f32,
    oct: i32,
    key_in_octal: u8,
    key_name: String,
    key_type: NoteType,
    offset_z: f32,
    key_height: f32,
}

impl Key {
    pub fn system_startup(
        mut cmds: Commands,
        mut standard_materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        for oct in 0..8 {
            for key_in_octal in 0..12 {
                Key::new(oct, key_in_octal).spawn_note(
                    &mut cmds,
                    &asset_server,
                    &mut standard_materials,
                );
            }
        }
    }

    pub fn new(oct: i32, key_in_octal: u8) -> Self {
        let (key_name, key_type, offset_z, key_height) = Self::pattern(key_in_octal);

        Self {
            key_val: format!("{}{}", key_name, oct),
            y_reset: key_height,
            oct,
            key_in_octal,
            key_name,
            key_type,
            offset_z,
            key_height,
        }
    }

    pub fn spawn_note(
        &self,
        commands: &mut Commands,
        // mat: &Handle<StandardMaterial>,

        // asset: &mut Handle<Mesh>,
        // key: &str,
        asset_server: &Res<AssetServer>,
        standard_materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let font: Handle<TextMeshFont> = asset_server.load("fonts/FiraSans-Medium.ttf");

        let color = match self.key_type {
            NoteType::White => Color::from(tailwind::NEUTRAL_300),
            NoteType::Black => Color::from(tailwind::NEUTRAL_800),
        };
        let mat = standard_materials.add(color);
        let mesh = Self::get_mesh(self.key_in_octal, asset_server);

        let pos: Vec3 = Vec3::new(0.0, self.key_height, 0.0);

        commands.spawn((
            self.to_owned(),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.to_owned()),
            Transform {
                translation: Vec3::new(
                    pos.x,
                    pos.y,
                    pos.z - &self.offset_z - (1.61 * self.oct as f32),
                ),
                scale: Vec3::new(10., 10., 10.),
                ..Default::default()
            },
            self.key_type.clone(),
            children![TextMeshBundle {
                text_mesh: TextMesh {
                    text: String::from(self.key_name.clone()),
                    style: TextMeshStyle {
                        font: font.clone(),
                        font_size: SizeUnit::NonStandard(1.),

                        color: match self.key_type {
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

    pub fn pattern(key_in_octal: u8) -> (String, NoteType, f32, f32) {
        // let key_range = [
        //     "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
        // ];

        let offsets = [
            0.0, 0.15, 0.27, 0.39, 0.54, 0.69, 0.85, 0.96, 1.08, 1.19, 1.31, 1.46,
        ];

        let key_str = KEY_RANGE[key_in_octal as usize % 12].to_string();
        let key_offset = offsets[key_in_octal as usize % 12];

        let note_type: NoteType = match key_in_octal {
            1 | 3 | 6 | 8 | 10 => NoteType::Black,
            _ => NoteType::White,
        };

        let key_height = match note_type {
            NoteType::White => 0.00,
            NoteType::Black => 0.06,
        };

        return (key_str, note_type, key_offset, key_height);
    }

    pub fn get_key_colour(&self) -> Color {
        match self.key_type {
            NoteType::White => Color::from(tailwind::NEUTRAL_300),
            NoteType::Black => Color::from(tailwind::NEUTRAL_800),
        }
    }

    pub fn get_mesh(key_in_octal: u8, asset_server: &Res<AssetServer>) -> Handle<Mesh> {
        match key_in_octal {
            0 => asset_server.load("models/white_key_0.gltf#Mesh0/Primitive0"),
            1 => asset_server.load("models/black_key_3.gltf#Mesh0/Primitive0"),
            2 => asset_server.load("models/white_key_1.gltf#Mesh0/Primitive0"),
            3 => asset_server.load("models/black_key_3.gltf#Mesh0/Primitive0"),
            4 => asset_server.load("models/white_key_2.gltf#Mesh0/Primitive0"),
            5 => asset_server.load("models/white_key_0.gltf#Mesh0/Primitive0"),
            6 => asset_server.load("models/black_key_3.gltf#Mesh0/Primitive0"),
            7 => asset_server.load("models/white_key_1.gltf#Mesh0/Primitive0"),
            8 => asset_server.load("models/black_key_3.gltf#Mesh0/Primitive0"),
            9 => asset_server.load("models/white_key_1.gltf#Mesh0/Primitive0"),
            10 => asset_server.load("models/black_key_3.gltf#Mesh0/Primitive0"),
            11 => asset_server.load("models/white_key_2.gltf#Mesh0/Primitive0"),
            _ => panic!("Invalid key in octal: {}", key_in_octal),
        }
    }

    pub fn display_press(
        mut query: Query<
            (&mut Transform, &Key, &MeshMaterial3d<StandardMaterial>),
            With<PressedKey>,
        >,
        // timer: Res<Time>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for (mut t, key, mat) in &mut query {
            t.translation.y = (t.translation.y - 0.03).max(-0.06);

            if let Some(material) = materials.get_mut(mat) {
                println!("Material Color: {:?}", material.base_color);
                material.base_color = Color::from(tailwind::RED_500);
            }
        }
    }

    pub fn display_release(
        mut query: Query<
            (&mut Transform, &Key, &MeshMaterial3d<StandardMaterial>),
            Without<PressedKey>,
        >,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for (mut t, k, mat) in &mut query {
            // t.translation.y = k.y_reset;
            t.translation.y = (t.translation.y + 0.03).min(k.y_reset);

            if let Some(material) = materials.get_mut(mat) {
                material.base_color = k.get_key_colour();
            }
        }
    }

    pub fn handle_midi_input(
        mut commands: Commands,
        mut midi_events: EventReader<MidiData>,
        query: Query<(Entity, &Key)>,
    ) {
        for data in midi_events.read() {
            let [_, index, _value] = data.message.msg;
            let off = index % 12;
            let oct = index.overflowing_div(12).0;
            let key_str = KEY_RANGE.iter().nth(off.into()).unwrap();

            println!("MIDI Event: {:?} {} {}", data.message, key_str, oct);

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
}

#[derive(Component)]
struct PressedKey;

#[derive(Component, Debug, Clone)]
enum NoteType {
    White,
    Black,
}

// #[rustfmt::skip]
#[hot(rerun_on_hot_patch = true)]
fn setup(
    mut cmds: Commands,
    standard_materials: ResMut<Assets<StandardMaterial>>,
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

    cmds.spawn((
        PointLight {
            intensity: 3000000.0,

            shadows_enabled: true,
            // range: 10.0,
            color: Color::from(tailwind::SKY_200),
            ..Default::default()
        },
        Transform::from_xyz(3.0, 5.0, mid * 2.0),
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

    Key::system_startup(cmds, standard_materials, asset_server);
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
