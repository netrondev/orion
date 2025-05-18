use bevy::prelude::*;
use midir::{Ignore, MidiInput};
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};
use std::thread;

const NOTE_SPEED: f32 = 120.0; // pixels per second
const NOTE_START_Y: f32 = 300.0;
const NOTE_END_Y: f32 = -300.0;
const NOTE_WIDTH: f32 = 40.0;
const NOTE_HEIGHT: f32 = 20.0;
const NOTE_LANE_WIDTH: f32 = 60.0;
const NOTE_OFFSET_X: f32 = -180.0; // Center notes

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, start_midi_listener, spawn_song_notes))
        .add_systems(Update, animate_notes)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// MIDI event resource
enum MidiEvent {
    NoteOn(u8, u8),  // (note, velocity)
    NoteOff(u8, u8), // (note, velocity)
}

#[derive(Resource, Clone)]
struct MidiReceiver(Arc<Mutex<Receiver<MidiEvent>>>);

fn start_midi_listener(mut commands: Commands) {
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
        let in_port = &in_ports[0];
        println!("Opening MIDI port: {}", midi_in.port_name(in_port).unwrap());
        let _conn_in = midi_in.connect(
            in_port,
            "bevy-midi-read",
            move |_, message, _| {
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

// --- Song Data: Faithless - Insomnia (Simplified Main Riff) ---
// MIDI note numbers for C4 = 60
// Format: (note, start_time_in_beats, duration_in_beats)
const SONG_TEMPO_BPM: f32 = 120.0;
const SONG_BEAT_DURATION: f32 = 60.0 / SONG_TEMPO_BPM;

#[derive(Clone, Copy)]
struct SongNote {
    midi_note: u8,
    start_beat: f32,
    duration_beats: f32,
}

const SONG_NOTES: [SongNote; 15] = [
    SongNote {
        midi_note: 64,
        start_beat: 0.0,
        duration_beats: 1.0,
    }, // E4
    SongNote {
        midi_note: 67,
        start_beat: 1.0,
        duration_beats: 1.0,
    }, // G4
    SongNote {
        midi_note: 69,
        start_beat: 2.0,
        duration_beats: 1.0,
    }, // A4
    SongNote {
        midi_note: 67,
        start_beat: 3.0,
        duration_beats: 1.0,
    }, // G4
    SongNote {
        midi_note: 64,
        start_beat: 4.0,
        duration_beats: 1.0,
    }, // E4
    SongNote {
        midi_note: 62,
        start_beat: 5.0,
        duration_beats: 1.0,
    }, // D4
    SongNote {
        midi_note: 60,
        start_beat: 6.0,
        duration_beats: 2.0,
    }, // C4 (hold)
    SongNote {
        midi_note: 62,
        start_beat: 8.0,
        duration_beats: 1.0,
    }, // D4
    SongNote {
        midi_note: 64,
        start_beat: 9.0,
        duration_beats: 1.0,
    }, // E4
    SongNote {
        midi_note: 67,
        start_beat: 10.0,
        duration_beats: 1.0,
    }, // G4
    SongNote {
        midi_note: 69,
        start_beat: 11.0,
        duration_beats: 1.0,
    }, // A4
    SongNote {
        midi_note: 67,
        start_beat: 12.0,
        duration_beats: 1.0,
    }, // G4
    SongNote {
        midi_note: 64,
        start_beat: 13.0,
        duration_beats: 1.0,
    }, // E4
    SongNote {
        midi_note: 62,
        start_beat: 14.0,
        duration_beats: 1.0,
    }, // D4
    SongNote {
        midi_note: 60,
        start_beat: 15.0,
        duration_beats: 2.0,
    }, // C4 (hold)
];

#[derive(Component)]
struct FallingNote;

fn spawn_song_notes(mut commands: Commands, time: Res<Time>) {
    let start_time = time.elapsed_seconds();
    for (i, note) in SONG_NOTES.iter().enumerate() {
        // Each note is a colored rectangle in a "lane" based on its pitch
        let lane = (note.midi_note as i32) - 60; // C4 = lane 0
        let x = NOTE_OFFSET_X + lane as f32 * NOTE_LANE_WIDTH;
        let y = NOTE_START_Y + note.start_beat * NOTE_SPEED;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            FallingNote,
        ));
    }
}

fn animate_notes(mut query: Query<&mut Transform, With<FallingNote>>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for mut transform in &mut query {
        transform.translation.y -= NOTE_SPEED * delta;
    }
}

// TODO: Add systems for scoring, feedback, and MIDI note matching
