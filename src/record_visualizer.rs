use bevy::{log::tracing::span::Record, prelude::*};

pub struct RecordVisualizerPlugin;

impl Plugin for RecordVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, VolumeBar::startup)
            .add_systems(Update, VolumeBar::update);
    }
}

#[derive(Component)]
pub struct VolumeBar {
    id: usize,
    frequency: f64,
}

impl VolumeBar {
    pub fn startup(mut commands: Commands) {
        for idx in 1..1025 {
            let bar = VolumeBar::calculate_positions(&idx);
            commands.spawn(bar);
        }
    }

    fn calculate_positions(idx: &usize) -> (Node, BackgroundColor, VolumeBar) {
        let idx_f32 = *idx as f32;
        let height = Val::Px(20.0);
        let width = Val::Px(1.0);
        let left = Val::Percent(300.0 * (idx_f32 / 1025.0));

        return (
            Node {
                height,
                width,
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(idx_f32 / 1024.0, 1024.0 / idx_f32, 0.0)),
            VolumeBar {
                id: *idx,
                frequency: idx_f32 as f64 / (48000.0 / 2.0 + 1.0),
            },
        );
    }

    pub fn update(
        // sink: Single<&mut bevy_mic::spatial_audio::SpatialAudioSink>,
        mic: Res<crate::bevy_mic::microphone::MicrophoneAudio>,
        // mut query: Single<&mut Node, With<VolumeBar>>,
        mut query: Query<(&mut Node, &VolumeBar)>,
    ) {
        // info!("mic count: {}", mic.len());

        for owo in mic.try_iter() {
            let total_volume: f32 = owo.iter().map(|&x| x.abs()).sum();

            let frequencies = VolumeBar::analyze_frequencies(&owo);

            // todo detect piano notes

            for (mut volbar, voldata) in query.iter_mut() {
                let freqency = frequencies.get(voldata.id);
                if let Some(freq) = freqency {
                    let height = (freq * 10.0).max(1.0);

                    volbar.height = Val::Px(height as f32);
                    volbar.bottom = Val::Px(0.0);
                };
            }

            // sink.append(rodio::buffer::SamplesBuffer::new(
            //     mic.config.channels,
            //     mic.config.sample_rate,
            //     owo,
            // ));
        }
    }

    fn analyze_frequencies(samples: &[f32]) -> Vec<f64> {
        let mut planner = rustfft::FftPlanner::new();
        let fft = planner.plan_fft_forward(samples.len());

        // Apply window function
        let windowed: Vec<f32> = samples
            .iter()
            .zip(apodize::hanning_iter(samples.len()))
            .map(|(s, w)| s * w as f32)
            .collect();

        use rustfft::num_complex::Complex;

        // Convert to complex for FFT
        let mut buffer: Vec<Complex<f32>> =
            windowed.iter().map(|&x| Complex::new(x, 0.0)).collect();

        fft.process(&mut buffer);

        // Convert to magnitude spectrum
        buffer.iter().map(|c| c.norm() as f64).collect()
    }

    fn note_frequency(midi_note: u8) -> f64 {
        440.0 * 2.0_f64.powf((midi_note as f64 - 69.0) / 12.0)
    }
}
