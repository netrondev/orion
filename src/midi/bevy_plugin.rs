// see https://github.com/bevyengine/bevy/blob/v0.14.0/examples/app/plugin_group.rs
// and bevy_midi
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct PianoPlugin;

impl PluginGroup for PianoPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PrintHelloPlugin)
            .add(PrintWorldPlugin)
    }
}

struct PrintHelloPlugin;

impl Plugin for PrintHelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_hello_system);
    }
}

fn print_hello_system() {
    info!("hello");
}

struct PrintWorldPlugin;

impl Plugin for PrintWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_world_system);
    }
}

fn print_world_system() {
    info!("world");
}
