#![allow(clippy::precedence)]

use bevy::prelude::*;

use bevy_procedural_audio::dsp_graph::DspGraph;
use bevy_procedural_audio::prelude::*;

use uuid::Uuid;

use crate::bevy_midi::input::MidiData;
use crate::synth::{Filter, SynthEngine, Waveform};

use std::sync::{Arc, Mutex};

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(DspPlugin::default())
//         .add_plugins(PianoPlugin)
//         .run();
// }

pub struct PianoPlugin;

struct PianoDsp<F>(F);

impl<F: Send + Sync + 'static + Fn() -> Box<dyn AudioUnit>> DspGraph for PianoDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        (self.0)()
    }
}

#[derive(Debug, Resource)]
struct PianoId(Uuid);

#[derive(Resource)]
struct SharedSynthEngine(Arc<Mutex<SynthEngine>>);

impl Plugin for PianoPlugin {
    fn build(&self, app: &mut App) {
        let synth = SynthEngine::new();
        let synth_mutex = Arc::new(Mutex::new(synth));
        let synth_mutex_for_dsp = synth_mutex.clone();
        let piano = move || {
            let mut synth = synth_mutex_for_dsp.lock().unwrap();
            synth.backend()
        };
        let piano_dsp = PianoDsp(piano);
        let piano_id = DspGraph::id(&piano_dsp);

        app.add_dsp_source(piano_dsp, SourceType::Dynamic)
            .insert_resource(SharedSynthEngine(synth_mutex))
            .insert_resource(PianoId(piano_id))
            .add_systems(Update, switch_key)
            .add_systems(PostStartup, play_piano);
    }
}

fn switch_key(mut midi_events: EventReader<MidiData>, synth: Res<SharedSynthEngine>) {
    for data in midi_events.read() {
        if data.message.is_note_on() {
            let note = data.message.msg[1];
            let velocity = data.message.msg[2] as f32 / 127.0;
            let mut synth = synth.0.lock().unwrap();
            synth.note_on(note, velocity);
        }
        if data.message.is_note_off() {
            let note = data.message.msg[1];
            let mut synth = synth.0.lock().unwrap();
            synth.note_off(note);
        }
    }
}

fn play_piano(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    piano_id: Res<PianoId>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph_by_id(&piano_id.0)
            .unwrap_or_else(|| panic!("DSP source not found!")),
    );
    commands.spawn(AudioPlayer(source));
}
