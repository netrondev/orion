use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes::tailwind,
    input::mouse::AccumulatedMouseMotion,
    input::mouse::MouseButton,
    input::ButtonInput,
    prelude::*,
    render::view::RenderLayers,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{game::environment::EnvironmentPlugin, midi::piano::Piano};

pub fn start() {
    App::new()
        .add_plugins((DefaultPlugins, EnvironmentPlugin))
        .add_systems(
            Startup,
            (
                super::gizmo::create_gizmo,
                spawn_view_model,
                // spawn_world_model,
                spawn_lights,
                spawn_text,
                crate::midi::game::spawn_midi_world,
                crate::midi::bevy_resource::start_midi_listener,
                // capture_mouse_on_startup,
            ),
        )
        .add_systems(
            Update,
            (
                move_player,
                move_player_wasd,
                change_fov,
                release_mouse_on_esc,
                relock_mouse_on_click,
                Piano::animate,
                Piano::animate_midi,
            ),
        )
        .run();
}

#[derive(Debug, Component)]
struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            // These factors are just arbitrary mouse sensitivity values.
            // It's often nicer to have a faster horizontal sensitivity than vertical.
            // We use a component for them so that we can make them user-configurable at runtime
            // for accessibility reasons.
            // It also allows you to inspect them in an editor if you `Reflect` the component.
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Used implicitly by all entities without a `RenderLayers` component.
/// Our world model camera and all objects other than the player are on this layer.
/// The light source belongs to both layers.
pub const DEFAULT_RENDER_LAYER: usize = 0;

/// Used by the view model camera and the player's arm.
/// The light source belongs to both layers.
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    // let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    // piano
    // let cube = Cuboid::new(1.0, 1.0, 1.0);

    let player = commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
        ))
        .id();

    commands.spawn((
        WorldModelCamera,
        Camera3d::default(),
        DistanceFog {
            color: Color::srgb(0.05, 0.05, 0.05),
            falloff: FogFalloff::Exponential { density: 0.1 },
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        ChildOf(player),
    ));

    // create_gizmo(commands, meshes, materials);

    // Spawn view model camera.
    commands.spawn((
        Camera3d::default(),
        Camera {
            // Bump the order to render on top of the world model.
            order: 1,
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        // Only render objects belonging to the view model.
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        ChildOf(player),
    ));

    Piano::new(15).spawn(commands, meshes, materials, player);
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::NEUTRAL_100),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Move the camera with your mouse.\n",
            "Connect your midi controller",
        )));
}

fn move_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn move_player_wasd(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyS) {
        direction.z -= 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.z += 1.0;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if direction != Vec3::ZERO {
        let transform = player.as_mut();
        // Move relative to the player's facing direction (yaw only)
        let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        let forward = Quat::from_rotation_y(yaw) * Vec3::NEG_Z;
        let right = Quat::from_rotation_y(yaw) * Vec3::X;
        let move_dir = (forward * direction.z + right * direction.x).normalize_or_zero();
        let speed = 3.0; // units per second
        transform.translation += move_dir * speed * time.delta_secs();
    }
}

fn change_fov(
    input: Res<ButtonInput<KeyCode>>,
    mut world_model_projection: Single<&mut Projection, With<WorldModelCamera>>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Perspective`"
        );
    };

    if input.pressed(KeyCode::ArrowUp) {
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());
    }
    if input.pressed(KeyCode::ArrowDown) {
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());
    }
}

fn capture_mouse_on_startup(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn release_mouse_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.single_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

fn relock_mouse_on_click(
    input: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if input.just_pressed(MouseButton::Left) {
        if let Ok(mut window) = windows.single_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    }
}
