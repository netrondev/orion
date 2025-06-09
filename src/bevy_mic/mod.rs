pub mod audio_output;
pub mod microphone;
pub mod spatial_audio;

use audio_output::AudioOutputPlugin;
use bevy::app::PluginGroupBuilder;
use microphone::MicrophonePlugin;

pub struct ModAudioPlugins;

impl bevy::prelude::PluginGroup for ModAudioPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(spatial_audio::SpatialAudioPlugin)
            .add(AudioOutputPlugin)
            .add(MicrophonePlugin)
    }
}
