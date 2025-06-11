struct SongNote {
    pub time_start_sec: f64,
    pub time_end_sec: f64,
    pub key: u8,
    pub velocity: u8,
}

async fn load_midi_binary() -> Vec<SongNote> {
    let data = std::fs::read("/home/rouan/work/orion/assets/songs/happy_bday_v1.mid").unwrap();
    let smf = midly::Smf::parse(&data).unwrap();

    let mut song_notes: Vec<SongNote> = vec![];
    let mut time_tracker = 0.0;

    for (i, track) in smf.tracks.iter().enumerate() {
        // println!("Track {}: {:?}", i, track.len());

        for &event in track {
            // println!("{:?}", event);

            time_tracker += u32::from(event.delta) as f64;

            match event.kind {
                midly::TrackEventKind::Midi { channel, message } => match message {
                    midly::MidiMessage::NoteOn { key, vel } => {
                        // println!(
                        //     "Note On: Channel {}, Key {}, Velocity {}",
                        //     channel, key, vel
                        // );

                        let note = SongNote {
                            time_start_sec: time_tracker,
                            time_end_sec: 0.0,
                            key: u8::from(key),
                            velocity: u8::from(vel),
                        };

                        song_notes.push(note);
                    }
                    midly::MidiMessage::NoteOff { key, vel } => {
                        song_notes.iter_mut().for_each(|note| {
                            if note.key == u8::from(key) && note.time_end_sec == 0.0 {
                                note.time_end_sec = time_tracker;
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

                        let ms_per_tick = 60000.0 / (bpm * 192.0);
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

    println!("Total notes: {}", song_notes.len());
    for note in &song_notes {
        println!(
            "Note: Start: {:.2} sec, End: {:.2} sec, Key: {}, Velocity: {}",
            note.time_start_sec, note.time_end_sec, note.key, note.velocity
        );
    }

    song_notes
}

#[tokio::test]
async fn test_load_midi_binary() {
    // This is a test to ensure that the MIDI file can be loaded and parsed correctly.
    load_midi_binary().await;
    // You can add assertions here to check the contents of the MIDI file if needed.
}
