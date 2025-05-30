use super::types::SongNote;

// --- Song Data: Faithless - Insomnia (Simplified Main Riff) ---
// MIDI note numbers for C4 = 60
// Format: (note, start_time_in_beats, duration_in_beats)
pub const SONG_TEMPO_BPM: f32 = 120.0;
pub const SONG_BEAT_DURATION: f32 = 60.0 / SONG_TEMPO_BPM;
// Center notes

pub const SONG_NOTES: [SongNote; 15] = [
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
