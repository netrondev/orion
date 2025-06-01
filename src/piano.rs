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
