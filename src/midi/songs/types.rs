#[derive(Clone, Copy)]
pub struct SongNote {
    pub midi_note: u8,
    pub start_beat: f32,
    pub duration_beats: f32,
}
