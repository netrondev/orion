use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

mod piano;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(piano::PlayerPiano)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
