use crate::hot_despawn;
use bevy::prelude::*;
use bevy_ecs_macros::Component;
use bevy_simple_subsecond_system::hot;

pub struct SongLoaderPlugin;

impl Plugin for SongLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component, Debug)]
pub struct SongNote {
    pub time_start_sec: f64,
    pub time_end_sec: f64,
    pub key: u8,
    pub velocity: u8,
    pub key_name: String,
}

#[derive(Component)]
pub struct SongLoader {
    pub path: String,
    pub notes: Vec<SongNote>,
}

#[hot(rerun_on_hot_patch = true)]
pub fn setup(mut commands: Commands, prev_setup: Query<Entity, With<SongNote>>) {
    hot_despawn(&mut commands, prev_setup);

    let song = SongLoader::new("/home/rouan/work/orion/assets/songs/happy_bday_v1.mid");

    for note in song.notes {
        commands.spawn((note, Transform::default(), GlobalTransform::default()));
    }
}

impl SongLoader {
    pub fn new(path: &str) -> Self {
        let song = Self {
            path: path.to_string(),
            notes: vec![],
        };

        song.load_midi_binary();
        song
    }

    fn load_midi_binary(&self) -> Vec<SongNote> {
        let data = std::fs::read(&self.path).unwrap();
        let smf = midly::Smf::parse(&data).unwrap();

        let mut song_notes: Vec<SongNote> = vec![];
        let mut time_tracker = 0.0;
        let mut ms_per_tick = 2.6;

        for track in smf.tracks.iter() {
            // println!("Track {}: {:?}", i, track.len());

            for &event in track {
                // println!("{:?}", event);

                time_tracker += u32::from(event.delta) as f64;

                match event.kind {
                    midly::TrackEventKind::Midi {
                        channel: _,
                        message,
                    } => match message {
                        midly::MidiMessage::NoteOn { key, vel } => {
                            // println!(
                            //     "Note On: Channel {}, Key {}, Velocity {}",
                            //     channel, key, vel
                            // );

                            let note = SongNote {
                                time_start_sec: time_tracker * ms_per_tick / 1000.0,
                                time_end_sec: 0.0,
                                key: u8::from(key),
                                velocity: u8::from(vel),
                                key_name: Self::midi_to_piano_key(u8::from(key))
                                    .unwrap_or_else(|| "ERR".to_string()),
                            };

                            song_notes.push(note);
                        }
                        midly::MidiMessage::NoteOff { key, vel: _ } => {
                            song_notes.iter_mut().for_each(|note| {
                                if note.key == u8::from(key) && note.time_end_sec == 0.0 {
                                    note.time_end_sec = time_tracker * ms_per_tick / 1000.0;
                                }
                            });

                            // println!(
                            //     "Note Off: Channel {}, Key {}, Velocity {}",
                            //     channel, key, vel
                            // );
                        }
                        _ => {
                            println!("Other MIDI message: {:?}", message);
                        }
                    },
                    midly::TrackEventKind::Meta(meta) => match meta {
                        midly::MetaMessage::Tempo(tempo) => {
                            let tempo_ms_per_beat = u32::from(tempo) as f64;
                            let bpm = 60_000_000.0 / tempo_ms_per_beat;

                            ms_per_tick = 60000.0 / (bpm * 192.0);

                            println!(
                                "tempo_ms_per_beat: {} bpm {} ms_per_tick {}",
                                tempo_ms_per_beat, bpm, ms_per_tick
                            );
                        }
                        _ => {
                            println!("Other Meta message: {:?}", meta);
                        }
                    },
                    _ => {}
                }
            }
        }

        // println!("Total notes: {}", song_notes.len());
        // for note in &song_notes {
        //     println!(
        //         "{} \t{:.2}-{:.2} \t(vel {})",
        //         Self::midi_to_piano_key(note.key).unwrap(),
        //         note.time_start_sec,
        //         note.time_end_sec,
        //         note.velocity
        //     );
        // }

        song_notes
    }

    pub fn midi_to_piano_key(midi_key: u8) -> Option<String> {
        if midi_key > 127 {
            return None;
        }

        let note_names = [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
        ];
        let octave = (midi_key / 12) as i32 - 1;
        let note_index = (midi_key % 12) as usize;

        Some(format!("{}{}", note_names[note_index], octave))
    }
}

#[test]
fn test_load_midi_binary() {
    let song = SongLoader::new("/home/rouan/work/orion/assets/songs/happy_bday_v1.mid");
}

#[test]
fn test_midi_to_piano_key() {
    assert_eq!(SongLoader::midi_to_piano_key(60), Some("C4".to_string())); // Middle C
    assert_eq!(SongLoader::midi_to_piano_key(69), Some("A4".to_string())); // A440
    assert_eq!(SongLoader::midi_to_piano_key(21), Some("A0".to_string())); // Lowest A on piano
    assert_eq!(SongLoader::midi_to_piano_key(108), Some("C8".to_string())); // Highest C on piano
    assert_eq!(SongLoader::midi_to_piano_key(0), Some("C-1".to_string())); // Lowest MIDI note
    assert_eq!(SongLoader::midi_to_piano_key(127), Some("G9".to_string())); // Highest MIDI note
    assert_eq!(SongLoader::midi_to_piano_key(128), None); // Invalid MIDI note
}
