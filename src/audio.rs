#[derive(Resource)]
struct PitchVar(Shared<f32>);

impl PitchVar {
    fn set_pitch(&self, pitch: Pitch) {
        self.0.set_value(pitch.into());
    }
}
