use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    color::palettes::tailwind,
    ecs::{resource::Resource, system::Commands},
};

use bevy_simple_subsecond_system::hot;
use midir::{Ignore, MidiInput};
use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
};
use std::thread;
pub struct PlayerPiano;

impl Plugin for PlayerPiano {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (shapes, midi_key_listener, spawn_view_model))
            .add_systems(Update, configure_ui);
    }
}

#[hot(rerun_on_hot_patch = true)]
fn shapes(
    previous_setup: Query<Entity, With<PianoKeyComponent>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    // Clear all entities that were spawned on `Startup` so that
    // hot-patching does not spawn them again
    for entity in previous_setup.iter() {
        commands.entity(entity).despawn();
    }

    Piano::new(15).spawn(commands, meshes, materials);
}

#[derive(Component)]
#[require(Node)]
struct Ui;

#[hot]
fn configure_ui(ui: Single<Entity, With<Ui>>, mut commands: Commands) {
    commands.entity(*ui).despawn_related::<Children>().insert((
        Node {
            // You can change the `Node` however you want at runtime
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        children![
            Text::new("Hello, world!"),
            Text::new("Try adding new texts below"),
        ],
    ));
}

pub enum MidiEvent {
    NoteOn(u8, u8),  // (note, velocity)
    NoteOff(u8, u8), // (note, velocity)
}

#[derive(Resource, Clone)]
pub struct MidiReceiver(Arc<Mutex<Receiver<MidiEvent>>>);

pub fn midi_key_listener(
    mut commands: Commands, // mut ev_levelup: EventWriter<LevelUpEvent>,
    mut text: Single<&mut Text>,
) {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    thread::spawn(move || {
        let mut midi_in = MidiInput::new("bevy-midi-input").expect("Failed to open MIDI input");
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        if in_ports.is_empty() {
            println!("No MIDI input ports found.");
            return;
        }
        let in_port = &in_ports[1];
        println!("Opening MIDI port: {}", midi_in.port_name(in_port).unwrap());
        let _conn_in = midi_in.connect(
            in_port,
            "bevy-midi-read",
            move |_, message, _| {
                println!("Received MIDI message: {:?}", message);

                if message.len() >= 3 {
                    match message[0] & 0xF0 {
                        0x90 => {
                            // Note On
                            let note = message[1];
                            let velocity = message[2];
                            let _ = tx.send(MidiEvent::NoteOn(note, velocity));
                        }
                        0x80 => {
                            // Note Off
                            let note = message[1];
                            let velocity = message[2];
                            let _ = tx.send(MidiEvent::NoteOff(note, velocity));
                        }
                        _ => {}
                    }
                }
            },
            (),
        );
        // Keep thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    commands.insert_resource(MidiReceiver(rx));
}

fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    Piano::new(15).spawn(commands, meshes, materials);
}

#[derive(Component)]
pub struct PianoKeyComponent;

#[derive(Debug, Component, Default)]
pub struct PianoKey {
    pub id: u8,
    // pub note: u8,
    // pub pressed: bool,
    pub key_length: f32,
    pub key_height: f32,
    pub key_width: f32,
    pub key_type: PianoKeyType,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PianoKeyType {
    #[default]
    White,
    Black,
}

impl PianoKeyType {
    pub fn to_string(&self) -> String {
        match self {
            PianoKeyType::White => "white".to_string(),
            PianoKeyType::Black => "black".to_string(),
        }
    }
}

const X_EXTENT: f32 = 900.;

#[derive(Component)]
pub struct Piano {
    pub number_of_keys: u8,
    pub scale: f32,
    pub key_spacing: f32,
    /// the length of the white keys
    pub key_length_main: f32,
    pub keys: Vec<PianoKey>,
}

impl Piano {
    pub fn new(number_of_keys: u8) -> Self {
        let scale = 200.0;
        let key_length_main = 0.90 * scale;

        let mut keys = Vec::new();

        for i in 0..number_of_keys {
            let has_black_key = match i % 7 {
                0 | 1 | 3 | 4 | 5 => true,
                _ => false,
            };

            keys.push(PianoKey {
                id: i,
                // note: i,
                // pressed: false,
                key_length: key_length_main,
                key_height: 0.10 * scale,
                key_width: 0.19 * scale,
                key_type: PianoKeyType::White,
            });

            if has_black_key && i < number_of_keys - 1 {
                keys.push(PianoKey {
                    id: i,
                    // note: i + 1,
                    // pressed: false,
                    key_length: 0.5 * scale,
                    key_height: 0.2 * scale,
                    key_width: 0.08 * scale,
                    key_type: PianoKeyType::Black,
                });
            }
        }

        Piano {
            number_of_keys,
            key_spacing: 0.2 * scale,
            key_length_main,
            scale,
            keys,
        }
    }

    pub fn spawn(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        // mut materials: ResMut<Assets<StandardMaterial>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        // entity: Entity,
    ) {
        // let piano = commands
        //     .spawn((
        //         Transform::from_xyz(0.0, 0.0, 0.0),
        //         Visibility::default(),
        //         // ChildOf(entity),
        //     ))
        //     .id();

        // Spawn piano keys
        for key in self.keys.iter() {
            let x = (self.key_spacing * key.id as f32
                - (self.key_spacing * self.number_of_keys as f32 / 2.0));

            info!("Spawning piano key {:#?} at x: {}", key, x);

            match key.key_type {
                PianoKeyType::White => {
                    // let color = Color::hsl(360 as f32, 0.95, 0.7);
                    commands.spawn((
                        PianoKeyComponent,
                        Mesh2d(meshes.add(Rectangle::new(key.key_width, key.key_length))),
                        MeshMaterial2d(materials.add(Color::from(tailwind::NEUTRAL_200))),
                        Transform::from_xyz(
                            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                            x,
                            key.key_length / 2.0,
                            key.key_height,
                        ),
                    ));

                    // commands.spawn((
                    //     Name::new(format!("PianoKey {} {}", key.id, key.key_type.to_string())),
                    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    //     MeshMaterial3d(materials.add(Color::from(tailwind::NEUTRAL_100))),
                    //     Transform::from_xyz(x, -1.0, -2.2).with_scale(Vec3::new(
                    //         key.key_width,
                    //         key.key_height,
                    //         0.5,
                    //     )),
                    //     ChildOf(piano), // NotShadowCaster,
                    // ));
                }
                PianoKeyType::Black => {
                    commands.spawn((
                        PianoKeyComponent,
                        Mesh2d(meshes.add(Rectangle::new(key.key_width, key.key_length))),
                        MeshMaterial2d(materials.add(Color::from(tailwind::NEUTRAL_900))),
                        Transform::from_xyz(
                            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                            x + self.key_spacing / 2.0,
                            self.key_length_main - key.key_length / 2.0,
                            key.key_height,
                        ),
                    ));
                    // commands.spawn((
                    //     Name::new(format!("PianoKey {} {}", key.id, key.key_type.to_string())),
                    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    //     MeshMaterial3d(materials.add(Color::from(tailwind::NEUTRAL_900))),
                    //     Transform::from_xyz(x + self.key_spacing / 2.0, -0.95, -2.2 - 0.5 / 4.0)
                    //         .with_scale(Vec3::new(key.key_width, key.key_height / 2.0, 0.5)),
                    //     ChildOf(piano), // NotShadowCaster,
                    // ));
                }
            }
        }
    }

    pub fn animate(mut items: Query<(&mut Transform, &Name)>, time: Res<Time>) -> Result {
        // println!("Animating piano keys {:#?}", items);

        for (mut item_of, name) in &mut items {
            // info!(?name);

            if name.to_string().contains("black") {
                item_of.rotation.x = time.elapsed_secs().sin() / 10.0;
            }
        }

        Ok(())
    }

    pub fn animate_midi(mut items: Query<(&mut Transform, &Name)>, time: Res<Time>) -> Result {
        // println!("Animating piano keys {:#?}", items);

        // let midi = midi.lock().unwrap();

        for (mut item_of, name) in &mut items {
            // info!(?name);

            if name.to_string().contains("black") {
                item_of.rotation.x = time.elapsed_secs().sin() / 10.0;
            }
        }

        Ok(())
    }
}
