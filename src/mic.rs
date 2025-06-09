use anyhow::Result;
use bevy::{color::palettes::basic::*, prelude::*};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use flume::Receiver;
use realfft::RealFftPlanner;
use std::sync::{Arc, Mutex};

use crate::bevy_mic;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource, Default)]
pub struct RecordingState {
    pub is_recording: bool,
}

#[derive(Component)]
pub struct VolumeBar;

#[derive(Resource, Default)]
pub struct MicAmplitude(pub Arc<Mutex<f32>>);

#[derive(Resource, Default)]
pub struct AudioBuffer(pub Arc<Mutex<Vec<f32>>>);

static mut MIC_STREAM: Option<cpal::Stream> = None;

fn detect_frequencies(samples: &[f32], sample_rate: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
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

    // Return top dominant frequencies
    frequencies
        .into_iter()
        .filter(|(_, amp)| *amp > 0.1)
        .map(|(f, _)| f)
        .collect()
}

fn start_microphone_stream<F: FnMut(&[f32]) + Send + 'static>(
    mut callback: F,
) -> Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");
    let config = device.default_input_config()?;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| callback(data),
            move |err| eprintln!("stream error: {}", err),
            None,
        )?,
        _ => panic!("unsupported format"),
    };

    stream.play()?;
    Ok(stream)
}

pub fn ui_system_update_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut recording_state: ResMut<RecordingState>,
    mic_amplitude: Res<MicAmplitude>,
    audio_buffer: Res<AudioBuffer>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // Toggle recording state
                recording_state.is_recording = !recording_state.is_recording;

                // if recording_state.is_recording {
                //     **text = "Stop".to_string();
                //     *color = PRESSED_BUTTON.into();
                //     border_color.0 = RED.into();
                //     // Start mic stream
                //     unsafe {
                //         if MIC_STREAM.is_none() {
                //             let buffer = audio_buffer.0.clone();
                //             let mic_amplitude_clone = mic_amplitude.0.clone();
                //             MIC_STREAM = Some(
                //                 start_microphone_stream(move |data: &[f32]| {
                //                     let mut buf = buffer.lock().unwrap();
                //                     buf.extend_from_slice(data);
                //                     // Optionally update amplitude for volume bar
                //                     let rms = if !data.is_empty() {
                //                         (data.iter().map(|s| s * s).sum::<f32>()
                //                             / data.len() as f32)
                //                             .sqrt()
                //                     } else {
                //                         0.0
                //                     };
                //                     // This is a hack: update amplitude globally for the UI
                //                     if let Ok(mut amp) = mic_amplitude_clone.lock() {
                //                         *amp = rms;
                //                     }
                //                 })
                //                 .expect("Failed to start mic stream"),
                //             );
                //         }
                //     }
                // } else {
                //     **text = "Record".to_string();
                //     *color = NORMAL_BUTTON.into();
                //     border_color.0 = Color::BLACK;
                //     // Stop mic stream and process buffer
                //     let sample_rate = 44100; // TODO: get from config
                //     let samples = {
                //         let mut buf = audio_buffer.0.lock().unwrap();
                //         let samples = buf.clone();
                //         buf.clear();
                //         samples
                //     };
                //     let freqs = detect_frequencies(&samples, sample_rate);
                //     println!("Detected frequencies: {:?}", freqs);
                //     unsafe {
                //         MIC_STREAM = None;
                //     }
                //     let mic_amplitude_clone = mic_amplitude.0.clone();
                //     *mic_amplitude_clone.lock().unwrap() = 0.0;
                // }
            }
            Interaction::Hovered => {
                if recording_state.is_recording {
                    **text = "Stop".to_string();
                } else {
                    **text = "Record".to_string();
                }
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                if recording_state.is_recording {
                    **text = "Stop".to_string();
                } else {
                    **text = "Record".to_string();
                }
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn ui_system_startup(mut commands: Commands, assets: Res<AssetServer>) {
    // ui camera
    commands.spawn(button(&assets)).observe(on_click_spawn_cube);

    // Volume bar
    commands.spawn((
        Node {
            width: Val::Px(100.0), // start at 0, will be updated
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
        VolumeBar,
    ));
}

fn button(asset_server: &AssetServer) -> impl Bundle + use<> {
    (
        Node {
            width: Val::Px(100.0),
            height: Val::Px(100.0),

            position_type: PositionType::Absolute,
            top: Val::Px(2.0),
            right: Val::Px(2.0),

            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Record"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}

fn on_click_spawn_cube(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut num: Local<usize>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
        ))
        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
        .observe(on_drag_rotate);
    *num += 1;
}

fn on_drag_rotate(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(drag.target()) {
        transform.rotate_y(drag.delta.x * 0.02);
        transform.rotate_x(drag.delta.y * 0.02);
    }
}

pub fn update_volume_bar_ui(
    mic_amplitude: Res<MicAmplitude>,
    mut query: Query<&mut Node, With<VolumeBar>>,
    recording_state: Res<RecordingState>,
) {
    let amplitude = *mic_amplitude.0.lock().unwrap();
    for mut node in &mut query {
        if recording_state.is_recording {
            let width = (amplitude * 300.0).clamp(0.0, 300.0); // scale for visibility
            node.width = Val::Px(width);
        } else {
            node.width = Val::Px(0.0);
        }
    }
}

// pub fn mic_setup(
//     mut commands: Commands,
//     audio_output: ResMut<crate::bevy_mic::audio_output::AudioOutput>,
// ) {
//     commands.spawn(audio_output.new_sink().expect("Unable to spawn audio sink"));
// }
pub fn mic_update(
    // sink: Single<&mut bevy_mic::spatial_audio::SpatialAudioSink>,
    mic: Res<crate::bevy_mic::microphone::MicrophoneAudio>,
    mut query: Single<&mut Node, With<VolumeBar>>,
) {
    info!("mic count: {}", mic.len());

    for owo in mic.try_iter() {
        let max = owo.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        // info!("{}", max);
        query.width = Val::Px(100.0 * max);

        // sink.append(rodio::buffer::SamplesBuffer::new(
        //     mic.config.channels,
        //     mic.config.sample_rate,
        //     owo,
        // ));
    }
}
