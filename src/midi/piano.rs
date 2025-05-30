use bevy::{color::palettes::tailwind, prelude::*, render::view::RenderLayers};

use crate::{
    game::game_main::{CameraSensitivity, VIEW_MODEL_RENDER_LAYER},
    midi,
};

use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

use super::bevy_resource::MidiReceiver;

#[derive(Debug, Component, Default)]
pub struct PianoKey {
    pub id: u8,
    // pub note: u8,
    // pub pressed: bool,
    pub key_height: f32,
    pub key_width: f32,
    pub key_type: PianoKeyType,
}

// impl PianoKey {
//     pub fn new(id: u8, key_height: f32, key_width: f32, key_type: PianoKeyType) -> Self {
//         let thisthing = PianoKey {
//             id,
//             // note,
//             // pressed: false,
//             key_height,
//             key_width,
//             key_type,
//         };

//         thisthing
//     }
// }

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

#[derive(Component)]
pub struct Piano {
    pub number_of_keys: u8,
    pub key_spacing: f32,
    pub keys: Vec<PianoKey>,
}

impl Piano {
    pub fn new(number_of_keys: u8) -> Self {
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
                key_height: 0.1,
                key_width: 0.19,
                key_type: PianoKeyType::White,
            });

            if has_black_key && i < number_of_keys - 1 {
                keys.push(PianoKey {
                    id: i,
                    // note: i + 1,
                    // pressed: false,
                    key_height: 0.2,
                    key_width: 0.05,
                    key_type: PianoKeyType::Black,
                });
            }
        }

        Piano {
            number_of_keys,
            key_spacing: 0.2,
            keys,
        }
    }

    pub fn spawn(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        entity: Entity,
    ) {
        let piano = commands
            .spawn((
                CameraSensitivity::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Visibility::default(),
                ChildOf(entity),
            ))
            .id();

        // Spawn piano keys
        for key in self.keys.iter() {
            let x = self.key_spacing * key.id as f32
                - (self.key_spacing * self.number_of_keys as f32 / 2.0);

            match key.key_type {
                crate::midi::piano::PianoKeyType::White => {
                    commands.spawn((
                        Name::new(format!("PianoKey {} {}", key.id, key.key_type.to_string())),
                        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                        MeshMaterial3d(materials.add(Color::from(tailwind::NEUTRAL_100))),
                        Transform::from_xyz(x, -1.0, -2.2).with_scale(Vec3::new(
                            key.key_width,
                            key.key_height,
                            0.5,
                        )),
                        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                        ChildOf(piano), // NotShadowCaster,
                    ));
                }
                crate::midi::piano::PianoKeyType::Black => {
                    commands.spawn((
                        Name::new(format!("PianoKey {} {}", key.id, key.key_type.to_string())),
                        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                        MeshMaterial3d(materials.add(Color::from(tailwind::NEUTRAL_900))),
                        Transform::from_xyz(x + self.key_spacing / 2.0, -0.95, -2.2 - 0.5 / 4.0)
                            .with_scale(Vec3::new(key.key_width, key.key_height / 2.0, 0.5)),
                        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
                        ChildOf(piano), // NotShadowCaster,
                    ));
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

    pub fn animate_midi(
        mut items: Query<(&mut Transform, &Name)>,
        time: Res<Time>,
        midi: Res<MidiReceiver>,
    ) -> Result {
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

pub fn piano_create(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let number_of_keys = 15;

    for x in 0..number_of_keys {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::from(tailwind::RED_500))),
            Transform::from_xyz(0.2 * x as f32, 0.10, 0.0).with_scale(Vec3::new(0.19, 0.1, -0.1)),
        ));
    }
}
