// see https://github.com/bevyengine/bevy/blob/v0.14.0/examples/app/plugin_group.rs
// and bevy_midi
use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::midi::{game::spawn_midi_world, midi_key_listener::midi_key_listener};

pub struct PlayerPiano;

impl Plugin for PlayerPiano {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (midi_key_listener, spawn_midi_world));
    }
}

// fn print_hello_system() {
//     info!("hello");
// }

// struct PrintWorldPlugin;

// impl Plugin for PrintWorldPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, print_world_system);
//     }
// }

// fn print_world_system() {
//     info!("world");
// }
