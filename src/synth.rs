//! Synth engine extracted from keys.rs for use in both egui and Bevy MIDI systems.
#![allow(clippy::precedence)]

use fundsp::hacker::*;
use funutd::Rnd;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
    Organ,
    Hammond,
    Pulse,
    Pluck,
    Noise,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Filter {
    None,
    Moog,
    Butterworth,
    Bandpass,
    Peak,
    DirtyBiquad,
    FeedbackBiquad,
}

pub struct SynthEngine {
    pub rnd: Rnd,
    pub sequencer: Sequencer,
    pub waveform: Waveform,
    pub filter: Filter,
    pub vibrato_amount: f64,

    /// Chorus amount.
    chorus_amount: Shared,
    /// Reverb amount.
    reverb_amount: Shared,
    /// Reverb room size.
    room_size: f64,
    /// Reverb time in seconds.
    reverb_time: f64,
    /// Reverb diffusion.
    reverb_diffusion: f64,
    /// Reverb node ID.
    // reverb_id: NodeId,
    /// Phaser node ID.
    // phaser_id: NodeId,
    /// Phaser state.
    phaser_enabled: bool,
    /// Flanger node ID.
    flanger_id: NodeId,
    /// Flanger state.
    flanger_enabled: bool,
    /// Left channel data for the oscilloscope.
    snoop0: Snoop,
    /// Right channel data for the oscilloscope.
    snoop1: Snoop,
}

impl SynthEngine {
    pub fn new() -> Self {
        let room_size = 10.0;
        let reverb_amount = shared(0.25);
        let reverb_time = 2.0;
        let reverb_diffusion = 0.5;
        let chorus_amount = shared(1.0);

        let (snoop0, snoop_backend0) = snoop(32768);
        let (snoop1, snoop_backend1) = snoop(32768);

        Self {
            rnd: Rnd::from_u64(0),
            sequencer: Sequencer::new(false, 1),
            waveform: Waveform::Organ,
            filter: Filter::None,
            vibrato_amount: 0.25,
            chorus_amount,
            reverb_amount,
            room_size,
            reverb_time,
            reverb_diffusion,
            snoop0,
            snoop1,
            phaser_enabled: false,
            flanger_id: NodeId::new(),
            flanger_enabled: false,
        }
    }

    pub fn note_on(&mut self, midi_note: u8, velocity: f32) {
        let pitch_hz = midi_hz(midi_note as f64);
        let v = self.vibrato_amount * 0.006;
        let pitch = lfo(move |t| {
            pitch_hz
                * xerp11(
                    1.0 / (1.0 + v),
                    1.0 + v,
                    0.5 * (sin_hz(6.0, t) + sin_hz(6.1, t)),
                )
        });
        let waveform = match self.waveform {
            Waveform::Sine => Net::wrap(Box::new(pitch * 2.0 >> sine() * 0.1 * velocity as f32)),
            Waveform::Saw => Net::wrap(Box::new(pitch >> saw() * 0.2 * velocity as f32)),
            Waveform::Square => Net::wrap(Box::new(pitch >> square() * 0.2 * velocity as f32)),
            Waveform::Triangle => Net::wrap(Box::new(pitch >> triangle() * 0.2 * velocity as f32)),
            Waveform::Organ => Net::wrap(Box::new(pitch >> organ() * 0.2 * velocity as f32)),
            Waveform::Hammond => Net::wrap(Box::new(pitch >> hammond() * 0.2 * velocity as f32)),
            Waveform::Pulse => Net::wrap(Box::new(
                (pitch | lfo(move |t| lerp11(0.01, 0.99, sin_hz(0.1, t))))
                    >> pulse() * 0.2 * velocity as f32,
            )),
            Waveform::Pluck => Net::wrap(Box::new(
                zero() >> pluck(pitch_hz as f32, 0.5, 0.5) * 0.5 * velocity as f32,
            )),
            Waveform::Noise => Net::wrap(Box::new(
                (noise() | pitch * 4.0 | lfo(|t| funutd::math::lerp(2.0, 20.0, clamp01(t * 3.0))))
                    >> !resonator()
                    >> resonator()
                    >> shape(Adaptive::new(0.1, Atan(0.05))) * 0.5 * velocity as f32,
            )),
        };
        let filter = match self.filter {
            Filter::None => Net::wrap(Box::new(pass())),
            Filter::Moog => Net::wrap(Box::new(
                (pass() | lfo(move |t| (xerp11(400.0, 10000.0, cos_hz(0.1, t)), 0.6))) >> moog(),
            )),
            Filter::Butterworth => Net::wrap(Box::new(
                (pass() | lfo(move |t| max(400.0, 20000.0 * exp(-t * 5.0)))) >> butterpass(),
            )),
            Filter::Bandpass => Net::wrap(Box::new(
                (pass() | lfo(move |t| (xerp11(200.0, 10000.0, sin_hz(0.2, t)), 2.0)))
                    >> bandpass(),
            )),
            Filter::Peak => Net::wrap(Box::new(
                (pass() | lfo(move |t| (xerp11(200.0, 10000.0, sin_hz(0.2, t)), 2.0))) >> peak(),
            )),
            Filter::DirtyBiquad => Net::wrap(Box::new(
                (pass() | lfo(move |t| (max(800.0, 20000.0 * exp(-t * 6.0)), 3.0)))
                    >> !dlowpass(Tanh(1.02))
                    >> mul((1.0, 0.666, 1.0))
                    >> dlowpass(Tanh(1.02)),
            )),
            Filter::FeedbackBiquad => Net::wrap(Box::new(
                (mul(2.0) | lfo(move |t| (xerp11(200.0, 10000.0, sin_hz(0.2, t)), 5.0)))
                    >> fresonator(Softsign(1.10)),
            )),
        };
        let mut note = Box::new(waveform >> filter >> dcblock());
        note.ping(false, AttoHash::new(self.rnd.u64()));

        self.sequencer
            .push_relative(0.0, 2.0, Fade::Smooth, 0.03, 2.0, note);
    }

    pub fn note_off(&mut self, midi_note: u8) {
        // For now, just fade out all notes (improve to track note IDs per note)
        // for id in self.sequencer.events() {
        //     self.sequencer.edit_relative(*id, 0.2, 0.2);
        // }

        // self.sequencer.

        // self.sequencer.
    }

    pub fn backend(&mut self) -> Box<dyn AudioUnit> {
        Box::new(self.sequencer.backend())

        // self.sequencer.backend()
    }
}
