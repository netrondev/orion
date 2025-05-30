pub mod bevy_plugin;
pub mod connection;
pub mod game;
pub mod midi_key_listener;
pub mod piano;
pub mod songs;
use crate::midi::bevy_plugin::PlayerPiano;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct PianoPlugin;
impl PluginGroup for PianoPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(PlayerPiano)
        // .add(PrintWorldPlugin)
    }
}
