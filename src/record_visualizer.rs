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
            let frequencies = detect_frequencies(&owo, 48000);

            // let max = owo.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            // info!("{}", max);
            // query.width = Val::Px(100.0 * max);

            for (mut volbar, voldata) in query.iter_mut() {
                let freqency = frequencies.get(voldata.id);
                if let Some(freq) = freqency {
                    let height = (freq.1 * 10.0).max(1.0);

                    volbar.height = Val::Px(height);
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
}

fn detect_frequencies(samples: &[f32], sample_rate: usize) -> Vec<(f32, f32)> {
    let mut planner = realfft::RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(samples.len());
    let mut spectrum = fft.make_output_vec();
    let mut buffer = samples.to_vec();
    fft.process(&mut buffer, &mut spectrum).unwrap();

    // Convert bin to frequency
    let frequencies = spectrum
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let freq = i as f32 * sample_rate as f32 / samples.len() as f32;
            (freq, c.norm())
        })
        .collect::<Vec<_>>();

    frequencies
}
