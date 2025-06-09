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
            if total_volume > 10.0 {
                let piano_notes = VolumeBar::detect_piano_notes(&frequencies, 48000.0);
                // No significant audio detected, skip processing
                continue;
            }

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

    fn detect_piano_notes(spectrum: &[f64], sample_rate: f64) -> Vec<(u8, f64)> {
        let mut detected_notes = Vec::new();

        for midi_note in 21..=108 {
            let fundamental_freq = VolumeBar::note_frequency(midi_note);
            let bin_index = (fundamental_freq * spectrum.len() as f64 / sample_rate) as usize;

            if bin_index < spectrum.len() && spectrum[bin_index] > 0.01 {
                let harmonic_strength = check_harmonics(spectrum, fundamental_freq, sample_rate);

                // Threshold for considering it a piano note
                if harmonic_strength > 1.10 {
                    println!(
                        "[{}] {}Hz: {:.2}",
                        midi_note, fundamental_freq, harmonic_strength
                    );

                    detected_notes.push((midi_note, harmonic_strength));
                }
            }
        }

        detected_notes
    }
}

use std::cmp;

/// Checks for harmonic content that's characteristic of piano notes
/// Returns a score between 0.0 and 1.0 indicating how "piano-like" the harmonics are
fn check_harmonics(spectrum: &[f64], fundamental_freq: f64, sample_rate: f64) -> f64 {
    let nyquist = sample_rate / 2.0;
    let bin_resolution = sample_rate / spectrum.len() as f64;

    // Piano harmonics typically have these relative strengths (approximate)
    // This is based on typical piano harmonic analysis
    let expected_harmonic_ratios = vec![
        (1.0, 1.0),  // Fundamental (100%)
        (2.0, 0.8),  // 2nd harmonic (strong)
        (3.0, 0.6),  // 3rd harmonic (moderate)
        (4.0, 0.4),  // 4th harmonic (weaker)
        (5.0, 0.3),  // 5th harmonic
        (6.0, 0.2),  // 6th harmonic
        (7.0, 0.15), // 7th harmonic
        (8.0, 0.1),  // 8th harmonic (very weak)
    ];

    let mut harmonic_score = 0.0;
    let mut total_weight = 0.0;
    let mut detected_harmonics = 0;

    // Get the fundamental magnitude for reference
    let fundamental_bin = freq_to_bin(fundamental_freq, bin_resolution);
    if fundamental_bin >= spectrum.len() {
        return 0.0;
    }

    let fundamental_magnitude = get_peak_magnitude(spectrum, fundamental_bin, 2);

    // If fundamental is too weak, this probably isn't a real note
    if fundamental_magnitude < 0.01 {
        return 0.0;
    }

    // Check each expected harmonic
    for (harmonic_ratio, expected_strength) in expected_harmonic_ratios {
        let harmonic_freq = fundamental_freq * harmonic_ratio;

        // Skip harmonics above Nyquist frequency
        if harmonic_freq >= nyquist {
            break;
        }

        let harmonic_bin = freq_to_bin(harmonic_freq, bin_resolution);
        if harmonic_bin >= spectrum.len() {
            break;
        }

        // Get magnitude around the harmonic (allow for some frequency drift)
        let harmonic_magnitude = get_peak_magnitude(spectrum, harmonic_bin, 3);

        // Calculate the ratio relative to fundamental
        let actual_ratio = harmonic_magnitude / fundamental_magnitude;

        // Compare to expected ratio - piano harmonics should follow a pattern
        let expected_ratio = expected_strength;

        // Calculate how well this harmonic matches expectations
        let harmonic_match = if actual_ratio > 0.05 {
            // Only consider significant harmonics
            let ratio_error = (actual_ratio - expected_ratio).abs() / expected_ratio;
            let match_score = (1.0 - ratio_error.min(1.0)).max(0.0);

            // Weight higher harmonics less (they're less reliable)
            let weight = 1.0 / harmonic_ratio;

            detected_harmonics += 1;
            weight * match_score
        } else if harmonic_ratio <= 4.0 {
            // Penalize missing low harmonics (2nd, 3rd, 4th should be present)
            -0.2 / harmonic_ratio
        } else {
            0.0
        };

        harmonic_score += harmonic_match;
        total_weight += 1.0 / harmonic_ratio;
    }

    // Additional checks for piano-specific characteristics

    // 1. Check for inharmonicity (piano strings are slightly inharmonic)
    let inharmonicity_bonus = check_inharmonicity(spectrum, fundamental_freq, sample_rate);

    // 2. Check spectral centroid (pianos have characteristic brightness)
    let spectral_centroid_bonus = check_spectral_centroid(spectrum, fundamental_freq, sample_rate);

    // 3. Penalize if we detect very few harmonics (noise might have strong fundamental)
    let harmonic_count_bonus = if detected_harmonics >= 3 { 0.1 } else { -0.2 };

    // Final score calculation
    let base_score = if total_weight > 0.0 {
        harmonic_score / total_weight
    } else {
        0.0
    };

    let final_score =
        base_score + inharmonicity_bonus + spectral_centroid_bonus + harmonic_count_bonus;

    final_score
}

/// Convert frequency to FFT bin index
fn freq_to_bin(freq: f64, bin_resolution: f64) -> usize {
    (freq / bin_resolution).round() as usize
}

/// Get the peak magnitude in a small window around a bin (handles frequency drift)
fn get_peak_magnitude(spectrum: &[f64], center_bin: usize, window_size: usize) -> f64 {
    let start = center_bin.saturating_sub(window_size);
    let end = cmp::min(center_bin + window_size + 1, spectrum.len());

    spectrum[start..end].iter().copied().fold(0.0, f64::max)
}

/// Check for inharmonicity characteristic of piano strings
/// Piano strings are under high tension and exhibit slight inharmonicity
fn check_inharmonicity(spectrum: &[f64], fundamental_freq: f64, sample_rate: f64) -> f64 {
    let bin_resolution = sample_rate / spectrum.len() as f64;

    // Piano inharmonicity typically shifts higher harmonics slightly sharp
    // Check if 2nd and 3rd harmonics are slightly higher than exact multiples
    let mut inharmonicity_score = 0.0;

    for harmonic_num in 2..=4 {
        let expected_freq = fundamental_freq * harmonic_num as f64;
        let expected_bin = freq_to_bin(expected_freq, bin_resolution);

        if expected_bin + 5 < spectrum.len() {
            // Look for peak slightly above the expected harmonic
            let exact_magnitude = spectrum[expected_bin];
            let sharp_magnitude = get_peak_magnitude(spectrum, expected_bin + 2, 2);

            // Piano harmonics tend to be slightly sharp due to inharmonicity
            if sharp_magnitude > exact_magnitude * 1.1 {
                inharmonicity_score += 0.05;
            }
        }
    }

    inharmonicity_score
}

/// Check spectral centroid to identify piano-like brightness
fn check_spectral_centroid(spectrum: &[f64], fundamental_freq: f64, sample_rate: f64) -> f64 {
    let bin_resolution = sample_rate / spectrum.len() as f64;
    let fundamental_bin = freq_to_bin(fundamental_freq, bin_resolution);

    // Calculate spectral centroid for the region around this note
    let start_bin = fundamental_bin;
    let end_bin = cmp::min(
        fundamental_bin + (8.0 * fundamental_freq / bin_resolution) as usize,
        spectrum.len(),
    );

    let mut weighted_sum = 0.0;
    let mut magnitude_sum = 0.0;

    for (i, &magnitude) in spectrum[start_bin..end_bin].iter().enumerate() {
        let freq = (start_bin + i) as f64 * bin_resolution;
        weighted_sum += freq * magnitude;
        magnitude_sum += magnitude;
    }

    if magnitude_sum > 0.0 {
        let centroid = weighted_sum / magnitude_sum;
        let centroid_ratio = centroid / fundamental_freq;

        // Piano notes typically have centroid between 2-6 times the fundamental
        // depending on the register and playing style
        if centroid_ratio >= 2.0 && centroid_ratio <= 8.0 {
            0.1
        } else {
            -0.05
        }
    } else {
        0.0
    }
}
